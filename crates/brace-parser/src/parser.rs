use crate::error::Error;

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
                Err(Error::found(ch))
            }
        }
        None => Err(Error::found_end()),
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
                    Err(Error::found(ch))
                }
            }
            None => Err(Error::found_end()),
        }
    }
}

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

impl<'a> Parser<'a, ()> for () {
    fn parse(&self, input: &'a str) -> Result<((), &'a str), Error> {
        Ok(((), input))
    }
}

impl<'a> Parser<'a, char> for char {
    fn parse(&self, input: &'a str) -> Result<(char, &'a str), Error> {
        crate::character::character(self).parse(input)
    }
}

impl<'a, 'b> Parser<'a, &'a str> for &'b str {
    fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), Error> {
        crate::sequence::sequence(self).parse(input)
    }
}

impl<'a> Parser<'a, &'a str> for String {
    fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), Error> {
        crate::sequence::sequence(self).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Custom;

    impl<'a> Parser<'a, &'a str> for Custom {
        fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), Error> {
            take(|ch| ch == &'$').parse(input)
        }
    }

    #[test]
    fn test_parser_struct() {
        assert_eq!(parse("", Custom), Err(Error::found_end()));
        assert_eq!(parse("a", Custom), Err(Error::found('a')));
        assert_eq!(parse("$", Custom), Ok(("$", "")));
        assert_eq!(parse("$$", Custom), Ok(("$", "$")));
    }

    #[test]
    fn test_parser_unit() {
        assert_eq!(parse("", ()), Ok(((), "")));
        assert_eq!(parse("hello", ()), Ok(((), "hello")));
    }

    #[test]
    fn test_parser_char() {
        assert_eq!(parse("", 'h'), Err(Error::expect('h').but_found_end()));
        assert_eq!(parse("$", 'h'), Err(Error::expect('h').but_found('$')));
        assert_eq!(parse("h", 'h'), Ok(('h', "")));
        assert_eq!(parse("hello", 'h'), Ok(('h', "ello")));
    }

    #[test]
    fn test_parser_str() {
        assert_eq!(parse("", "h"), Err(Error::expect('h').but_found_end()));
        assert_eq!(parse("$", "h"), Err(Error::expect('h').but_found('$')));
        assert_eq!(parse("h", "h"), Ok(("h", "")));
        assert_eq!(parse("hello", "h"), Ok(("h", "ello")));
        assert_eq!(parse("", "hello"), Err(Error::expect('h').but_found_end()));
        assert_eq!(parse("h", "hello"), Err(Error::expect('e').but_found_end()));
        assert_eq!(
            parse("help", "hello"),
            Err(Error::expect('l').but_found('p'))
        );
        assert_eq!(parse("hello", "hello"), Ok(("hello", "")));
        assert_eq!(parse("hello world", "hello"), Ok(("hello", " world")));
    }

    #[test]
    fn test_parser_string() {
        assert_eq!(
            parse("", "h".to_owned()),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("$", "h".to_owned()),
            Err(Error::expect('h').but_found('$'))
        );
        assert_eq!(parse("h", "h".to_owned()), Ok(("h", "")));
        assert_eq!(parse("hello", "h".to_owned()), Ok(("h", "ello")));
        assert_eq!(
            parse("", "hello".to_owned()),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("h", "hello".to_owned()),
            Err(Error::expect('e').but_found_end())
        );
        assert_eq!(
            parse("help", "hello".to_owned()),
            Err(Error::expect('l').but_found('p'))
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
            Err(Error::found_end())
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
            Err(Error::found('h'))
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
            Err(Error::found_end())
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
            Err(Error::found('h'))
        );
        assert_eq!(parse("ß", take_while(|_| true)), Ok(("ß", "")));
        assert_eq!(parse("ℝ", take_while(|_| true)), Ok(("ℝ", "")));
        assert_eq!(parse("💣", take_while(|_| true)), Ok(("💣", "")));
        assert_eq!(parse("ßℝ💣", take_while(|_| true)), Ok(("ßℝ💣", "")));
    }
}
