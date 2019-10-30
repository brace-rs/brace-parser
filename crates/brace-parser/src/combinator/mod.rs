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

pub fn consume<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, &'a str> {
    move |input| {
        parser
            .parse(input)
            .map(|(_, rem)| input.split_at(input.len() - rem.len()))
    }
}

pub fn not<'a>(parser: impl Parser<'a, char>) -> impl Parser<'a, char> {
    move |input| match parser.parse(input) {
        Ok((ch, _)) => Err(Error::found(ch)),
        Err(_) => match input.chars().next() {
            Some(ch) => Ok((ch, &input[ch.len_utf8()..])),
            None => Err(Error::found_end()),
        },
    }
}

pub fn escaped<'a>(
    valid: impl Parser<'a, char>,
    escaped: impl Parser<'a, char>,
) -> impl Parser<'a, &'a str> {
    move |input: &'a str| {
        let mut iter = input.chars();
        let mut idx = 0;

        while let Some(ch) = iter.next() {
            if ch == '\\' {
                idx += ch.len_utf8();

                match iter.next() {
                    Some(ch) => match escaped.parse(&input[idx..]) {
                        Ok(_) => {
                            idx += ch.len_utf8();
                        }
                        Err(err) => return Err(err),
                    },
                    None => return Err(Error::found('\\')),
                }
            } else {
                match valid.parse(&input[idx..]) {
                    Ok(_) => {
                        idx += ch.len_utf8();
                    }
                    Err(err) => {
                        if idx == 0 {
                            return Err(err);
                        }

                        break;
                    }
                }
            }
        }

        if idx == 0 {
            Err(Error::found_end())
        } else {
            Ok(input.split_at(idx))
        }
    }
}

pub fn unescape<'a>(
    parser: impl Parser<'a, &'a str>,
    escaped: impl Parser<'a, char>,
) -> impl Parser<'a, String> {
    move |input: &'a str| {
        parser.parse(input).and_then(|(input, rem)| {
            let mut iter = input.chars();
            let mut idx = 0;
            let mut out = String::new();

            while let Some(ch) = iter.next() {
                if ch == '\\' {
                    idx += ch.len_utf8();

                    match iter.next() {
                        Some(ch) => match escaped.parse(&input[idx..]) {
                            Ok((o, _)) => {
                                idx += ch.len_utf8();
                                out.push(o);
                            }
                            Err(err) => return Err(err),
                        },
                        None => return Err(Error::found('\\')),
                    }
                } else {
                    idx += ch.len_utf8();
                    out.push(ch);
                }
            }

            Ok((out, rem))
        })
    }
}

pub fn pass<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, O> {
    move |input| parser.parse(input).map_err(|err| err.into_pass())
}

pub fn fail<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, O> {
    move |input| parser.parse(input).map_err(|err| err.into_fail())
}

pub fn peek<'a, O>(parser: impl Parser<'a, O>) -> impl Parser<'a, O> {
    move |input| parser.parse(input).map(|(out, _)| (out, input))
}

#[cfg(test)]
mod tests {
    use super::branch::{either, optional};
    use super::series::leading;
    use super::*;
    use crate::parser::parse;
    use crate::sequence::{alphabetic, Sequence};

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

    #[test]
    fn test_consume() {
        assert_eq!(
            parse("", consume(('h', 'e', 'l', 'l', 'o'))),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("help", consume(('h', 'e', 'l', 'l', 'o'))),
            Err(Error::expect('l').but_found('p'))
        );
        assert_eq!(
            parse("hello", consume(('h', 'e', 'l', 'l', 'o'))),
            Ok(("hello", ""))
        );
        assert_eq!(
            parse("hello world", consume(('h', 'e', 'l', 'l', 'o'))),
            Ok(("hello", " world"))
        );
        assert_eq!(parse("", consume("")), Ok(("", "")));
        assert_eq!(parse("hello", consume("")), Ok(("", "hello")));
    }

    #[test]
    fn test_not() {
        assert_eq!(parse("", not('h')), Err(Error::found_end()));
        assert_eq!(parse("h", not('h')), Err(Error::found('h')));
        assert_eq!(parse("hello", not('h')), Err(Error::found('h')));
        assert_eq!(parse("g", not('h')), Ok(('g', "")));
        assert_eq!(parse("goodbye", not('h')), Ok(('g', "oodbye")));
    }

    #[test]
    fn test_escaped() {
        assert_eq!(
            parse("", escaped(not('"'), either('"', '\\'))),
            Err(Error::found_end())
        );
        assert_eq!(
            parse("\"", escaped(not('"'), either('"', '\\'))),
            Err(Error::found('"'))
        );
        assert_eq!(
            parse("\\", escaped(not('"'), either('"', '\\'))),
            Err(Error::found('\\'))
        );
        assert_eq!(
            parse("hello world", escaped(not('"'), either('"', '\\'))),
            Ok(("hello world", ""))
        );
        assert_eq!(
            parse(r#""hello world""#, escaped(not('"'), either('"', '\\'))),
            Err(Error::found('"'))
        );
        assert_eq!(
            parse(r#"\"hello world\""#, escaped(not('"'), either('"', '\\'))),
            Ok(("\\\"hello world\\\"", ""))
        );
        assert_eq!(
            parse(r#"\\"hello world\\""#, escaped(not('"'), either('"', '\\'))),
            Ok(("\\\\", "\"hello world\\\\\""))
        );
        assert_eq!(
            parse(
                r#"\\\"hello world\\\""#,
                escaped(not('"'), either('"', '\\'))
            ),
            Ok(("\\\\\\\"hello world\\\\\\\"", ""))
        );
        assert_eq!(
            parse(
                r#"\\\\"hello world\\\\""#,
                escaped(not('"'), either('"', '\\'))
            ),
            Ok(("\\\\\\\\", "\"hello world\\\\\\\\\""))
        );
        assert_eq!(
            parse(
                r#"\\\\\"hello world\\\\\""#,
                escaped(not('"'), either('"', '\\'))
            ),
            Ok(("\\\\\\\\\\\"hello world\\\\\\\\\\\"", ""))
        );
        assert_eq!(
            parse(
                r#"\\\\\\"hello world\\\\\\""#,
                escaped(not('"'), either('"', '\\'))
            ),
            Ok(("\\\\\\\\\\\\", "\"hello world\\\\\\\\\\\\\""))
        );
        assert_eq!(
            parse(
                r#"\\\\\\\"hello world\\\\\\\""#,
                escaped(not('"'), either('"', '\\'))
            ),
            Ok(("\\\\\\\\\\\\\\\"hello world\\\\\\\\\\\\\\\"", ""))
        );
        assert_eq!(
            parse(
                r#"\"\\\"hello world\\\"\""#,
                escaped(not('"'), either('"', '\\'))
            ),
            Ok(("\\\"\\\\\\\"hello world\\\\\\\"\\\"", ""))
        );
        assert_eq!(
            parse(
                r#"\"\\\"\\\\\\\"hello world\\\\\\\"\\\"\""#,
                escaped(not('"'), either('"', '\\'))
            ),
            Ok((
                "\\\"\\\\\\\"\\\\\\\\\\\\\\\"hello world\\\\\\\\\\\\\\\"\\\\\\\"\\\"",
                ""
            ))
        );
    }

    #[test]
    fn test_unescape() {
        assert_eq!(
            parse("", unescape(escaped(not('\n'), 'n'), map('n', |_| '\n'))),
            Err(Error::found_end())
        );
        assert_eq!(
            parse("\n", unescape(escaped(not('\n'), 'n'), map('n', |_| '\n'))),
            Err(Error::found('\n'))
        );
        assert_eq!(
            parse(
                "hello",
                unescape(escaped(not('\n'), 'n'), map('n', |_| '\n'))
            ),
            Ok(("hello".to_owned(), ""))
        );
        assert_eq!(
            parse(
                "hello world",
                unescape(escaped(not('\n'), 'n'), map('n', |_| '\n'))
            ),
            Ok(("hello world".to_owned(), ""))
        );
        assert_eq!(
            parse(
                "hello\nworld",
                unescape(escaped(not('\n'), 'n'), map('n', |_| '\n'))
            ),
            Ok(("hello".to_owned(), "\nworld"))
        );
        assert_eq!(
            parse(
                "hello\\nworld",
                unescape(escaped(not('\n'), 'n'), map('n', |_| '\n'))
            ),
            Ok(("hello\nworld".to_owned(), ""))
        );
        assert_eq!(
            parse(
                "\\nhello\\nworld\\n",
                unescape(escaped(not('\n'), 'n'), map('n', |_| '\n'))
            ),
            Ok(("\nhello\nworld\n".to_owned(), ""))
        );
    }

    #[test]
    fn test_pass() {
        assert_eq!(
            parse(
                "",
                (alphabetic, optional(leading(':', pass(fail(alphabetic)))))
            ),
            Err(Error::expect(Sequence::Alphabetic).but_found_end())
        );
        assert_eq!(
            parse(
                "hello",
                (alphabetic, optional(leading(':', pass(fail(alphabetic)))))
            ),
            Ok((("hello", None), ""))
        );
        assert_eq!(
            parse(
                "hello:",
                (alphabetic, optional(leading(':', pass(fail(alphabetic)))))
            ),
            Ok((("hello", None), ":"))
        );
        assert_eq!(
            parse(
                "hello:world",
                (alphabetic, optional(leading(':', pass(fail(alphabetic)))))
            ),
            Ok((("hello", Some("world")), ""))
        );
        assert_eq!(
            parse(
                "hello:123",
                (alphabetic, optional(leading(':', pass(fail(alphabetic)))))
            ),
            Ok((("hello", None), ":123"))
        );
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            parse("", (alphabetic, optional(leading(':', fail(alphabetic))))),
            Err(Error::expect(Sequence::Alphabetic).but_found_end())
        );
        assert_eq!(
            parse(
                "hello",
                (alphabetic, optional(leading(':', fail(alphabetic))))
            ),
            Ok((("hello", None), ""))
        );
        assert_eq!(
            parse(
                "hello:",
                (alphabetic, optional(leading(':', fail(alphabetic))))
            ),
            Err(Error::expect(Sequence::Alphabetic)
                .but_found_end()
                .into_fail())
        );
        assert_eq!(
            parse(
                "hello:world",
                (alphabetic, optional(leading(':', fail(alphabetic))))
            ),
            Ok((("hello", Some("world")), ""))
        );
        assert_eq!(
            parse(
                "hello:123",
                (alphabetic, optional(leading(':', fail(alphabetic))))
            ),
            Err(Error::expect(Sequence::Alphabetic)
                .but_found('1')
                .into_fail())
        );
    }

    #[test]
    fn test_peek() {
        assert_eq!(
            parse("", ("hello", peek(" world"))),
            Err(Error::expect('h').but_found_end())
        );
        assert_eq!(
            parse("hello", ("hello", peek(" world"))),
            Err(Error::expect(' ').but_found_end())
        );
        assert_eq!(
            parse("hello universe", ("hello", peek(" world"))),
            Err(Error::expect('w').but_found('u'))
        );
        assert_eq!(
            parse("hello world", ("hello", peek(" world"))),
            Ok((("hello", " world"), " world"))
        );
    }
}
