use crate::Parser;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::{alphabetics, whitespaces};
    use crate::{parse, Error};

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
                pair("hello", pair(whitespaces, alphabetics))
            ),
            Ok((("hello", (" \n ", "world")), ""))
        );
        assert_eq!(
            parse(
                "hello \n universe",
                pair("hello", pair(whitespaces, alphabetics))
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
            parse("hello \n world", trio("hello", whitespaces, alphabetics)),
            Ok((("hello", " \n ", "world"), ""))
        );
        assert_eq!(
            parse("hello \n universe", trio("hello", whitespaces, alphabetics)),
            Ok((("hello", " \n ", "universe"), ""))
        );
    }
}
