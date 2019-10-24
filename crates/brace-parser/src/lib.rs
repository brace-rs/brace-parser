pub mod character;
pub mod combinator;
pub mod error;
pub mod parser;
pub mod sequence;

pub mod prelude {
    pub use crate::combinator::branch::{branch, either, optional};
    pub use crate::combinator::map;
    pub use crate::combinator::series::{delimited, leading, list, pair, series, trailing, trio};
    pub use crate::error::{Error, Expect};
    pub use crate::parser::{parse, take, take_while};
    pub use crate::{character, sequence};
}
