use crate::{take_while, Error, Parser};

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
                        return Err(Error::unexpected(character));
                    }
                }
                None => return Err(Error::incomplete()),
            }
        }

        Ok(input.split_at(pos))
    }
}

pub fn decimal(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_digit).parse(input)
}

pub fn hexadecimal(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_hexdigit).parse(input)
}

pub fn alphabetic(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_alphabetic).parse(input)
}

pub fn alphanumeric(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_alphanumeric).parse(input)
}

pub fn lowercase(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_lowercase).parse(input)
}

pub fn uppercase(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_uppercase).parse(input)
}

pub fn indent(input: &str) -> Result<(&str, &str), Error> {
    take_while(crate::util::is_ascii_indent).parse(input)
}

pub fn linebreak(input: &str) -> Result<(&str, &str), Error> {
    take_while(crate::util::is_ascii_linebreak).parse(input)
}

pub fn whitespace(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_whitespace).parse(input)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Sequence {
    Decimal,
    Hexadecimal,
    Alphabetic,
    Alphanumeric,
    Lowercase,
    Uppercase,
    Indent,
    Linebreak,
    Whitespace,
}

impl<'a> Parser<'a, &'a str> for Sequence {
    fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), Error> {
        match self {
            Self::Decimal => decimal.parse(input),
            Self::Hexadecimal => hexadecimal.parse(input),
            Self::Alphabetic => alphabetic.parse(input),
            Self::Alphanumeric => alphanumeric.parse(input),
            Self::Lowercase => lowercase.parse(input),
            Self::Uppercase => uppercase.parse(input),
            Self::Indent => indent.parse(input),
            Self::Linebreak => linebreak.parse(input),
            Self::Whitespace => whitespace.parse(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse, Error};

    #[test]
    fn test_sequence() {
        assert_eq!(parse("", sequence("hello")), Err(Error::incomplete()));
        assert_eq!(parse("h", sequence("hello")), Err(Error::incomplete()));
        assert_eq!(
            parse("help", sequence("hello")),
            Err(Error::unexpected('p'))
        );
        assert_eq!(parse("hello", sequence("hello")), Ok(("hello", "")));
        assert_eq!(parse("hello$", sequence("hello")), Ok(("hello", "$")));
        assert_eq!(parse("hello", sequence("")), Ok(("", "hello")));
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
            assert_eq!(parse(&ch.to_string(), decimal), Err(Error::unexpected(ch)));
        }

        assert_eq!(parse("", decimal), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", Sequence::Decimal), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", hexadecimal), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", Sequence::Hexadecimal), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", alphabetic), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", Sequence::Alphabetic), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", alphanumeric), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", Sequence::Alphanumeric), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", lowercase), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", Sequence::Lowercase), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", uppercase), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", Sequence::Uppercase), Err(Error::incomplete()));
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
            assert_eq!(parse(&ch.to_string(), indent), Err(Error::unexpected(ch)));
        }

        assert_eq!(parse("", indent), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", Sequence::Indent), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", linebreak), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", Sequence::Linebreak), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", whitespace), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", Sequence::Whitespace), Err(Error::incomplete()));
        assert_eq!(
            parse(" \t\n\r\u{000C}", Sequence::Whitespace),
            Ok((" \t\n\r\u{000C}", ""))
        );
        assert_eq!(
            parse(" \t\n\r\u{000C}$", Sequence::Whitespace),
            Ok((" \t\n\r\u{000C}", "$"))
        );
    }
}
