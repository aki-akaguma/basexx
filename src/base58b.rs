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
    #[inline]
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base58b(&self.ags, a)
    }
    #[inline]
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        _decode_base58b(&self.ags, a)
    }
}

/*
 * Base58 format:
 *      A B C D E F G
 *                  +-- LSB
 *      +-------------- MSB
 *      bigendian
*/
#[inline(never)]
fn _encode_base58b(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    // encode binary
    let zero_count = a.iter().take_while(|&&x| x == 0).count();
    let oup = {
        let oup_sz = (a.len() - zero_count) * 138 / 100 + 1;
        let mut oup = vec![0u8; oup_sz];
        let mut high = oup_sz - 1;
        for &c in a[zero_count..].iter() {
            let mut carry = c as u32;
            high = {
                let mut j = oup_sz - 1;
                while j > high || carry != 0 {
                    carry += 256 * oup[j] as u32;
                    oup[j] = (carry % 58) as u8;
                    if j == 0 {
                        break;
                    }
                    carry = carry.wrapping_div(58);
                    j -= 1;
                }
                j
            };
        }
        oup
    };
    let mut oup = {
        let st = oup
            .iter()
            .take_while(|&&x| x == 0)
            .collect::<Vec<_>>()
            .len();
        let mut r = vec![0u8; zero_count];
        r.extend_from_slice(&oup[st..]);
        r
    };
    // from binary to ascii
    ags.binary_to_ascii(&mut oup)?;
    let oup_sz = oup.len();
    let string = unsafe { String::from_utf8_unchecked(oup) };
    assert!(string.len() == oup_sz);
    Ok(string)
}

#[inline(never)]
fn _decode_base58b(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
    // from ascii to binary
    let mut inp = a.as_bytes().to_vec();
    // from ascii to binary
    ags.ascii_to_binary(&mut inp)?;
    // decode binary
    let zero_count = inp.iter().take_while(|&&x| x == 0).count();
    let bytesleft: u8 = (inp.len() % 4) as u8;
    let obuf = {
        let obuf_sz = (inp.len() * 138 / 100).div_ceil(4);
        let mut obuf = vec![0_u32; obuf_sz];
        let (mut j, mut t, mut c): (_, u64, u32);
        let zero_mask: u32 = if bytesleft != 0 {
            0xFFFF_FFFF << (bytesleft * 8)
        } else {
            0
        };
        for &a in inp[zero_count..].iter() {
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
        rr[(zzcount - zero_count)..].to_vec()
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

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
