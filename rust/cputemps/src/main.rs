use std::env;

use cputemps::{pairs::Pairs, parser::Parser, writer::Writer};
use launearalg::{
    interpolater,
    matrix::Matrix,
    solver::gauss,
    traits::{Augment, Interpolate, Transpose},
};

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
    println!("Output data can be found alongside input data with -out-core-#.txt appended.");
    println!("🖖🏽 Live long and interpolate");
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        std::process::exit(0x69)
    }

    args[1..]
        .iter()
        .map(|data_file_path| match Parser::new(&data_file_path[..]) {
            Ok(p) => process_data_single_pass_all_cores(p),
            Err(_e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not parse",
            )),
        })
        .collect::<Result<(), _>>()
}

// In single pass -> read 2 lines of file and computer linear interpolation.
// Save one line and build matrix from which to compute global least squares
// Takes 1 pass through each file and computes while reading.
// Writes file next to input with -out-core-#.txt appended.
fn process_data_single_pass_all_cores(parser: Parser) -> Result<(), std::io::Error> {
    let mut writer = Writer::new(&parser.file_path[..], parser.cores)?;

    let mut proto_x = vec![vec![]; parser.cores];
    let mut proto_y = vec![vec![]; parser.cores];

    let data_pairs = parser.iter().pairs().map(|line_pair| {
        let first_line = line_pair.0;
        first_line
            .iter()
            .zip(line_pair.1.iter())
            .map(|z| (*z.0, *z.1))
            // line_pair goes out of scope so this needs collected
            .collect::<Vec<(f64, f64)>>()
    });

    for (i, pairs) in data_pairs.enumerate() {
        for (core, core_data_endpoints) in pairs.iter().enumerate() {
            let step = i as f64;
            let x1 = step * STEP_SIZE;
            let x2 = (step + 1.0) * STEP_SIZE;

            let res = interpolater::LinearPiecewiseInterpolater::interpolate(vec![
                (x1, core_data_endpoints.0),
                (x2, core_data_endpoints.1),
            ]);

            let x_row = vec![1.0, x1];
            let y_row = vec![core_data_endpoints.0];
            proto_x[core].push(x_row);
            proto_y[core].push(y_row);

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

    for core in 0..parser.cores {
        let core_x = Matrix::from(proto_x[core].clone());
        let core_y = Matrix::from(proto_y[core].clone());

        let core_xt = core_x.transpose();
        let core_xtx = &core_xt * &core_x;
        let core_xty = &core_xt * &core_y;

        let core_xtxxty = core_xtx.augment(&core_xty);

        let glsa = gauss::solve(core_xtxxty);
        writer.write(core, std::format!("{:16}phi_hat = {}", "", glsa));
    }

    Ok(())
}
