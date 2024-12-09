use super::super::*;

#[inline(never)]
#[target_feature(enable = "sse2")]
pub(crate) unsafe fn _encode_base32_ssse3(
    ags: &AsciiGraphicSet,
    inp: &[u8],
) -> Result<String, EncodeError> {
    let oup_sz = 1 + ((inp.len() + 4) / 5) * 8;
    // encode binary
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = _encode_base32_scalar_chunks10(ags, inp, &mut oup[0..])?;
    oup.resize(oup_idx, 0u8);
    let string = unsafe { String::from_utf8_unchecked(oup) };
    assert!(string.len() == oup_idx);
    Ok(string)
}

#[inline(never)]
#[target_feature(enable = "sse2")]
pub(crate) unsafe fn _decode_base32_ssse3(
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

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
