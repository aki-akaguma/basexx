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

#[cfg(feature = "ubench")]
#[cfg(test)]
use criterion::*;

#[cfg(feature = "ubench")]
#[cfg(test)]
criterion_group!(
    benches,
    ags::bench_ags_scalar_enc,
    ags::bench_ags_scalar_dec,
    base64::bench_base64_scalar_enc,
    base64::bench_base64_scalar_dec,
    base64::bench_base64_ssse3_enc,
    base64::bench_base64_ssse3_dec,
    base32::bench_base32_scalar_enc,
    base32::bench_base32_scalar_dec,
    base32::bench_base32_ssse3_enc,
    base32::bench_base32_ssse3_dec,
    base64g::bench_base64g_enc,
    base64g::bench_base64g_dec,
    base32i::bench_base32i_enc,
    base32i::bench_base32i_dec,
    base58b::bench_base58b_enc,
    base58b::bench_base58b_dec,
);

#[cfg(feature = "rug")]
#[cfg(feature = "ubench")]
#[cfg(test)]
criterion_group!(
    benches2,
    base58r::bench_base58r_enc,
    base58r::bench_base58r_dec,
);

#[cfg(not(feature = "rug"))]
#[cfg(feature = "ubench")]
#[cfg(test)]
criterion_main!(benches);

#[cfg(feature = "rug")]
#[cfg(feature = "ubench")]
#[cfg(test)]
criterion_main!(benches, benches2);
