#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_json;
extern crate slog_term;

use actix_cors::Cors;
use actix_web::{middleware, web, Responder};
use slog::info;

use crate::state::AppState;

mod api;
pub mod lib;
mod model;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("/Users/liuyang/Documents/sjtu/2023autumn/library/git_repo/rust-server/src/data/config/app.toml"))
        .unwrap();

    let port = settings.get::<String>("app.port").unwrap();

    // log
    let logger = lib::log::get_logger();
    info!(
        logger,
        "==> 🚀 {} listening at {}",
        settings.get::<String>("app.name").unwrap(),
        settings.get::<String>("app.port").unwrap()
    );

    // database
    let db_pool = lib::db::conn(&settings).await;
    use actix_web::{App, HttpServer};
    HttpServer::new(move || {
        let cors = Cors::permissive();
        println!(
            "==> 🚀 {} listening at {}",
            settings.get::<String>("app.name").unwrap(),
            settings.get::<String>("app.port").unwrap()
        );
        App::new()
            .app_data(web::Data::new(AppState {
                config: settings.clone(),
                log: logger.clone(),
                db: db_pool.clone(),
            }))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::scope("/hello").configure(api::hello::route))
            .service(web::scope("/movie").configure(api::movie::controller::route))
            .service(
                web::scope("/authorizations").configure(api::authorizations::controller::route),
            )
            .service(web::scope("/user").configure(api::user::controller::route))
            .service(web::scope("/comment").configure(api::comment::controller::route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
