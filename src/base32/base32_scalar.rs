use super::super::*;
/*
 * Base32 format:
 *      chunk from 8bit sequence to 5bit sequence:
 *          Z         *          E         I          A
 *          01111_010 00_10101_0 0100_0101 0_10010_01 010_00001
 *          01111 010_00 10101 0_0100 0101_0 10010 01_010 00001
 *      result from 5 bytes to 8bytes
*/
#[inline(never)]
pub(crate) fn _encode_base32_scalar(
    ags: &AsciiGraphicSet,
    inp: &[u8],
) -> Result<String, EncodeError> {
    let oup_sz = 1 + ((inp.len() + 4) / 5) * 8;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = _encode_base32_scalar_chunks10(ags, inp, &mut oup[0..])?;
    oup.resize(oup_idx, 0u8);
    let string = unsafe { String::from_utf8_unchecked(oup) };
    assert!(string.len() == oup_idx);
    Ok(string)
}

#[inline(always)]
pub(crate) fn _encode_base32_scalar_chunks10(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut oup_idx = 0;
    let inp = if inp.len() < 10 {
        inp
    } else {
        let mut iter = inp.chunks_exact(10);
        let mut nx = iter.next();
        while let Some(aa) = nx {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = aa[2];
            let b3 = aa[3];
            let b4 = aa[4];
            //
            let v0 = b0 >> 3;
            let v1 = ((b0 & 0b111) << 2) | (b1 >> 6);
            let v2 = (b1 & 0b111110) >> 1;
            let v3 = ((b1 & 0b1) << 4) | (b2 >> 4);
            let v4 = ((b2 & 0b1111) << 1) | (b3 >> 7);
            let v5 = (b3 & 0b1111100) >> 2;
            let v6 = ((b3 & 0b11) << 3) | (b4 >> 5);
            let v7 = b4 & 0b11111;
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
            //
            let b0 = aa[5];
            let b1 = aa[6];
            let b2 = aa[7];
            let b3 = aa[8];
            let b4 = aa[9];
            //
            let v0 = b0 >> 3;
            let v1 = ((b0 & 0b111) << 2) | (b1 >> 6);
            let v2 = (b1 & 0b111110) >> 1;
            let v3 = ((b1 & 0b1) << 4) | (b2 >> 4);
            let v4 = ((b2 & 0b1111) << 1) | (b3 >> 7);
            let v5 = (b3 & 0b1111100) >> 2;
            let v6 = ((b3 & 0b11) << 3) | (b4 >> 5);
            let v7 = b4 & 0b11111;
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
            //
            nx = iter.next();
        }
        iter.remainder()
    };
    let oo_idx = _encode_base32_scalar_chunks5(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[inline(always)]
pub(crate) fn _encode_base32_scalar_chunks5(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut oup_idx = 0;
    let inp = if inp.len() < 5 {
        inp
    } else {
        let mut iter = inp.chunks_exact(5);
        let mut nx = iter.next();
        while let Some(aa) = nx {
            let b0 = aa[0];
            let b1 = aa[1];
            let b2 = aa[2];
            let b3 = aa[3];
            let b4 = aa[4];
            let v0 = b0 >> 3;
            let v1 = ((b0 & 0b111) << 2) | (b1 >> 6);
            let v2 = (b1 & 0b111110) >> 1;
            let v3 = ((b1 & 0b1) << 4) | (b2 >> 4);
            let v4 = ((b2 & 0b1111) << 1) | (b3 >> 7);
            let v5 = (b3 & 0b1111100) >> 2;
            let v6 = ((b3 & 0b11) << 3) | (b4 >> 5);
            let v7 = b4 & 0b11111;
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
    let oo_idx = _encode_base32_scalar_rest(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[inline(always)]
fn _encode_base32_scalar_rest(
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
            let v0 = b0 >> 3;
            let v1 = ((b0 & 0b111) << 2) | (b1 >> 6);
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
            let v0 = b0 >> 3;
            let v1 = ((b0 & 0b111) << 2) | (b1 >> 6);
            let v2 = (b1 & 0b111110) >> 1;
            let v3 = ((b1 & 0b1) << 4) | (b2 >> 4);
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
        }
        3 => {
            let b0 = inp[0];
            let b1 = inp[1];
            let b2 = inp[2];
            let b3 = 0;
            let v0 = b0 >> 3;
            let v1 = ((b0 & 0b111) << 2) | (b1 >> 6);
            let v2 = (b1 & 0b111110) >> 1;
            let v3 = ((b1 & 0b1) << 4) | (b2 >> 4);
            let v4 = ((b2 & 0b1111) << 1) | (b3 >> 7);
            //
            // from binary to ascii
            let v0 = ags.getq(v0)?;
            let v1 = ags.getq(v1)?;
            let v2 = ags.getq(v2)?;
            let v3 = ags.getq(v3)?;
            let v4 = ags.getq(v4)?;
            //
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup[oup_idx + 3] = v3;
            oup[oup_idx + 4] = v4;
            oup_idx += 5;
        }
        4 => {
            let b0 = inp[0];
            let b1 = inp[1];
            let b2 = inp[2];
            let b3 = inp[3];
            let b4 = 0;
            let v0 = b0 >> 3;
            let v1 = ((b0 & 0b111) << 2) | (b1 >> 6);
            let v2 = (b1 & 0b111110) >> 1;
            let v3 = ((b1 & 0b1) << 4) | (b2 >> 4);
            let v4 = ((b2 & 0b1111) << 1) | (b3 >> 7);
            let v5 = (b3 & 0b1111100) >> 2;
            let v6 = ((b3 & 0b11) << 3) | (b4 >> 5);
            //
            // from binary to ascii
            let v0 = ags.getq(v0)?;
            let v1 = ags.getq(v1)?;
            let v2 = ags.getq(v2)?;
            let v3 = ags.getq(v3)?;
            let v4 = ags.getq(v4)?;
            let v5 = ags.getq(v5)?;
            let v6 = ags.getq(v6)?;
            //
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup[oup_idx + 3] = v3;
            oup[oup_idx + 4] = v4;
            oup[oup_idx + 5] = v5;
            oup[oup_idx + 6] = v6;
            oup_idx += 7;
        }
        _ => unreachable!(),
    }
    Ok(oup_idx)
}

#[inline(never)]
pub(crate) fn _decode_base32_scalar(
    ags: &AsciiGraphicSet,
    a: &str,
) -> Result<Vec<u8>, DecodeError> {
    let inp = a.as_bytes();
    let oup_sz = (inp.len() / 8) * 5 + 4;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = _decode_base32_scalar_chunks16(ags, inp, &mut oup[0..])?;
    oup.resize(oup_idx, 0u8);
    Ok(oup)
}

#[inline(always)]
pub(crate) fn _decode_base32_scalar_chunks16(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, DecodeError> {
    let mut oup_idx = 0;
    let inp = if inp.len() < 16 {
        inp
    } else {
        let mut iter = inp.chunks_exact(16);
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
            //
            let c0 = ags.posq(c0)?;
            let c1 = ags.posq(c1)?;
            let c2 = ags.posq(c2)?;
            let c3 = ags.posq(c3)?;
            let c4 = ags.posq(c4)?;
            let c5 = ags.posq(c5)?;
            let c6 = ags.posq(c6)?;
            let c7 = ags.posq(c7)?;
            //
            let v0 = (c0 << 3) | (c1 >> 2);
            let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
            let v2 = (c3 << 4) | (c4 >> 1);
            let v3 = (c4 << 7) | (c5 << 2) | (c6 >> 3);
            let v4 = (c6 << 5) | c7;
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup[oup_idx + 3] = v3;
            oup[oup_idx + 4] = v4;
            oup_idx += 5;
            //
            let c0 = aa[8];
            let c1 = aa[9];
            let c2 = aa[10];
            let c3 = aa[11];
            let c4 = aa[12];
            let c5 = aa[13];
            let c6 = aa[14];
            let c7 = aa[15];
            //
            let c0 = ags.posq(c0)?;
            let c1 = ags.posq(c1)?;
            let c2 = ags.posq(c2)?;
            let c3 = ags.posq(c3)?;
            let c4 = ags.posq(c4)?;
            let c5 = ags.posq(c5)?;
            let c6 = ags.posq(c6)?;
            let c7 = ags.posq(c7)?;
            //
            let v0 = (c0 << 3) | (c1 >> 2);
            let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
            let v2 = (c3 << 4) | (c4 >> 1);
            let v3 = (c4 << 7) | (c5 << 2) | (c6 >> 3);
            let v4 = (c6 << 5) | c7;
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup[oup_idx + 3] = v3;
            oup[oup_idx + 4] = v4;
            oup_idx += 5;
            //
            nx = iter.next();
        }
        iter.remainder()
    };
    let oo_idx = _decode_base32_scalar_chunks8(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}
#[inline(always)]
pub(crate) fn _decode_base32_scalar_chunks8(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, DecodeError> {
    let mut oup_idx = 0;
    let inp = if inp.len() < 8 {
        inp
    } else {
        let mut iter = inp.chunks_exact(8);
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
            //
            let c0 = ags.posq(c0)?;
            let c1 = ags.posq(c1)?;
            let c2 = ags.posq(c2)?;
            let c3 = ags.posq(c3)?;
            let c4 = ags.posq(c4)?;
            let c5 = ags.posq(c5)?;
            let c6 = ags.posq(c6)?;
            let c7 = ags.posq(c7)?;
            //
            let v0 = (c0 << 3) | (c1 >> 2);
            let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
            let v2 = (c3 << 4) | (c4 >> 1);
            let v3 = (c4 << 7) | (c5 << 2) | (c6 >> 3);
            let v4 = (c6 << 5) | c7;
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup[oup_idx + 3] = v3;
            oup[oup_idx + 4] = v4;
            oup_idx += 5;
            nx = iter.next();
        }
        iter.remainder()
    };
    let oo_idx = _decode_base32_scalar_rest(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[inline(always)]
fn _decode_base32_scalar_rest(
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
            let v0 = (c0 << 3) | (c1 >> 2);
            assert!(0b11 & c1 == 0);
            oup[oup_idx] = v0;
            oup_idx += 1;
        }
        4 => {
            let c0 = inp[0];
            let c1 = inp[1];
            let c2 = inp[2];
            let c3 = inp[3];
            //
            let c0 = ags.posq(c0)?;
            let c1 = ags.posq(c1)?;
            let c2 = ags.posq(c2)?;
            let c3 = ags.posq(c3)?;
            //
            let v0 = (c0 << 3) | (c1 >> 2);
            let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
            assert!(0b1111 & c3 == 0);
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup_idx += 2;
        }
        5 => {
            let c0 = inp[0];
            let c1 = inp[1];
            let c2 = inp[2];
            let c3 = inp[3];
            let c4 = inp[4];
            //
            let c0 = ags.posq(c0)?;
            let c1 = ags.posq(c1)?;
            let c2 = ags.posq(c2)?;
            let c3 = ags.posq(c3)?;
            let c4 = ags.posq(c4)?;
            //
            let v0 = (c0 << 3) | (c1 >> 2);
            let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
            let v2 = (c3 << 4) | (c4 >> 1);
            assert!(0b1 & c4 == 0);
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup_idx += 3;
        }
        7 => {
            let c0 = inp[0];
            let c1 = inp[1];
            let c2 = inp[2];
            let c3 = inp[3];
            let c4 = inp[4];
            let c5 = inp[5];
            let c6 = inp[6];
            //
            let c0 = ags.posq(c0)?;
            let c1 = ags.posq(c1)?;
            let c2 = ags.posq(c2)?;
            let c3 = ags.posq(c3)?;
            let c4 = ags.posq(c4)?;
            let c5 = ags.posq(c5)?;
            let c6 = ags.posq(c6)?;
            //
            let v0 = (c0 << 3) | (c1 >> 2);
            let v1 = (c1 << 6) | (c2 << 1) | (c3 >> 4);
            let v2 = (c3 << 4) | (c4 >> 1);
            let v3 = (c4 << 7) | (c5 << 2) | (c6 >> 3);
            assert!(0b111 & c6 == 0);
            oup[oup_idx] = v0;
            oup[oup_idx + 1] = v1;
            oup[oup_idx + 2] = v2;
            oup[oup_idx + 3] = v3;
            oup_idx += 4;
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
