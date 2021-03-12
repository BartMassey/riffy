use crate::*;

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

pub trait Native<N>: Sized {
    fn to_native(self) -> Result<N, ConvertError>;
    fn from_native(v: N) -> Result<Self, ConvertError>;
}

impl Native<char> for CHAR {
    fn to_native(self) -> Result<char, ConvertError> {
        Ok(self.0 as u8 as char)
    }
    fn from_native(v: char) -> Result<Self, ConvertError> {
        let v = v as u32;
        if v <= 0x7f {
            return Ok(Self(v as u8 as i8));
        }
        Err(ConvertError::Range)
    }
}

impl Native<char> for BYTE {
    fn to_native(self) -> Result<char, ConvertError> {
        Ok(self.0 as char)
    }
    fn from_native(v: char) -> Result<Self, ConvertError> {
        let v = v as u32;
        if v <= 0x7f {
            return Ok(Self(v as u8));
        }
        Err(ConvertError::Range)
    }
}

impl Native<u8> for BYTE {
    fn to_native(self) -> Result<u8, ConvertError> {
        Ok(self.0)
    }
    fn from_native(v: u8) -> Result<Self, ConvertError> {
        Ok(Self(v))
    }
}

impl Native<i16> for INT {
    fn to_native(self) -> Result<i16, ConvertError> {
        Ok(i16::from_le(self.0))
    }
    fn from_native(v: i16) -> Result<Self, ConvertError> {
        Ok(Self(i16::to_le(v)))
    }
}

impl Native<u16> for WORD {
    fn to_native(self) -> Result<u16, ConvertError> {
        Ok(u16::from_le(self.0))
    }
    fn from_native(v: u16) -> Result<Self, ConvertError> {
        Ok(Self(u16::to_le(v)))
    }
}


impl Native<i32> for LONG {
    fn to_native(self) -> Result<i32, ConvertError> {
        Ok(i32::from_le(self.0))
    }
    fn from_native(v: i32) -> Result<Self, ConvertError> {
        Ok(Self(i32::to_le(v)))
    }
}

impl Native<u32> for DWORD {
    fn to_native(self) -> Result<u32, ConvertError> {
        Ok(u32::from_le(self.0))
    }
    fn from_native(v: u32) -> Result<Self, ConvertError> {
        Ok(Self(u32::to_le(v)))
    }
}

impl Native<f32> for FLOAT {
    fn to_native(self) -> Result<f32, ConvertError> {
        Ok(self.0)
    }
    fn from_native(v: f32) -> Result<Self, ConvertError> {
        Ok(Self(v))
    }
}

impl Native<f64> for DOUBLE {
    fn to_native(self) -> Result<f64, ConvertError> {
        Ok(self.0)
    }
    fn from_native(v: f64) -> Result<Self, ConvertError> {
        Ok(Self(v))
    }
}
