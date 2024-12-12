use super::*;

/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Base64
 *     http://cvs.savannah.gnu.org/viewvc/gnulib/gnulib/lib/base64_g.c?view=markup&content-type=text%2Fvnd.viewcvs-markup&revision=HEAD
*/

#[derive(Debug)]
pub struct Base64G {
    ags: AsciiGraphicSet,
}

impl Default for Base64G {
    fn default() -> Self {
        Base64G::new()
    }
}

impl Base64G {
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

impl Base64G {
    #[inline]
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base64g(&self.ags, a)
    }
    #[inline]
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        _decode_base64g(&self.ags, a)
    }
}

/*
 * Base64G format:
 *      chunk from 8bit sequence to 6bit sequence:
 *          Z         *         E
 *          011110_10 0010_1010 01_000101
 *          011110 10_0010 1010_01 000101
 *      result from 3 bytes to 4bytes
*/
#[inline(never)]
fn _encode_base64g(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    let mut in_len = a.len();
    let mut out_len = 1 + ((in_len + 2) / 3) * 4;
    let mut out = vec![0u8; out_len];
    let mut i_idx = 0;
    let mut o_idx = 0;
    while in_len > 0 && out_len > 0 {
        out[o_idx] = {
            let bb = (a[i_idx] >> 2) & 0x3f;
            match ags.get(bb) {
                Some(ascii) => ascii,
                None => return Err(EncodeError::InvalidIndex(bb)),
            }
        };
        out_len -= 1;
        if out_len == 0 {
            break;
        }
        out[o_idx + 1] = {
            in_len -= 1;
            let bb = ((a[i_idx] << 4) + if in_len > 0 { a[i_idx + 1] >> 4 } else { 0 }) & 0x3f;
            match ags.get(bb) {
                Some(ascii) => ascii,
                None => return Err(EncodeError::InvalidIndex(bb)),
            }
        };
        out_len -= 1;
        if out_len == 0 {
            break;
        }
        out[o_idx + 2] = {
            if in_len > 0 {
                in_len -= 1;
                let bb =
                    ((a[i_idx + 1] << 2) + if in_len > 0 { a[i_idx + 2] >> 6 } else { 0 }) & 0x3f;
                match ags.get(bb) {
                    Some(ascii) => ascii,
                    None => return Err(EncodeError::InvalidIndex(bb)),
                }
            } else {
                b'='
            }
        };
        out_len -= 1;
        if out_len == 0 {
            break;
        }
        out[o_idx + 3] = {
            if in_len > 0 {
                let bb = a[i_idx + 2] & 0x3f;
                match ags.get(bb) {
                    Some(ascii) => ascii,
                    None => return Err(EncodeError::InvalidIndex(bb)),
                }
            } else {
                b'='
            }
        };
        out_len -= 1;
        if out_len == 0 {
            break;
        }
        if in_len > 0 {
            in_len -= 1;
            i_idx += 3;
        }
        o_idx += 4;
    }
    out.resize(o_idx, 0u8);
    let out_sz = out.len();
    let string = unsafe { String::from_utf8_unchecked(out) };
    assert!(string.len() == out_sz);
    Ok(string)
}

#[inline(never)]
fn _decode_base64g(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
    let ina: Vec<u8> = a.as_bytes().to_vec();
    let mut in_sz = ina.len();
    let out_sz = (in_sz / 4) * 3 + 2;
    let mut out_left = out_sz;
    let mut i_idx = 0;
    let mut o_idx = 0;
    let mut out = vec![0u8; out_sz];
    while in_sz >= 2 {
        let b0 = {
            let c0 = ina[i_idx];
            match ags.position(c0) {
                Some(n) => n,
                None => return Err(DecodeError::InvalidByte(c0)),
            }
        };
        let b1 = {
            let c1 = ina[i_idx + 1];
            match ags.position(c1) {
                Some(n) => n,
                None => return Err(DecodeError::InvalidByte(c1)),
            }
        };
        if out_left > 0 {
            out[o_idx] = (b0 << 2) | (b1 >> 4);
            o_idx += 1;
            out_left -= 1;
        }
        if in_sz == 2 {
            break;
        }
        if ina[i_idx + 2] == b'=' {
            if in_sz != 4 {
                break;
            }
            if ina[i_idx + 3] != b'=' {
                break;
            }
        } else {
            let b2 = {
                let c2 = ina[i_idx + 2];
                match ags.position(c2) {
                    Some(n) => n,
                    None => return Err(DecodeError::InvalidByte(c2)),
                }
            };
            if out_left > 0 {
                out[o_idx] = ((b1 << 4) & 0xf0) | (b2 >> 2);
                o_idx += 1;
                out_left -= 1;
            }
            if ina[i_idx + 3] == b'=' {
                if in_sz != 4 {
                    break;
                }
            } else {
                let b3 = {
                    let c3 = ina[i_idx + 3];
                    match ags.position(c3) {
                        Some(n) => n,
                        None => return Err(DecodeError::InvalidByte(c3)),
                    }
                };
                if out_left > 0 {
                    out[o_idx] = ((b2 << 6) & 0xc0) | b3;
                    o_idx += 1;
                    out_left -= 1;
                }
            }
        }
        i_idx += 4;
        in_sz -= 4;
    }
    out.resize(out_sz - out_left, 0u8);
    Ok(out)
}

const _CMAP64: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
