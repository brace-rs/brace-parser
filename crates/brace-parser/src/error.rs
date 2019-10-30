use std::error;
use std::fmt;

use crate::character::Character;
use crate::sequence::Sequence;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Pass(InnerError),
    Fail(InnerError),
}

impl Error {
    pub fn invalid() -> Self {
        Self::Fail(InnerError(Some(Expect::Valid), None, None))
    }

    pub fn context<T>(ctx: T) -> Self
    where
        T: Into<String>,
    {
        Self::Pass(InnerError(None, None, Some(ctx.into())))
    }

    pub fn expect<T>(expect: T) -> Self
    where
        T: Into<Expect>,
    {
        Self::Pass(InnerError(Some(expect.into()), None, None))
    }

    pub fn found<T>(found: T) -> Self
    where
        T: Into<Expect>,
    {
        Self::Pass(InnerError(None, Some(found.into()), None))
    }

    pub fn found_end() -> Self {
        Self::Pass(InnerError(None, Some(Expect::End), None))
    }

    pub fn but_expect<T>(mut self, expect: T) -> Self
    where
        T: Into<Expect>,
    {
        match self {
            Self::Pass(ref mut inner) => inner.0 = Some(expect.into()),
            Self::Fail(ref mut inner) => inner.0 = Some(expect.into()),
        }

        self
    }

    pub fn but_found<T>(mut self, found: T) -> Self
    where
        T: Into<Expect>,
    {
        match self {
            Self::Pass(ref mut inner) => inner.1 = Some(found.into()),
            Self::Fail(ref mut inner) => inner.1 = Some(found.into()),
        }

        self
    }

    pub fn but_found_end(mut self) -> Self {
        match self {
            Self::Pass(ref mut inner) => inner.1 = Some(Expect::End),
            Self::Fail(ref mut inner) => inner.1 = Some(Expect::End),
        }

        self
    }

    pub fn with_context<T>(mut self, ctx: T) -> Self
    where
        T: Into<String>,
    {
        match self {
            Self::Pass(ref mut inner) => inner.2 = Some(ctx.into()),
            Self::Fail(ref mut inner) => inner.2 = Some(ctx.into()),
        }

        self
    }

    pub fn get_context(&self) -> Option<&str> {
        match self {
            Self::Pass(inner) => inner.2.as_ref().map(|ctx| ctx.as_ref()),
            Self::Fail(inner) => inner.2.as_ref().map(|ctx| ctx.as_ref()),
        }
    }

    pub fn set_context<T>(&mut self, ctx: T) -> &mut Self
    where
        T: Into<String>,
    {
        match self {
            Self::Pass(ref mut inner) => inner.2 = Some(ctx.into()),
            Self::Fail(ref mut inner) => inner.2 = Some(ctx.into()),
        }

        self
    }

    pub fn is_pass(&self) -> bool {
        match self {
            Self::Pass(_) => true,
            _ => false,
        }
    }

    pub fn as_pass(&self) -> Option<&InnerError> {
        match self {
            Self::Pass(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn into_pass(self) -> Self {
        match self {
            Self::Fail(inner) => Self::Pass(inner),
            _ => self,
        }
    }

    pub fn is_fail(&self) -> bool {
        match self {
            Self::Fail(_) => true,
            _ => false,
        }
    }

    pub fn as_fail(&self) -> Option<&InnerError> {
        match self {
            Self::Fail(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn into_fail(self) -> Self {
        match self {
            Self::Pass(inner) => Self::Fail(inner),
            _ => self,
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pass(inner) => write!(f, "{}", inner),
            Self::Fail(inner) => write!(f, "{}", inner),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InnerError(Option<Expect>, Option<Expect>, Option<String>);

impl fmt::Display for InnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error:")?;

        if let Some(ctx) = &self.2 {
            write!(f, " in {}", ctx)?;
        }

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
