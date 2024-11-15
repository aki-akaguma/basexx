use super::*;

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

impl Base32 {
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base32(&self.ags, a)
    }
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        _decode_base32(&self.ags, a)
    }
}

/*
 * Base32 format:
 *      chunk from 8bit sequence to 5bit sequence:
 *          Z         *          E         I          A
 *          01111_010 00_10101_0 0100_0101 0_10010_01 010_00001
 *          01111 010_00 10101 0_0100 0101_0 10010 01_010 00001
 *      result from 3 bytes to 4bytes
*/
fn _encode_base32(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    // encode binary
    let mut r = Vec::new();
    let mut iter = a.chunks_exact(5);
    let mut nx = iter.next();
    while let Some(aa) = nx {
        let b0 = aa[0];
        let b1 = aa[1];
        let b2 = aa[2];
        let b3 = aa[3];
        let b4 = aa[4];
        let v0 = b0 >> 3;
        let v1 = (b0 & 0b111) << 2 | (b1 >> 6);
        let v2 = (b1 & 0b111110) >> 1;
        let v3 = (b1 & 0b1) << 4 | (b2 >> 4);
        let v4 = (b2 & 0b1111) << 1 | (b3 >> 7);
        let v5 = (b3 & 0b1111100) >> 2;
        let v6 = (b3 & 0b11) << 3 | (b4 >> 5);
        let v7 = b4 & 0b11111;
        r.push(v0);
        r.push(v1);
        r.push(v2);
        r.push(v3);
        r.push(v4);
        r.push(v5);
        r.push(v6);
        r.push(v7);
        nx = iter.next();
    }
    let aa = iter.remainder();
    match aa.len() {
        0 => (),
        1 => {
            let b0 = aa[0];
            let b1 = 0;
            let v0 = b0 >> 3;
            let v1 = (b0 & 0b111) << 2 | (b1 >> 6);
            r.push(v0);
            r.push(v1);
        }
        2 => {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = 0;
            let v0 = b0 >> 3;
            let v1 = (b0 & 0b111) << 2 | (b1 >> 6);
            let v2 = (b1 & 0b111110) >> 1;
            let v3 = (b1 & 0b1) << 4 | (b2 >> 4);
            r.push(v0);
            r.push(v1);
            r.push(v2);
            r.push(v3);
        }
        3 => {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = aa[2];
            let b3 = 0;
            let v0 = b0 >> 3;
            let v1 = (b0 & 0b111) << 2 | (b1 >> 6);
            let v2 = (b1 & 0b111110) >> 1;
            let v3 = (b1 & 0b1) << 4 | (b2 >> 4);
            let v4 = (b2 & 0b1111) << 1 | (b3 >> 7);
            r.push(v0);
            r.push(v1);
            r.push(v2);
            r.push(v3);
            r.push(v4);
        }
        4 => {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = aa[2];
            let b3 = aa[3];
            let b4 = 0;
            let v0 = b0 >> 3;
            let v1 = (b0 & 0b111) << 2 | (b1 >> 6);
            let v2 = (b1 & 0b111110) >> 1;
            let v3 = (b1 & 0b1) << 4 | (b2 >> 4);
            let v4 = (b2 & 0b1111) << 1 | (b3 >> 7);
            let v5 = (b3 & 0b1111100) >> 2;
            let v6 = (b3 & 0b11) << 3 | (b4 >> 5);
            r.push(v0);
            r.push(v1);
            r.push(v2);
            r.push(v3);
            r.push(v4);
            r.push(v5);
            r.push(v6);
        }
        _ => unreachable!(),
    }
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

fn _decode_base32(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
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
    let mut rr = Vec::new();
    let mut iter = r.chunks_exact(8);
    let mut nx = iter.next();
    while let Some(aa) = nx {
        let c0 = aa[0];
        let c1 = aa[1];
        let c2 = aa[2];
        let c3 = aa[3];
        let c4 = aa[4];
        let c5 = aa[5];
        let c6 = aa[6];
        let c7 = aa[7];
        let v0 = (c0 << 3) | (c1 >> 2);
        let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
        let v2 = (c3 << 4) | (c4 >> 1);
        let v3 = (c4 << 7) | (c5 << 2) | (c6 >> 3);
        let v4 = (c6 << 5) | c7;
        rr.push(v0);
        rr.push(v1);
        rr.push(v2);
        rr.push(v3);
        rr.push(v4);
        nx = iter.next();
    }
    let aa = iter.remainder();
    match aa.len() {
        0 => (),
        2 => {
            let c0 = aa[0];
            let c1 = aa[1];
            let v0 = (c0 << 3) | (c1 >> 2);
            assert!(0b11 & c1 == 0);
            rr.push(v0);
        }
        4 => {
            let c0 = aa[0];
            let c1 = aa[1];
            let c2 = aa[2];
            let c3 = aa[3];
            let v0 = (c0 << 3) | (c1 >> 2);
            let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
            assert!(0b1111 & c3 == 0);
            rr.push(v0);
            rr.push(v1);
        }
        5 => {
            let c0 = aa[0];
            let c1 = aa[1];
            let c2 = aa[2];
            let c3 = aa[3];
            let c4 = aa[4];
            let v0 = (c0 << 3) | (c1 >> 2);
            let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
            let v2 = (c3 << 4) | (c4 >> 1);
            assert!(0b1 & c4 == 0);
            rr.push(v0);
            rr.push(v1);
            rr.push(v2);
        }
        7 => {
            let c0 = aa[0];
            let c1 = aa[1];
            let c2 = aa[2];
            let c3 = aa[3];
            let c4 = aa[4];
            let c5 = aa[5];
            let c6 = aa[6];
            let v0 = (c0 << 3) | (c1 >> 2);
            let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
            let v2 = (c3 << 4) | (c4 >> 1);
            let v3 = (c4 << 7) | (c5 << 2) | (c6 >> 3);
            assert!(0b111 & c6 == 0);
            rr.push(v0);
            rr.push(v1);
            rr.push(v2);
            rr.push(v3);
        }
        _ => return Err(DecodeError::InvalidLength(a.len())),
    }
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
        let r1 = _encode_base32(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base32(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_1() {
        let inp = b"ABCDEFGHIJKL".to_vec();
        let oup = "IFBEGRCFIZDUQSKKJNGA".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP32);
        let r1 = _encode_base32(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base32(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_2() {
        let inp = b"ABCDEFGHIJK".to_vec();
        let oup = "IFBEGRCFIZDUQSKKJM".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP32);
        let r1 = _encode_base32(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base32(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_3() {
        let inp = b"ABCDEFGHIJ".to_vec();
        let oup = "IFBEGRCFIZDUQSKK".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP32);
        let r1 = _encode_base32(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base32(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
}
