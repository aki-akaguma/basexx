use super::*;

mod base64_scalar;
pub(crate) use base64_scalar::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod base64_ssse3;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use base64_ssse3::*;

/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Base64
*/

#[derive(Debug)]
pub struct Base64 {
    ags: AsciiGraphicSet,
}

impl Default for Base64 {
    fn default() -> Self {
        Base64::new()
    }
}

impl Base64 {
    pub fn new() -> Self {
        Self::with_slice(&_CMAP64)
    }
    pub fn with_str(a: &str) -> Self {
        assert_eq!(a.len(), 64);
        Self {
            ags: AsciiGraphicSet::with_str(a),
        }
    }
    pub fn with_slice(a: &[u8]) -> Self {
        assert_eq!(a.len(), 64);
        Self {
            ags: AsciiGraphicSet::with_slice(a),
        }
    }
}

impl Base64 {
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        /*
        _encode_base64_scalar(&self.ags, a)
        */
        if cfg!(target_feature = "sse2") {
            if is_x86_feature_detected!("ssse3") {
                unsafe { _encode_base64_ssse3(&self.ags, a) }
            } else {
                _encode_base64_scalar(&self.ags, a)
            }
        } else {
            _encode_base64_scalar(&self.ags, a)
        }
    }
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        /*
        _decode_base64_scalar(&self.ags, a)
        */
        if cfg!(target_feature = "sse2") {
            if is_x86_feature_detected!("ssse3") {
                unsafe { _decode_base64_ssse3(&self.ags, a) }
            } else {
                _decode_base64_scalar(&self.ags, a)
            }
        } else {
            _decode_base64_scalar(&self.ags, a)
        }
    }
}

const _CMAP64: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];
