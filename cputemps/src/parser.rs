use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Parser {
    pub cores: usize,
    pub file_path: String,
}

pub struct ParserIterator {
    lines: std::io::Lines<BufReader<File>>,
}

impl Parser {
    /// # Example
    /// ```
    /// use cputemps::parser::Parser;
    /// let path = "data/sensors-2019.02.09.txt";
    /// let parser = Parser::new(path);
    /// ```
    pub fn new(path: &str) -> Result<Parser, Box<dyn Error>> {
        let f = open_temperature_data_file(path)?;
        let mut reader = BufReader::new(f);
        let mut buf = String::new();

        reader.read_line(&mut buf)?;
        let cores = parse_line(&buf[..])?;

        Ok(Parser {
            cores: cores.len(),
            file_path: path.to_string(),
        })
    }

    pub fn iter(&self) -> ParserIterator {
        let file = open_temperature_data_file(&self.file_path[..]).unwrap();
        let reader = BufReader::new(file);

        ParserIterator {
            lines: reader.lines(),
        }
    }
}

fn open_temperature_data_file(path: &str) -> Result<File, std::io::Error> {
    File::open(path)
}

impl Iterator for ParserIterator {
    type Item = Vec<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        match line {
            Some(Ok(line)) => match parse_line(&line[..]) {
                Ok(v) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }
}

fn parse_line(line: &str) -> Result<Vec<f64>, std::num::ParseFloatError> {
    // Safe to unwrap. Regex has been vetted for correctness.
    // If regex is incorrect it's a programming error and should not be handled.
    let re = Regex::new(r"(?P<temp>\b[0-9.]+)").unwrap();

    re.captures_iter(line)
        .map(|cap| String::from(&cap["temp"]).parse::<f64>())
        .collect()
}
#[cfg(test)]

mod tests {
    use super::*;

    use crate::pairs::Pairs;
    use rand::prelude::*;
    use std::io::Write;
    use tempfile::{tempdir, TempDir};

    const DATA_GOOD: [&str; 6] = [
        "+83.0°C +84.0°C +65.0°C +81.0°C",
        "+67.0°C +70.0°C +57.0°C +64.0°C",
        "+68.0°C +72.0°C +58.0°C +66.0°C",
        "+91.0°C +88.0°C +75.0°C +86.0°C",
        "+88.0°C +89.0°C +73.0°C +84.0°C",
        "+71.0°C +71.0°C +59.0°C +66.0°C",
    ];

    fn setup_path() -> (TempDir, std::path::PathBuf) {
        let dir = tempdir().unwrap();
        let filename: String = thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(15)
            .collect();

        let filepath = dir.path().join(filename);

        (dir, filepath)
    }

    fn make_good_temp_data_file() -> (TempDir, String) {
        make_temp_data_file(DATA_GOOD)
    }

    fn make_temp_data_file(data: [&str; 6]) -> (TempDir, String) {
        let temp_file_data = setup_path();
        let mut file = File::create(&temp_file_data.1).unwrap();

        for line in &data {
            (writeln!(file, "{}", line)).unwrap();
        }

        (
            temp_file_data.0,
            temp_file_data.1.to_str().unwrap().to_string(),
        )
    }

    #[test]
    fn test_create_parser_instance() {
        let (_dir, file_name) = make_good_temp_data_file();
        let sut = Parser::new(&file_name[..]);

        match sut {
            Ok(_parse) => return,
            Err(e) => panic!("Parser was not created: {}", e.to_string()),
        }
    }

    #[test]
    fn test_parser_is_iterator() {
        let (_dir, file_name) = make_good_temp_data_file();
        let sut = Parser::new(&file_name[..]).unwrap();
        let mut count: usize = 0;

        for _row in sut.iter() {
            count += 1;
        }

        assert_eq!(DATA_GOOD.len(), count);
    }

    #[test]
    fn test_pairs_of_rows() {
        let (_dir, file_name) = make_good_temp_data_file();
        let mut sut = Parser::new(&file_name[..]).unwrap().iter().pairs();
        let row_pair = sut.next().unwrap();
        assert_eq!(vec![83.0, 84.0, 65.0, 81.0], row_pair.0);
        assert_eq!(vec![67.0, 70.0, 57.0, 64.0], row_pair.1);
        let row_pair = sut.next().unwrap();
        assert_eq!(vec![67.0, 70.0, 57.0, 64.0], row_pair.0);
        assert_eq!(vec![68.0, 72.0, 58.0, 66.0], row_pair.1);
    }
}
