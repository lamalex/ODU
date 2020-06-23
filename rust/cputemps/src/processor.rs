use crate::{pairs::Pairs, parser::Parser, writer::Writer};
use launearalg::{
    approximator::least_squares::*, interpolater::cubic_spline::*,
    interpolater::linear_piecewise::*, traits::*,
};
use std::fmt;
use std::path::Path;

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

pub fn process_data_file(
    step: u32,
    file_path: &str,
    output_path: Option<&str>,
) -> Result<(), ProcessorError> {
    match Parser::new(file_path) {
        Ok(p) => process_all_cores_single_pass(step, p, output_path),
        Err(_e) => Err(ProcessorError::IOError),
    }
}

// In single pass -> read 2 lines of file and computer linear interpolation.
// Save one line and build matrix from which to compute global least squares
// Takes 1 pass through each file and computes while reading.
// Writes file next to input with -out-core-#.txt appended.
fn process_all_cores_single_pass(
    step: u32,
    parser: Parser,
    output_path: Option<&str>,
) -> Result<(), ProcessorError> {
    let output_path = get_outout_path(output_path, &parser.file_path[..]);
    let mut writer = match Writer::new(&output_path[..], parser.cores) {
        Ok(w) => w,
        Err(_e) => return Err(ProcessorError::IOError),
    };

    let mut analyzers: Vec<Vec<Box<dyn Analyzer<Output = dyn Solution>>>> = (0..parser.cores)
        .map(|_core| {
            vec![
                Box::new(LinearPiecewiseInterpolater::new())
                    as Box<dyn Analyzer<Output = dyn Solution>>,
                Box::new(LeastSquaresApproximator::new())
                    as Box<dyn Analyzer<Output = dyn Solution>>,
                Box::new(CubicSplineInterpolator::new())
                    as Box<dyn Analyzer<Output = dyn Solution>>,
            ]
        })
        .collect();

    process_pairwise(step, &parser, &mut writer, &mut analyzers[..]);
    process_global(step, &parser, &mut writer, &mut analyzers[..]);
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

fn process_pairwise(
    step: u32,
    parser: &Parser,
    writer: &mut Writer,
    analyzers: &mut [Vec<Box<dyn Analyzer<Output = dyn Solution>>>],
) {
    let data_pairs = parser.iter().pairs().map(|line_pair| {
        line_pair
            .0
            .iter()
            .zip(line_pair.1.iter())
            .map(|z| (*z.0, *z.1))
            .collect::<Vec<(f64, f64)>>()
    });

    for (i, pairs) in data_pairs.enumerate() {
        let i = i as f64;
        let step = step as f64;

        let x1 = i * step as f64;
        let x2 = (i + 1.0) * step;

        for (core, endpoints) in pairs.iter().enumerate() {
            let (y1, y2) = *endpoints;
            for analyzer in analyzers[core].iter_mut() {
                let sol = analyzer.analyze_piecewise(vec![(x1, y1), (x2, y2)]);
                match sol {
                    Some(sol) => {
                        let lhs = format!("{}{}", sol.lhs(), i);
                        writer.write_pairwise(core, (x1, x2), &lhs[..], sol)
                    }
                    None => continue,
                }
            }
        }
    }
}

fn process_global(
    _step: u32,
    parser: &Parser,
    writer: &mut Writer,
    analyzers: &mut [Vec<Box<dyn Analyzer<Output = dyn Solution>>>],
) {
    let mut globals = Vec::<(usize, Box<dyn Solution>)>::new();

    for core in 0..parser.cores {
        for analyzer in analyzers[core].iter_mut() {
            match analyzer.analyze_global() {
                Some(sol) => globals.push((core, sol)),
                None => continue,
            }
        }
    }

    for (core, sol) in globals {
        writer.write_global(core, sol.lhs(), sol);
    }
}
