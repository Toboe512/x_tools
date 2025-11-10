use std::io;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum Error {
    FileSystem,
    Malformed,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        if error.kind() == ErrorKind::UnexpectedEof {
            Error::Malformed
        } else {
            Error::FileSystem
        }
    }
}

pub fn log_err<T: ToString>(err: T) -> String {
    let e_str = format!("Error: {}", err.to_string());
    println!("{e_str}");
    e_str
}
