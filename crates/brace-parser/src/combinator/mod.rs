use crate::parser::Parser;

pub mod branch;
pub mod series;

pub fn map<'a, M, A, B>(parser: impl Parser<'a, A>, map: M) -> impl Parser<'a, B>
where
    M: Fn(A) -> B,
{
    move |input| parser.parse(input).map(|(out, rem)| (map(out), rem))
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
}
