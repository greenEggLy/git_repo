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
    let mut file = std::fs::File::create(output_file);
    match file {
        Ok(_) => {}
        Err(_) => {
            return Err(WriteFileErr::new(String::from("无法创建文件")));
        }
    }
    let res = file.write_all(frequency.to_string().as_ref());
    match res {
        Ok(_) => {}
        Err(_) => {
            return Err(WriteFileErr::new(String::from("无法写入文件")));
        }
    }
    Ok(())
}

pub fn write_file_mul(output_file: &String, files: Vec<ReadFile>) -> Result<(), WriteFileErr> {
    let mut file = std::fs::File::create(output_file);
    for (index, read_file) in files.iter().enumerate() {
        let str = format!("{}. in file{}, {}'s frequency: {}\n", index, read_file.get_path(), read_file.get_word(), read_file.get_frequency());
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