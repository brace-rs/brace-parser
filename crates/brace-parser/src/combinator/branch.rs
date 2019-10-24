use crate::error::Error;
use crate::parser::Parser;

pub fn branch<'a, O>(branch: impl Branch<'a, O>) -> impl Parser<'a, O> {
    move |input| branch.parse_branch(input)
}

pub fn either<'a, O>(a: impl Parser<'a, O>, b: impl Parser<'a, O>) -> impl Parser<'a, O> {
    move |input| a.parse(input).or_else(|_| b.parse(input))
}

pub fn optional<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, Option<O>> {
    move |input| match parser.parse(input) {
        Ok((out, rem)) => Ok((Some(out), rem)),
        Err(_) => Ok((None, input)),
    }
}

pub trait Branch<'a, O> {
    fn parse_branch(&self, input: &'a str) -> Result<(O, &'a str), Error>;
}

impl<'a> Branch<'a, ()> for () {
    fn parse_branch(&self, input: &'a str) -> Result<((), &'a str), Error> {
        Ok(((), input))
    }
}

impl<'a, T, O> Branch<'a, O> for Vec<T>
where
    T: Parser<'a, O>,
{
    fn parse_branch(&self, input: &'a str) -> Result<(O, &'a str), Error> {
        let mut out = Err(Error::invalid());

        for parser in self {
            out = parser.parse(input);

            if out.is_ok() {
                return out;
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::parser::parse;

    #[test]
    fn test_branch() {
        assert_eq!(parse("", branch(Vec::<&str>::new())), Err(Error::invalid()));
        assert_eq!(
            parse("a", branch(Vec::<&str>::new())),
            Err(Error::invalid())
        );
        assert_eq!(
            parse("", branch(vec!["a", "b", "c"])),
            Err(Error::expect('c').but_found_end())
        );
        assert_eq!(parse("a", branch(vec!["a", "b", "c"])), Ok(("a", "")));
        assert_eq!(parse("b", branch(vec!["a", "b", "c"])), Ok(("b", "")));
        assert_eq!(parse("c", branch(vec!["a", "b", "c"])), Ok(("c", "")));
        assert_eq!(parse("a!", branch(vec!["a", "b", "c"])), Ok(("a", "!")));
        assert_eq!(parse("b!", branch(vec!["a", "b", "c"])), Ok(("b", "!")));
        assert_eq!(parse("c!", branch(vec!["a", "b", "c"])), Ok(("c", "!")));
        assert_eq!(
            parse("d", branch(vec!["a", "b", "c"])),
            Err(Error::expect('c').but_found('d'))
        );
        assert_eq!(parse("", branch(())), Ok(((), "")));
        assert_eq!(parse("hello", branch(())), Ok(((), "hello")));
    }

    #[test]
    fn test_either() {
        assert_eq!(
            parse("", either("one", "two")),
            Err(Error::expect('t').but_found_end())
        );
        assert_eq!(
            parse("$", either("one", "two")),
            Err(Error::expect('t').but_found('$'))
        );
        assert_eq!(parse("one", either("one", "two")), Ok(("one", "")));
        assert_eq!(parse("two", either("one", "two")), Ok(("two", "")));
        assert_eq!(
            parse("three", either("one", "two")),
            Err(Error::expect('w').but_found('h'))
        );
        assert_eq!(
            parse("three", either("two", "one")),
            Err(Error::expect('o').but_found('t'))
        );
        assert_eq!(parse("onetwo", either("one", "two")), Ok(("one", "two")));
    }

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
}
