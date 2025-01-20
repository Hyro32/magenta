use std::num::NonZeroUsize;

use bytes::{Buf, BufMut};

mod varint;
mod varlong;

pub trait Codec<T> {
    const MAX_SIZE: NonZeroUsize;
    fn encode(&self, write: &mut impl BufMut);
    fn decode(read: &mut impl Buf) -> Result<T, DecodeError>;
}

pub enum DecodeError {
    Incomplete,
    TooLarge,
}
