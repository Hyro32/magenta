use std::num::NonZeroUsize;

use super::{Codec, DecodeError};
use bytes::{Buf, BufMut};

type VarLongType = i64;
pub struct VarLong(VarLongType);

impl Codec<Self> for VarLong {
    const MAX_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(10) };

    fn encode(&self, write: &mut impl BufMut) {
        let mut x = self.0;

        for _ in 0..Self::MAX_SIZE.get() {
            let byte = (x & 0x7F) as u8;
            x >>= 7;

            if x == 0 {
                write.put_slice(&[byte]);
                break;
            }

            write.put_slice(&[byte | 0x80]);
        }
    }

    fn decode(read: &mut impl Buf) -> Result<Self, DecodeError> {
        let mut val = 0;

        for i in 0..Self::MAX_SIZE.get() {
            if !read.has_remaining() {
                return Err(DecodeError::Incomplete);
            }

            let byte = read.get_u8();
            val |= (i64::from(byte) & 0b01111111) << (i * 7);

            if byte & 0b10000000 == 0 {
                return Ok(VarLong(val));
            }
        }
        
        Err(DecodeError::TooLarge)
    }
}
