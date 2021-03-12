macro_rules! native_type {
    ($rty:ident, $nty:ty) => {
        #[derive(Clone, Copy)]
        pub struct $rty($nty);
    }
}
native_type!(CHAR, i8);
native_type!(BYTE, u8);
native_type!(INT, i16);
native_type!(WORD, u16);
native_type!(LONG, i32);
native_type!(DWORD, u32);
native_type!(FLOAT, f32);
native_type!(DOUBLE, f64);

type ByteString = Box<[u8]>;
pub struct STR(ByteString);
pub struct ZSTR(ByteString);
pub struct BSTR(ByteString);
pub struct WSTR(ByteString);
pub struct BZSTR(ByteString);
pub struct WZSTR(ByteString);

pub enum ConvertError {
    Range,
}

pub trait Native<N>: Sized {
    fn to_native(self) -> Result<N, ConvertError>;
    fn from_native(v: N) -> Result<Self, ConvertError>;
}

impl Native<char> for CHAR {
    fn to_native(self) -> Result<char, ConvertError> {
        Ok(self.0 as u8 as char)
    }
    fn from_native(v: char) -> Result<CHAR, ConvertError> {
        let v = v as u32;
        if v <= 0x7f {
            return Ok(CHAR(v as u8 as i8));
        }
        Err(ConvertError::Range)
    }
}

impl Native<char> for BYTE {
    fn to_native(self) -> Result<char, ConvertError> {
        Ok(self.0 as char)
    }
    fn from_native(v: char) -> Result<BYTE, ConvertError> {
        let v = v as u32;
        if v <= 0x7f {
            return Ok(BYTE(v as u8));
        }
        Err(ConvertError::Range)
    }
}

impl Native<u8> for BYTE {
    fn to_native(self) -> Result<u8, ConvertError> {
        Ok(self.0)
    }
    fn from_native(v: u8) -> Result<BYTE, ConvertError> {
        Ok(BYTE(v))
    }
}

impl Native<i16> for INT {
    fn to_native(self) -> Result<i16, ConvertError> {
        Ok(i16::from_le(self.0))
    }
    fn from_native(v: i16) -> Result<INT, ConvertError> {
        Ok(INT(i16::to_le(v)))
    }
}
