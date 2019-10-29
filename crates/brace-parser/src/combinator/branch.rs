use crate::error::Error;
use crate::parser::{Output, Parser};

pub fn branch<'a, O>(branch: impl Branch<'a, O>) -> impl Parser<'a, O> {
    move |input| branch.parse_branch(input)
}

pub fn either<'a, O>(a: impl Parser<'a, O>, b: impl Parser<'a, O>) -> impl Parser<'a, O> {
    move |input| {
        a.parse(input).or_else(|err| match err {
            Error::Pass(_) => b.parse(input),
            Error::Fail(inner) => Err(Error::Fail(inner)),
        })
    }
}

pub fn optional<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, Option<O>> {
    move |input| match parser.parse(input) {
        Ok((out, rem)) => Ok((Some(out), rem)),
        Err(err) => match err {
            Error::Pass(_) => Ok((None, input)),
            Error::Fail(inner) => Err(Error::Fail(inner)),
        },
    }
}

pub trait Branch<'a, O> {
    fn parse_branch(&self, input: &'a str) -> Output<'a, O>;
}

impl<'a> Branch<'a, ()> for () {
    fn parse_branch(&self, input: &'a str) -> Output<'a, ()> {
        Ok(((), input))
    }
}

impl<'a, T, O> Branch<'a, O> for Vec<T>
where
    T: Parser<'a, O>,
{
    fn parse_branch(&self, input: &'a str) -> Output<'a, O> {
        let mut out = Err(Error::invalid());

        for parser in self {
            match parser.parse(input) {
                Ok(res) => return Ok(res),
                Err(Error::Fail(inner)) => return Err(Error::Fail(inner)),
                Err(Error::Pass(inner)) => out = Err(Error::Pass(inner)),
            }
        }

        out
    }
}

macro_rules! impl_branch {
    ($(($a:tt, $b:ident),)+) => {
        impl_branch!(@iter $(($a, $b),)+;);
    };

    (@iter ($a:tt, $b:ident),; $(($c:tt, $d:ident),)*) => {
        impl_branch!(@impl $(($c, $d),)* ($a, $b),);
    };

    (@iter ($a:tt, $b:ident), $(($c:tt, $d:ident),)+; $(($e:tt, $f:ident),)*) => {
        impl_branch!(@impl $(($e, $f),)* ($a, $b),);
        impl_branch!(@iter $(($c, $d),)*; $(($e, $f),)* ($a, $b),);
    };

    (@impl $(($idx:tt, $T:ident),)+) => {
        impl<'a, O, $($T,)+> Branch<'a, O> for ($($T,)+)
        where
            $($T: Parser<'a, O>,)+
        {
            fn parse_branch(&self, input: &'a str) -> Output<'a, O> {
                impl_branch!(@start self; input; $($idx,)+)
            }
        }
    };

    (@start $self:expr; $input:expr; $($idx:tt,)+) => {
        impl_branch!(@inner $self; $input; $($idx,)+);
    };

    (@inner $self:expr; $input:expr; $i:tt,) => {
        $self.$i.parse($input)
    };

    (@inner $self:expr; $input:expr; $i:tt, $($idx:tt,)+) => {
        match $self.$i.parse($input) {
            Ok(res) => Ok(res),
            Err(Error::Fail(inner)) => Err(Error::Fail(inner)),
            Err(_) => impl_branch!(@inner $self; $input; $($idx,)+),
        }
    };
}

impl_branch! {
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G),
    (7, H),
    (8, I),
    (9, J),
    (10, K),
    (11, L),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::parser::parse;

    fn pass(_: &str) -> Output<&str> {
        Err(Error::expect('!'))
    }

    fn fail(_: &str) -> Output<&str> {
        Err(Error::invalid())
    }

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
        assert_eq!(parse("a", branch(vec![pass])), Err(Error::expect('!')));
        assert_eq!(parse("a", branch(vec![fail])), Err(Error::invalid()));
        assert_eq!(parse("", branch(())), Ok(((), "")));
        assert_eq!(parse("hello", branch(())), Ok(((), "hello")));
        assert_eq!(
            parse("", branch(("a", "b", "c"))),
            Err(Error::expect('c').but_found_end())
        );
        assert_eq!(parse("a", branch(("a", "b", "c"))), Ok(("a", "")));
        assert_eq!(parse("b", branch(("a", "b", "c"))), Ok(("b", "")));
        assert_eq!(parse("c", branch(("a", "b", "c"))), Ok(("c", "")));
        assert_eq!(parse("a!", branch(("a", "b", "c"))), Ok(("a", "!")));
        assert_eq!(parse("b!", branch(("a", "b", "c"))), Ok(("b", "!")));
        assert_eq!(parse("c!", branch(("a", "b", "c"))), Ok(("c", "!")));
        assert_eq!(
            parse("d", branch(("a", "b", "c"))),
            Err(Error::expect('c').but_found('d'))
        );
        assert_eq!(parse("a", branch(("a", pass, "b"))), Ok(("a", "")));
        assert_eq!(parse("b", branch(("a", pass, "b"))), Ok(("b", "")));
        assert_eq!(parse("a", branch(("a", fail, "b"))), Ok(("a", "")));
        assert_eq!(parse("b", branch(("a", fail, "b"))), Err(Error::invalid()));
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
        assert_eq!(parse("one", either(pass, "one")), Ok(("one", "")));
        assert_eq!(parse("one", either(fail, "one")), Err(Error::invalid()));
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
        assert_eq!(parse("", optional(pass)), Ok((None, "")));
        assert_eq!(parse("", optional(fail)), Err(Error::invalid()));
    }
}
