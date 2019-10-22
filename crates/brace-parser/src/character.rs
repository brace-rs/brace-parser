use std::borrow::Borrow;
use std::fmt;

use crate::{take, Error, Parser};

pub fn character<'a, T>(ch: T) -> impl Parser<'a, char>
where
    T: Borrow<char>,
{
    move |input| {
        take(|character| character == ch.borrow())
            .parse(input)
            .map(|(_, rem)| (*ch.borrow(), rem))
            .map_err(|err| err.but_expect(*ch.borrow()))
    }
}

pub fn any(input: &str) -> Result<(char, &str), Error> {
    take(|_| true)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Any))
}

pub fn decimal(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_digit)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Decimal))
}

pub fn hexadecimal(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_hexdigit)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Hexadecimal))
}

pub fn alphabetic(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_alphabetic)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Alphabetic))
}

pub fn alphanumeric(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_alphanumeric)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Alphanumeric))
}

pub fn lowercase(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_lowercase)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Lowercase))
}

pub fn uppercase(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_uppercase)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Uppercase))
}

pub fn indent(input: &str) -> Result<(char, &str), Error> {
    take(crate::util::is_ascii_indent)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Indent))
}

pub fn linebreak(input: &str) -> Result<(char, &str), Error> {
    take(crate::util::is_ascii_linebreak)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Linebreak))
}

pub fn whitespace(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_whitespace)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
        .map_err(|err| err.but_expect(Character::Whitespace))
}

#[derive(Clone, Debug, PartialEq)]
pub enum Character {
    Any,
    Decimal,
    Hexadecimal,
    Alphabetic,
    Alphanumeric,
    Lowercase,
    Uppercase,
    Indent,
    Linebreak,
    Whitespace,
    Custom(char),
}

impl Character {
    pub fn custom<T>(character: T) -> Self
    where
        T: Into<char>,
    {
        Self::Custom(character.into())
    }
}

impl<'a> Parser<'a, char> for Character {
    fn parse(&self, input: &'a str) -> Result<(char, &'a str), Error> {
        match self {
            Self::Any => any.parse(input),
            Self::Decimal => decimal.parse(input),
            Self::Hexadecimal => hexadecimal.parse(input),
            Self::Alphabetic => alphabetic.parse(input),
            Self::Alphanumeric => alphanumeric.parse(input),
            Self::Lowercase => lowercase.parse(input),
            Self::Uppercase => uppercase.parse(input),
            Self::Indent => indent.parse(input),
            Self::Linebreak => linebreak.parse(input),
            Self::Whitespace => whitespace.parse(input),
            Self::Custom(ch) => ch.parse(input),
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::Decimal => write!(f, "decimal"),
            Self::Hexadecimal => write!(f, "hexadecimal"),
            Self::Alphabetic => write!(f, "alphabetic"),
            Self::Alphanumeric => write!(f, "alphanumeric"),
            Self::Lowercase => write!(f, "lowercase"),
            Self::Uppercase => write!(f, "uppercase"),
            Self::Indent => write!(f, "indent"),
            Self::Linebreak => write!(f, "linebreak"),
            Self::Whitespace => write!(f, "whitespace"),
            Self::Custom(ch) => write!(f, "'{}'", ch),
        }
    }
}

impl From<char> for Character {
    fn from(from: char) -> Self {
        Self::Custom(from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse, Error};

    #[test]
    fn test_character() {
        assert_eq!(
            parse("", character('h')),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("$", character('h')),
            Err(Error::expect('h').but_found('$'))
        );
        assert_eq!(parse("h", character('h')), Ok(('h', "")));
        assert_eq!(parse("hello", character('h')), Ok(('h', "ello")));
        assert_eq!(parse("hello", character(&'h')), Ok(('h', "ello")));
    }

    #[test]
    fn test_any() {
        for ch in "$0123456789 \t\n\r\u{000C}".chars() {
            assert_eq!(parse(&ch.to_string(), any), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), any), Ok((ch, "$")));
        }

        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&ch.to_string(), any), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), any), Ok((ch, "$")));
        }

        assert_eq!(
            parse("", any),
            Err(Error::expect(Character::Any).but_found_end())
        );
    }

    #[test]
    fn test_any_variant() {
        for ch in "$0123456789 \t\n\r\u{000C}".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Any), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Any),
                Ok((ch, "$"))
            );
        }

        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Any), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Any),
                Ok((ch, "$"))
            );
        }

        assert_eq!(
            parse("", Character::Any),
            Err(Error::expect(Character::Any).but_found_end())
        );
    }

    #[test]
    fn test_decimal() {
        for ch in "0123456789".chars() {
            assert_eq!(parse(&ch.to_string(), decimal), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), decimal), Ok((ch, "$")));
        }

        for ch in "$aZ\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), decimal),
                Err(Error::expect(Character::Decimal).but_found(ch))
            );
        }

        assert_eq!(
            parse("", decimal),
            Err(Error::expect(Character::Decimal).but_found_end())
        );
    }

    #[test]
    fn test_decimal_variant() {
        for ch in "0123456789".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Decimal), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Decimal),
                Ok((ch, "$"))
            );
        }

        for ch in "$aZ\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Decimal),
                Err(Error::expect(Character::Decimal).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Character::Decimal),
            Err(Error::expect(Character::Decimal).but_found_end())
        );
    }

    #[test]
    fn test_hexadecimal() {
        for ch in "0123456789abcdefABCDEF".chars() {
            assert_eq!(parse(&ch.to_string(), hexadecimal), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), hexadecimal), Ok((ch, "$")));
        }

        for ch in "$gZ\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), hexadecimal),
                Err(Error::expect(Character::Hexadecimal).but_found(ch))
            );
        }

        assert_eq!(
            parse("", hexadecimal),
            Err(Error::expect(Character::Hexadecimal).but_found_end())
        );
    }

    #[test]
    fn test_hexadecimal_variant() {
        for ch in "0123456789abcdefABCDEF".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Hexadecimal), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Hexadecimal),
                Ok((ch, "$"))
            );
        }

        for ch in "$gZ\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Hexadecimal),
                Err(Error::expect(Character::Hexadecimal).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Character::Hexadecimal),
            Err(Error::expect(Character::Hexadecimal).but_found_end())
        );
    }

    #[test]
    fn test_alphabetic() {
        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&ch.to_string(), alphabetic), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), alphabetic), Ok((ch, "$")));
        }

        for ch in "$0 \n".chars() {
            assert_eq!(
                parse(&ch.to_string(), alphabetic),
                Err(Error::expect(Character::Alphabetic).but_found(ch))
            );
        }

        assert_eq!(
            parse("", alphabetic),
            Err(Error::expect(Character::Alphabetic).but_found_end())
        );
    }

    #[test]
    fn test_alphabetic_variant() {
        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Alphabetic), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Alphabetic),
                Ok((ch, "$"))
            );
        }

        for ch in "$0 \n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Alphabetic),
                Err(Error::expect(Character::Alphabetic).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Character::Alphabetic),
            Err(Error::expect(Character::Alphabetic).but_found_end())
        );
    }

    #[test]
    fn test_alphanumeric() {
        for ch in "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&ch.to_string(), alphanumeric), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), alphanumeric), Ok((ch, "$")));
        }

        for ch in "$ \n".chars() {
            assert_eq!(
                parse(&ch.to_string(), alphanumeric),
                Err(Error::expect(Character::Alphanumeric).but_found(ch))
            );
        }

        assert_eq!(
            parse("", alphanumeric),
            Err(Error::expect(Character::Alphanumeric).but_found_end())
        );
    }

    #[test]
    fn test_alphanumeric_variant() {
        for ch in "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Alphanumeric),
                Ok((ch, ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Alphanumeric),
                Ok((ch, "$"))
            );
        }

        for ch in "$ \n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Alphanumeric),
                Err(Error::expect(Character::Alphanumeric).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Character::Alphanumeric),
            Err(Error::expect(Character::Alphanumeric).but_found_end())
        );
    }

    #[test]
    fn test_lowercase() {
        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(parse(&ch.to_string(), lowercase), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), lowercase), Ok((ch, "$")));
        }

        for ch in "$ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), lowercase),
                Err(Error::expect(Character::Lowercase).but_found(ch))
            );
        }

        assert_eq!(
            parse("", lowercase),
            Err(Error::expect(Character::Lowercase).but_found_end())
        );
    }

    #[test]
    fn test_lowercase_variant() {
        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Lowercase), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Lowercase),
                Ok((ch, "$"))
            );
        }

        for ch in "$ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Lowercase),
                Err(Error::expect(Character::Lowercase).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Character::Lowercase),
            Err(Error::expect(Character::Lowercase).but_found_end())
        );
    }

    #[test]
    fn test_uppercase() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&ch.to_string(), uppercase), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), uppercase), Ok((ch, "$")));
        }

        for ch in "$abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(
                parse(&ch.to_string(), uppercase),
                Err(Error::expect(Character::Uppercase).but_found(ch))
            );
        }

        assert_eq!(
            parse("", uppercase),
            Err(Error::expect(Character::Uppercase).but_found_end())
        );
    }

    #[test]
    fn test_uppercase_variant() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Uppercase), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Uppercase),
                Ok((ch, "$"))
            );
        }

        for ch in "$abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Uppercase),
                Err(Error::expect(Character::Uppercase).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Character::Uppercase),
            Err(Error::expect(Character::Uppercase).but_found_end())
        );
    }

    #[test]
    fn test_indent() {
        for ch in " \t".chars() {
            assert_eq!(parse(&ch.to_string(), indent), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), indent), Ok((ch, "$")));
        }

        for ch in "$\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), indent),
                Err(Error::expect(Character::Indent).but_found(ch))
            );
        }

        assert_eq!(
            parse("", indent),
            Err(Error::expect(Character::Indent).but_found_end())
        );
    }

    #[test]
    fn test_indent_variant() {
        for ch in " \t".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Indent), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Indent),
                Ok((ch, "$"))
            );
        }

        for ch in "$\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Indent),
                Err(Error::expect(Character::Indent).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Character::Indent),
            Err(Error::expect(Character::Indent).but_found_end())
        );
    }

    #[test]
    fn test_linebreak() {
        for ch in "\n\r\u{000C}".chars() {
            assert_eq!(parse(&ch.to_string(), linebreak), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), linebreak), Ok((ch, "$")));
        }

        for ch in "$ \t".chars() {
            assert_eq!(
                parse(&ch.to_string(), linebreak),
                Err(Error::expect(Character::Linebreak).but_found(ch))
            );
        }

        assert_eq!(
            parse("", linebreak),
            Err(Error::expect(Character::Linebreak).but_found_end())
        );
    }

    #[test]
    fn test_linebreak_variant() {
        for ch in "\n\r\u{000C}".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Linebreak), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Linebreak),
                Ok((ch, "$"))
            );
        }

        for ch in "$ \t".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Linebreak),
                Err(Error::expect(Character::Linebreak).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Character::Linebreak),
            Err(Error::expect(Character::Linebreak).but_found_end())
        );
    }

    #[test]
    fn test_whitespace() {
        for ch in " \t\n\t\u{000C}".chars() {
            assert_eq!(parse(&ch.to_string(), whitespace), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), whitespace), Ok((ch, "$")));
        }

        for ch in "$a".chars() {
            assert_eq!(
                parse(&ch.to_string(), whitespace),
                Err(Error::expect(Character::Whitespace).but_found(ch))
            );
        }

        assert_eq!(
            parse("", whitespace),
            Err(Error::expect(Character::Whitespace).but_found_end())
        );
    }

    #[test]
    fn test_whitespace_variant() {
        for ch in " \t\n\t\u{000C}".chars() {
            assert_eq!(parse(&ch.to_string(), Character::Whitespace), Ok((ch, "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), Character::Whitespace),
                Ok((ch, "$"))
            );
        }

        for ch in "$a".chars() {
            assert_eq!(
                parse(&ch.to_string(), Character::Whitespace),
                Err(Error::expect(Character::Whitespace).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Character::Whitespace),
            Err(Error::expect(Character::Whitespace).but_found_end())
        );
    }

    #[test]
    fn test_custom_variant() {
        assert_eq!(
            parse("", Character::custom('h')),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("$", Character::custom('h')),
            Err(Error::expect('h').but_found('$'))
        );
        assert_eq!(parse("h", Character::custom('h')), Ok(('h', "")));
        assert_eq!(parse("hello", Character::custom('h')), Ok(('h', "ello")));
        assert_eq!(parse("hello", Character::custom('h')), Ok(('h', "ello")));
    }
}
