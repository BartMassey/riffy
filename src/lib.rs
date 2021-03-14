mod scalars;
mod strings;

pub use scalars::*;
pub use strings::*;

pub enum ConvertError {
    Range,
    StringConvert(codepage_strings::ConvertError),
}
