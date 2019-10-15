use crate::{take, take_while, Error, Parser};

pub fn digit(input: &str) -> Result<(&str, &str), Error> {
    take(char::is_ascii_digit).parse(input)
}

pub fn digits(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_digit).parse(input)
}

pub fn hexdigit(input: &str) -> Result<(&str, &str), Error> {
    take(char::is_ascii_hexdigit).parse(input)
}

pub fn hexdigits(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_hexdigit).parse(input)
}

pub fn alphabetic(input: &str) -> Result<(&str, &str), Error> {
    take(char::is_ascii_alphabetic).parse(input)
}

pub fn alphabetics(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_alphabetic).parse(input)
}

pub fn alphanumeric(input: &str) -> Result<(&str, &str), Error> {
    take(char::is_ascii_alphanumeric).parse(input)
}

pub fn alphanumerics(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_alphanumeric).parse(input)
}

pub fn lowercase(input: &str) -> Result<(&str, &str), Error> {
    take(char::is_ascii_lowercase).parse(input)
}

pub fn lowercases(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_lowercase).parse(input)
}

pub fn uppercase(input: &str) -> Result<(&str, &str), Error> {
    take(char::is_ascii_uppercase).parse(input)
}

pub fn uppercases(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_uppercase).parse(input)
}

pub fn space(input: &str) -> Result<(&str, &str), Error> {
    take(|ch| ch == &' ').parse(input)
}

pub fn spaces(input: &str) -> Result<(&str, &str), Error> {
    take_while(|ch| ch == &' ').parse(input)
}

pub fn tab(input: &str) -> Result<(&str, &str), Error> {
    take(|ch| ch == &'\t').parse(input)
}

pub fn tabs(input: &str) -> Result<(&str, &str), Error> {
    take_while(|ch| ch == &'\t').parse(input)
}

pub fn indent(input: &str) -> Result<(&str, &str), Error> {
    take(|ch| ch == &' ' || ch == &'\t').parse(input)
}

pub fn indents(input: &str) -> Result<(&str, &str), Error> {
    take_while(|ch| ch == &' ' || ch == &'\t').parse(input)
}

pub fn linebreak(input: &str) -> Result<(&str, &str), Error> {
    take(|ch| ch == &'\n' || ch == &'\r' || ch == &'\u{000C}').parse(input)
}

pub fn linebreaks(input: &str) -> Result<(&str, &str), Error> {
    take_while(|ch| ch == &'\n' || ch == &'\r' || ch == &'\u{000C}').parse(input)
}

pub fn whitespace(input: &str) -> Result<(&str, &str), Error> {
    take(char::is_ascii_whitespace).parse(input)
}

pub fn whitespaces(input: &str) -> Result<(&str, &str), Error> {
    take_while(char::is_ascii_whitespace).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse, Error};

    #[test]
    fn test_digit() {
        for ch in "0123456789".chars() {
            assert_eq!(parse(&format!("{}", ch), digit), Ok((&*ch.to_string(), "")));
        }

        assert_eq!(parse("", digit), Err(Error::incomplete()));
        assert_eq!(parse("$", digit), Err(Error::unexpected('$')));
        assert_eq!(parse("a", digit), Err(Error::unexpected('a')));
        assert_eq!(parse("Z", digit), Err(Error::unexpected('Z')));
    }

    #[test]
    fn test_digits() {
        for ch in "0123456789".chars() {
            assert_eq!(
                parse(&format!("{}", ch), digits),
                Ok((&*ch.to_string(), ""))
            );
        }

        assert_eq!(parse("", digits), Err(Error::incomplete()));
        assert_eq!(parse("$", digits), Err(Error::unexpected('$')));
        assert_eq!(parse("a", digits), Err(Error::unexpected('a')));
        assert_eq!(parse("Z", digits), Err(Error::unexpected('Z')));
        assert_eq!(parse("0123456789", digits), Ok(("0123456789", "")));
        assert_eq!(parse("0123456789$", digits), Ok(("0123456789", "$")));
    }

    #[test]
    fn test_hexdigit() {
        for ch in "0123456789abcdefABCDEF".chars() {
            assert_eq!(
                parse(&format!("{}", ch), hexdigit),
                Ok((&*ch.to_string(), ""))
            );
        }

        assert_eq!(parse("", hexdigit), Err(Error::incomplete()));
        assert_eq!(parse("$", hexdigit), Err(Error::unexpected('$')));
        assert_eq!(parse("g", hexdigit), Err(Error::unexpected('g')));
        assert_eq!(parse("Z", hexdigit), Err(Error::unexpected('Z')));
    }

    #[test]
    fn test_hexdigits() {
        for ch in "0123456789abcdefABCDEF".chars() {
            assert_eq!(
                parse(&format!("{}", ch), hexdigits),
                Ok((&*ch.to_string(), ""))
            );
        }

        assert_eq!(parse("", hexdigits), Err(Error::incomplete()));
        assert_eq!(parse("$", hexdigits), Err(Error::unexpected('$')));
        assert_eq!(parse("g", hexdigits), Err(Error::unexpected('g')));
        assert_eq!(parse("Z", hexdigits), Err(Error::unexpected('Z')));
        assert_eq!(
            parse("0123456789abcdefABCDEF", hexdigits),
            Ok(("0123456789abcdefABCDEF", ""))
        );
        assert_eq!(
            parse("0123456789abcdefABCDEF$", hexdigits),
            Ok(("0123456789abcdefABCDEF", "$"))
        );
    }

    #[test]
    fn test_alphabetic() {
        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&format!("{}", ch), alphabetic),
                Ok((&*ch.to_string(), ""))
            );
        }

        assert_eq!(parse("", alphabetic), Err(Error::incomplete()));
        assert_eq!(parse("$", alphabetic), Err(Error::unexpected('$')));
        assert_eq!(parse("0", alphabetic), Err(Error::unexpected('0')));
    }

    #[test]
    fn test_alphabetics() {
        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&format!("{}", ch), alphabetics),
                Ok((&*ch.to_string(), ""))
            );
        }

        assert_eq!(parse("", alphabetics), Err(Error::incomplete()));
        assert_eq!(parse("$", alphabetics), Err(Error::unexpected('$')));
        assert_eq!(parse("0", alphabetics), Err(Error::unexpected('0')));
        assert_eq!(
            parse(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                alphabetics
            ),
            Ok(("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", ""))
        );
        assert_eq!(
            parse(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$",
                alphabetics
            ),
            Ok(("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", "$"))
        );
    }

    #[test]
    fn test_alphanumeric() {
        for ch in "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&format!("{}", ch), alphanumeric),
                Ok((&*ch.to_string(), ""))
            );
        }

        assert_eq!(parse("", alphanumeric), Err(Error::incomplete()));
        assert_eq!(parse("$", alphanumeric), Err(Error::unexpected('$')));
    }

    #[test]
    fn test_alphanumerics() {
        for ch in "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&format!("{}", ch), alphanumerics),
                Ok((&*ch.to_string(), ""))
            );
        }

        assert_eq!(parse("", alphanumerics), Err(Error::incomplete()));
        assert_eq!(parse("$", alphanumerics), Err(Error::unexpected('$')));
        assert_eq!(
            parse(
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                alphanumerics
            ),
            Ok((
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                ""
            ))
        );
        assert_eq!(
            parse(
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$",
                alphanumerics
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
                parse(&format!("{}", ch), lowercase),
                Ok((&*ch.to_string(), ""))
            );
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
    fn test_lowercases() {
        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(
                parse(&format!("{}", ch), lowercases),
                Ok((&*ch.to_string(), ""))
            );
        }

        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&format!("{}", ch), lowercases),
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", lowercases), Err(Error::incomplete()));
        assert_eq!(parse("$", lowercases), Err(Error::unexpected('$')));
        assert_eq!(
            parse("abcdefghijklmnopqrstuvwxyz", alphanumerics),
            Ok(("abcdefghijklmnopqrstuvwxyz", ""))
        );
        assert_eq!(
            parse("abcdefghijklmnopqrstuvwxyz$", alphanumerics),
            Ok(("abcdefghijklmnopqrstuvwxyz", "$"))
        );
    }

    #[test]
    fn test_uppercase() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&format!("{}", ch), uppercase),
                Ok((&*ch.to_string(), ""))
            );
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
    fn test_uppercases() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert_eq!(
                parse(&format!("{}", ch), uppercases),
                Ok((&*ch.to_string(), ""))
            );
        }

        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert_eq!(
                parse(&format!("{}", ch), uppercases),
                Err(Error::unexpected(ch))
            );
        }

        assert_eq!(parse("", uppercases), Err(Error::incomplete()));
        assert_eq!(parse("$", uppercases), Err(Error::unexpected('$')));
        assert_eq!(
            parse("ABCDEFGHIJKLMNOPQRSTUVWXYZ", alphanumerics),
            Ok(("ABCDEFGHIJKLMNOPQRSTUVWXYZ", ""))
        );
        assert_eq!(
            parse("ABCDEFGHIJKLMNOPQRSTUVWXYZ$", alphanumerics),
            Ok(("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "$"))
        );
    }

    #[test]
    fn test_space() {
        assert_eq!(parse("", space), Err(Error::incomplete()));
        assert_eq!(parse("$", space), Err(Error::unexpected('$')));
        assert_eq!(parse("\n", space), Err(Error::unexpected('\n')));
        assert_eq!(parse("\t", space), Err(Error::unexpected('\t')));
        assert_eq!(parse(" ", space), Ok((" ", "")));
    }

    #[test]
    fn test_spaces() {
        assert_eq!(parse("", spaces), Err(Error::incomplete()));
        assert_eq!(parse("$", spaces), Err(Error::unexpected('$')));
        assert_eq!(parse("\n", spaces), Err(Error::unexpected('\n')));
        assert_eq!(parse("\t", spaces), Err(Error::unexpected('\t')));
        assert_eq!(parse(" ", spaces), Ok((" ", "")));
        assert_eq!(parse("   ", spaces), Ok(("   ", "")));
    }

    #[test]
    fn test_tab() {
        assert_eq!(parse("", tab), Err(Error::incomplete()));
        assert_eq!(parse("$", tab), Err(Error::unexpected('$')));
        assert_eq!(parse("\n", tab), Err(Error::unexpected('\n')));
        assert_eq!(parse(" ", tab), Err(Error::unexpected(' ')));
        assert_eq!(parse("\t", tab), Ok(("\t", "")));
    }

    #[test]
    fn test_tabs() {
        assert_eq!(parse("", tabs), Err(Error::incomplete()));
        assert_eq!(parse("$", tabs), Err(Error::unexpected('$')));
        assert_eq!(parse("\n", tabs), Err(Error::unexpected('\n')));
        assert_eq!(parse(" ", tabs), Err(Error::unexpected(' ')));
        assert_eq!(parse("\t", tabs), Ok(("\t", "")));
        assert_eq!(parse("\t\t\t", tabs), Ok(("\t\t\t", "")));
    }

    #[test]
    fn test_indent() {
        assert_eq!(parse("", indent), Err(Error::incomplete()));
        assert_eq!(parse("$", indent), Err(Error::unexpected('$')));
        assert_eq!(parse("\n", indent), Err(Error::unexpected('\n')));
        assert_eq!(parse("\t", indent), Ok(("\t", "")));
        assert_eq!(parse(" ", indent), Ok((" ", "")));
    }

    #[test]
    fn test_indents() {
        assert_eq!(parse("", indents), Err(Error::incomplete()));
        assert_eq!(parse("$", indents), Err(Error::unexpected('$')));
        assert_eq!(parse("\n", indents), Err(Error::unexpected('\n')));
        assert_eq!(parse("\t", indents), Ok(("\t", "")));
        assert_eq!(parse(" ", indents), Ok((" ", "")));
        assert_eq!(parse(" \t \t ", indents), Ok((" \t \t ", "")));
    }

    #[test]
    fn test_linebreak() {
        assert_eq!(parse("", linebreak), Err(Error::incomplete()));
        assert_eq!(parse("$", linebreak), Err(Error::unexpected('$')));
        assert_eq!(parse(" ", linebreak), Err(Error::unexpected(' ')));
        assert_eq!(parse("\t", linebreak), Err(Error::unexpected('\t')));
        assert_eq!(parse("\n", linebreak), Ok(("\n", "")));
        assert_eq!(parse("\r", linebreak), Ok(("\r", "")));
        assert_eq!(parse("\u{000C}", linebreak), Ok(("\u{000C}", "")));
    }

    #[test]
    fn test_linebreaks() {
        assert_eq!(parse("", linebreaks), Err(Error::incomplete()));
        assert_eq!(parse("$", linebreaks), Err(Error::unexpected('$')));
        assert_eq!(parse(" ", linebreaks), Err(Error::unexpected(' ')));
        assert_eq!(parse("\t", linebreaks), Err(Error::unexpected('\t')));
        assert_eq!(parse("\n", linebreaks), Ok(("\n", "")));
        assert_eq!(parse("\r", linebreaks), Ok(("\r", "")));
        assert_eq!(parse("\u{000C}", linebreaks), Ok(("\u{000C}", "")));
        assert_eq!(parse("\n\r\u{000C}", linebreaks), Ok(("\n\r\u{000C}", "")));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(parse("", whitespace), Err(Error::incomplete()));
        assert_eq!(parse("$", whitespace), Err(Error::unexpected('$')));
        assert_eq!(parse(" ", whitespace), Ok((" ", "")));
        assert_eq!(parse("\t", whitespace), Ok(("\t", "")));
        assert_eq!(parse("\n", whitespace), Ok(("\n", "")));
        assert_eq!(parse("\r", whitespace), Ok(("\r", "")));
        assert_eq!(parse("\u{000C}", whitespace), Ok(("\u{000C}", "")));
    }

    #[test]
    fn test_whitespaces() {
        assert_eq!(parse("", whitespaces), Err(Error::incomplete()));
        assert_eq!(parse("$", whitespaces), Err(Error::unexpected('$')));
        assert_eq!(parse(" ", whitespaces), Ok((" ", "")));
        assert_eq!(parse("\t", whitespaces), Ok(("\t", "")));
        assert_eq!(parse("\n", whitespaces), Ok(("\n", "")));
        assert_eq!(parse("\r", whitespaces), Ok(("\r", "")));
        assert_eq!(parse("\u{000C}", whitespaces), Ok(("\u{000C}", "")));
        assert_eq!(
            parse(" \t\n\r\u{000C}", whitespaces),
            Ok((" \t\n\r\u{000C}", ""))
        );
    }
}
