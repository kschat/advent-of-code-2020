use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct AppError {
    message: String
}

impl AppError {
    pub fn new(msg: &str) -> AppError {
        AppError { message: msg.to_string() }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError { message: err.to_string() }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError { message: err.to_string() }
    }
}

pub type AppResult<T> = Result<T, AppError>;
