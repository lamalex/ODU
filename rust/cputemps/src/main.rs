use clap::{crate_version, value_t, App, Arg};
use cputemps::processor::*;
use rayon::prelude::*;
use std::env;

fn main() -> Result<(), ProcessorError> {
    const DEFAULT_STEP: u32 = 30;
    const ABOUT: &'static str =
        "Analyzes n-core CPU temperature data via interpolation, and least squares approximation.
🖖🏽 Live long and interpolate.";

    let matches = App::new("CS517 Semester Project: CPU Temperatur interp-o-matic")
        .version(crate_version!())
        .author("Alex L. Launi <alaun001@odu.edu>")
        .about(ABOUT)
        .arg(
            Arg::with_name("step-size")
                .help("Data point spacing (X-axis)")
                .long("step")
                .takes_value(true)
                .value_name("STEP"),
        )
        .arg(
            Arg::with_name("output-path")
                .help("Set path for output. Default output is alongside input file.")
                .short("o")
                .long("output-path")
                .takes_value(true)
                .value_name("DIRECTORY"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file(s) to use")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let files: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let output_path = matches.value_of("output-path");
    let step = value_t!(matches.value_of("step_size"), u32).unwrap_or(DEFAULT_STEP);

    files
        .par_iter()
        .map(|data_file_path| process_data_file(step, data_file_path, output_path))
        .collect()
}
