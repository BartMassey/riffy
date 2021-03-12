use crate::*;

use encoding::types::Encoding;

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
    fn to_native(&self, enc: &dyn Encoding) -> Result<String, ConvertError>;
    fn from_native(v: &str, enc: &dyn Encoding) -> Result<Self, ConvertError>;
}

