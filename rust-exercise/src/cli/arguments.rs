use std::env;

pub trait ParseArg {
    fn parse(&mut self) -> Result<Config, ReadEnvErr>;
    fn add_argument(&mut self, arg: String) -> Result<(), AddArgumentErr>;
}

#[derive(Debug)]
pub struct Config {
    input: String,
    word: String,
    output: String,
}

#[derive(Debug)]
pub struct AddArgumentErr(String);

#[derive(Debug)]
pub struct ReadEnvErr(String);

#[derive(Debug)]
pub struct ArgParser {
    arguments: Vec<String>,
}

impl Config {
    pub fn new(output: String) -> Config {
        Config { input: String::from(""), word: String::from(""), output }
    }
    pub fn get_input(&self) -> &String {
        &self.input
    }
    pub fn get_word(&self) -> &String {
        &self.word
    }
    pub fn get_output(&self) -> &String {
        &self.output
    }
}

impl AddArgumentErr {
    pub fn new(msg: String) -> AddArgumentErr {
        AddArgumentErr(msg)
    }
    pub fn get_message(&self) -> &str {
        &self.0
    }
}

impl ReadEnvErr {
    pub fn new(msg: String) -> ReadEnvErr {
        ReadEnvErr(msg)
    }
    pub fn get_message(&self) -> &str {
        &self.0
    }
}


impl ArgParser {
    pub fn new() -> ArgParser {
        ArgParser { arguments: Vec::new() }
    }
}


impl ParseArg for ArgParser {
    fn parse(&mut self) -> Result<Config, ReadEnvErr> {
        let output_file = match env::var("OUTPUT_FILE") {
            Ok(value) => {
                value
            }
            _ => return Err(ReadEnvErr::new(String::from("无法读取环境变量"))),
        };
        let mut conf: Config = Config::new(output_file);
        for (index, arg) in self.arguments.iter().enumerate() {
            match index {
                0 => {
                    conf.input = String::from(arg);
                }
                1 => {
                    conf.word = String::from(arg);
                }
                2 => {
                    conf.output = String::from(arg);
                }
                _ => {}
            }
        }
        Ok(conf)
    }

    fn add_argument(&mut self, arg: String) -> Result<(), AddArgumentErr> {
        let mut args = arg.split_whitespace();
        for _ in 0..2 {
            match args.next() {
                None => {
                    return Err(AddArgumentErr::new(String::from("at least two params")));
                }
                Some(param) => {
                    if param == "--output" {
                        return Err(AddArgumentErr::new(String::from("at least tow params")));
                    }
                    self.arguments.push(String::from(param));
                }
            }
        }
        while let Some(arg) = args.next() {
            match arg {
                "--output" => {
                    self.arguments.push(String::from(args.next().unwrap()));
                }
                _ => return Err(AddArgumentErr::new(String::from("undefined param")))
            }
        }

        Ok(())
    }
}