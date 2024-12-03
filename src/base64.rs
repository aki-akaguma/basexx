use super::*;

/*
 * ref.)
 *     https://en.wikipedia.org/wiki/Base64
*/

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

impl Base64 {
    pub fn encode(&self, a: &[u8]) -> Result<String, EncodeError> {
        _encode_base64(&self.ags, a)
        /*
        if cfg!(target_feature = "sse2") {
            unsafe { _encode_base64_sse2(&self.ags, a) }
        } else {
            _encode_base64(&self.ags, a)
        }
        */
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
// http://0x80.pl/notesen/2016-01-12-sse-base64-encoding.html
#[inline(never)]
#[target_feature(enable = "sse2")]
unsafe fn _encode_base64_sse2(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    //panic!("PASS: SSE2");
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    let rsz = 1 + ((a.len() + 2) / 3) * 4;
    // encode binary
    let mut r = vec![0u8; rsz];
    let mut r_idx;
    let a_len = a.len();
    let mut a_ptr = a.as_ptr();
    let mut a_out = r.as_ptr();
    let end_ptr = unsafe { a_ptr.add(a_len) };
    let end_ptr_limit = unsafe { end_ptr.sub(16) };
    //
    let mm_shuf: __m128i = _mm_set_epi8(10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1);
    let mm_t0_set1 = _mm_set1_epi32(0x0fc0fc00);
    let mm_t1_set1 = _mm_set1_epi32(0x04000040);
    let mm_t2_set1 = _mm_set1_epi32(0x003f03f0);
    let mm_t3_set1 = _mm_set1_epi32(0x01000010);
    //
    while a_ptr < end_ptr_limit {
        let mm_in = _mm_loadu_si128(a_ptr as *const __m128i);
        let mm_in = _mm_shuffle_epi8(mm_in, mm_shuf);
        let mm_t0 = _mm_and_si128(mm_in, mm_t0_set1);
        let mm_t1 = _mm_mulhi_epu16(mm_t0, mm_t1_set1);
        let mm_t2 = _mm_and_si128(mm_in, mm_t2_set1);
        let mm_t3 = _mm_mullo_epi16(mm_t2, mm_t3_set1);
        let indices = _mm_or_si128(mm_t1, mm_t3);
        _mm_storeu_si128(a_out as *mut __m128i, indices);
        a_out = unsafe { a_out.add(16) };
        a_ptr = unsafe { a_ptr.add(4 * 3) };
    }
    //
    r_idx = unsafe { a_out.offset_from(r.as_ptr()) as usize };
    let new_a_len = unsafe { end_ptr.offset_from(a_ptr) as usize };
    if new_a_len > 0 {
        let a = unsafe { std::slice::from_raw_parts(a_ptr, new_a_len) };
        //
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
            r[r_idx] = v0;
            r[r_idx + 1] = v1;
            r[r_idx + 2] = v2;
            r[r_idx + 3] = v3;
            r_idx += 4;
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
                r[r_idx] = v0;
                r[r_idx + 1] = v1;
                r_idx += 2;
            }
            2 => {
                let b0 = aa[0];
                let b1 = aa[1];
                let b2 = 0;
                let v0 = b0 >> 2;
                let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
                let v2 = (b1 & 0b1111) << 2 | (b2 >> 6);
                r[r_idx] = v0;
                r[r_idx + 1] = v1;
                r[r_idx + 2] = v2;
                r_idx += 3;
            }
            _ => unreachable!(),
        }
    }
    r.resize(r_idx, 0u8);
    // from binary to ascii
    match ags.binary_to_ascii(&mut r) {
        Ok(()) => (),
        Err(err) => return Err(err),
    }
    let s = String::from_utf8_lossy(&r).to_string();
    assert!(s.len() == r.len());
    Ok(s)
}
#[inline(never)]
fn _encode_base64(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    let rsz = 1 + ((a.len() + 2) / 3) * 4;
    // encode binary
    let mut r = vec![0u8; rsz];
    let mut r_idx = 0;
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
        r[r_idx] = v0;
        r[r_idx + 1] = v1;
        r[r_idx + 2] = v2;
        r[r_idx + 3] = v3;
        r_idx += 4;
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
            r[r_idx] = v0;
            r[r_idx + 1] = v1;
            r_idx += 2;
        }
        2 => {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = 0;
            let v0 = b0 >> 2;
            let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
            let v2 = (b1 & 0b1111) << 2 | (b2 >> 6);
            r[r_idx] = v0;
            r[r_idx + 1] = v1;
            r[r_idx + 2] = v2;
            r_idx += 3;
        }
        _ => unreachable!(),
    }
    r.resize(r_idx, 0u8);
    // from binary to ascii
    match ags.binary_to_ascii(&mut r) {
        Ok(()) => (),
        Err(err) => return Err(err),
    }
    let s = String::from_utf8_lossy(&r).to_string();
    assert!(s.len() == r.len());
    Ok(s)
}

#[inline(never)]
fn _decode_base64(ags: &AsciiGraphicSet, a: &str) -> Result<Vec<u8>, DecodeError> {
    // from ascii to binary
    let mut r = a.as_bytes().to_vec();
    match ags.ascii_to_binary(&mut r) {
        Ok(()) => (),
        Err(err) => return Err(err),
    }
    // decode binary
    let rsz = (r.len() / 4) * 3 + 2;
    //let mut rr = Vec::new();
    let mut rr = vec![0u8; rsz];
    let mut r_idx = 0;
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
        rr[r_idx] = v0;
        rr[r_idx + 1] = v1;
        rr[r_idx + 2] = v2;
        r_idx += 3;
        nx = iter.next();
    }
    let aa = iter.remainder();
    match aa.len() {
        0 => (),
        2 => {
            let c0 = aa[0];
            let c1 = aa[1];
            let v0 = (c0 << 2) | (c1 >> 4);
            assert!(0b1111 & c1 == 0);
            rr[r_idx] = v0;
            r_idx += 1;
        }
        3 => {
            let c0 = aa[0];
            let c1 = aa[1];
            let c2 = aa[2];
            let v0 = (c0 << 2) | (c1 >> 4);
            let v1 = (c1 << 4) | (c2 >> 2);
            assert!(0b11 & c2 == 0);
            rr[r_idx] = v0;
            rr[r_idx + 1] = v1;
            r_idx += 2;
        }
        _ => return Err(DecodeError::InvalidLength(a.len())),
    }
    rr.resize(r_idx, 0u8);
    Ok(rr)
}

const _CMAP64: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        let inp = [0u8, 0, 1, 1].to_vec();
        let oup = "AAABAQ".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP64);
        let r1 = _encode_base64(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base64(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_1() {
        let inp = b"ABCDEFGHIJKL".to_vec();
        let oup = "QUJDREVGR0hJSktM".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP64);
        let r1 = _encode_base64(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base64(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_2() {
        let inp = b"ABCDEFGHIJK".to_vec();
        let oup = "QUJDREVGR0hJSks".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP64);
        let r1 = _encode_base64(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base64(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
    #[test]
    fn it_works_3() {
        let inp = b"ABCDEFGHIJ".to_vec();
        let oup = "QUJDREVGR0hJSg".to_string();
        let ags = AsciiGraphicSet::with_slice(&_CMAP64);
        let r1 = _encode_base64(&ags, &inp).unwrap();
        assert_eq!(r1, oup);
        let r2 = _decode_base64(&ags, &r1).unwrap();
        assert_eq!(r2, inp);
    }
}
