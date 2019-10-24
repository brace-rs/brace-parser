use std::fmt;

use crate::error::Error;
use crate::parser::{take_while, Output, Parser};

pub fn sequence<'a, T>(sequence: T) -> impl Parser<'a, &'a str>
where
    T: AsRef<str>,
{
    move |input: &'a str| {
        let mut iter = input.chars();
        let mut pos = 0;

        for ch in sequence.as_ref().chars() {
            match iter.next() {
                Some(character) => {
                    if ch == character {
                        pos += ch.len_utf8();
                    } else {
                        return Err(Error::expect(ch).but_found(character));
                    }
                }
                None => return Err(Error::expect(ch).but_found_end()),
            }
        }

        Ok(input.split_at(pos))
    }
}

pub fn any(input: &str) -> Output<&str> {
    take_while(|_| true)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Any))
}

pub fn decimal(input: &str) -> Output<&str> {
    take_while(char::is_ascii_digit)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Decimal))
}

pub fn hexadecimal(input: &str) -> Output<&str> {
    take_while(char::is_ascii_hexdigit)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Hexadecimal))
}

pub fn alphabetic(input: &str) -> Output<&str> {
    take_while(char::is_ascii_alphabetic)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Alphabetic))
}

pub fn alphanumeric(input: &str) -> Output<&str> {
    take_while(char::is_ascii_alphanumeric)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Alphanumeric))
}

pub fn lowercase(input: &str) -> Output<&str> {
    take_while(char::is_ascii_lowercase)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Lowercase))
}

pub fn uppercase(input: &str) -> Output<&str> {
    take_while(char::is_ascii_uppercase)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Uppercase))
}

pub fn indent(input: &str) -> Output<&str> {
    take_while(crate::character::is_ascii_indent)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Indent))
}

pub fn linebreak(input: &str) -> Output<&str> {
    take_while(crate::character::is_ascii_linebreak)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Linebreak))
}

pub fn whitespace(input: &str) -> Output<&str> {
    take_while(char::is_ascii_whitespace)
        .parse(input)
        .map_err(|err| err.but_expect(Sequence::Whitespace))
}

#[derive(Clone, Debug, PartialEq)]
pub enum Sequence {
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
    Custom(String),
}

impl Sequence {
    pub fn custom<T>(sequence: T) -> Self
    where
        T: Into<String>,
    {
        Self::Custom(sequence.into())
    }
}

impl<'a> Parser<'a, &'a str> for Sequence {
    fn parse(&self, input: &'a str) -> Output<'a, &'a str> {
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
            Self::Custom(string) => string.parse(input),
        }
    }
}

impl fmt::Display for Sequence {
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
            Self::Custom(string) => write!(f, "\"{}\"", string),
        }
    }
}

impl From<&str> for Sequence {
    fn from(from: &str) -> Self {
        Self::Custom(from.to_owned())
    }
}

impl From<String> for Sequence {
    fn from(from: String) -> Self {
        Self::Custom(from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::parser::parse;

    #[test]
    fn test_sequence() {
        assert_eq!(
            parse("", sequence("hello")),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("h", sequence("hello")),
            Err(Error::expect('e').but_found_end())
        );
        assert_eq!(
            parse("help", sequence("hello")),
            Err(Error::expect('l').but_found('p'))
        );
        assert_eq!(parse("hello", sequence("hello")), Ok(("hello", "")));
        assert_eq!(parse("hello$", sequence("hello")), Ok(("hello", "$")));
        assert_eq!(parse("hello", sequence("")), Ok(("", "hello")));
    }

    #[test]
    fn test_any() {
        for ch in "$0123456789 \t\n\r\u{000C}".chars() {
            assert_eq!(parse(&ch.to_string(), any), Ok((&*ch.to_string(), "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), any),
                Ok((&*(ch.to_string() + "$"), ""))
            );
        }

        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&ch.to_string(), any), Ok((&*ch.to_string(), "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), any),
                Ok((&*(ch.to_string() + "$"), ""))
            );
        }

        assert_eq!(
            parse("", any),
            Err(Error::expect(Sequence::Any).but_found_end())
        );
    }

    #[test]
    fn test_any_variant() {
        for ch in "$0123456789 \t\n\r\u{000C}".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Any),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Any),
                Ok((&*(ch.to_string() + "$"), ""))
            );
        }

        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Any),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Any),
                Ok((&*(ch.to_string() + "$"), ""))
            );
        }

        assert_eq!(
            parse("", Sequence::Any),
            Err(Error::expect(Sequence::Any).but_found_end())
        );
    }

    #[test]
    fn test_decimal() {
        for ch in "0123456789".chars() {
            assert_eq!(parse(&ch.to_string(), decimal), Ok((&*ch.to_string(), "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), decimal),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$aZ\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), decimal),
                Err(Error::expect(Sequence::Decimal).but_found(ch))
            );
        }

        assert_eq!(
            parse("", decimal),
            Err(Error::expect(Sequence::Decimal).but_found_end())
        );
        assert_eq!(parse("0123456789", decimal), Ok(("0123456789", "")));
        assert_eq!(parse("0123456789$", decimal), Ok(("0123456789", "$")));
    }

    #[test]
    fn test_decimal_variant() {
        for ch in "0123456789".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Decimal),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Decimal),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$aZ\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Decimal),
                Err(Error::expect(Sequence::Decimal).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Sequence::Decimal),
            Err(Error::expect(Sequence::Decimal).but_found_end())
        );
        assert_eq!(
            parse("0123456789", Sequence::Decimal),
            Ok(("0123456789", ""))
        );
        assert_eq!(
            parse("0123456789$", Sequence::Decimal),
            Ok(("0123456789", "$"))
        );
    }

    #[test]
    fn test_hexadecimal() {
        for ch in "0123456789abcdefABCDEF".chars() {
            assert_eq!(
                parse(&ch.to_string(), hexadecimal),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), hexadecimal),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$gZ\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), hexadecimal),
                Err(Error::expect(Sequence::Hexadecimal).but_found(ch))
            );
        }

        assert_eq!(
            parse("", hexadecimal),
            Err(Error::expect(Sequence::Hexadecimal).but_found_end())
        );
        assert_eq!(
            parse("0123456789abcdefABCDEF", hexadecimal),
            Ok(("0123456789abcdefABCDEF", ""))
        );
        assert_eq!(
            parse("0123456789abcdefABCDEF$", hexadecimal),
            Ok(("0123456789abcdefABCDEF", "$"))
        );
    }

    #[test]
    fn test_hexadecimal_variant() {
        for ch in "0123456789abcdefABCDEF".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Hexadecimal),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Hexadecimal),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$gZ\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Hexadecimal),
                Err(Error::expect(Sequence::Hexadecimal).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Sequence::Hexadecimal),
            Err(Error::expect(Sequence::Hexadecimal).but_found_end())
        );
        assert_eq!(
            parse("0123456789abcdefABCDEF", Sequence::Hexadecimal),
            Ok(("0123456789abcdefABCDEF", ""))
        );
        assert_eq!(
            parse("0123456789abcdefABCDEF$", Sequence::Hexadecimal),
            Ok(("0123456789abcdefABCDEF", "$"))
        );
    }

    #[test]
    fn test_alphabetic() {
        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), alphabetic),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), alphabetic),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$0 \n".chars() {
            assert_eq!(
                parse(&ch.to_string(), alphabetic),
                Err(Error::expect(Sequence::Alphabetic).but_found(ch))
            );
        }

        assert_eq!(
            parse("", alphabetic),
            Err(Error::expect(Sequence::Alphabetic).but_found_end())
        );
        assert_eq!(
            parse(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                alphabetic
            ),
            Ok(("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", ""))
        );
        assert_eq!(
            parse(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$",
                alphabetic
            ),
            Ok(("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", "$"))
        );
    }

    #[test]
    fn test_alphabetic_variant() {
        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Alphabetic),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Alphabetic),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$0 \n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Alphabetic),
                Err(Error::expect(Sequence::Alphabetic).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Sequence::Alphabetic),
            Err(Error::expect(Sequence::Alphabetic).but_found_end())
        );
        assert_eq!(
            parse(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                Sequence::Alphabetic
            ),
            Ok(("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", ""))
        );
        assert_eq!(
            parse(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$",
                Sequence::Alphabetic
            ),
            Ok(("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", "$"))
        );
    }

    #[test]
    fn test_alphanumeric() {
        for ch in "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), alphanumeric),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), alphanumeric),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$ \n".chars() {
            assert_eq!(
                parse(&ch.to_string(), alphanumeric),
                Err(Error::expect(Sequence::Alphanumeric).but_found(ch))
            );
        }

        assert_eq!(
            parse("", alphanumeric),
            Err(Error::expect(Sequence::Alphanumeric).but_found_end())
        );
        assert_eq!(
            parse(
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                alphanumeric
            ),
            Ok((
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                ""
            ))
        );
        assert_eq!(
            parse(
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$",
                alphanumeric
            ),
            Ok((
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                "$"
            ))
        );
    }

    #[test]
    fn test_alphanumeric_variant() {
        for ch in "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Alphanumeric),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Alphanumeric),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$ \n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Alphanumeric),
                Err(Error::expect(Sequence::Alphanumeric).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Sequence::Alphanumeric),
            Err(Error::expect(Sequence::Alphanumeric).but_found_end())
        );
        assert_eq!(
            parse(
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                Sequence::Alphanumeric
            ),
            Ok((
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                ""
            ))
        );
        assert_eq!(
            parse(
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$",
                Sequence::Alphanumeric
            ),
            Ok((
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                "$"
            ))
        );
    }

    #[test]
    fn test_lowercase() {
        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(
                parse(&ch.to_string(), lowercase),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), lowercase),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), lowercase),
                Err(Error::expect(Sequence::Lowercase).but_found(ch))
            );
        }

        assert_eq!(
            parse("", lowercase),
            Err(Error::expect(Sequence::Lowercase).but_found_end())
        );
        assert_eq!(
            parse("abcdefghijklmnopqrstuvwxyz", lowercase),
            Ok(("abcdefghijklmnopqrstuvwxyz", ""))
        );
        assert_eq!(
            parse("abcdefghijklmnopqrstuvwxyz$", lowercase),
            Ok(("abcdefghijklmnopqrstuvwxyz", "$"))
        );
    }

    #[test]
    fn test_lowercase_variant() {
        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Lowercase),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Lowercase),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Lowercase),
                Err(Error::expect(Sequence::Lowercase).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Sequence::Lowercase),
            Err(Error::expect(Sequence::Lowercase).but_found_end())
        );
        assert_eq!(
            parse("abcdefghijklmnopqrstuvwxyz", Sequence::Lowercase),
            Ok(("abcdefghijklmnopqrstuvwxyz", ""))
        );
        assert_eq!(
            parse("abcdefghijklmnopqrstuvwxyz$", Sequence::Lowercase),
            Ok(("abcdefghijklmnopqrstuvwxyz", "$"))
        );
    }

    #[test]
    fn test_uppercase() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), uppercase),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), uppercase),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(
                parse(&ch.to_string(), uppercase),
                Err(Error::expect(Sequence::Uppercase).but_found(ch))
            );
        }

        assert_eq!(
            parse("", uppercase),
            Err(Error::expect(Sequence::Uppercase).but_found_end())
        );
        assert_eq!(
            parse("ABCDEFGHIJKLMNOPQRSTUVWXYZ", uppercase),
            Ok(("ABCDEFGHIJKLMNOPQRSTUVWXYZ", ""))
        );
        assert_eq!(
            parse("ABCDEFGHIJKLMNOPQRSTUVWXYZ$", uppercase),
            Ok(("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "$"))
        );
    }

    #[test]
    fn test_uppercase_variant() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Uppercase),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Uppercase),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Uppercase),
                Err(Error::expect(Sequence::Uppercase).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Sequence::Uppercase),
            Err(Error::expect(Sequence::Uppercase).but_found_end())
        );
        assert_eq!(
            parse("ABCDEFGHIJKLMNOPQRSTUVWXYZ", Sequence::Uppercase),
            Ok(("ABCDEFGHIJKLMNOPQRSTUVWXYZ", ""))
        );
        assert_eq!(
            parse("ABCDEFGHIJKLMNOPQRSTUVWXYZ$", Sequence::Uppercase),
            Ok(("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "$"))
        );
    }

    #[test]
    fn test_indent() {
        for ch in " \t".chars() {
            assert_eq!(parse(&ch.to_string(), indent), Ok((&*ch.to_string(), "")));
            assert_eq!(
                parse(&(ch.to_string() + "$"), indent),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), indent),
                Err(Error::expect(Sequence::Indent).but_found(ch))
            );
        }

        assert_eq!(
            parse("", indent),
            Err(Error::expect(Sequence::Indent).but_found_end())
        );
        assert_eq!(parse(" \t \t ", indent), Ok((" \t \t ", "")));
    }

    #[test]
    fn test_indent_variant() {
        for ch in " \t".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Indent),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Indent),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Indent),
                Err(Error::expect(Sequence::Indent).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Sequence::Indent),
            Err(Error::expect(Sequence::Indent).but_found_end())
        );
        assert_eq!(parse(" \t \t ", Sequence::Indent), Ok((" \t \t ", "")));
    }

    #[test]
    fn test_linebreak() {
        for ch in "\n\r\u{000C}".chars() {
            assert_eq!(
                parse(&ch.to_string(), linebreak),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), linebreak),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$ \t".chars() {
            assert_eq!(
                parse(&ch.to_string(), linebreak),
                Err(Error::expect(Sequence::Linebreak).but_found(ch))
            );
        }

        assert_eq!(
            parse("", linebreak),
            Err(Error::expect(Sequence::Linebreak).but_found_end())
        );
        assert_eq!(parse("\n\r\u{000C}", linebreak), Ok(("\n\r\u{000C}", "")));
        assert_eq!(parse("\n\r\u{000C}$", linebreak), Ok(("\n\r\u{000C}", "$")));
    }

    #[test]
    fn test_linebreak_variant() {
        for ch in "\n\r\u{000C}".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Linebreak),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Linebreak),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$ \t".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Linebreak),
                Err(Error::expect(Sequence::Linebreak).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Sequence::Linebreak),
            Err(Error::expect(Sequence::Linebreak).but_found_end())
        );
        assert_eq!(
            parse("\n\r\u{000C}", Sequence::Linebreak),
            Ok(("\n\r\u{000C}", ""))
        );
        assert_eq!(
            parse("\n\r\u{000C}$", Sequence::Linebreak),
            Ok(("\n\r\u{000C}", "$"))
        );
    }

    #[test]
    fn test_whitespace() {
        for ch in " \t\n\r\u{000C}".chars() {
            assert_eq!(
                parse(&ch.to_string(), whitespace),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), whitespace),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$a".chars() {
            assert_eq!(
                parse(&ch.to_string(), whitespace),
                Err(Error::expect(Sequence::Whitespace).but_found(ch))
            );
        }

        assert_eq!(
            parse("", whitespace),
            Err(Error::expect(Sequence::Whitespace).but_found_end())
        );
        assert_eq!(
            parse(" \t\n\r\u{000C}", whitespace),
            Ok((" \t\n\r\u{000C}", ""))
        );
        assert_eq!(
            parse(" \t\n\r\u{000C}$", whitespace),
            Ok((" \t\n\r\u{000C}", "$"))
        );
    }

    #[test]
    fn test_whitespace_variant() {
        for ch in " \t\n\r\u{000C}".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Whitespace),
                Ok((&*ch.to_string(), ""))
            );
            assert_eq!(
                parse(&(ch.to_string() + "$"), Sequence::Whitespace),
                Ok((&*ch.to_string(), "$"))
            );
        }

        for ch in "$a".chars() {
            assert_eq!(
                parse(&ch.to_string(), Sequence::Whitespace),
                Err(Error::expect(Sequence::Whitespace).but_found(ch))
            );
        }

        assert_eq!(
            parse("", Sequence::Whitespace),
            Err(Error::expect(Sequence::Whitespace).but_found_end())
        );
        assert_eq!(
            parse(" \t\n\r\u{000C}", Sequence::Whitespace),
            Ok((" \t\n\r\u{000C}", ""))
        );
        assert_eq!(
            parse(" \t\n\r\u{000C}$", Sequence::Whitespace),
            Ok((" \t\n\r\u{000C}", "$"))
        );
    }

    #[test]
    fn test_custom_variant() {
        assert_eq!(
            parse("", Sequence::custom("hello")),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("h", Sequence::custom("hello")),
            Err(Error::expect('e').but_found_end())
        );
        assert_eq!(
            parse("help", Sequence::custom("hello")),
            Err(Error::expect('l').but_found('p'))
        );
        assert_eq!(parse("hello", Sequence::custom("hello")), Ok(("hello", "")));
        assert_eq!(
            parse("hello$", Sequence::custom("hello")),
            Ok(("hello", "$"))
        );
        assert_eq!(parse("hello", Sequence::custom("")), Ok(("", "hello")));
    }
}
