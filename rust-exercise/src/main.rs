#![feature(test)]
extern crate clap;
extern crate test;


use std::{env, io};

use crate::cli::arguments;
use crate::cli::arguments::ParseArg;
use crate::cli::search::{ReadFile, ReadFileAndCount};
use crate::cli::writer::write_file_mul;

mod cli {
    pub mod arguments;
    pub mod search;
    pub mod writer;
}

fn main() {
    // set env
    let key = "OUTPUT_FILE";
    let value = "src/output.txt";
    env::set_var(key, value);
    // get input
    let mut input = String::new();
    let mut arg_tool: arguments::ArgParser = arguments::ArgParser::new();

    io::stdin()
        .read_line(&mut input)
        .expect("无法读取输入");
    let res = arg_tool.add_argument(String::from(input));
    match res {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{:?}", e.get_message());
            return;
        }
    }
    // parse input
    let conf = arg_tool.parse().expect("无法解析输入");
    eprintln!("{:?}", conf);


    let mut my_file = ReadFile::new();
    let res = my_file.read_conf(&conf);
    match res {
        Ok(_) => {
            write_file_mul(&conf.get_output(), my_file).expect("无法写入文件");
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    }
}
