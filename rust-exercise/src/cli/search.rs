use std::io::Read;

use crate::cli::arguments::Config;

pub trait ReadFileAndCount {
    fn read_conf(&mut self, conf: &Config) -> Result<(), String>;
    fn frequency(&self, word: &String) -> Result<i32, String>;
}

pub struct ReadFile {
    path: String,
    word: String,
    frequency: i32,
}

impl ReadFile {
    pub fn new() -> ReadFile {
        ReadFile {
            path: String::from(""),
            word: String::from(""),
            frequency: 0,
        }
    }

    pub fn get_frequency(&self) -> i32 {
        self.frequency
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }
}


impl ReadFileAndCount for ReadFile {
    fn read_conf(&mut self, conf: &Config) -> Result<(), String> {
        self.path = conf.get_input().clone();
        self.word = conf.get_word().clone();
        if self.path == "" {
            return Err(String::from("输入文件路径为空"));
        }
        if self.word == "" {
            return Err(String::from("输入单词为空"));
        }
        Ok(())
    }

    fn frequency(&self, word: &String) -> Result<i32, String> {
        let mut file = std::fs::File::open(&self.path).expect("无法打开文件");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("无法读取文件");
        let mut count = 0;
        for line in contents.lines() {
            for w in line.split_whitespace() {
                if w == word {
                    count += 1;
                }
            }
        }
        Ok(count)
    }
}