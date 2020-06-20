use std::fs::File;
use std::io::Write;
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

    pub fn write(&mut self, core: usize, data: impl std::fmt::Display) {
        let _ = writeln!(self.files[core], "{}", data);
    }
}
