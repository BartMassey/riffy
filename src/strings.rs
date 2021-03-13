use crate::*;

use std::borrow::Cow;

#[non_exhaustive]
pub enum Coding {
    ERS(&'static encoding_rs::Encoding),
    OCC {
        encode: &'static oem_cp::ahash::AHashMap<char, u8>,
        decode: &'static oem_cp::code_table_type::TableType,
    },
}

type ByteString = Box<[u8]>;

impl Coding {
    fn new(cp: u16) -> Result<Self, ConvertError> {
        if let Some(c) = codepage::to_encoding(cp) {
            return Ok(Self::ERS(c));
        }
        let encode = match (*oem_cp::code_table::ENCODING_TABLE_CP_MAP).get(&cp) {
            Some(e) => e,
            None => return Err(ConvertError::CodePage),
        };
        let decode = match (*oem_cp::code_table::DECODING_TABLE_CP_MAP).get(&cp) {
            Some(e) => e,
            None => return Err(ConvertError::CodePage),
        };
        Ok(Self::OCC { encode, decode })
    }

    fn encode<'a, S>(&self, src: S) -> Result<ByteString, ConvertError>
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
                    Err(ConvertError::StringEncoding)
                }
            }
            Coding::OCC { encode: et, .. } => {
                match oem_cp::encode_string_checked(src, et) {
                    Some(out) => Ok(out.into_boxed_slice()),
                    None => Err(ConvertError::StringEncoding),
                }
            },
        }
    }

    fn decode<'a>(&self, src: &'a [u8]) -> Result<Cow<'a, str>, ConvertError> {
        match self {
            Coding::ERS(c) => {
                let (out, _, ok) = c.decode(src.as_ref());
                if ok {
                    Ok(out)
                } else {
                    Err(ConvertError::StringDecoding)
                }
            }
            Coding::OCC { decode: dt, .. } => {
                match dt.decode_string_checked(src) {
                    Some(s) => Ok(Cow::from(s)),
                    None => Err(ConvertError::StringDecoding),
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
