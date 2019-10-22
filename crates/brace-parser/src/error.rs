use std::error;
use std::fmt;

use crate::character::Character;
use crate::sequence::Sequence;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Incomplete,
    Unexpected(char),
    Expected(Expect),
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
            Self::Expected(expect) => write!(f, "Error: Expected {}", expect),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expect {
    Character(Character),
    Sequence(Sequence),
}

impl fmt::Display for Expect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Character(ch) => write!(f, "character: {}", ch),
            Self::Sequence(seq) => write!(f, "sequence: {}", seq),
        }
    }
}
