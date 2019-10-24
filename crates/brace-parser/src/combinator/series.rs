use crate::error::Error;
use crate::parser::Parser;

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
    fn parse_series(&self, input: &'a str) -> Result<(O, &'a str), Error>;
}

impl<'a> Series<'a, ()> for () {
    fn parse_series(&self, input: &'a str) -> Result<((), &'a str), Error> {
        Ok(((), input))
    }
}

impl<'a, T, O> Series<'a, Vec<O>> for Vec<T>
where
    T: Parser<'a, O>,
{
    fn parse_series(&self, input: &'a str) -> Result<(Vec<O>, &'a str), Error> {
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
