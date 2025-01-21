use bytes::{Buf, BufMut};

pub mod var_int;
pub mod var_long;

pub trait Codec<T> {
    const MAX_SIZE: usize;

    fn decode(value: &mut impl Buf) -> Result<T, DecodeError>;
    fn encode(&mut self, value: &mut impl BufMut);
    fn size(&self) -> usize;
}

pub enum DecodeError {
    Incomplete,
    TooLarge,
}
