use cputemps::{pairs::Pairs, parser::Parser, writer::Writer, ProtoMatrix};
use launearalg::{
    interpolater::{linear_piecewise::LinearPiecewiseInterpolater, traits::Interpolate},
    matrix::Matrix,
    solver::gauss,
    traits::{Augment, Solution, Transpose},
};
use std::env;

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

    // TODO drop in rayon for parallelism
    args[1..]
        .iter()
        .map(|data_file_path| match Parser::new(&data_file_path[..]) {
            Ok(p) => process_all_cores_single_pass(p),
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
fn process_all_cores_single_pass(parser: Parser) -> Result<(), std::io::Error> {
    let mut writer = Writer::new(&parser.file_path[..], parser.cores)?;

    let (data_x, data_y) = process_pairwise(parser, &mut writer);
    process_full_dataset(data_x, data_y, &mut writer);
    Ok(())
}

fn process_pairwise(parser: Parser, writer: &mut Writer) -> (ProtoMatrix, ProtoMatrix) {
    let mut proto_x = vec![vec![]; parser.cores];
    let mut proto_y = vec![vec![]; parser.cores];

    let data_pairs = parser.iter().pairs().map(|line_pair| {
        line_pair
            .0
            .iter()
            .zip(line_pair.1.iter())
            .map(|z| (*z.0, *z.1))
            .collect::<Vec<(f64, f64)>>()
    });

    for (i, pairs) in data_pairs.enumerate() {
        let step = i as f64;
        let x1 = step * STEP_SIZE;
        let x2 = (step + 1.0) * STEP_SIZE;

        for (core, core_data_endpoints) in pairs.iter().enumerate() {
            let sol = LinearPiecewiseInterpolater::interpolate(vec![
                (x1, core_data_endpoints.0),
                (x2, core_data_endpoints.1),
            ]);

            match sol {
                Some(sol) => {
                    let lhs = format!("{}_{}", sol.lhs(), i);
                    writer.write_pairwise(core, (x1, x2), &lhs[..], sol)
                }
                None => continue,
            }

            // Iteratively build matrices for processors which need full dataset
            let x_row = vec![1.0, x1];
            let y_row = vec![core_data_endpoints.0];
            proto_x[core].push(x_row);
            proto_y[core].push(y_row);
        }
    }

    (proto_x, proto_y)
}

fn process_full_dataset(data_x: ProtoMatrix, data_y: ProtoMatrix, writer: &mut Writer) {
    // TODO drop in rayon for parallelism
    let cores = data_x.len();

    for core in 0..cores {
        let core_x = Matrix::from(data_x[core].clone());
        let core_y = Matrix::from(data_y[core].clone());

        let core_xt = core_x.transpose();
        let core_xtx = &core_xt * &core_x;
        let core_xty = &core_xt * &core_y;

        let core_xtxxty = core_xtx.augment(&core_xty);

        let glsa = gauss::solve(core_xtxxty);
        writer.write_global(core, glsa.lhs(), glsa);
    }
}
