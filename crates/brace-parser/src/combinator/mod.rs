use crate::error::Error;
use crate::parser::Parser;

pub mod branch;
pub mod series;

pub fn map<'a, M, A, B>(parser: impl Parser<'a, A>, map: M) -> impl Parser<'a, B>
where
    M: Fn(A) -> B,
{
    move |input| parser.parse(input).map(|(out, rem)| (map(out), rem))
}

pub fn map_err<'a, O, M>(parser: impl Parser<'a, O>, map: M) -> impl Parser<'a, O>
where
    M: Fn(Error) -> Error,
{
    move |input| parser.parse(input).map_err(|err| map(err))
}

pub fn context<'a, O, C>(ctx: C, parser: impl Parser<'a, O>) -> impl Parser<'a, O>
where
    C: AsRef<str>,
{
    move |input| {
        parser
            .parse(input)
            .map_err(|err| Error::context(ctx.as_ref(), err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_map() {
        assert_eq!(
            parse("hello", map("hello", |seq| seq.to_owned() + "!")),
            Ok(("hello!".to_owned(), ""))
        );
        assert_eq!(
            parse("hello", map('h', Option::Some)),
            Ok((Some('h'), "ello"))
        );
    }

    #[test]
    fn test_map_err() {
        assert_eq!(
            parse("", map_err("hello", |_| Error::expect('!'))),
            Err(Error::expect('!'))
        );
        assert_eq!(
            parse("h", map_err("hello", |err| err.but_found('!'))),
            Err(Error::expect('e').but_found('!'))
        );
    }

    #[test]
    fn test_context() {
        assert_eq!(
            parse("", context("greeting", "hello")),
            Err(Error::context(
                "greeting",
                Error::expect('h').but_found_end()
            ))
        );
        assert_eq!(
            parse("h", context("greeting", "hello")),
            Err(Error::context(
                "greeting",
                Error::expect('e').but_found_end()
            ))
        );
    }
}
