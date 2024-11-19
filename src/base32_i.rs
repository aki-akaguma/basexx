use super::*;
use num_bigint::BigUint;

/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Base32
*/

#[derive(Debug)]
pub struct Base32I {
    ags: AsciiGraphicSet,
}

impl Default for Base32I {
    fn default() -> Self {
        Base32I::new()
    }
}

impl Base32I {
    pub fn new() -> Self {
        Self::with_slice(&_CMAP32)
    }
    pub fn with_str(a: &str) -> Self {
        assert_eq!(a.as_bytes().len(), 32);
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

impl Base32I {
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base32i(&self.ags, a)
    }
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        _decode_base32i(&self.ags, a)
    }
}

/*
 * Base32I format:
 *      A B C D E F G
 *                  +-- LSB
 *      +-------------- MSB
 *      bigendian
*/
fn _encode_base32i(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    // encode binary
    let (a, aa_sz) = {
        let mut aa = vec![];
        aa.extend(a);
        let old_sz = aa.len();
        let new_sz = ((old_sz + 4) / 5) * 5;
        aa.resize(new_sz, 0u8);
        let aa_sz = match new_sz - old_sz {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 4,
            4 => 6,
            _ => panic!("this is bug!"),
        };
        (aa, aa_sz)
    };
    let zcount = a.iter().take_while(|&&x| x == 0).count();
    let r = {
        let bigu = BigUint::from_bytes_be(&a[zcount..]);
        let mut r: Vec<u8> = bigu.to_radix_le(32);
        if zcount > 0 {
            let azc = a[zcount];
            let zzcount = (zcount / 5) * 8;
            let (_, zzrest) = match zcount % 5 {
                0 => (0, if azc > 0b0111 { 0 } else { 1 }),
                1 => (
                    0,
                    1 + if azc > 0b111111 {
                        0
                    } else if azc > 0b1 {
                        1
                    } else {
                        2
                    },
                ),
                2 => (0, 3 + if azc > 0b1111 { 0 } else { 1 }),
                3 => (
                    0,
                    4 + if azc > 0b1111111 {
                        0
                    } else if azc > 0b11 {
                        1
                    } else {
                        2
                    },
                ),
                4 => (0, 6 + if azc > 0b11111 { 0 } else { 1 }),
                _ => (0, 0),
            };
            r.resize(r.len() + zzcount + zzrest, 0u8);
        }
        r
    };
    // from binary to ascii
    let mut rr = r[aa_sz..].to_vec();
    rr.reverse();
    for c in &mut rr {
        *c = match ags.get(*c) {
            Some(ascii) => ascii,
            None => return Err(EncodeError::InvalidIndex(*c)),
        };
    }
    let s = String::from_utf8_lossy(&rr).to_string();
    assert!(s.len() == rr.len());
    Ok(s)
}

fn _decode_base32i(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
    // from ascii to binary
    let mut r = a.as_bytes().to_vec();
    for c in &mut r {
        *c = match ags.position(*c) {
            Some(ascii) => ascii,
            None => return Err(DecodeError::InvalidByte(*c)),
        };
    }
    // decode binary
    let (r, bb_sz) = {
        let mut r0 = vec![];
        r0.extend(r);
        let old_sz = r0.len();
        let bb_sz = 8 - (old_sz % 8);
        let bb_sz = if bb_sz == 8 { 0 } else { bb_sz };
        r0.resize(old_sz + bb_sz, 0);
        let bb_sz = match bb_sz {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 2,
            4 => 3,
            5 => 4,
            6 => 4,
            7 => 5,
            _ => panic!("this is bug!"),
        };
        (r0, bb_sz)
    };
    let zcount = r.iter().take_while(|&&x| x == 0).count();
    let rr = {
        let bigu = BigUint::from_radix_be(&r[zcount..], 32).unwrap();
        let mut rr: Vec<u8> = bigu.to_radix_le(256);
        if zcount > 0 {
            let rzc = r[zcount];
            let zzcount = (zcount / 8) * 5;
            let (_, zzrest) = match zcount % 8 {
                1 => (0, if rzc > 0b11 { 0 } else { 1 }),
                2 => (0, 1 + if rzc > 0 { 0 } else { 1 }),
                3 => (0, 1 + if rzc > 0b1111 { 0 } else { 1 }),
                4 => (0, 2 + if rzc > 0b1 { 0 } else { 1 }),
                5 => (0, 3 + if rzc > 0 { 0 } else { 1 }),
                6 => (0, 3 + if rzc > 0b111 { 0 } else { 1 }),
                7 => (0, 4 + if rzc > 0 { 0 } else { 1 }),
                _ => (0, 0),
            };
            rr.resize(rr.len() + zzcount + zzrest, 0u8);
        }
        rr.reverse();
        rr.resize(rr.len() - bb_sz, 0);
        rr
    };
    Ok(rr)
}

const _CMAP32: [u8; 32] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'2', b'3', b'4', b'5', b'6', b'7',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        let inp = [0u8, 0, 1, 1].to_vec();
        let oup = "AAAACAI".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP32);
        let r1 = _encode_base32i(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base32i(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_1() {
        let inp = b"ABCDEFGHIJKL".to_vec();
        let oup = "IFBEGRCFIZDUQSKKJNGA".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP32);
        let r1 = _encode_base32i(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base32i(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_2() {
        let inp = b"ABCDEFGHIJK".to_vec();
        let oup = "IFBEGRCFIZDUQSKKJM".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP32);
        let r1 = _encode_base32i(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base32i(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_3() {
        let inp = b"ABCDEFGHIJ".to_vec();
        let oup = "IFBEGRCFIZDUQSKK".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP32);
        let r1 = _encode_base32i(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base32i(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
}
