use super::*;

/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Base58
 *     https://github.com/bitcoin/libbase58/blob/master/base58.c
*/

#[derive(Debug)]
pub struct Base58B {
    ags: AsciiGraphicSet,
}

impl Default for Base58B {
    fn default() -> Self {
        Base58B::new()
    }
}

impl Base58B {
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

impl Base58B {
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base58(&self.ags, a)
    }
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        _decode_base58(&self.ags, a)
    }
}

/*
 * Base58 format:
 *      A B C D E F G
 *                  +-- LSB
 *      +-------------- MSB
 *      bigendian
*/
fn _encode_base58(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    // encode binary
    let zcount = a.iter().take_while(|&&x| x == 0).count();
    let buf = {
        let buf_sz = (a.len() - zcount) * 138 / 100 + 1;
        let mut buf = vec![0u8; buf_sz];
        let mut high = buf_sz - 1;
        for &c in a[zcount..].iter() {
            let mut carry = c as u32;
            high = {
                let mut j = buf_sz - 1;
                while j > high || carry != 0 {
                    carry += 256 * buf[j] as u32;
                    buf[j] = (carry % 58) as u8;
                    if j == 0 {
                        break;
                    }
                    carry = carry.wrapping_div(58);
                    j -= 1;
                }
                j
            };
        }
        buf
    };
    let r = {
        let st = buf
            .iter()
            .take_while(|&&x| x == 0)
            .collect::<Vec<_>>()
            .len();
        let mut r = vec![0u8; zcount];
        r.extend_from_slice(&buf[st..]);
        r
    };
    // from binary to ascii
    let rr = r
        .iter()
        .map(|&b| match ags.get(b) {
            Some(ascii) => Ok(ascii),
            None => Err(EncodeError::InvalidIndex(b)),
        })
        .collect::<Result<Vec<u8>, EncodeError>>()?;
    let s = String::from_utf8_lossy(&rr).to_string();
    assert!(s.len() == rr.len());
    Ok(s)
}

fn _decode_base58(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
    // from ascii to binary
    let r = a
        .as_bytes()
        .iter()
        .map(|&b| match ags.position(b) {
            Some(n) => Ok(n),
            None => Err(DecodeError::InvalidByte(b)),
        })
        .collect::<Result<Vec<u8>, _>>()?;
    // decode binary
    let zcount = r.iter().take_while(|&&x| x == 0).count();
    let bytesleft: u8 = (r.len() % 4) as u8;
    let obuf = {
        let obuf_sz = (r.len() * 138 / 100).div_ceil(4);
        let mut obuf = vec![0_u32; obuf_sz];
        let (mut j, mut t, mut c): (_, u64, u32);
        let zero_mask: u32 = if bytesleft != 0 {
            0xFFFF_FFFF << (bytesleft * 8)
        } else {
            0
        };
        for &a in r[zcount..].iter() {
            c = a.into();
            j = obuf_sz - 1;
            loop {
                t = obuf[j] as u64 * 58u64 + c as u64;
                c = (t >> 32) as u32;
                obuf[j] = (t & 0xFFFF_FFFF) as u32;
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            if c != 0 {
                return Err(DecodeError::OutputNumberTooBig(
                    c,
                    "carry to the next i32".to_string(),
                ));
            }
            if (obuf[0] & zero_mask) != 0 {
                return Err(DecodeError::OutputNumberTooBig(
                    obuf[0] & zero_mask,
                    "last i32 filled too far".to_string(),
                ));
            }
        }
        obuf
    };
    let rrr = {
        let mut rr: Vec<u8> = Vec::new();
        let push4 = |rr: &mut Vec<u8>, v, mut i| {
            while i > 0 {
                rr.push(((v >> (8 * (i - 1))) & 0xFF) as u8);
                i -= 1;
            }
        };
        let mut j = 0;
        if bytesleft != 0 {
            push4(&mut rr, obuf[j], bytesleft);
            j += 1;
        }
        while j < obuf.len() {
            push4(&mut rr, obuf[j], 4);
            j += 1;
        }
        let zzcount = rr.iter().take_while(|&&x| x == 0).count();
        rr[(zzcount - zcount)..].to_vec()
    };
    Ok(rrr)
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
