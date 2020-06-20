use std::env;

use cputemps::{pairs::Pairs, parser::Parser, writer::Writer};
use launearalg::{interpolater, traits::Interpolate};

const STEP_SIZE: f64 = 30.0;

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

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    println!("ARGS: {:?}", args);
    if args.len() < 2 {
        help();
        std::process::exit(0x69)
    }

    for data_file_path in &args[1..] {
        match Parser::new(&data_file_path[..]) {
            Ok(p) => {
                let mut writer = Writer::new(&data_file_path[..], p.cores)?;
                for (i, pairs) in p
                    .iter()
                    .pairs()
                    .map(|line_pair| {
                        line_pair
                            .0
                            .iter()
                            .zip(line_pair.1.iter())
                            .map(|z| (*z.0, *z.1))
                            .collect::<Vec<(f64, f64)>>()
                    })
                    .enumerate()
                {
                    for (core, core_data_endpoints) in pairs.iter().enumerate() {
                        let step = i as f64;
                        let x1 = step * STEP_SIZE;
                        let x2 = (step + 1.0) * STEP_SIZE;

                        let res = interpolater::LinearPiecewiseInterpolater::interpolate(vec![
                            (x1, core_data_endpoints.0),
                            (x2, core_data_endpoints.1),
                        ]);

                        match res {
                            Some(res) => writer.write(
                                core,
                                std::format!(
                                    "{:6} <= {:6}; {:5} = {}",
                                    x1,
                                    x2,
                                    std::format!("y_{}", i),
                                    res
                                ),
                            ),
                            None => continue,
                        }
                    }
                }
            }
            Err(e) => panic!(e.to_string()),
        }
    }

    Ok(())
}
