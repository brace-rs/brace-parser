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

    pub fn expected<T>(expect: T) -> Self
    where
        T: Into<Expect>,
    {
        Self::Expected(expect.into())
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

impl From<char> for Expect {
    fn from(from: char) -> Self {
        Self::Character(Character::custom(from))
    }
}

impl From<Character> for Expect {
    fn from(from: Character) -> Self {
        Self::Character(from)
    }
}

impl From<&str> for Expect {
    fn from(from: &str) -> Self {
        Self::Sequence(Sequence::custom(from))
    }
}

impl From<String> for Expect {
    fn from(from: String) -> Self {
        Self::Sequence(Sequence::custom(from))
    }
}

impl From<Sequence> for Expect {
    fn from(from: Sequence) -> Self {
        Self::Sequence(from)
    }
}
