use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum FileReadError {
    FileNotFound(String),
    IOError(std::io::Error)
}

pub fn read_file(file_path: &str) -> Result<Vec<u8>, FileReadError> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            return if error.kind() == io::ErrorKind::NotFound {
                Err(FileReadError::FileNotFound(file_path.to_owned()))
            } else {
                Err(FileReadError::IOError(error))
            };
        }
    };

    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents).map_err(FileReadError::IOError)?;
    Ok(file_contents)
}

pub fn trim_null_bytes(data_str: &str) -> String {
    data_str.trim_matches(char::from(0)).trim().to_string()
}
