use std::{fmt::Display, fs::File, io::Write};

pub struct Writer {
    files: Vec<File>,
}

impl Writer {
    pub fn new(base_path: &str, cores: usize) -> Result<Writer, std::io::Error> {
        let files: Vec<File> = (0..cores)
            .map(|core| File::create(format!("{}-out-core-{}.txt", base_path, core)))
            .collect::<Result<Vec<File>, std::io::Error>>()?;

        Ok(Writer { files })
    }

    pub fn write_global(&mut self, core: usize, lhs: &str, rhs: impl Display) {
        self.write(core, std::format!("{2:16}{0} = {1}", lhs, rhs, ""))
    }

    pub fn write_pairwise(&mut self, core: usize, bound: (f64, f64), lhs: &str, rhs: impl Display) {
        self.write(
            core,
            std::format!("{:6} <= {:6}; {:5} = {}", bound.0, bound.1, lhs, rhs),
        );
    }

    pub fn write(&mut self, core: usize, data: impl std::fmt::Display) {
        let _ = writeln!(self.files[core], "{}", data);
    }
}
