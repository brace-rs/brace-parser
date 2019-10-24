use crate::parser::{Output, Parser};

pub fn series<'a, O>(series: impl Series<'a, O>) -> impl Parser<'a, O> {
    move |input| series.parse_series(input)
}

pub fn pair<'a, A, B>(a: impl Parser<'a, A>, b: impl Parser<'a, B>) -> impl Parser<'a, (A, B)> {
    move |input| {
        a.parse(input)
            .and_then(|(oa, rem)| b.parse(rem).map(|(ob, rem)| ((oa, ob), rem)))
    }
}

pub fn trio<'a, A, B, C>(
    a: impl Parser<'a, A>,
    b: impl Parser<'a, B>,
    c: impl Parser<'a, C>,
) -> impl Parser<'a, (A, B, C)> {
    move |input| {
        a.parse(input)
            .and_then(|(oa, rem)| b.parse(rem).map(|(ob, rem)| ((oa, ob), rem)))
            .and_then(|((oa, ob), rem)| c.parse(rem).map(|(oc, rem)| ((oa, ob, oc), rem)))
    }
}

pub fn leading<'a, O, L>(
    leading: impl Parser<'a, L>,
    parser: impl Parser<'a, O>,
) -> impl Parser<'a, O> {
    move |input| leading.parse(input).and_then(|(_, rem)| parser.parse(rem))
}

pub fn trailing<'a, O, T>(
    parser: impl Parser<'a, O>,
    trailing: impl Parser<'a, T>,
) -> impl Parser<'a, O> {
    move |input| {
        parser
            .parse(input)
            .and_then(|(out, rem)| trailing.parse(rem).map(|(_, rem)| (out, rem)))
    }
}

pub fn delimited<'a, A, B, C>(
    a: impl Parser<'a, A>,
    b: impl Parser<'a, B>,
    c: impl Parser<'a, C>,
) -> impl Parser<'a, B> {
    leading(a, trailing(b, c))
}

pub fn list<'a, T, S>(
    parser: impl Parser<'a, T>,
    separator: impl Parser<'a, S>,
) -> impl Parser<'a, Vec<T>> {
    move |input| {
        parser.parse(input).and_then(|(out, mut rem)| {
            let mut out = vec![out];

            loop {
                if let Ok((_, next)) = separator.parse(rem) {
                    if let Ok((item, next)) = parser.parse(next) {
                        out.push(item);
                        rem = next;

                        continue;
                    }
                }

                return Ok((out, rem));
            }
        })
    }
}

pub trait Series<'a, O> {
    fn parse_series(&self, input: &'a str) -> Output<'a, O>;
}

impl<'a> Series<'a, ()> for () {
    fn parse_series(&self, input: &'a str) -> Output<'a, ()> {
        Ok(((), input))
    }
}

impl<'a, T, O> Series<'a, Vec<O>> for Vec<T>
where
    T: Parser<'a, O>,
{
    fn parse_series(&self, input: &'a str) -> Output<'a, Vec<O>> {
        let mut out = Vec::new();
        let mut rem = input;

        for parser in self {
            match parser.parse(rem) {
                Ok((item, next)) => {
                    out.push(item);
                    rem = next;
                }
                Err(err) => return Err(err),
            }
        }

        Ok((out, rem))
    }
}

macro_rules! impl_series {
    ($(($a:tt, $b:ident, $c:ident),)+) => {
        impl_series!(@iter $(($a, $b, $c),)+;);
    };

    (@iter ($a:tt, $b:ident, $c:ident),; $(($d:tt, $e:ident, $f:ident),)*) => {
        impl_series!(@impl $(($d, $e, $f),)* ($a, $b, $c),);
    };

    (@iter ($a:tt, $b:ident, $c:ident), $(($d:tt, $e:ident, $f:ident),)+; $(($g:tt, $h:ident, $i:ident),)*) => {
        impl_series!(@impl $(($g, $h, $i),)* ($a, $b, $c),);
        impl_series!(@iter $(($d, $e, $f),)*; $(($g, $h, $i),)* ($a, $b, $c),);
    };

    (@impl $(($idx:tt, $T:ident, $O:ident),)+) => {
        impl<'a, $($T, $O,)+> Series<'a, ($($O,)+)> for ($($T,)+)
        where
            $($T: Parser<'a, $O>,)+
        {
            fn parse_series(&self, input: &'a str) -> Output<'a, ($($O,)+)> {
                impl_series!(@start self; input; $($idx,)+)
            }
        }
    };

    (@start $self:expr; $input:expr; $($idx:tt,)+) => {
        impl_series!(@inner $self; $input; a, b, c, d, e, f, g, h, i, j, k, l,;; $($idx,)+);
    };

    (@inner $self:expr; $input:expr; $out:ident, $($arg:ident,)*; $($acc:ident,)*; $i:tt,) => {
        match $self.$i.parse($input) {
            Ok(($out, rem)) => Ok((($($acc,)* $out,), rem)),
            Err(err) => Err(err),
        }
    };

    (@inner $self:expr; $input:expr; $out:ident, $($arg:ident,)*; $($acc:ident,)*; $i:tt, $($idx:tt,)+) => {
        match $self.$i.parse($input) {
            Ok(($out, rem)) => impl_series!(@inner $self; rem; $($arg,)*; $($acc,)* $out,; $($idx,)+),
            Err(err) => Err(err),
        }
    };
}

impl_series! {
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
    use crate::error::Error;
    use crate::parser::parse;
    use crate::sequence::{alphabetic, whitespace};

    #[test]
    fn test_series() {
        assert_eq!(
            parse("", series(vec!["hello", " ", "world"])),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("hello", series(vec!["hello", " ", "world"])),
            Err(Error::expect(' ').but_found_end())
        );
        assert_eq!(
            parse("hello ", series(vec!["hello", " ", "world"])),
            Err(Error::expect('w').but_found_end())
        );
        assert_eq!(
            parse("hello world", series(vec!["hello", " ", "world"])),
            Ok((vec!["hello", " ", "world"], ""))
        );
        assert_eq!(
            parse("hello world!", series(vec!["hello", " ", "world"])),
            Ok((vec!["hello", " ", "world"], "!"))
        );
        assert_eq!(
            parse("hello universe!", series(vec!["hello", " ", "world"])),
            Err(Error::expect('w').but_found('u'))
        );
        assert_eq!(parse("", series(())), Ok(((), "")));
        assert_eq!(parse("hello", series(())), Ok(((), "hello")));
        assert_eq!(
            parse("", series(("hello", ' ', "world"))),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("hello", series(("hello", ' ', "world"))),
            Err(Error::expect(' ').but_found_end())
        );
        assert_eq!(
            parse("hello ", series(("hello", ' ', "world"))),
            Err(Error::expect('w').but_found_end())
        );
        assert_eq!(
            parse("hello world", series(("hello", ' ', "world"))),
            Ok((("hello", ' ', "world"), ""))
        );
        assert_eq!(
            parse("hello world!", series(("hello", ' ', "world"))),
            Ok((("hello", ' ', "world"), "!"))
        );
        assert_eq!(
            parse("hello universe!", series(("hello", ' ', "world"))),
            Err(Error::expect('w').but_found('u'))
        );
        assert_eq!(
            parse("hello world!", series(('h',))),
            Ok((('h',), "ello world!"))
        );
        assert_eq!(
            parse("hello world!", series(('h', 'e'))),
            Ok((('h', 'e'), "llo world!"))
        );
        assert_eq!(
            parse("hello world!", series(('h', 'e', 'l'))),
            Ok((('h', 'e', 'l'), "lo world!"))
        );
        assert_eq!(
            parse("hello world!", series(('h', 'e', 'l', 'l'))),
            Ok((('h', 'e', 'l', 'l'), "o world!"))
        );
        assert_eq!(
            parse("hello world!", series(('h', 'e', 'l', 'l', 'o'))),
            Ok((('h', 'e', 'l', 'l', 'o'), " world!"))
        );
        assert_eq!(
            parse("hello world!", series(('h', 'e', 'l', 'l', 'o', ' '))),
            Ok((('h', 'e', 'l', 'l', 'o', ' '), "world!"))
        );
        assert_eq!(
            parse("hello world!", series(('h', 'e', 'l', 'l', 'o', ' ', 'w'))),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w'), "orld!"))
        );
        assert_eq!(
            parse(
                "hello world!",
                series(('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o'))
            ),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o'), "rld!"))
        );
        assert_eq!(
            parse(
                "hello world!",
                series(('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r'))
            ),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r'), "ld!"))
        );
        assert_eq!(
            parse(
                "hello world!",
                series(('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l'))
            ),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l'), "d!"))
        );
        assert_eq!(
            parse(
                "hello world!",
                series(('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'))
            ),
            Ok((('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'), "!"))
        );
        assert_eq!(
            parse(
                "hello world!",
                series(('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!'))
            ),
            Ok((
                ('h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!'),
                ""
            ))
        );
    }

    #[test]
    fn test_pair() {
        assert_eq!(
            parse("", pair("hello", " world")),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("hello", pair("hello", " world")),
            Err(Error::expect(' ').but_found_end())
        );
        assert_eq!(
            parse("hello world", pair("hello", " world")),
            Ok((("hello", " world"), ""))
        );
        assert_eq!(
            parse("hello universe", pair("hello", " world")),
            Err(Error::expect('w').but_found('u'))
        );
        assert_eq!(
            parse("goodbye world", pair("hello", " world")),
            Err(Error::expect('h').but_found('g'))
        );
        assert_eq!(
            parse(
                "hello \n world",
                pair("hello", pair(whitespace, alphabetic))
            ),
            Ok((("hello", (" \n ", "world")), ""))
        );
        assert_eq!(
            parse(
                "hello \n universe",
                pair("hello", pair(whitespace, alphabetic))
            ),
            Ok((("hello", (" \n ", "universe")), ""))
        );
    }

    #[test]
    fn test_trio() {
        assert_eq!(
            parse("", trio("hello", ' ', "world")),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("hello", trio("hello", ' ', "world")),
            Err(Error::expect(' ').but_found_end())
        );
        assert_eq!(
            parse("hello world", trio("hello", ' ', "world")),
            Ok((("hello", ' ', "world"), ""))
        );
        assert_eq!(
            parse("hello universe", trio("hello", ' ', "world")),
            Err(Error::expect('w').but_found('u'))
        );
        assert_eq!(
            parse("goodbye world", trio("hello", ' ', "world")),
            Err(Error::expect('h').but_found('g'))
        );
        assert_eq!(
            parse("hello \n world", trio("hello", whitespace, alphabetic)),
            Ok((("hello", " \n ", "world"), ""))
        );
        assert_eq!(
            parse("hello \n universe", trio("hello", whitespace, alphabetic)),
            Ok((("hello", " \n ", "universe"), ""))
        );
    }

    #[test]
    fn test_leading() {
        assert_eq!(
            parse("", leading("hello world", "!")),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("hello", leading("hello world", "!")),
            Err(Error::expect(' ').but_found_end())
        );
        assert_eq!(
            parse("hello world", leading("hello world", "!")),
            Err(Error::expect('!').but_found_end())
        );
        assert_eq!(
            parse("hello world!", leading("hello world", "!")),
            Ok(("!", ""))
        );
        assert_eq!(
            parse("hello world!!", leading("hello world", "!")),
            Ok(("!", "!"))
        );
        assert_eq!(
            parse("hello world?", leading("hello world", "!")),
            Err(Error::expect('!').but_found('?'))
        );
        assert_eq!(
            parse("hello universe!", leading("hello world", "!")),
            Err(Error::expect('w').but_found('u'))
        );
    }

    #[test]
    fn test_trailing() {
        assert_eq!(
            parse("", trailing("hello world", "!")),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("hello", trailing("hello world", "!")),
            Err(Error::expect(' ').but_found_end())
        );
        assert_eq!(
            parse("hello world", trailing("hello world", "!")),
            Err(Error::expect('!').but_found_end())
        );
        assert_eq!(
            parse("hello world!", trailing("hello world", "!")),
            Ok(("hello world", ""))
        );
        assert_eq!(
            parse("hello world!!", trailing("hello world", "!")),
            Ok(("hello world", "!"))
        );
        assert_eq!(
            parse("hello world?", trailing("hello world", "!")),
            Err(Error::expect('!').but_found('?'))
        );
        assert_eq!(
            parse("hello universe!", trailing("hello world", "!")),
            Err(Error::expect('w').but_found('u'))
        );
    }

    #[test]
    fn test_delimited() {
        assert_eq!(
            parse("(hello)", delimited('(', "hello", ')')),
            Ok(("hello", ""))
        );
        assert_eq!(
            parse("\"hello\"", delimited('"', "hello", '"')),
            Ok(("hello", ""))
        );
        assert_eq!(
            parse("\"hello\" world", delimited('"', "hello", '"')),
            Ok(("hello", " world"))
        );
        assert_eq!(
            parse("\"hello", delimited('"', "hello", '"')),
            Err(Error::expect('"').but_found_end())
        );
        assert_eq!(
            parse("hello", delimited('"', "hello", '"')),
            Err(Error::expect('"').but_found('h'))
        );
    }

    #[test]
    fn test_list() {
        assert_eq!(
            parse("", list('a', ',')),
            Err(Error::expect('a').but_found_end())
        );
        assert_eq!(parse("a", list('a', ',')), Ok((vec!['a'], "")));
        assert_eq!(parse("a,a", list('a', ',')), Ok((vec!['a', 'a'], "")));
        assert_eq!(
            parse("a,a,a", list('a', ',')),
            Ok((vec!['a', 'a', 'a'], ""))
        );
        assert_eq!(
            parse("a,a,a b", list('a', ',')),
            Ok((vec!['a', 'a', 'a'], " b"))
        );
        assert_eq!(
            parse("a,a,a,b", list('a', ',')),
            Ok((vec!['a', 'a', 'a'], ",b"))
        );
    }
}
