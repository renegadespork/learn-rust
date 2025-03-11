use std::env;
use std::fs;
use core::error;

struct Arguments {
    filepath: String,
    search: String
}

fn main() {
    read_file(parse_arguments().filepath);
}

fn parse_arguments() -> Arguments {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    if argnum < 3 {
            panic!("Missing arguments.");
        }
    else {}
    let filepath = &args[1];
    let search = &args[2];
    let arguments = Arguments {
        filepath: String::from(filepath),
        search: String::from(search),
    };
    arguments
}

fn read_file(filepath: String) {
    let contents = fs::read_to_string(&filepath);
    match contents {
        Ok(c) => {
            println!("{c}");
        },
        Err(_) => {
            println!("Error reading {}.\n", &filepath);
        },
    }
}