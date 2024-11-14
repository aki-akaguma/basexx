mod ascii_graphic_set;
use ascii_graphic_set::*;

mod base32;
mod base64;

mod base56;
mod base58;
mod base58_b;
mod base58_r;

pub use base32::*;
pub use base64::*;

pub use base56::*;
pub use base58::*;
pub use base58_b::*;
pub use base58_r::*;

#[derive(Debug)]
pub enum EncodeError {
    InvalidIndex(u8),
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidByte(u8),
    InvalidLength(usize),
    OutputNumberTooBig(u32, String),
}
