use crate::{sequence, Error};

pub fn digit(input: &str) -> Result<(char, &str), Error> {
    sequence::digit(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn hexdigit(input: &str) -> Result<(char, &str), Error> {
    sequence::hexdigit(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn alphabetic(input: &str) -> Result<(char, &str), Error> {
    sequence::alphabetic(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn alphanumeric(input: &str) -> Result<(char, &str), Error> {
    sequence::alphanumeric(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn lowercase(input: &str) -> Result<(char, &str), Error> {
    sequence::lowercase(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn uppercase(input: &str) -> Result<(char, &str), Error> {
    sequence::uppercase(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn space(input: &str) -> Result<(char, &str), Error> {
    sequence::space(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn tab(input: &str) -> Result<(char, &str), Error> {
    sequence::tab(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn indent(input: &str) -> Result<(char, &str), Error> {
    sequence::indent(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn linebreak(input: &str) -> Result<(char, &str), Error> {
    sequence::linebreak(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

pub fn whitespace(input: &str) -> Result<(char, &str), Error> {
    sequence::whitespace(input).and_then(|(out, rem)| Ok((out.chars().next().unwrap(), rem)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse, Error};

    #[test]
    fn test_digit() {
        for ch in "0123456789".chars() {
            assert_eq!(parse(&format!("{}", ch), digit), Ok((ch, "")));
        }

        assert_eq!(parse("", digit), Err(Error::incomplete()));
        assert_eq!(parse("$", digit), Err(Error::unexpected('$')));
        assert_eq!(parse("a", digit), Err(Error::unexpected('a')));
        assert_eq!(parse("Z", digit), Err(Error::unexpected('Z')));
    }

    #[test]
    fn test_hexdigit() {
        for ch in "0123456789abcdefABCDEF".chars() {
            assert_eq!(parse(&format!("{}", ch), hexdigit), Ok((ch, "")));
        }

        assert_eq!(parse("", hexdigit), Err(Error::incomplete()));
        assert_eq!(parse("$", hexdigit), Err(Error::unexpected('$')));
        assert_eq!(parse("g", hexdigit), Err(Error::unexpected('g')));
        assert_eq!(parse("Z", hexdigit), Err(Error::unexpected('Z')));
    }

    #[test]
    fn test_alphabetic() {
        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&format!("{}", ch), alphabetic), Ok((ch, "")));
        }

        assert_eq!(parse("", alphabetic), Err(Error::incomplete()));
        assert_eq!(parse("$", alphabetic), Err(Error::unexpected('$')));
        assert_eq!(parse("0", alphabetic), Err(Error::unexpected('0')));
    }

    #[test]
    fn test_alphanumeric() {
        for ch in "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&format!("{}", ch), alphanumeric), Ok((ch, "")));
        }

        assert_eq!(parse("", alphanumeric), Err(Error::incomplete()));
        assert_eq!(parse("$", alphanumeric), Err(Error::unexpected('$')));
    }

    #[test]
    fn test_lowercase() {
        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(parse(&format!("{}", ch), lowercase), Ok((ch, "")));
        }

        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&format!("{}", ch), lowercase),
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", lowercase), Err(Error::incomplete()));
        assert_eq!(parse("$", lowercase), Err(Error::unexpected('$')));
    }

    #[test]
    fn test_uppercase() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(parse(&format!("{}", ch), uppercase), Ok((ch, "")));
        }

        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(
                parse(&format!("{}", ch), uppercase),
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", uppercase), Err(Error::incomplete()));
        assert_eq!(parse("$", uppercase), Err(Error::unexpected('$')));
    }

    #[test]
    fn test_space() {
        assert_eq!(parse("", space), Err(Error::incomplete()));
        assert_eq!(parse("$", space), Err(Error::unexpected('$')));
        assert_eq!(parse("\n", space), Err(Error::unexpected('\n')));
        assert_eq!(parse("\t", space), Err(Error::unexpected('\t')));
        assert_eq!(parse(" ", space), Ok((' ', "")));
    }

    #[test]
    fn test_tab() {
        assert_eq!(parse("", tab), Err(Error::incomplete()));
        assert_eq!(parse("$", tab), Err(Error::unexpected('$')));
        assert_eq!(parse("\n", tab), Err(Error::unexpected('\n')));
        assert_eq!(parse(" ", tab), Err(Error::unexpected(' ')));
        assert_eq!(parse("\t", tab), Ok(('\t', "")));
    }

    #[test]
    fn test_indent() {
        assert_eq!(parse("", indent), Err(Error::incomplete()));
        assert_eq!(parse("$", indent), Err(Error::unexpected('$')));
        assert_eq!(parse("\n", indent), Err(Error::unexpected('\n')));
        assert_eq!(parse("\t", indent), Ok(('\t', "")));
        assert_eq!(parse(" ", indent), Ok((' ', "")));
    }

    #[test]
    fn test_linebreak() {
        assert_eq!(parse("", linebreak), Err(Error::incomplete()));
        assert_eq!(parse("$", linebreak), Err(Error::unexpected('$')));
        assert_eq!(parse(" ", linebreak), Err(Error::unexpected(' ')));
        assert_eq!(parse("\t", linebreak), Err(Error::unexpected('\t')));
        assert_eq!(parse("\r", linebreak), Ok(('\r', "")));
        assert_eq!(parse("\u{000C}", linebreak), Ok(('\u{000C}', "")));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(parse("", whitespace), Err(Error::incomplete()));
        assert_eq!(parse("$", whitespace), Err(Error::unexpected('$')));
        assert_eq!(parse(" ", whitespace), Ok((' ', "")));
        assert_eq!(parse("\t", whitespace), Ok(('\t', "")));
        assert_eq!(parse("\n", whitespace), Ok(('\n', "")));
        assert_eq!(parse("\r", whitespace), Ok(('\r', "")));
        assert_eq!(parse("\u{000C}", whitespace), Ok(('\u{000C}', "")));
    }
}
