use std::io;
use std::io::Write;

use crate::cli::search::ReadFile;

#[derive(Debug)]
pub struct WriteFileErr(String);

impl WriteFileErr {
    pub fn new(msg: String) -> WriteFileErr {
        WriteFileErr(msg)
    }
    pub fn get_message(&self) -> &str {
        &self.0
    }
}


pub fn write_file(output_file: &String, frequency: i32) -> Result<(), WriteFileErr> {
    let file = std::fs::File::create(output_file);
    match file {
        Ok(mut file) => {
            let res = file.write_all(frequency.to_string().as_ref());
            match res {
                Ok(_) => {}
                Err(_) => {
                    return Err(WriteFileErr::new(String::from("无法写入文件")));
                }
            }
        }
        Err(_) => {
            return Err(WriteFileErr::new(String::from("无法创建文件")));
        }
    }
    Ok(())
}

pub fn write_file_mul(output_file: &String, files: ReadFile) -> Result<(), WriteFileErr> {
    if output_file == "STDOUT" {
        let stdout = io::stdout();
        let mut file = stdout.lock();
        for (index, data) in files.get_data().iter().enumerate() {
            let str = format!("{}. in file {}, {}'s frequency: {}\n", index, data.0, data.1, data.2);
            let res = file.write_all(str.as_ref());
            match res {
                Ok(_) => {}
                Err(_) => {
                    return Err(WriteFileErr::new(String::from("无法写入文件")));
                }
            }
        }
        return Ok(());
    }
    let file = std::fs::File::create(output_file);
    match file {
        Ok(mut file) => {
            for (index, data) in files.get_data().iter().enumerate() {
                let str = format!("{}. in file {}, {}'s frequency: {}\n", index, data.0, data.1, data.2);
                let res = file.write_all(str.as_ref());
                match res {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(WriteFileErr::new(String::from("无法写入文件")));
                    }
                }
            }
        }
        Err(_) => {
            return Err(WriteFileErr::new(String::from("无法创建文件")));
        }
    }

    return Ok(());
}