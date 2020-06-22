use crate::{pairs::Pairs, parser::Parser, writer::Writer};
use launearalg::{approximator::least_squares::*, matrix::Matrix, solver::gauss, traits::*};
use rayon::prelude::*;
use std::fmt;
use std::path::Path;

type ProtoMatrix = Vec<Vec<f64>>;

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
        step: u32,
        file_path: &str,
        output_path: Option<&str>,
    ) -> Result<(), ProcessorError> {
        match Parser::new(file_path) {
            Ok(p) => process_all_cores_single_pass(step, p, output_path),
            Err(_e) => Err(ProcessorError::IOError),
        }
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

    let analyzers = vec![]; //Box::new(LinearPiecewiseInterpolater {})];

    process_pairwise(step, &parser, &mut writer, analyzers);
    //process_global(step, &parser, &mut writer, analyzers);
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
    _step: u32,
    parser: &Parser,
    writer: &mut Writer,
    analyzers: Vec<Box<dyn Analyzer<Output = Box<dyn Solution>>>>,
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
        let step = i as f64;
        let x1 = step * step;
        let x2 = (step + 1.0) * step;

        for (core, endpoints) in pairs.iter().enumerate() {
            let (y1, y2) = *endpoints;
            for analyzer in analyzers.iter() {
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
    _parser: &Parser,
    _writer: &mut Writer,
    _analyzers: Vec<Box<dyn Analyzer<Output = Box<dyn Solution>>>>,
) {
    /*
    let cores = parser.cores;
    let globals: Vec<(usize, LeastSquaresApproximationSolution<f64>)> = (0..cores)
        .into_par_iter()
        .map(|core| {
            process_cubic_spline(&cs_data.0[core][..], &cs_data.1[core][..]);
            (
                core,
                process_global_least_squares(&glsa_data.0[core], &glsa_data.1[core]),
            )
        })
        .collect();

    for (core, glsa) in globals {
        writer.write_global(core, glsa.lhs(), glsa);
    }
    */
}

fn process_global_least_squares(
    data_x: &ProtoMatrix,
    data_y: &ProtoMatrix,
) -> LeastSquaresApproximationSolution {
    let core_x = Matrix::from(data_x.clone());
    let core_y = Matrix::from(data_y.clone());

    let core_xt = core_x.transpose();
    let core_xtx = &core_xt * &core_x;
    let core_xty = &core_xt * &core_y;
    let core_xtxxty = core_xtx.augment(&core_xty);

    let weights = gauss::solve(core_xtxxty);
    LeastSquaresApproximationSolution { weights }
}

fn process_cubic_spline(delta_x: &[f64], delta_y: &[f64]) {
    let size = delta_x.len();

    let mut a = Matrix::<f64>::new(size, size);
    let mut b = Matrix::<f64>::new(size, 1);
    a[0][0] = 1.0;
    a[size - 1][size - 1] = 1.0;

    for i in 1..(size - 1) {
        a[i][i - 1] = delta_x[i - 1];
        a[i][i + 1] = delta_x[i];
        a[i][i] = 2.0 * (delta_x[i - 1] + delta_x[i]);
        b[i][0] = 3.0 * delta_y[i] / delta_x[i] - delta_y[i - 1] / delta_x[i - 1];
    }

    let ab = a.augment(&b);
    let c_i = gauss::solve(ab);
    let mut b_i = vec![0.0; size - 1];
    let mut d_i = vec![0.0; size - 1];
    for i in 0..(size - 1) {
        b_i[i] = (delta_y[i] / delta_x[i]) - (delta_x[i] / 3.0) * (2.0 * c_i[i] + c_i[i + 1]);
        d_i[i] = (c_i[i + 1] - c_i[i]) / (3.0 * delta_x[i]);
        println!(
            "S_{0}(x) = y{0} + {2}(x - x{0}) + {3}(x - x{0})\u{00B2} + {4}(x - x{0})\u{00B3} on [x{0}, x{1}",
            i, i+1, b_i[i], c_i[i], d_i[i],
        );
    }
}
