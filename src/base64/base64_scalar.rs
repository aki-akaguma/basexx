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
    inp: &[u8],
) -> Result<String, EncodeError> {
    let oup_sz = 1 + ((inp.len() + 2) / 3) * 4;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = _encode_base64_scalar_chunks6(ags, inp, &mut oup[0..])?;
    oup.resize(oup_idx, 0u8);
    let string = unsafe { String::from_utf8_unchecked(oup) };
    assert!(string.len() == oup_idx);
    Ok(string)
}

#[inline(always)]
pub(crate) fn _encode_base64_scalar_chunks6(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut oup_idx = 0;
    let inp = if inp.len() < 6 {
        inp
    } else {
        let mut iter = inp.chunks_exact(6);
        let mut nx = iter.next();
        while let Some(aa) = nx {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = aa[2];
            let b3 = aa[3];
            let b4 = aa[4];
            let b5 = aa[5];
            //
            let v0 = b0 >> 2;
            let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
            let v2 = (b1 & 0b1111) << 2 | (b2 >> 6);
            let v3 = b2 & 0b111111;
            let v4 = b3 >> 2;
            let v5 = (b3 & 0b11) << 4 | (b4 >> 4);
            let v6 = (b4 & 0b1111) << 2 | (b5 >> 6);
            let v7 = b5 & 0b111111;
            //
            // from binary to ascii
            let v0 = ags.getq(v0)?;
            let v1 = ags.getq(v1)?;
            let v2 = ags.getq(v2)?;
            let v3 = ags.getq(v3)?;
            let v4 = ags.getq(v4)?;
            let v5 = ags.getq(v5)?;
            let v6 = ags.getq(v6)?;
            let v7 = ags.getq(v7)?;
            //
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup[oup_idx + 3] = v3;
            oup[oup_idx + 4] = v4;
            oup[oup_idx + 5] = v5;
            oup[oup_idx + 6] = v6;
            oup[oup_idx + 7] = v7;
            oup_idx += 8;
            nx = iter.next();
        }
        iter.remainder()
    };
    let oo_idx = _encode_base64_scalar_chunks3(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[inline(always)]
fn _encode_base64_scalar_chunks3(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut oup_idx = 0;
    let inp = if inp.len() < 3 {
        inp
    } else {
        let mut iter = inp.chunks_exact(3);
        let mut nx = iter.next();
        while let Some(aa) = nx {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = aa[2];
            let v0 = b0 >> 2;
            let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
            let v2 = (b1 & 0b1111) << 2 | (b2 >> 6);
            let v3 = b2 & 0b111111;
            //
            // from binary to ascii
            let v0 = ags.getq(v0)?;
            let v1 = ags.getq(v1)?;
            let v2 = ags.getq(v2)?;
            let v3 = ags.getq(v3)?;
            //
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup[oup_idx + 3] = v3;
            oup_idx += 4;
            nx = iter.next();
        }
        iter.remainder()
    };
    let oo_idx = _encode_base64_scalar_rest(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[inline(always)]
fn _encode_base64_scalar_rest(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut oup_idx = 0;
    match inp.len() {
        0 => (),
        1 => {
            let b0 = inp[0];
            let b1 = 0;
            let v0 = b0 >> 2;
            let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
            //
            // from binary to ascii
            let v0 = ags.getq(v0)?;
            let v1 = ags.getq(v1)?;
            //
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup_idx += 2;
        }
        2 => {
            let b0 = inp[0];
            let b1 = inp[1];
            let b2 = 0;
            let v0 = b0 >> 2;
            let v1 = (b0 & 0b11) << 4 | (b1 >> 4);
            let v2 = (b1 & 0b1111) << 2 | (b2 >> 6);
            //
            // from binary to ascii
            let v0 = ags.getq(v0)?;
            let v1 = ags.getq(v1)?;
            let v2 = ags.getq(v2)?;
            //
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup_idx += 3;
        }
        _ => unreachable!(),
    }
    Ok(oup_idx)
}

#[inline(never)]
pub(crate) fn _decode_base64_scalar(
    ags: &AsciiGraphicSet,
    a: &str,
) -> Result<Vec<u8>, DecodeError> {
    let inp = a.as_bytes();
    let oup_sz = (inp.len() / 4) * 3 + 2;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = _decode_base64_scalar_chunks4(ags, inp, &mut oup[0..])?;
    oup.resize(oup_idx, 0u8);
    Ok(oup)
}

#[inline(always)]
pub(crate) fn _decode_base64_scalar_chunks4(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, DecodeError> {
    let mut oup_idx = 0;
    let inp = if inp.len() < 4 {
        inp
    } else {
        let mut iter = inp.chunks_exact(4);
        let mut nx = iter.next();
        while let Some(aa) = nx {
            let c0 = aa[0];
            let c1 = aa[1];
            let c2 = aa[2];
            let c3 = aa[3];
            //
            let c0 = ags.posq(c0)?;
            let c1 = ags.posq(c1)?;
            let c2 = ags.posq(c2)?;
            let c3 = ags.posq(c3)?;
            //
            let v0 = (c0 << 2) | (c1 >> 4);
            let v1 = (c1 << 4) | (c2 >> 2);
            let v2 = (c2 << 6) | c3;
            //
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup_idx += 3;
            nx = iter.next();
        }
        iter.remainder()
    };
    let oo_idx = _decode_base64_scalar_rest(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[inline(always)]
fn _decode_base64_scalar_rest(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, DecodeError> {
    let mut oup_idx = 0;
    match inp.len() {
        0 => (),
        2 => {
            let c0 = inp[0];
            let c1 = inp[1];
            //
            let c0 = ags.posq(c0)?;
            let c1 = ags.posq(c1)?;
            //
            let v0 = (c0 << 2) | (c1 >> 4);
            assert!(0b1111 & c1 == 0);
            oup[oup_idx] = v0;
            oup_idx += 1;
        }
        3 => {
            let c0 = inp[0];
            let c1 = inp[1];
            let c2 = inp[2];
            //
            let c0 = ags.posq(c0)?;
            let c1 = ags.posq(c1)?;
            let c2 = ags.posq(c2)?;
            //
            let v0 = (c0 << 2) | (c1 >> 4);
            let v1 = (c1 << 4) | (c2 >> 2);
            assert!(0b11 & c2 == 0);
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup_idx += 2;
        }
        _ => return Err(DecodeError::InvalidLength(inp.len())),
    }
    Ok(oup_idx)
}

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
