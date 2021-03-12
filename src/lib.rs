mod scalars;
mod strings;

pub use scalars::*;
pub use strings::*;

pub use encoding;

pub enum ConvertError {
    Range,
}
