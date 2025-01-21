use super::{Codec, DecodeError};
use bytes::{Buf, BufMut};

pub struct VarInt(pub i32);

impl Codec<Self> for VarInt {
    const MAX_SIZE: usize = 5;

    fn decode(value: &mut impl Buf) -> Result<Self, DecodeError> {
        let mut val = 0;
        for i in 0..Self::MAX_SIZE {
            if !value.has_remaining() {
                return Err(DecodeError::Incomplete);
            }
            let byte = value.get_u8();
            val |= (i32::from(byte) & 0x7F) << (i * 7);
            if byte & 0x80 == 0 {
                return Ok(VarInt(val));
            }
        }
        Err(DecodeError::TooLarge)
    }

    fn encode(&mut self, value: &mut impl BufMut) {
        let mut val = self.0;
        for _ in 0..Self::MAX_SIZE {
            let b: u8 = val as u8 & 0b01111111;
            val >>= 7;
            value.put_u8(if val == 0 { b } else { b | 0b10000000 });
            if val == 0 {
                break;
            }
        }
    }

    fn size(&self) -> usize {
        match self.0 {
            0 => 1,
            n => (31 - n.leading_zeros() as usize) / 7 + 1,
        }
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

impl From<u32> for VarInt {
    fn from(value: u32) -> Self {
        VarInt(value as i32)
    }
}

impl From<u8> for VarInt {
    fn from(value: u8) -> Self {
        VarInt(value as i32)
    }
}

impl From<usize> for VarInt {
    fn from(value: usize) -> Self {
        VarInt(value as i32)
    }
}

impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        value.0
    }
}
