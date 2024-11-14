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
        assert_eq!(a.as_bytes().len(), 58);
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
        _encode_base58(&self.ags, a)
    }
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        _decode_base58(&self.ags, a)
    }
}

/*
 * Base58R format:
 *      A B C D E F G
 *                  +-- LSB
 *      +-------------- MSB
 *      bigendian
*/
fn _encode_base58(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    // encode binary
    let zcount = a.iter().take_while(|&&x| x == 0).count();
    let r = {
        let mut bigu = Integer::ZERO;
        for &c in a.iter() {
            //bigu *= 256u32;
            //bigu += c;
            bigu = bigu * 256u32 + c;
        }
        let mut r: Vec<u8> = Vec::new();
        let bigu_58 = Integer::from(58);
        while bigu > Integer::ZERO {
            let mut rem = bigu_58.clone();
            bigu.div_rem_mut(&mut rem);
            r.push(rem.to_u8().unwrap());
        }
        r.reverse();
        let mut r0 = vec![0u8; zcount];
        r0.extend_from_slice(&r);
        r0
    };
    // from binary to ascii
    let rr = match r
        .iter()
        .map(|&b| match ags.get(b) {
            Some(ascii) => Ok(ascii),
            None => Err(EncodeError::InvalidIndex(b)),
        })
        .collect::<Result<Vec<u8>, EncodeError>>()
    {
        Ok(rr) => rr,
        Err(err) => return Err(err),
    };
    let s = String::from_utf8_lossy(&rr).to_string();
    assert!(s.len() == rr.len());
    Ok(s)
}

fn _decode_base58(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
    // from ascii to binary
    let r = match a
        .as_bytes()
        .iter()
        .map(|&b| match ags.position(b) {
            Some(n) => Ok(n),
            None => Err(DecodeError::InvalidByte(b)),
        })
        .collect::<Result<Vec<u8>, _>>()
    {
        Ok(r) => r,
        Err(err) => return Err(err),
    };
    // decode binary
    let zcount = r.iter().take_while(|&&x| x == 0).count();
    let rr = {
        let mut bigu = Integer::ZERO;
        for &c in r[zcount..].iter() {
            bigu *= 58u8;
            bigu += c;
        }
        let mut rr: Vec<u8> = Vec::new();
        let bigu_256 = Integer::from(256);
        while bigu > Integer::ZERO {
            let mut rem = bigu_256.clone();
            bigu.div_rem_mut(&mut rem);
            rr.push(rem.to_u8().unwrap());
        }
        rr.reverse();
        let mut r0 = vec![0u8; zcount];
        r0.extend_from_slice(&rr);
        r0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        let inp = [0u8, 0, 1, 1].to_vec();
        let oup = "115S".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP58);
        let r1 = _encode_base58(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base58(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_1() {
        let inp = b"ABCDEFGHIJKL".to_vec();
        let oup = "2ERjaFfYv6E4EfgR1".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP58);
        let r1 = _encode_base58(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base58(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_2() {
        let inp = b"ABCDEFGHIJK".to_vec();
        let oup = "HBb7dQEaKrdXjkN".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP58);
        let r1 = _encode_base58(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base58(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_3() {
        let inp = b"ABCDEFGHIJ".to_vec();
        let oup = "4fedr2e4UP7vBb".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP58);
        let r1 = _encode_base58(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base58(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
}
