mod scalars;
mod strings;

pub use scalars::*;
pub use strings::*;

pub use encoding_rs;
pub use codepage;
pub use oem_cp;

#[non_exhaustive]
pub enum ConvertError {
    Range,
    Encoding,
}
