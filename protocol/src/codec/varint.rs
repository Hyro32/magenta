use std::num::NonZeroUsize;

use super::{Codec, DecodeError};
use bytes::{Buf, BufMut};

type VarIntType = i32;
pub struct VarInt(VarIntType);

impl Codec<Self> for VarInt {
    const MAX_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(5) };

    fn encode(&self, write: &mut impl BufMut) {
        let mut val = self.0;

        for _ in 0..Self::MAX_SIZE.get() {
            let b: u8 = val as u8 & 0b01111111;
            val >>= 7;

            let n = if val == 0 { b } else { b | 0b10000000 };
            write.put_u8(n);

            if val == 0 {
                break;
            }
        }
    }

    fn decode(read: &mut impl Buf) -> Result<Self, DecodeError> {
        let mut val = 0;

        for i in 0..Self::MAX_SIZE.get() {
            if !read.has_remaining() {
                return Err(DecodeError::Incomplete);
            }

            let byte = read.get_u8();
            val |= (i32::from(byte) & 0x7F) << (i * 7);

            if byte & 0x80 == 0 {
                return Ok(VarInt(val));
            }
        }

        Err(DecodeError::TooLarge)
    }
}
