use super::super::*;
/*
 * Base64 format:
 *      chunk from 8bit sequence to 6bit sequence:
 *          Z         *         E
 *          011110_10 0010_1010 01_000101
 *          011110 10_0010 1010_01 000101
 *      result from 3 bytes to 4bytes
*/
#[inline(never)]
pub(crate) fn _encode_base64_scalar(
    ags: &AsciiGraphicSet,
    a: &[u8],
) -> Result<String, EncodeError> {
    let rsz = 1 + ((a.len() + 2) / 3) * 4;
    // encode binary
    let mut r = vec![0u8; rsz];
    //
    let r_idx = _encode_base64_scalar_chunks6(a, &mut r[0..]);
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

#[inline(always)]
pub(crate) fn _encode_base64_scalar_chunks6<'a>(a: &'a [u8], r: &'a mut [u8]) -> usize {
    let mut r_idx = 0;
    let a = if a.len() >= 6 {
        let mut iter = a.chunks_exact(6);
        let mut nx = iter.next();
        while let Some(aa) = nx {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = aa[2];
            let b3 = aa[3];
            let b4 = aa[4];
            let b5 = aa[5];
            let v0 = b0 >> 2;
            let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
            let v2 = (b1 & 0b1111) << 2 | (b2 >> 6);
            let v3 = b2 & 0b111111;
            let v4 = b3 >> 2;
            let v5 = (b3 & 0b11) << 4 | (b4 >> 4);
            let v6 = (b4 & 0b1111) << 2 | (b5 >> 6);
            let v7 = b5 & 0b111111;
            r[r_idx] = v0;
            r[r_idx + 1] = v1;
            r[r_idx + 2] = v2;
            r[r_idx + 3] = v3;
            r[r_idx + 4] = v4;
            r[r_idx + 5] = v5;
            r[r_idx + 6] = v6;
            r[r_idx + 7] = v7;
            r_idx += 8;
            nx = iter.next();
        }
        iter.remainder()
    } else {
        a
    };
    let rr_idx = _encode_base64_scalar_chunks3(a, &mut r[r_idx..]);
    r_idx + rr_idx
}

#[inline(always)]
fn _encode_base64_scalar_chunks3<'a>(a: &'a [u8], r: &'a mut [u8]) -> usize {
    let mut r_idx = 0;
    let a = if a.len() >= 3 {
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
        iter.remainder()
    } else {
        a
    };
    let rr_idx = _encode_base64_scalar_rest(a, &mut r[r_idx..]);
    r_idx + rr_idx
}

#[inline(always)]
fn _encode_base64_scalar_rest(a: &[u8], r: &mut [u8]) -> usize {
    let mut r_idx = 0;
    match a.len() {
        0 => (),
        1 => {
            let b0 = a[0];
            let b1 = 0;
            let v0 = b0 >> 2;
            let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
            r[r_idx] = v0;
            r[r_idx + 1] = v1;
            r_idx += 2;
        }
        2 => {
            let b0 = a[0];
            let b1 = a[1];
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
    r_idx
}

/*
#[inline(never)]
pub fn _encode_base64_scalar(ags: &AsciiGraphicSet, a: &[u8]) -> Result<String, EncodeError> {
    let rsz = 1 + ((a.len() + 2) / 3) * 4;
    // encode binary
    let mut r = vec![0u8; rsz];
    let mut r_idx = 0;
    //
    let mut iter = a.chunks_exact(6);
    let mut nx = iter.next();
    while let Some(aa) = nx {
        let b0 = aa[0];
        let b1 = aa[1];
        let b2 = aa[2];
        let b3 = aa[3];
        let b4 = aa[4];
        let b5 = aa[5];
        let v0 = b0 >> 2;
        let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
        let v2 = (b1 & 0b1111) << 2 | (b2 >> 6);
        let v3 = b2 & 0b111111;
        let v4 = b3 >> 2;
        let v5 = (b3 & 0b11) << 4 | (b4 >> 4);
        let v6 = (b4 & 0b1111) << 2 | (b5 >> 6);
        let v7 = b5 & 0b111111;
        r[r_idx] = v0;
        r[r_idx + 1] = v1;
        r[r_idx + 2] = v2;
        r[r_idx + 3] = v3;
        r[r_idx + 4] = v4;
        r[r_idx + 5] = v5;
        r[r_idx + 6] = v6;
        r[r_idx + 7] = v7;
        r_idx += 8;
        nx = iter.next();
    }
    let a = iter.remainder();
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
*/

#[inline(never)]
pub(crate) fn _decode_base64_scalar(
    ags: &AsciiGraphicSet,
    a: &str,
) -> Result<Vec<u8>, DecodeError> {
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

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
