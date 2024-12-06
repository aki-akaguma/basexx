use super::*;
use num_bigint::BigUint;

/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Base56
*/

#[derive(Debug)]
pub struct Base56 {
    ags: AsciiGraphicSet,
}

impl Default for Base56 {
    fn default() -> Self {
        Base56::new()
    }
}

impl Base56 {
    pub fn new() -> Self {
        Self::with_slice(&_CMAP56)
    }
    pub fn with_str(a: &str) -> Self {
        assert_eq!(a.len(), 56);
        Self {
            ags: AsciiGraphicSet::with_str(a),
        }
    }
    pub fn with_slice(a: &[u8]) -> Self {
        assert_eq!(a.len(), 56);
        Self {
            ags: AsciiGraphicSet::with_slice(a),
        }
    }
}

impl Base56 {
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base56(&self.ags, a)
    }
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        _decode_base56(&self.ags, a)
    }
}

/*
 * Base56 format:
 *      A B C D E F G
 *                  +-- LSB
 *      +-------------- MSB
 *      bigendian
*/
fn _encode_base56(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    // encode binary
    let zcount = a.iter().take_while(|&&x| x == 0).count();
    let mut r = {
        let bigu = BigUint::from_bytes_be(&a[zcount..]);
        let mut r: Vec<u8> = bigu.to_radix_le(56);
        if zcount > 0 {
            r.resize(r.len() + zcount, 0u8);
        }
        r.reverse();
        r
    };
    // from binary to ascii
    for c in &mut r {
        *c = match ags.get(*c) {
            Some(ascii) => ascii,
            None => return Err(EncodeError::InvalidIndex(*c)),
        };
    }
    let s = String::from_utf8_lossy(&r).to_string();
    assert!(s.len() == r.len());
    Ok(s)
}

fn _decode_base56(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
    // from ascii to binary
    let mut r = a.as_bytes().to_vec();
    for c in &mut r {
        *c = match ags.position(*c) {
            Some(ascii) => ascii,
            None => return Err(DecodeError::InvalidByte(*c)),
        };
    }
    // decode binary
    let zcount = r.iter().take_while(|&&x| x == 0).count();
    let rr = {
        let bigu = BigUint::from_radix_be(&r[zcount..], 56).unwrap();
        let mut rr: Vec<u8> = bigu.to_radix_le(256);
        if zcount > 0 {
            rr.resize(rr.len() + zcount, 0u8);
        }
        rr.reverse();
        rr
    };
    Ok(rr)
}

// exclusive ascii:
//     b'0' numeric zero, b'I' large ai, b'O' large o, b'l' small el
//     b'1' numeric one, b'o' small oh
const _CMAP56: [u8; 56] = [
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
    b'J', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'm', b'n', b'p', b'q', b'r',
    b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
];

#[cfg(all(test, not(feature = "bench")))]
mod tests;
