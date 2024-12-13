mod aligned_data;
#[allow(unused_imports)]
use aligned_data::*;

mod ags;
use ags::*;

mod base32;
mod base32i;
mod base64;
mod base64g;

mod base56;
mod base58;
mod base58b;

pub use base32::*;
pub use base32i::*;
pub use base64::*;
pub use base64g::*;

pub use base56::*;
pub use base58::*;
pub use base58b::*;

#[cfg(feature = "rug")]
mod base58r;

#[cfg(feature = "rug")]
pub use base58r::*;

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

#[cfg(test)]
mod test_utils;
