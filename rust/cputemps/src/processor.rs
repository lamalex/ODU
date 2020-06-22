use crate::{pairs::Pairs, parser::Parser, writer::Writer, ProtoMatrix};
use launearalg::{
    interpolater::{linear_piecewise::LinearPiecewiseInterpolater, traits::Interpolate},
    matrix::Matrix,
    solver::{gauss, gauss::GaussianEliminationSolution},
    traits::{Augment, Solution, Transpose},
};
use rayon::prelude::*;
use std::fmt;
use std::path::Path;

const STEP_SIZE: f64 = 30.0;

#[derive(Debug)]
pub enum ProcessorError {
    IOError,
    Error,
}

impl std::error::Error for ProcessorError {}
impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            &ProcessorError::IOError => write!(f, "Encountered a filesystem error"),
            &ProcessorError::Error => write!(f, "An error occurred while processing"),
        }
    }
}

pub struct Processor;
impl Processor {
    pub fn process_data_file(
        file_path: &str,
        output_path: Option<&str>,
    ) -> Result<(), ProcessorError> {
        match Parser::new(file_path) {
            Ok(p) => process_all_cores_single_pass(p, output_path),
            Err(_e) => Err(ProcessorError::IOError),
        }
    }
}

// In single pass -> read 2 lines of file and computer linear interpolation.
// Save one line and build matrix from which to compute global least squares
// Takes 1 pass through each file and computes while reading.
// Writes file next to input with -out-core-#.txt appended.
fn process_all_cores_single_pass(
    parser: Parser,
    output_path: Option<&str>,
) -> Result<(), ProcessorError> {
    let output_path = get_outout_path(output_path, &parser.file_path[..]);
    let mut writer = match Writer::new(&output_path[..], parser.cores) {
        Ok(w) => w,
        Err(_e) => return Err(ProcessorError::IOError),
    };

    let (data_x, data_y) = process_pairwise(parser, &mut writer);
    process_full_dataset(data_x, data_y, &mut writer);
    Ok(())
}

fn get_outout_path(output_path: Option<&str>, input_file: &str) -> String {
    match output_path {
        Some(path) => {
            let path = Path::new(path);
            let file_name = Path::new(input_file).file_name().unwrap_or_default();
            let full_path = path.join(file_name);
            full_path.to_string_lossy().to_string()
        }
        None => String::from(input_file),
    }
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
    let cores = data_x.len();

    let globals: Vec<(usize, GaussianEliminationSolution<f64>)> = (0..cores)
        .into_par_iter()
        .map(|core| {
            let core_x = Matrix::from(data_x[core].clone());
            let core_y = Matrix::from(data_y[core].clone());

            let core_xt = core_x.transpose();
            let core_xtx = &core_xt * &core_x;
            let core_xty = &core_xt * &core_y;
            let core_xtxxty = core_xtx.augment(&core_xty);

            let glsa = gauss::solve(core_xtxxty);
            (core, glsa)
        })
        .collect();

    for (core, glsa) in globals {
        writer.write_global(core, glsa.lhs(), glsa);
    }
}
