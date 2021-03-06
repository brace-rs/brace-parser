pub mod character;
pub mod combinator;
pub mod error;
pub mod parser;
pub mod sequence;

pub mod prelude {
    pub use crate::combinator::branch::{branch, either, optional};
    pub use crate::combinator::series::{
        delimited, leading, list, pair, repeat, series, trailing, trio,
    };
    pub use crate::combinator::{
        complete, consume, context, escaped, fail, fold, map, map_err, not, pass, peek, unescape,
    };
    pub use crate::error::{Error, Expect};
    pub use crate::parser::{parse, take, take_while, Output, Parser};
    pub use crate::sequence::end;
    pub use crate::{character, sequence};
}
