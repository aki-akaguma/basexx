use super::*;

mod base32_scalar;
pub(crate) use base32_scalar::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod base32_ssse3;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use base32_ssse3::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod base32_avx2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use base32_avx2::*;

/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Base32
*/

#[derive(Debug)]
pub struct Base32 {
    ags: AsciiGraphicSet,
}

impl Default for Base32 {
    fn default() -> Self {
        Base32::new()
    }
}

impl Base32 {
    pub fn new() -> Self {
        Self::with_slice(&_CMAP32)
    }
    pub fn with_str(a: &str) -> Self {
        assert_eq!(a.len(), 32);
        Self {
            ags: AsciiGraphicSet::with_str(a),
        }
    }
    pub fn with_slice(a: &[u8]) -> Self {
        assert_eq!(a.len(), 32);
        Self {
            ags: AsciiGraphicSet::with_slice(a),
        }
    }
}

impl Base32 {
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        /*
        _encode_base32_scalar(&self.ags, a)
        */
        if cfg!(target_feature = "sse2") {
            if is_x86_feature_detected!("avx2") {
                unsafe { _encode_base32_avx2(&self.ags, a) }
            } else if is_x86_feature_detected!("ssse3") {
                unsafe { _encode_base32_ssse3(&self.ags, a) }
            } else {
                _encode_base32_scalar(&self.ags, a)
            }
        } else {
            _encode_base32_scalar(&self.ags, a)
        }
    }
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        /*
        _decode_base32_scalar(&self.ags, a)
        */
        if cfg!(target_feature = "sse2") {
            if is_x86_feature_detected!("avx2") {
                unsafe { _decode_base32_avx2(&self.ags, a) }
            } else if is_x86_feature_detected!("ssse3") {
                unsafe { _decode_base32_ssse3(&self.ags, a) }
            } else {
                _decode_base32_scalar(&self.ags, a)
            }
        } else {
            _decode_base32_scalar(&self.ags, a)
        }
    }
}

const _CMAP32: [u8; 32] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'2', b'3', b'4', b'5', b'6', b'7',
];
