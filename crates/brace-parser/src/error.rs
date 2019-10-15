use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Incomplete,
    Unexpected(char),
}

impl Error {
    pub fn incomplete() -> Self {
        Self::Incomplete
    }

    pub fn unexpected(ch: char) -> Self {
        Self::Unexpected(ch)
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Incomplete => write!(f, "Error: Unexpected end of input"),
            Self::Unexpected(ch) => write!(f, "Error: Unexpected character: '{}'", ch),
        }
    }
}
