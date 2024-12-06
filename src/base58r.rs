use super::*;
use rug::Integer;

/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Base58R
*/

#[derive(Debug)]
pub struct Base58R {
    ags: AsciiGraphicSet,
}

impl Default for Base58R {
    fn default() -> Self {
        Base58R::new()
    }
}

impl Base58R {
    pub fn new() -> Self {
        Self::with_slice(&_CMAP58)
    }
    pub fn with_str(a: &str) -> Self {
        assert_eq!(a.len(), 58);
        Self {
            ags: AsciiGraphicSet::with_str(a),
        }
    }
    pub fn with_slice(a: &[u8]) -> Self {
        assert_eq!(a.len(), 58);
        Self {
            ags: AsciiGraphicSet::with_slice(a),
        }
    }
}

impl Base58R {
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base58r(&self.ags, a)
    }
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        _decode_base58r(&self.ags, a)
    }
}

/*
 * Base58R format:
 *      A B C D E F G
 *                  +-- LSB
 *      +-------------- MSB
 *      bigendian
*/
fn _encode_base58r(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    // encode binary
    let zcount = a.iter().take_while(|&&x| x == 0).count();
    let mut r = {
        let mut bigu = Integer::from_digits(a, rug::integer::Order::MsfBe);
        let mut r: Vec<u8> = Vec::with_capacity(a.len() * 256 / 58);
        let bigu_58 = Integer::from(58);
        while bigu > Integer::ZERO {
            let mut rem = bigu_58.clone();
            bigu.div_rem_mut(&mut rem);
            r.push(rem.to_u8().unwrap());
        }
        let r0 = vec![0u8; zcount];
        r.extend_from_slice(&r0);
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

fn _decode_base58r(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
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
        let mut bigu = Integer::ZERO;
        for &c in r[zcount..].iter() {
            bigu *= 58u8;
            bigu += c;
        }
        let mut rr: Vec<u8> = Vec::with_capacity(r.len());
        let bigu_256 = Integer::from(256);
        while bigu > Integer::ZERO {
            let mut rem = bigu_256.clone();
            bigu.div_rem_mut(&mut rem);
            rr.push(rem.to_u8().unwrap());
        }
        let r0 = vec![0u8; zcount];
        rr.extend_from_slice(&r0);
        rr.reverse();
        rr
    };
    Ok(rr)
}

// exclusive ascii: b'0' numeric zero, b'I' large ai, b'O' large o, b'l' small el
const _CMAP58: [u8; 58] = [
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'J', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y',
    b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'm', b'n', b'o', b'p',
    b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
];

#[cfg(feature = "rug")]
#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(feature = "rug")]
#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(feature = "rug")]
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
