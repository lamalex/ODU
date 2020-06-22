use cputemps::processor::{Processor, ProcessorError};
use rayon::prelude::*;
use std::env;

fn help() {
    let exe = match env::current_exe() {
        Ok(exe) => exe.to_str().unwrap().to_owned(),
        Err(_e) => String::from("(unidentified-executable)"),
    };

    println!("Hello. Thank you for trying to interpolate your cpu temperature data.");
    println!("I regret to inform you that you must pass at least 1 file path to this program.");
    println!("Try something like this:");
    println!("\t{} <temperatures_1.txt> ...", exe);
    println!("Output data can be found alongside input data with -out-core-#.txt appended.");
    println!("🖖🏽 Live long and interpolate");
}

fn main() -> Result<(), ProcessorError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        std::process::exit(0x69)
    }

    args[1..]
        .par_iter()
        .map(|data_file_path| Processor::process_data_file(&data_file_path[..]))
        .collect()
}
