use cputemps::pairs::Pairs;
use cputemps::parser;
use launearalg::{interpolater, traits::Interpolate};

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
    println!("\t🖖🏽 Live long and interpolate");
}

fn main() {
    const STEP_SIZE: f64 = 30.0;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        std::process::exit(0x69);
    }

    for data_file_path in &args[1..] {
        match parser::Parser::new(&data_file_path[..]) {
            Ok(p) => {
                for (i, lines) in p.iter().pairs().enumerate() {
                    let step = i as f64;
                    let x1 = step * STEP_SIZE;
                    let x2 = (step + 1.0) * STEP_SIZE;

                    let interp: Vec<Vec<f64>> = lines
                        .0
                        .iter()
                        .zip(lines.1.iter())
                        .filter_map(|fs| {
                            interpolater::LinearPiecewiseInterpolater::interpolate(vec![
                                (x1, *fs.0),
                                (x2, *fs.1),
                            ])
                        })
                        .collect();

                    println!(
                        "{} <= {}; y_0 = {:.4} + {:.4}x; interpolation",
                        x1, x2, interp[0][0], interp[0][1]
                    );
                }
            }
            Err(e) => panic!(e.to_string()),
        }
    }
}
