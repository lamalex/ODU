use clap::{crate_version, App, Arg};
use cputemps::processor::{Processor, ProcessorError};
use rayon::prelude::*;
use std::env;

fn main() -> Result<(), ProcessorError> {
    const ABOUT: &'static str =
        "Analyzes n-core CPU temperature data via interpolation, and least squares approximation.
Output data can be found alongside input data with -out-core-#.txt appended.
🖖🏽 Live long and interpolate.";

    let matches = App::new("CS517 Semester Project: CPU Temperatur intep-o-matic")
        .version(crate_version!())
        .author("Alex L. Launi <alaun001@odu.edu>")
        .about(ABOUT)
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file(s) to use")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let files: Vec<&str> = matches.values_of("INPUT").unwrap().collect();

    files
        .par_iter()
        .map(|data_file_path| Processor::process_data_file(data_file_path))
        .collect()
}
