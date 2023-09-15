use std::env;

pub trait ParseArg {
    fn parse(&mut self) -> Result<Config, ReadEnvErr>;
    fn add_argument(&mut self, arg: String) -> Result<(), AddArgumentErr>;
}

#[derive(Debug)]
pub struct Config {
    inputs: Vec<String>,
    words: Vec<String>,
    output: String,
}

#[derive(Debug)]
pub struct AddArgumentErr(String);

#[derive(Debug)]
pub struct ReadEnvErr(String);

#[derive(Debug)]
pub struct ArgParser {
    input_args: Vec<String>,
    word_args: Vec<String>,
    output_arg: String,
}

impl Config {
    pub fn new(output: String) -> Config {
        Config { inputs: Vec::new(), words: Vec::new(), output }
    }
    pub fn get_input(&self) -> &Vec<String> {
        &self.inputs
    }
    pub fn get_word(&self) -> &Vec<String> {
        &self.words
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
        ArgParser { input_args: Vec::new(), word_args: Vec::new(), output_arg: String::from("") }
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
        for input in self.input_args.iter() {
            conf.inputs.push(String::from(input));
        }
        for word in self.word_args.iter() {
            conf.words.push(String::from(word));
        }
        if self.output_arg != "" {
            conf.output = String::from(&self.output_arg);
        }
        Ok(conf)
    }

    fn add_argument(&mut self, arg: String) -> Result<(), AddArgumentErr> {
        let mut args = arg.split_whitespace();
        let mut arg = args.next();
        let err_msg = String::from("input: --input <file>... --word <word>... --output <file>");
        // match --input
        match arg {
            None => {
                return Err(AddArgumentErr::new(err_msg));
            }
            Some(param) => {
                if param != "--input" {
                    return Err(AddArgumentErr::new(err_msg));
                }
            }
        }
        // read input_files
        loop {
            match args.next() {
                None => {
                    return Err(AddArgumentErr::new(err_msg));
                }
                Some(param) => {
                    let prefix = &param[..2];
                    if prefix == "--" {
                        if param == "--word" {
                            break;
                        } else {
                            return Err(AddArgumentErr::new(err_msg));
                        }
                    }
                    self.input_args.push(String::from(param));
                }
            }
        }
        // read words
        let mut output_param = false;
        loop {
            match args.next() {
                None => {
                    break;
                }
                Some(param) => {
                    let prefix = &param[..2];
                    if prefix == "--" {
                        if param == "--output" {
                            output_param = true;
                            break;
                        } else {
                            return Err(AddArgumentErr::new(err_msg));
                        }
                    }
                    self.word_args.push(String::from(param));
                }
            }
        }
        // read output
        if output_param {
            let output = args.next();
            match output {
                None => {
                    self.output_arg = String::from("STDOUT");
                }
                Some(output) => {
                    self.output_arg = String::from(output);
                }
            }
            if args.next().is_some() {
                return Err(AddArgumentErr::new(String::from("cannot receive more than one output")));
            }
        }
        // check input and word count
        if self.input_args.len() == 0 || self.word_args.len() == 0 {
            return Err(AddArgumentErr::new(String::from("cannot receive empty input or word!")));
        }
        Ok(())
    }
}