mod ascii_graphic_set;
use ascii_graphic_set::*;

/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Base64
*/

#[derive(Debug)]
pub enum EncodeError {
    InvalidIndex(u8),
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidByte(u8),
    InvalidLength(usize),
}

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
        Self {
            ags: AsciiGraphicSet::new(),
        }
    }
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base64(&self.ags, a)
    }
    pub fn decode(&self, a: &str) -> Result<Vec<u8>, DecodeError> {
        _decode_base64(&self.ags, a)
    }
}

/*
 * Base64 format:
 *      chunk from 8bit sequence to 6bit sequence:
 *          Z         *         E
 *          011110_10 0010_1010 01_000101
 *          011110 10_0010 1010_01 000101
 *      result from 3 bytes to 4bytes
*/
fn _encode_base64(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    let mut r = Vec::new();
    let mut iter = a.chunks_exact(3);
    let mut nx = iter.next();
    while let Some(aa) = nx {
        let b0 = aa[0];
        let b1 = aa[1];
        let b2 = aa[2];
        let v0 = b0 >> 2;
        let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
        let v2 = (b1 & 0b1111) << 2 | (b2 >> 6);
        let v3 = b2 & 0b111111;
        r.push(v0);
        r.push(v1);
        r.push(v2);
        r.push(v3);
        nx = iter.next();
    }
    let aa = iter.remainder();
    match aa.len() {
        0 => (),
        1 => {
            let b0 = aa[0];
            let b1 = 0;
            let v0 = b0 >> 2;
            let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
            r.push(v0);
            r.push(v1);
        }
        2 => {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = 0;
            let v0 = b0 >> 2;
            let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
            let v2 = (b1 & 0b1111) << 2 | (b2 >> 6);
            r.push(v0);
            r.push(v1);
            r.push(v2);
        }
        _ => unreachable!(),
    }
    let rr: Result<Vec<u8>, EncodeError> = r
        .iter()
        .map(|&b| match ags.get(b) {
            Some(ascii) => Ok(ascii),
            None => Err(EncodeError::InvalidIndex(b)),
        })
        .collect();
    let rrr = match rr {
        Ok(rrr) => rrr,
        Err(err) => return Err(err),
    };
    let s = String::from_utf8_lossy(&rrr).to_string();
    assert!(s.len() == rrr.len());
    Ok(s)
}

fn _decode_base64(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
    /*
    let mut r = Vec::new();
    let a = a.as_bytes();
    for aa in a {
        if let Some(n) = CMAP64.iter().position(|x| x == aa) {
            r.push(n as u8);
        } else {
            unreachable!();
        }
    }
    */
    let r: Result<Vec<u8>, _> = a
        .as_bytes()
        .iter()
        .map(|&b| {
            if let Some(n) = ags.position(b) {
                Ok(n)
            } else {
                Err(DecodeError::InvalidByte(b))
            }
        })
        .collect();
    let r = match r {
        Ok(r) => r,
        Err(err) => return Err(err),
    };
    let mut rr = Vec::new();
    let mut iter = r.chunks_exact(4);
    let mut nx = iter.next();
    while let Some(aa) = nx {
        let c0 = aa[0];
        let c1 = aa[1];
        let c2 = aa[2];
        let c3 = aa[3];
        let v0 = (c0 << 2) | (c1 >> 4);
        let v1 = (c1 << 4) | (c2 >> 2);
        let v2 = (c2 << 6) | c3;
        rr.push(v0);
        rr.push(v1);
        rr.push(v2);
        nx = iter.next();
    }
    let aa = iter.remainder();
    match aa.len() {
        0 => (),
        1 => return Err(DecodeError::InvalidLength(a.len())),
        2 => {
            let c0 = aa[0];
            let c1 = aa[1];
            let v0 = (c0 << 2) | (c1 >> 4);
            assert!(0b1111 & c1 == 0);
            rr.push(v0);
        }
        3 => {
            let c0 = aa[0];
            let c1 = aa[1];
            let c2 = aa[2];
            let v0 = (c0 << 2) | (c1 >> 4);
            let v1 = (c1 << 4) | (c2 >> 2);
            assert!(0b11 & c2 == 0);
            rr.push(v0);
            rr.push(v1);
        }
        _ => return Err(DecodeError::InvalidLength(a.len())),
    }
    Ok(rr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let inp = b"ABCDEFGHIJKL".to_vec();
        let oup = "QUJDREVGR0hJSktM".to_string();
        let ags = AsciiGraphicSet::new();
        let r1 = _encode_base64(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base64(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_2() {
        let inp = b"ABCDEFGHIJK".to_vec();
        let oup = "QUJDREVGR0hJSks".to_string();
        let ags = AsciiGraphicSet::new();
        let r1 = _encode_base64(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base64(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_3() {
        let inp = b"ABCDEFGHIJ".to_vec();
        let oup = "QUJDREVGR0hJSg".to_string();
        let ags = AsciiGraphicSet::new();
        let r1 = _encode_base64(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base64(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
}
