use crate::*;

use std::borrow::Cow;

#[non_exhaustive]
pub enum Coding {
    ERS(encoding_rs::Encoding),
    OCC {
        encode: &'static oem_cp::ahash::AHashMap<char, u8>,
        decode: oem_cp::code_table_type::TableType,
    }
}

type ByteString = Box<[u8]>;

impl Coding {
    fn encode<'a, S>(&'static self, src: S) -> Result<ByteString, ConvertError>
        where S: Into<Cow<'a, str>>
    {
        match self {
            Coding::ERS(c) => {
                let src = src.into();
                let oe = c.output_encoding();
                let (out, _, ok) = oe.encode(src.as_ref());
                if ok {
                    Ok(out.to_owned().to_vec().into_boxed_slice())
                } else {
                    Err(ConvertError::Encoding)
                }
            }
            Coding::OCC { encode: et, .. } => {
                match oem_cp::encode_string_checked(src, et) {
                    Some(out) => Ok(out.into_boxed_slice()),
                    None => Err(ConvertError::Encoding),
                }
            },
        }
    }
}


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
