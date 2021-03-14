use crate::*;

use codepage_strings::Coding;

type ByteString = Box<[u8]>;

macro_rules! native_type {
    ($rty:ident) => {
        #[derive(Clone)]
        pub struct $rty(ByteString);
    }
}

native_type!(STR);
native_type!(ZSTR);
native_type!(BSTR);
native_type!(WSTR);
native_type!(BZSTR);
native_type!(WZSTR);

pub trait NativeString: Sized {
    fn to_native(&self, enc: &Coding) -> Result<String, ConvertError>;
    fn from_native(v: &str, enc: &Coding) -> Result<Self, ConvertError>;
}
