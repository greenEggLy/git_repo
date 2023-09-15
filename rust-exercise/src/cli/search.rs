use std::io::Read;

use crate::cli::arguments::Config;

pub trait ReadFileAndCount {
    fn read_conf(&mut self, conf: &Config) -> Result<(), String>;
    // fn frequency(&mut self, path: &String) -> Result<(), String>;
}

pub struct ReadFile {
    // path word frequency
    data: Vec<(String, String, i32)>,
}

impl ReadFile {
    pub fn new() -> ReadFile {
        ReadFile {
            data: Vec::new()
        }
    }
    pub fn get_data(&self) -> &Vec<(String, String, i32)> {
        &self.data
    }
}


impl ReadFileAndCount for ReadFile {
    fn read_conf(&mut self, conf: &Config) -> Result<(), String> {
        for path in conf.get_input().iter() {
            let mut file = std::fs::File::open(path).expect("无法打开文件");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("无法读取文件");
            for line in contents.lines() {
                for word in line.split_whitespace() {
                    if !conf.get_word().contains(&String::from(word)) {
                        continue;
                    }
                    let mut flag = false;
                    for (_index, data) in self.data.iter_mut().enumerate() {
                        if data.0 == *path && data.1 == word {
                            data.2 += 1;
                            flag = true;
                            break;
                        }
                    }
                    if !flag {
                        self.data.push((path.clone(), word.to_string(), 1));
                    }
                }
            }
        }
        Ok(())
    }
}