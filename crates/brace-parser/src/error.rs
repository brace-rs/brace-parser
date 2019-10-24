use std::error;
use std::fmt;

use crate::character::Character;
use crate::sequence::Sequence;

#[derive(Clone, Debug, PartialEq)]
pub struct Error(Option<Expect>, Option<Expect>);

impl Error {
    pub fn invalid() -> Self {
        Self(Some(Expect::Valid), None)
    }

    pub fn expect<T>(expect: T) -> Self
    where
        T: Into<Expect>,
    {
        Self(Some(expect.into()), None)
    }

    pub fn found<T>(found: T) -> Self
    where
        T: Into<Expect>,
    {
        Self(None, Some(found.into()))
    }

    pub fn found_end() -> Self {
        Self(None, Some(Expect::End))
    }

    pub fn but_expect<T>(mut self, expect: T) -> Self
    where
        T: Into<Expect>,
    {
        self.0 = Some(expect.into());
        self
    }

    pub fn but_found<T>(mut self, found: T) -> Self
    where
        T: Into<Expect>,
    {
        self.1 = Some(found.into());
        self
    }

    pub fn but_found_end(mut self) -> Self {
        self.1 = Some(Expect::End);
        self
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error:")?;

        if let Some(expect) = &self.0 {
            write!(f, "\nExpected {}", expect)?;
        }

        if let Some(found) = &self.1 {
            write!(f, "\nFound {}", found)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expect {
    End,
    Valid,
    Character(Character),
    Sequence(Sequence),
}

impl fmt::Display for Expect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::End => write!(f, "end of input"),
            Self::Valid => write!(f, "valid parser"),
            Self::Character(ch) => write!(f, "character: {}", ch),
            Self::Sequence(seq) => write!(f, "sequence: {}", seq),
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
