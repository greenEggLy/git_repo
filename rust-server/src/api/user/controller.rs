use actix_web::dev::ConnectionInfo;
use actix_web::{get, post, put, web, FromRequest, HttpRequest, HttpResponse};
use chrono::prelude::*;
use serde::Deserialize;

use crate::api::authorizations;
use crate::api::user::service::get_user_info_by_id;
use crate::api::user::{service, User};
use crate::lib::{auth, client, error, validator};
use crate::AppState;

pub fn route(cfg: &mut web::ServiceConfig) {
    // cfg.service(get_info);
    cfg.service(change_password);
    cfg.service(register);
    cfg.service(get_userinfo_id);
}

// #[derive(Deserialize)]
// pub struct RegisterReq {
//     username: String,
//     password: String,
//     email: Option<String>,
// }

#[post("/register")]
pub async fn register(
    req_info: web::Json<User>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let username = req_info.username.clone().expect("需要用户名");
    req_info.password.clone().expect("需要密码");
    match service::get_by_username(&username, &state).await? {
        Some(_v) => return Err(error::new(114514, "用户已存在", 422)),
        _ => {}
    };
    service::insert(&req_info, &state).await?;
    return Ok(HttpResponse::Ok().finish());
}

// #[get("/")]
// pub async fn get_info(
//     req: HttpRequest,
//     state: web::Data<AppState>,
// ) -> Result<HttpResponse, error::Error> {
//     let auth_info = auth::verify("ROLE_MEMBER", &req, &state).await?;
//
//     let user_data = match service::get_user_info_by_id(auth_info.id, &state).await? {
//         None => return Err(error::new(400007, "无法获得用户信息", 422)),
//         Some(v) => v,
//     };
//
//     Ok(HttpResponse::Ok().json(user_data))
// }

#[get("/{id}")]
pub async fn get_userinfo_id(
    path: web::Path<u64>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let user_data = match get_user_info_by_id(path.into_inner(), &state).await? {
        None => return Err(error::new(400007, "无法获得用户信息", 422)),
        Some(v) => v,
    };
    Ok(HttpResponse::Ok().json(user_data))
}

#[derive(Deserialize)]
pub struct ChangePasswordReqJson {
    old_password: Option<String>,
    new_password: Option<String>,
    confirm_password: Option<String>,
}

#[put("/password")]
pub async fn change_password(
    req_info: web::Json<ChangePasswordReqJson>,
    req: HttpRequest,
    state: web::Data<AppState>,
    conn: ConnectionInfo,
) -> Result<HttpResponse, error::Error> {
    let auth_info = auth::verify("ROLE_MEMBER", &req, &state).await?;

    let old_password = validator::required_str(&req_info.old_password, "原密码")?;
    let new_password = validator::required_str(&req_info.new_password, "新密码")?;
    let confirm_password = validator::required_str(&req_info.confirm_password, "确认密码")?;

    if new_password != confirm_password {
        return Err(error::new(100301, "新密码和确认密码不一致", 422));
    }

    let user_data = match service::get_by_id(auth_info.id, &state).await? {
        None => return Err(error::new(400007, "无法获得用户信息", 422)),
        Some(v) => v,
    };

    // 用户被删除
    match user_data.is_del {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => {
            if v != 0 {
                return Err(error::new(100403, "Authentication failure", 401));
            }
        }
    };

    // 用户被禁用
    match user_data.is_enabled {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => {
            if v != 1 {
                return Err(error::new(100403, "Authentication failure", 401));
            }
        }
    };

    let old_salt = match user_data.salt {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    let old_password_store = match user_data.password {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    let old_pwd = auth::crypt_password(&old_password, &old_salt);
    if old_pwd != old_password_store {
        return Err(error::new(100407, "原密码错误", 422));
    }

    let salt = auth::salt();
    let pwd = auth::crypt_password(&new_password, &salt);

    let mut user = User::new();
    user.id = Some(auth_info.id);
    user.password = Some(pwd);
    user.salt = Some(salt);
    user.update_time = Some(Utc::now());

    let client = client::get_client_info(&state, &req, &conn);

    service::update(&user, &state).await?;
    authorizations::service::insert_log(5, "", auth_info.id, 0, &client, &state).await?;

    Ok(HttpResponse::Ok().body(""))
}
