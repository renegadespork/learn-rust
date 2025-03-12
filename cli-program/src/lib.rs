pub mod tests;

pub mod arguments {
    use std::{env, process::exit};
    
    pub enum CMD {
        Read,
        Append,
        Replace,
        Search
    }
    pub struct Command {
        pub cmd: CMD,
        pub filepath: String,
        pub text1: String,
        pub text2: String
    }

fn check_arg_presence(arg_needed: u8, arg_num: usize) -> bool {
    let arg_num:u8 = arg_num.try_into().unwrap();
    if arg_num < arg_needed {
        println!("Missing arguments.");
        false
    }
    else if arg_num > arg_needed {
        println!("Too many agruments.");
        false
    }
    else {true}
}

    impl Command {
        pub fn parse_arguments() -> Command {
            let args: Vec<String> = env::args().collect();
            let arg_num = args.len();
            let cmd = &args[1];
            match cmd.as_str() {
                "read" => {
                    let arg_needed: u8 = 3;
                    if check_arg_presence(arg_needed, arg_num) {
                    }
                    else {
                        println!("exiting");
                        exit(1)
                    }
                    let command = Command {
                        cmd: CMD::Read,
                        filepath: String::from(&args[2]),
                        text1: String::new(),
                        text2: String::new()
                    };
                    command
                },
                "append" => {
                    let arg_needed: u8 = 4;
                    if check_arg_presence(arg_needed, arg_num) {
                    }
                    else {
                        println!("exiting");
                        exit(1)
                    }
                    let command = Command {
                        cmd: CMD::Append,
                        filepath: String::from(&args[2]),
                        text1: String::from(&args[3]),
                        text2: String::new()
                    };
                    command
                },
                "replace" => {
                    let arg_needed: u8 = 5;
                    if check_arg_presence(arg_needed, arg_num) {
                    }
                    else {
                        println!("exiting");
                        exit(1)
                    }
                    let command = Command {
                        cmd: CMD::Replace,
                        filepath: String::from(&args[2]),
                        text1: String::from(&args[3]),
                        text2: String::from(&args[4])
                    };
                    command
                },
                "search" => {
                    let arg_needed: u8 = 4;
                    if check_arg_presence(arg_needed, arg_num) {
                    }
                    else {
                        println!("exiting");
                        exit(1)
                    }
                    let command = Command {
                        cmd: CMD::Read,
                        filepath: String::from(&args[2]),
                        text1: String::from(&args[3]),
                        text2: String::new()
                    };
                    command
                },
                _ => {
                    println!("Invalid command. Please choose the following:");
                    let command = Command {
                        cmd: CMD::Read,
                        filepath: String::new(),
                        text1: String::new(),
                        text2: String::new()
                    };
                    command
                }
            }
        }   
    }
}

pub mod read {
    use crate::arguments::Command;
    use std::fs;

    impl Command {
        pub fn read_file(&self) -> String {
            let contents = fs::read_to_string(&self.filepath);
            match contents {
                Ok(c) => {
                    c
                },
                Err(_) => {
                    println!("Error reading {}.\n", self.filepath);
                    self.filepath.clone()
                },
            }
        }
        pub fn search(search: String) {
        }
    }
}

pub mod edit {
    use std::fs;
    use crate::{arguments::Command, read};
    impl Command {
        pub fn append_to_file(&self) {
            let contents = self.read_file();
            let contents = format!("{}\n{}", contents, self.text1);
            let result = fs::write(&self.filepath, contents);
            match result {
                Ok(_r) => {
                    println!("Successfully appended {} to {}.", self.text1, self.filepath)
                },
                Err(e) => {
                    println!("Error writing to {}: {}", self.filepath, e)
                }
            }
        }
    }
    impl Command {
        pub fn replace(&self) {

        }
    }
}