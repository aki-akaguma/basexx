use super::super::*;
//
// http://0x80.pl/notesen/2016-01-12-sse-base64-encoding.html
//
#[inline(never)]
#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _encode_base64_ssse3(
    ags: &AsciiGraphicSet,
    inp: &[u8],
) -> Result<String, EncodeError> {
    let oup_sz = 1 + inp.len().div_ceil(3) * 4;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = unsafe { _encode_base64_ssse3_chunks12(ags, inp, &mut oup[0..])? };
    oup.resize(oup_idx, 0u8);
    let string = unsafe { String::from_utf8_unchecked(oup) };
    assert!(string.len() == oup_idx);
    Ok(string)
}

#[inline(always)]
pub(crate) unsafe fn _encode_base64_ssse3_chunks12(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, EncodeError> {
    //panic!("PASS: SSE2");
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    let mut oup_idx = 0;
    let inp = if inp.len() < 16 {
        inp
    } else {
        let inp_len = inp.len();
        let mut inp_ptr = inp.as_ptr();
        let mut oup_ptr = oup.as_mut_ptr();
        let end_ptr = unsafe { inp_ptr.add(inp_len) };
        let end_ptr_limit = unsafe { end_ptr.sub(16 - 1) };
        //
        let mm_shuf: __m128i =
            unsafe { _mm_set_epi8(10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1) };
        let mut cc16 = [0u64; 2];
        //
        while inp_ptr < end_ptr_limit {
            unsafe {
                // 01234567|01234567|01234567|01234567|01234567|01234567|
                // aaaaaabb|bbbbcccc|ccdddddd|eeeeeeff|ffffgggg|gghhhhhh|
                //
                // gghhhhhh|ffffgggg|eeeeeeff|ccdddddd|bbbbcccc|aaaaaabb|
                let mm_in = _mm_loadu_si128(inp_ptr as *const __m128i);
                //
                // ffffgggg|gghhhhhh|eeeeeeff|ffffgggg|bbbbcccc|ccdddddd|aaaaaabb|bbbbcccc|
                let mm_in = _mm_shuffle_epi8(mm_in, mm_shuf); /* ssse3 */
                // 0000gggg|gg000000|eeeeee00|00000000|0000cccc|cc000000|aaaaaa00|00000000|
                let mm_t0 = _mm_and_si128(mm_in, _mm_set1_epi32(0x0fc0_fc00));
                // 00000000|00gggggg|00000000|00eeeeee|00000000|00cccccc|00000000|00aaaaaa|
                let mm_t1 = _mm_mulhi_epu16(mm_t0, _mm_set1_epi32(0x0400_0040));
                // 00000000|00hhhhhh|000000ff|ffff0000|00000000|00dddddd|000000bb|bbbb0000|
                let mm_t2 = _mm_and_si128(mm_in, _mm_set1_epi32(0x003f_03f0));
                // 00hhhhhh|00000000|00ffffff|00000000|00dddddd|00000000|00bbbbbb|00000000|
                let mm_t3 = _mm_mullo_epi16(mm_t2, _mm_set1_epi32(0x0100_0010));
                //
                // 00hhhhhh|00gggggg|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa|
                let mm_out = _mm_or_si128(mm_t1, mm_t3);
                //
                _mm_storeu_si128(cc16.as_mut_ptr() as *mut __m128i, mm_out);
            }
            //
            let c16 = unsafe { std::slice::from_raw_parts_mut(oup_ptr, 16) };
            oup_ptr = unsafe { oup_ptr.add(16) };
            inp_ptr = unsafe { inp_ptr.add(4 * 3) };
            //_mm_prefetch(inp_ptr.cast::<i8>(), _MM_HINT_T0);
            //
            // from binary to ascii
            let cc16_slice =
                unsafe { std::slice::from_raw_parts_mut(cc16.as_mut_ptr() as *mut u8, 16) };
            ags.binary_to_ascii_64_ssse3(&mut cc16)?;
            c16.copy_from_slice(cc16_slice);
        }
        oup_idx = unsafe { oup_ptr.offset_from(oup.as_ptr()) as usize };
        let new_inp_len = unsafe { end_ptr.offset_from(inp_ptr) as usize };
        let remaind = unsafe { std::slice::from_raw_parts(inp_ptr, new_inp_len) };
        remaind
    };
    //
    let oo_idx = _encode_base64_scalar_chunks6(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[inline(never)]
#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _decode_base64_ssse3(
    ags: &AsciiGraphicSet,
    a: &str,
) -> Result<Vec<u8>, DecodeError> {
    let inp = a.as_bytes();
    let oup_sz = (inp.len() / 4) * 3 + 2;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = unsafe { _decode_base64_ssse3_chunks16(ags, inp, &mut oup[0..])? };
    oup.resize(oup_idx, 0u8);
    Ok(oup)
}

#[inline(always)]
pub(crate) unsafe fn _decode_base64_ssse3_chunks16(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, DecodeError> {
    //panic!("PASS: SSE2");
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    let mut oup_idx = 0;
    let inp = if inp.len() < 16 {
        inp
    } else {
        let inp_len = inp.len();
        let mut inp_ptr = inp.as_ptr();
        let mut oup_ptr = oup.as_mut_ptr();
        let end_ptr = unsafe { inp_ptr.add(inp_len) };
        let end_ptr_limit = unsafe { end_ptr.sub(16 - 1) };
        //
        let mm_shuf: __m128i =
            unsafe { _mm_setr_epi8(2, 1, 0, 6, 5, 4, 10, 9, 8, 14, 13, 12, -1, -1, -1, -1) };
        let mut cc16 = [0u64; 2];
        //
        while inp_ptr < end_ptr_limit {
            // from ascii to binary
            let a16 = unsafe { std::slice::from_raw_parts(inp_ptr as *const u64, 2) };
            cc16.copy_from_slice(a16);
            ags.ascii_to_binary_64_ssse3(&mut cc16)?;
            unsafe {
                //
                // 00hhhhhh|00gggggg|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa|
                let mm_in = _mm_loadu_si128(cc16.as_ptr() as *const __m128i);
                //
                // 0000gggg|gghhhhhh|0000eeee|eeffffff|0000cccc|ccdddddd|0000aaaa|aabbbbbb|
                let mm_t0 = _mm_maddubs_epi16(mm_in, _mm_set1_epi32(0x0140_0140)); /* ssse3 */
                // 00000000|eeeeeeff|ffffgggg|gghhhhhh|00000000|aaaaaabb|bbbbcccc|ccdddddd|
                let mm_t1 = _mm_madd_epi16(mm_t0, _mm_set1_epi32(0x0001_1000));
                //
                // gghhhhhh|ffffgggg|eeeeeeff|ccdddddd|bbbbcccc|aaaaaabb|
                let mm_out = _mm_shuffle_epi8(mm_t1, mm_shuf); /* ssse3 */
                //
                _mm_storeu_si128(oup_ptr as *mut __m128i, mm_out);
            }
            //
            oup_ptr = unsafe { oup_ptr.add(12) };
            inp_ptr = unsafe { inp_ptr.add(16) };
            unsafe { _mm_prefetch(inp_ptr.cast::<i8>(), _MM_HINT_T0) };
        }
        oup_idx = unsafe { oup_ptr.offset_from(oup.as_ptr()) as usize };
        let new_inp_len = unsafe { end_ptr.offset_from(inp_ptr) as usize };
        let remaind = unsafe { std::slice::from_raw_parts(inp_ptr, new_inp_len) };
        remaind
    };
    //
    let oo_idx = _decode_base64_scalar_chunks4(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
