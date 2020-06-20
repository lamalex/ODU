use std::env;

use cputemps::parser;

fn help() {
    let exe = match env::current_exe() {
        Ok(exe) => exe.to_str().unwrap().to_owned(),
        Err(_e) => String::from("(unidentified-executable)"),
    };

    println!("Hello. Thank you for trying to interpolate your cpu temperature data.");
    println!("I regret to inform you that you must pass at least 1 file path to this program.");
    println!("Try something like this:");
    println!("\t{} <temperatures_1.txt> ...", exe);
    println!("\t🖖🏽 Live long and interpolate");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        std::process::exit(0x69);
    }

    for data_file_path in &args[1..] {
        match parser::Parser::new(&data_file_path[..]) {
            Ok(p) => {}
            Err(e) => panic!(e.to_string()),
        }
    }
}
