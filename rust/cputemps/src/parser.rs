use num_traits::Num;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use launearalg::matrix::Matrix;

#[derive(PartialEq, Debug)]
/// `ParserError` enumerates possible failures parsing to Matrix
pub enum ParserError {
    FileNotFound,
    PermissionError,
    Unexpected,
}

impl ParserError {
    /// Human readable error message for `ParserError` variants
    ///
    /// # Example
    /// ```
    /// use cputemps::parser::ParserError;
    /// &ParserError::FileNotFound.as_str();
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            &ParserError::FileNotFound => "🤷🏻‍♂️ File not found",
            &ParserError::PermissionError => {
                "🙅🏽‍♀️ You do not have permissions to access that file"
            }
            &ParserError::Unexpected => {
                "⚰️ ☠️ Something unexpected and deeply troubling has happened. ☠️ ⚰️"
            }
        }
    }
}

/// Opens file from path given at `path` and returns a `Result` of
/// either a fully populated `launearalg::Matrix` with a row
/// for each sample timestep and 1 column per core or a `ParserError`.
///
/// # Example
/// ```
/// use cputemps::parser;
/// let path = "data/sensors-2019.02.09.txt";
/// let mat = parser::parse::<f64>(path);
/// ```
pub fn parse<T>(path: &str) -> Result<Matrix<T>, ParserError>
where
    T: Num + Copy + std::str::FromStr,
{
    println!("opening {}", path);
    let file = open_temperature_data_file(path);
    match file {
        Ok(f) => read_to_matrix(f),
        Err(e) => Err(e),
    }
}

fn open_temperature_data_file(path: &str) -> Result<File, ParserError> {
    let file = File::open(path);

    match file {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => Err(ParserError::FileNotFound),
            std::io::ErrorKind::PermissionDenied => Err(ParserError::PermissionError),
            _ => Err(ParserError::Unexpected),
        },
    }
}

fn read_to_matrix<T>(file: File) -> Result<Matrix<T>, ParserError>
where
    T: Num + Copy + std::str::FromStr,
{
    let reader = BufReader::new(file);

    let data_points: Vec<Vec<T>> = reader
        .lines()
        .filter_map(|line| match line {
            Ok(l) => Some(parse_line::<T>(&l[..])),
            _ => None,
        })
        .collect();

    if !sanity_check(&data_points[..]) {
        return Err(ParserError::Unexpected);
    }

    Ok(Matrix::from(data_points))
}

fn parse_line<T>(line: &str) -> Vec<T>
where
    T: Copy + std::str::FromStr,
{
    let re = Regex::new(r"(?P<temp>\b[0-9.]+)").unwrap();
    re.captures_iter(line)
        .filter_map(|cap| match &cap["temp"].parse::<T>() {
            Ok(val) => Some(*val),
            _ => None,
        })
        .collect()
}

fn sanity_check<T>(data_points: &[Vec<T>]) -> bool {
    data_points.len() > 0 && data_points.iter().all(|v| v.len() == data_points[0].len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file_not_found() {
        let filename = "by_the_prophet_peace_be_upon_him_this_shall_not_exist.txt";
        let sut = open_temperature_data_file(filename);
        assert_eq!(ParserError::FileNotFound, sut.unwrap_err());
    }

    fn test_open_file_no_permissions() {
        // needs changed so it can actually find test data
        let filename = "./test_data/unopenable";
        let sut = open_temperature_data_file(filename);
        assert_eq!(ParserError::PermissionError, sut.unwrap_err());
    }

    #[test]
    fn test_parse_line_happy_path_with_labels() {
        let line = "+61.0Â°C +63.0Â°C +50.0Â°C +58.0Â°C";
        let sut = parse_line::<f64>(line);
        assert_eq!(vec![61.0, 63.0, 50.0, 58.0], sut);
    }

    #[test]
    fn test_parse_line_happy_path_no_labels() {
        let line = "61.0 63.0 50.0 58.0";
        let sut = parse_line(line);
        assert_eq!(vec![61.0, 63.0, 50.0, 58.0], sut);
    }

    #[test]
    fn test_parse_line_missing_decimal_first() {
        let line = "61 63 50.0 58.0";
        let sut = parse_line(line);
        assert_eq!(vec![61.0, 63.0, 50.0, 58.0], sut);
    }

    #[test]
    fn test_parse_line_missing_decimal_mid() {
        let line = "61.0 63 50.0 58.0";
        let sut = parse_line(line);
        assert_eq!(vec![61.0, 63.0, 50.0, 58.0], sut);
    }
}
