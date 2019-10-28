use crate::combinator::series::Series;
use crate::error::Error;

pub type Output<'a, O> = Result<(O, &'a str), Error>;

pub fn parse<'a, P, O>(input: &'a str, parser: P) -> Output<'a, O>
where
    P: Parser<'a, O>,
{
    parser.parse(input)
}

pub fn take<'a, P>(predicate: P) -> impl Parser<'a, &'a str>
where
    P: Fn(char) -> bool,
{
    move |input: &'a str| match input.chars().next() {
        Some(ch) => {
            if predicate(ch) {
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
    P: Fn(char) -> bool,
{
    move |input: &'a str| {
        let mut iter = input.chars();
        let mut pos;

        match iter.next() {
            Some(ch) => {
                if predicate(ch) {
                    pos = ch.len_utf8();

                    for ch in iter {
                        if !predicate(ch) {
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
    fn parse(&self, input: &'a str) -> Output<'a, O>;
}

impl<'a, O, T> Parser<'a, O> for T
where
    T: Fn(&'a str) -> Output<'a, O>,
{
    fn parse(&self, input: &'a str) -> Output<'a, O> {
        (self)(input)
    }
}

impl<'a> Parser<'a, ()> for () {
    fn parse(&self, input: &'a str) -> Output<'a, ()> {
        Ok(((), input))
    }
}

impl<'a> Parser<'a, char> for char {
    fn parse(&self, input: &'a str) -> Output<'a, char> {
        take(|ch| ch == *self)
            .parse(input)
            .map(|(_, rem)| (*self, rem))
            .map_err(|err| err.but_expect(*self))
    }
}

impl<'a, 'b> Parser<'a, &'a str> for &'b str {
    fn parse(&self, input: &'a str) -> Output<'a, &'a str> {
        let mut iter = input.chars();
        let mut idx = 0;

        for ch in self.chars() {
            match iter.next() {
                Some(character) => {
                    if ch == character {
                        idx += ch.len_utf8();
                    } else {
                        return Err(Error::expect(ch).but_found(character));
                    }
                }
                None => return Err(Error::expect(ch).but_found_end()),
            }
        }

        Ok(input.split_at(idx))
    }
}

impl<'a> Parser<'a, &'a str> for String {
    fn parse(&self, input: &'a str) -> Output<'a, &'a str> {
        Parser::parse(&(&*self as &str), input)
    }
}

macro_rules! impl_parser {
    ($(($a:tt, $b:ident, $c:ident),)+) => {
        impl_parser!(@iter $(($a, $b, $c),)+;);
    };

    (@iter ($a:tt, $b:ident, $c:ident),; $(($d:tt, $e:ident, $f:ident),)*) => {
        impl_parser!(@impl $(($d, $e, $f),)* ($a, $b, $c),);
    };

    (@iter ($a:tt, $b:ident, $c:ident), $(($d:tt, $e:ident, $f:ident),)+; $(($g:tt, $h:ident, $i:ident),)*) => {
        impl_parser!(@impl $(($g, $h, $i),)* ($a, $b, $c),);
        impl_parser!(@iter $(($d, $e, $f),)*; $(($g, $h, $i),)* ($a, $b, $c),);
    };

    (@impl $(($idx:tt, $T:ident, $O:ident),)+) => {
        impl<'a, $($T, $O,)+> Parser<'a, ($($O,)+)> for ($($T,)+)
        where
            $($T: Parser<'a, $O>,)+
        {
            fn parse(&self, input: &'a str) -> Output<'a, ($($O,)+)> {
                self.parse_series(input)
            }
        }
    };
}

impl_parser! {
    (0, A, N),
    (1, B, O),
    (2, C, P),
    (3, D, Q),
    (4, E, R),
    (5, F, S),
    (6, G, T),
    (7, H, U),
    (8, I, V),
    (9, J, W),
    (10, K, X),
    (11, L, Y),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::is_alphabetic;

    struct Custom;

    impl<'a> Parser<'a, &'a str> for Custom {
        fn parse(&self, input: &'a str) -> Output<'a, &'a str> {
            take(|ch| ch == '$').parse(input)
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
    fn test_parser_tuple() {
        assert_eq!(
            parse("", ("hello", ' ', "world")),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("hello", ("hello", ' ', "world")),
            Err(Error::expect(' ').but_found_end())
        );
        assert_eq!(
            parse("hello ", ("hello", ' ', "world")),
            Err(Error::expect('w').but_found_end())
        );
        assert_eq!(
            parse("hello world", ("hello", ' ', "world")),
            Ok((("hello", ' ', "world"), ""))
        );
        assert_eq!(
            parse("hello world!", ("hello", ' ', "world")),
            Ok((("hello", ' ', "world"), "!"))
        );
        assert_eq!(
            parse("hello universe!", ("hello", ' ', "world")),
            Err(Error::expect('w').but_found('u'))
        );
        assert_eq!(parse("hello world!", ('h',)), Ok((('h',), "ello world!")));
        assert_eq!(
            parse("hello world!", ('h', 'e')),
            Ok((('h', 'e'), "llo world!"))
        );
        assert_eq!(
            parse("hello world!", ('h', 'e', 'l')),
            Ok((('h', 'e', 'l'), "lo world!"))
        );
        assert_eq!(
            parse("hello world!", ('h', 'e', 'l', 'l')),
            Ok((('h', 'e', 'l', 'l'), "o world!"))
        );
        assert_eq!(
            parse("hello world!", ('h', 'e', 'l', 'l', 'o')),
            Ok((('h', 'e', 'l', 'l', 'o'), " world!"))
        );
        assert_eq!(
            parse("hello world!", ('h', 'e', 'l', 'l', 'o', ' ')),
            Ok((('h', 'e', 'l', 'l', 'o', ' '), "world!"))
        );
        assert_eq!(
            parse("hello world!", ('h', 'e', 'l', 'l', 'o', ' ', 'w')),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w'), "orld!"))
        );
        assert_eq!(
            parse("hello world!", ('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o')),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o'), "rld!"))
        );
        assert_eq!(
            parse(
                "hello world!",
                ('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r')
            ),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r'), "ld!"))
        );
        assert_eq!(
            parse(
                "hello world!",
                ('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l')
            ),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l'), "d!"))
        );
        assert_eq!(
            parse(
                "hello world!",
                ('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd')
            ),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'), "!"))
        );
        assert_eq!(
            parse(
                "hello world!",
                ('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!')
            ),
            Ok((
                ('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!'),
                ""
            ))
        );
    }

    #[test]
    fn test_take() {
        assert_eq!(parse("", take(is_alphabetic)), Err(Error::found_end()));
        assert_eq!(parse("h", take(is_alphabetic)), Ok(("h", "")));
        assert_eq!(parse("hello", take(is_alphabetic)), Ok(("h", "ello")));
        assert_eq!(
            parse("hello world", take(is_alphabetic)),
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
            parse("", take_while(is_alphabetic)),
            Err(Error::found_end())
        );
        assert_eq!(parse("h", take_while(is_alphabetic)), Ok(("h", "")));
        assert_eq!(parse("hello", take_while(is_alphabetic)), Ok(("hello", "")));
        assert_eq!(
            parse("hello world", take_while(is_alphabetic)),
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
