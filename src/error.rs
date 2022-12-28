use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotEnoughChars(&'static str, usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotEnoughChars(msg, i) => write!(f, "{} '{}'", msg, i),
        }
    }
}
