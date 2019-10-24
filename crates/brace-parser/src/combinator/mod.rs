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
}
