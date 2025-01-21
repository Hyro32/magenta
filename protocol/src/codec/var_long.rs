use super::{Codec, DecodeError};
use bytes::{Buf, BufMut};

pub struct VarLong(pub i64);

impl Codec<Self> for VarLong {
    const MAX_SIZE: usize = 10;

    fn decode(value: &mut impl Buf) -> Result<Self, DecodeError> {
        let mut val = 0;
        for i in 0..Self::MAX_SIZE {
            if !value.has_remaining() {
                return Err(DecodeError::Incomplete);
            }
            let byte = value.get_u8();
            val |= (i64::from(byte) & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(VarLong(val));
            }
        }
        Err(DecodeError::TooLarge)
    }

    fn encode(&mut self, value: &mut impl BufMut) {
        let mut x = self.0;
        for _ in 0..Self::MAX_SIZE {
            let byte = (x & 0x7F) as u8;
            x >>= 7;
            if x == 0 {
                value.put_slice(&[byte]);
                break;
            }
            value.put_slice(&[byte | 0x80]);
        }
    }

    fn size(&self) -> usize {
        match self.0 {
            0 => 1,
            n => (63 - n.leading_zeros() as usize) / 7 + 1,
        }
    }
}

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        VarLong(value)
    }
}

impl From<u32> for VarLong {
    fn from(value: u32) -> Self {
        VarLong(value as i64)
    }
}

impl From<u8> for VarLong {
    fn from(value: u8) -> Self {
        VarLong(value as i64)
    }
}

impl From<usize> for VarLong {
    fn from(value: usize) -> Self {
        VarLong(value as i64)
    }
}

impl From<VarLong> for i64 {
    fn from(value: VarLong) -> Self {
        value.0
    }
}
