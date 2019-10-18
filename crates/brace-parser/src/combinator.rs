use crate::Parser;

pub fn optional<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, Option<O>> {
    move |input| match parser.parse(input) {
        Ok((out, rem)) => Ok((Some(out), rem)),
        Err(_) => Ok((None, input)),
    }
}

pub fn either<'a, O>(a: impl Parser<'a, O>, b: impl Parser<'a, O>) -> impl Parser<'a, O> {
    move |input| a.parse(input).or_else(|_| b.parse(input))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::{alphabetic, whitespace};
    use crate::{parse, Error};

    #[test]
    fn test_optional() {
        assert_eq!(parse("", optional("hello")), Ok((None, "")));
        assert_eq!(parse("$", optional("hello")), Ok((None, "$")));
        assert_eq!(parse("hello", optional("hello")), Ok((Some("hello"), "")));
        assert_eq!(
            parse("hello world", optional("hello")),
            Ok((Some("hello"), " world"))
        );
    }

    #[test]
    fn test_either() {
        assert_eq!(parse("", either("one", "two")), Err(Error::incomplete()));
        assert_eq!(
            parse("$", either("one", "two")),
            Err(Error::unexpected('$'))
        );
        assert_eq!(parse("one", either("one", "two")), Ok(("one", "")));
        assert_eq!(parse("two", either("one", "two")), Ok(("two", "")));
        assert_eq!(
            parse("three", either("one", "two")),
            Err(Error::unexpected('h'))
        );
        assert_eq!(
            parse("three", either("two", "one")),
            Err(Error::unexpected('t'))
        );
        assert_eq!(parse("onetwo", either("one", "two")), Ok(("one", "two")));
    }

    #[test]
    fn test_pair() {
        assert_eq!(parse("", pair("hello", " world")), Err(Error::incomplete()));
        assert_eq!(
            parse("hello", pair("hello", " world")),
            Err(Error::incomplete())
        );
        assert_eq!(
            parse("hello world", pair("hello", " world")),
            Ok((("hello", " world"), ""))
        );
        assert_eq!(
            parse("hello universe", pair("hello", " world")),
            Err(Error::unexpected('u'))
        );
        assert_eq!(
            parse("goodbye world", pair("hello", " world")),
            Err(Error::unexpected('g'))
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
            Err(Error::incomplete())
        );
        assert_eq!(
            parse("hello", trio("hello", ' ', "world")),
            Err(Error::incomplete())
        );
        assert_eq!(
            parse("hello world", trio("hello", ' ', "world")),
            Ok((("hello", ' ', "world"), ""))
        );
        assert_eq!(
            parse("hello universe", trio("hello", ' ', "world")),
            Err(Error::unexpected('u'))
        );
        assert_eq!(
            parse("goodbye world", trio("hello", ' ', "world")),
            Err(Error::unexpected('g'))
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
            Err(Error::incomplete())
        );
        assert_eq!(
            parse("hello", leading("hello world", "!")),
            Err(Error::incomplete())
        );
        assert_eq!(
            parse("hello world", leading("hello world", "!")),
            Err(Error::incomplete())
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
            Err(Error::unexpected('?'))
        );
        assert_eq!(
            parse("hello universe!", leading("hello world", "!")),
            Err(Error::unexpected('u'))
        );
    }

    #[test]
    fn test_trailing() {
        assert_eq!(
            parse("", trailing("hello world", "!")),
            Err(Error::incomplete())
        );
        assert_eq!(
            parse("hello", trailing("hello world", "!")),
            Err(Error::incomplete())
        );
        assert_eq!(
            parse("hello world", trailing("hello world", "!")),
            Err(Error::incomplete())
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
            Err(Error::unexpected('?'))
        );
        assert_eq!(
            parse("hello universe!", trailing("hello world", "!")),
            Err(Error::unexpected('u'))
        );
    }
}
