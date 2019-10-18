use std::borrow::Borrow;

use crate::{take, Error, Parser};

pub fn character<'a, T>(ch: T) -> impl Parser<'a, char>
where
    T: Borrow<char>,
{
    move |input| {
        take(|character| character == ch.borrow())
            .parse(input)
            .map(|(_, rem)| (*ch.borrow(), rem))
    }
}

pub fn decimal(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_digit)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
}

pub fn hexadecimal(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_hexdigit)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
}

pub fn alphabetic(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_alphabetic)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
}

pub fn alphanumeric(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_alphanumeric)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
}

pub fn lowercase(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_lowercase)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
}

pub fn uppercase(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_uppercase)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
}

pub fn indent(input: &str) -> Result<(char, &str), Error> {
    take(crate::util::is_ascii_indent)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
}

pub fn linebreak(input: &str) -> Result<(char, &str), Error> {
    take(crate::util::is_ascii_linebreak)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
}

pub fn whitespace(input: &str) -> Result<(char, &str), Error> {
    take(char::is_ascii_whitespace)
        .parse(input)
        .map(|(out, rem)| (out.chars().next().unwrap(), rem))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse, Error};

    #[test]
    fn test_character() {
        assert_eq!(parse("", character('h')), Err(Error::incomplete()));
        assert_eq!(parse("$", character('h')), Err(Error::unexpected('$')));
        assert_eq!(parse("h", character('h')), Ok(('h', "")));
        assert_eq!(parse("hello", character('h')), Ok(('h', "ello")));
        assert_eq!(parse("hello", character(&'h')), Ok(('h', "ello")));
    }

    #[test]
    fn test_decimal() {
        for ch in "0123456789".chars() {
            assert_eq!(parse(&ch.to_string(), decimal), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), decimal), Ok((ch, "$")));
        }

        for ch in "$aZ\n".chars() {
            assert_eq!(parse(&ch.to_string(), decimal), Err(Error::unexpected(ch)));
        }

        assert_eq!(parse("", decimal), Err(Error::incomplete()));
    }

    #[test]
    fn test_hexdigit() {
        for ch in "0123456789abcdefABCDEF".chars() {
            assert_eq!(parse(&ch.to_string(), hexadecimal), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), hexadecimal), Ok((ch, "$")));
        }

        for ch in "$gZ\n".chars() {
            assert_eq!(
                parse(&ch.to_string(), hexadecimal),
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", hexadecimal), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", alphabetic), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", alphanumeric), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", lowercase), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", uppercase), Err(Error::incomplete()));
    }

    #[test]
    fn test_indent() {
        for ch in " \t".chars() {
            assert_eq!(parse(&ch.to_string(), indent), Ok((ch, "")));
            assert_eq!(parse(&(ch.to_string() + "$"), indent), Ok((ch, "$")));
        }

        for ch in "$\n".chars() {
            assert_eq!(parse(&ch.to_string(), indent), Err(Error::unexpected(ch)));
        }

        assert_eq!(parse("", indent), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", linebreak), Err(Error::incomplete()));
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
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", whitespace), Err(Error::incomplete()));
    }
}
