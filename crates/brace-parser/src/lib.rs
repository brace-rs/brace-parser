pub use self::error::{Error, Expect};

pub mod character;
pub mod combinator;
pub mod error;
pub mod sequence;
pub mod util;

pub trait Parser<'a, O> {
    fn parse(&self, input: &'a str) -> Result<(O, &'a str), Error>;
}

impl<'a, O, T> Parser<'a, O> for T
where
    T: Fn(&'a str) -> Result<(O, &'a str), Error>,
{
    fn parse(&self, input: &'a str) -> Result<(O, &'a str), Error> {
        (self)(input)
    }
}

impl<'a> Parser<'a, char> for char {
    fn parse(&self, input: &'a str) -> Result<(char, &'a str), Error> {
        self::character::character(self).parse(input)
    }
}

impl<'a, 'b> Parser<'a, &'a str> for &'b str {
    fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), Error> {
        self::sequence::sequence(self).parse(input)
    }
}

impl<'a> Parser<'a, &'a str> for String {
    fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), Error> {
        self::sequence::sequence(self).parse(input)
    }
}

pub fn parse<'a, P, O>(input: &'a str, parser: P) -> Result<(O, &'a str), Error>
where
    P: Parser<'a, O>,
{
    parser.parse(input)
}

pub fn take<'a, P>(predicate: P) -> impl Parser<'a, &'a str>
where
    P: Fn(&char) -> bool,
{
    move |input: &'a str| match input.chars().next() {
        Some(ch) => {
            if predicate(&ch) {
                Ok(input.split_at(ch.len_utf8()))
            } else {
                Err(Error::unexpected(ch))
            }
        }
        None => Err(Error::incomplete()),
    }
}

pub fn take_while<'a, P>(predicate: P) -> impl Parser<'a, &'a str>
where
    P: Fn(&char) -> bool,
{
    move |input: &'a str| {
        let mut iter = input.chars();
        let mut pos;

        match iter.next() {
            Some(ch) => {
                if predicate(&ch) {
                    pos = ch.len_utf8();

                    for ch in iter {
                        if !predicate(&ch) {
                            break;
                        }

                        pos += ch.len_utf8();
                    }

                    Ok(input.split_at(pos))
                } else {
                    Err(Error::unexpected(ch))
                }
            }
            None => Err(Error::incomplete()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, take, take_while, Error, Expect, Parser};

    struct Custom;

    impl<'a> Parser<'a, &'a str> for Custom {
        fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), Error> {
            take(|ch| ch == &'$').parse(input)
        }
    }

    #[test]
    fn test_parser_struct() {
        assert_eq!(parse("", Custom), Err(Error::expected((Expect::Match, ()))));
        assert_eq!(
            parse("a", Custom),
            Err(Error::expected((Expect::Match, 'a')))
        );
        assert_eq!(parse("$", Custom), Ok(("$", "")));
        assert_eq!(parse("$$", Custom), Ok(("$", "$")));
    }

    #[test]
    fn test_parser_char() {
        assert_eq!(parse("", 'h'), Err(Error::expected(('h', ()))));
        assert_eq!(parse("$", 'h'), Err(Error::expected(('h', '$'))));
        assert_eq!(parse("h", 'h'), Ok(('h', "")));
        assert_eq!(parse("hello", 'h'), Ok(('h', "ello")));
    }

    #[test]
    fn test_parser_str() {
        assert_eq!(parse("", "h"), Err(Error::expected(('h', ()))));
        assert_eq!(parse("$", "h"), Err(Error::expected(('h', '$'))));
        assert_eq!(parse("h", "h"), Ok(("h", "")));
        assert_eq!(parse("hello", "h"), Ok(("h", "ello")));
        assert_eq!(parse("", "hello"), Err(Error::expected(('h', ()))));
        assert_eq!(parse("h", "hello"), Err(Error::expected(('e', ()))));
        assert_eq!(parse("help", "hello"), Err(Error::expected(('l', 'p'))));
        assert_eq!(parse("hello", "hello"), Ok(("hello", "")));
        assert_eq!(parse("hello world", "hello"), Ok(("hello", " world")));
    }

    #[test]
    fn test_parser_string() {
        assert_eq!(parse("", "h".to_owned()), Err(Error::expected(('h', ()))));
        assert_eq!(parse("$", "h".to_owned()), Err(Error::expected(('h', '$'))));
        assert_eq!(parse("h", "h".to_owned()), Ok(("h", "")));
        assert_eq!(parse("hello", "h".to_owned()), Ok(("h", "ello")));
        assert_eq!(
            parse("", "hello".to_owned()),
            Err(Error::expected(('h', ())))
        );
        assert_eq!(
            parse("h", "hello".to_owned()),
            Err(Error::expected(('e', ())))
        );
        assert_eq!(
            parse("help", "hello".to_owned()),
            Err(Error::expected(('l', 'p')))
        );
        assert_eq!(parse("hello", "hello".to_owned()), Ok(("hello", "")));
        assert_eq!(
            parse("hello world", "hello".to_owned()),
            Ok(("hello", " world"))
        );
    }

    #[test]
    fn test_take() {
        assert_eq!(
            parse("", take(char::is_ascii_alphabetic)),
            Err(Error::expected((Expect::Match, ())))
        );
        assert_eq!(parse("h", take(char::is_ascii_alphabetic)), Ok(("h", "")));
        assert_eq!(
            parse("hello", take(char::is_ascii_alphabetic)),
            Ok(("h", "ello"))
        );
        assert_eq!(
            parse("hello world", take(char::is_ascii_alphabetic)),
            Ok(("h", "ello world"))
        );
        assert_eq!(
            parse("hello world", take(|_| true)),
            Ok(("h", "ello world"))
        );
        assert_eq!(
            parse("hello world", take(|_| false)),
            Err(Error::unexpected('h'))
        );
        assert_eq!(parse("ß", take(|_| true)), Ok(("ß", "")));
        assert_eq!(parse("ℝ", take(|_| true)), Ok(("ℝ", "")));
        assert_eq!(parse("💣", take(|_| true)), Ok(("💣", "")));
        assert_eq!(parse("ßℝ💣", take(|_| true)), Ok(("ß", "ℝ💣")));
    }

    #[test]
    fn test_take_while() {
        assert_eq!(
            parse("", take_while(char::is_ascii_alphabetic)),
            Err(Error::incomplete())
        );
        assert_eq!(
            parse("h", take_while(char::is_ascii_alphabetic)),
            Ok(("h", ""))
        );
        assert_eq!(
            parse("hello", take_while(char::is_ascii_alphabetic)),
            Ok(("hello", ""))
        );
        assert_eq!(
            parse("hello world", take_while(char::is_ascii_alphabetic)),
            Ok(("hello", " world"))
        );
        assert_eq!(
            parse("hello world", take_while(|_| true)),
            Ok(("hello world", ""))
        );
        assert_eq!(
            parse("hello world", take_while(|_| false)),
            Err(Error::unexpected('h'))
        );
        assert_eq!(parse("ß", take_while(|_| true)), Ok(("ß", "")));
        assert_eq!(parse("ℝ", take_while(|_| true)), Ok(("ℝ", "")));
        assert_eq!(parse("💣", take_while(|_| true)), Ok(("💣", "")));
        assert_eq!(parse("ßℝ💣", take_while(|_| true)), Ok(("ßℝ💣", "")));
    }
}
