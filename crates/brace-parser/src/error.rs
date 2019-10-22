use std::error;
use std::fmt;

use crate::character::Character;
use crate::sequence::Sequence;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Expected(Expected),
}

impl Error {
    pub fn incomplete() -> Self {
        Self::Expected(Expected::expect(Expect::Match).found(()))
    }

    pub fn unexpected<T>(unexpected: T) -> Self
    where
        T: Into<Expect>,
    {
        Self::Expected(Expected::expect(Expect::Match).found(unexpected))
    }

    pub fn expected<T>(expected: T) -> Self
    where
        T: Into<Expected>,
    {
        Self::Expected(expected.into())
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Expected(expect) => write!(f, "Error: {}", expect),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expected(Expect, Option<Expect>);

impl Expected {
    pub fn new<T, U>(expect: T, found: U) -> Self
    where
        T: Into<Expect>,
        U: Into<Expect>,
    {
        Self(expect.into(), Some(found.into()))
    }

    pub fn expect<T>(expect: T) -> Self
    where
        T: Into<Expect>,
    {
        Self(expect.into(), None)
    }

    pub fn found<T>(mut self, found: T) -> Self
    where
        T: Into<Expect>,
    {
        self.1 = Some(found.into());
        self
    }
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.1 {
            Some(found) => write!(f, "Expected {}; Found {}", self.0, found),
            None => write!(f, "Expected {}", self.0),
        }
    }
}

impl From<()> for Expected {
    fn from(_: ()) -> Self {
        Self(Expect::End, None)
    }
}

impl From<char> for Expected {
    fn from(from: char) -> Self {
        Self(from.into(), None)
    }
}

impl From<Character> for Expected {
    fn from(from: Character) -> Self {
        Self(from.into(), None)
    }
}

impl From<&str> for Expected {
    fn from(from: &str) -> Self {
        Self(from.into(), None)
    }
}

impl From<String> for Expected {
    fn from(from: String) -> Self {
        Self(from.into(), None)
    }
}

impl From<Sequence> for Expected {
    fn from(from: Sequence) -> Self {
        Self(from.into(), None)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expect {
    End,
    Character(Character),
    Sequence(Sequence),
    Match,
}

impl fmt::Display for Expect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::End => write!(f, "end of input"),
            Self::Character(ch) => write!(f, "character: {}", ch),
            Self::Sequence(seq) => write!(f, "sequence: {}", seq),
            Self::Match => write!(f, "match"),
        }
    }
}

impl From<()> for Expect {
    fn from(_: ()) -> Self {
        Self::End
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
