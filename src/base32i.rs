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

impl Base32I {
    #[inline]
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base32i(&self.ags, a)
    }
    #[inline]
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
#[inline(never)]
fn _encode_base32i(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    // encode binary
    let (inp, inp_sz) = {
        let old_sz = a.len();
        let new_sz = ((old_sz + 4) / 5) * 5;
        let mut aa = Vec::with_capacity(new_sz);
        aa.extend(a);
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
    let zero_count = inp.iter().take_while(|&&x| x == 0).count();
    let oup = {
        let bigu = BigUint::from_bytes_be(&inp[zero_count..]);
        let mut oup: Vec<u8> = bigu.to_radix_le(32);
        if zero_count > 0 {
            let azc = inp[zero_count];
            let zzcount = (zero_count / 5) * 8;
            let (_, zzrest) = match zero_count % 5 {
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
            oup.resize(oup.len() + zzcount + zzrest, 0u8);
        }
        oup
    };
    // from binary to ascii
    let mut oupp = oup[inp_sz..].to_vec();
    oupp.reverse();
    ags.binary_to_ascii(&mut oupp)?;
    let oupp_sz = oupp.len();
    let string = unsafe { String::from_utf8_unchecked(oupp) };
    assert!(string.len() == oupp_sz);
    Ok(string)
}

#[inline(never)]
fn _decode_base32i(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
    // decode binary
    let (inp, bb_sz) = {
        let old_sz = a.len();
        let bb_sz = 8 - (old_sz % 8);
        let bb_sz = if bb_sz == 8 { 0 } else { bb_sz };
        let new_sz = old_sz + bb_sz;
        let mut r0 = Vec::with_capacity(new_sz);
        r0.extend(a.as_bytes());
        // from ascii to binary
        ags.ascii_to_binary(&mut r0)?;
        r0.resize(new_sz, 0u8);
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
    let zero_count = inp.iter().take_while(|&&x| x == 0).count();
    let oupp = {
        let bigu = BigUint::from_radix_be(&inp[zero_count..], 32).unwrap();
        let mut oupp: Vec<u8> = bigu.to_bytes_le();
        if zero_count > 0 {
            let rzc = inp[zero_count];
            let zzcount = (zero_count / 8) * 5;
            let (_, zzrest) = match zero_count % 8 {
                1 => (0, if rzc > 0b11 { 0 } else { 1 }),
                2 => (0, 1 + if rzc > 0 { 0 } else { 1 }),
                3 => (0, 1 + if rzc > 0b1111 { 0 } else { 1 }),
                4 => (0, 2 + if rzc > 0b1 { 0 } else { 1 }),
                5 => (0, 3 + if rzc > 0 { 0 } else { 1 }),
                6 => (0, 3 + if rzc > 0b111 { 0 } else { 1 }),
                7 => (0, 4 + if rzc > 0 { 0 } else { 1 }),
                _ => (0, 0),
            };
            oupp.resize(oupp.len() + zzcount + zzrest, 0u8);
        }
        oupp.reverse();
        oupp.resize(oupp.len() - bb_sz, 0);
        oupp
    };
    Ok(oupp)
}

const _CMAP32: [u8; 32] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'2', b'3', b'4', b'5', b'6', b'7',
];

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
