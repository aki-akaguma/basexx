use super::super::*;
//
// http://0x80.pl/notesen/2016-01-12-sse-base64-encoding.html
//
#[inline(never)]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn _encode_base64_avx2(
    ags: &AsciiGraphicSet,
    inp: &[u8],
) -> Result<String, EncodeError> {
    let oup_sz = 1 + ((inp.len() + 2) / 3) * 4;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = _encode_base64_avx2_chunks24(ags, inp, &mut oup[0..])?;
    oup.resize(oup_idx, 0u8);
    let string = unsafe { String::from_utf8_unchecked(oup) };
    assert!(string.len() == oup_idx);
    Ok(string)
}

#[inline(always)]
pub(crate) unsafe fn _encode_base64_avx2_chunks24(
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
    let inp = if inp.len() < 32 {
        inp
    } else {
        let inp_len = inp.len();
        let mut inp_ptr = inp.as_ptr();
        let mut oup_ptr = oup.as_mut_ptr();
        let end_ptr = unsafe { inp_ptr.add(inp_len) };
        let end_ptr_limit = unsafe { end_ptr.sub(32 - 1) };
        //
        #[rustfmt::skip]
        let mm_shuf: __m256i = _mm256_set_epi8(
            10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1,
            10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1,
        );
        let mut cc32 = [0u64; 4];
        //
        while inp_ptr < end_ptr_limit {
            // 01234567|01234567|01234567|01234567|01234567|01234567|
            // aaaaaabb|bbbbcccc|ccdddddd|eeeeeeff|ffffgggg|gghhhhhh|
            let inp_slice = unsafe { std::slice::from_raw_parts_mut(inp_ptr as *mut u32, 8) };
            let mut in32base = [0u32; 8];
            in32base.copy_from_slice(inp_slice);
            let mut in32 = [0u32; 8];
            in32[0..3].copy_from_slice(&in32base[0..3]);
            in32[4..7].copy_from_slice(&in32base[3..6]);
            //
            // gghhhhhh|ffffgggg|eeeeeeff|ccdddddd|bbbbcccc|aaaaaabb|
            let mm_in = _mm256_loadu_si256(in32.as_ptr() as *const __m256i);
            //
            // ffffgggg|gghhhhhh|eeeeeeff|ffffgggg|bbbbcccc|ccdddddd|aaaaaabb|bbbbcccc|
            let mm_in = _mm256_shuffle_epi8(mm_in, mm_shuf);
            // 0000gggg|gg000000|eeeeee00|00000000|0000cccc|cc000000|aaaaaa00|00000000|
            let mm_t0 = _mm256_and_si256(mm_in, _mm256_set1_epi32(0x0fc0_fc00));
            // 00000000|00gggggg|00000000|00eeeeee|00000000|00cccccc|00000000|00aaaaaa|
            let mm_t1 = _mm256_mulhi_epu16(mm_t0, _mm256_set1_epi32(0x0400_0040));
            // 00000000|00hhhhhh|000000ff|ffff0000|00000000|00dddddd|000000bb|bbbb0000|
            let mm_t2 = _mm256_and_si256(mm_in, _mm256_set1_epi32(0x003f_03f0));
            // 00hhhhhh|00000000|00ffffff|00000000|00dddddd|00000000|00bbbbbb|00000000|
            let mm_t3 = _mm256_mullo_epi16(mm_t2, _mm256_set1_epi32(0x0100_0010));
            //
            // 00hhhhhh|00gggggg|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa|
            let mm_out = _mm256_or_si256(mm_t1, mm_t3);
            //
            _mm256_storeu_si256(cc32.as_mut_ptr() as *mut __m256i, mm_out);
            //
            let c32 = unsafe { std::slice::from_raw_parts_mut(oup_ptr, 32) };
            oup_ptr = unsafe { oup_ptr.add(32) };
            inp_ptr = unsafe { inp_ptr.add(4 * 3 * 2) };
            //_mm_prefetch(inp_ptr.cast::<i8>(), _MM_HINT_T0);
            //
            // from binary to ascii
            let cc32_slice =
                unsafe { std::slice::from_raw_parts_mut(cc32.as_mut_ptr() as *mut u8, 32) };
            ags.binary_to_ascii_64_avx2(&mut cc32)?;
            c32.copy_from_slice(cc32_slice);
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
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn _decode_base64_avx2(
    ags: &AsciiGraphicSet,
    a: &str,
) -> Result<Vec<u8>, DecodeError> {
    let inp = a.as_bytes();
    let oup_sz = (inp.len() / 4) * 3 + 2;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = _decode_base64_avx2_chunks32(ags, inp, &mut oup[0..])?;
    oup.resize(oup_idx, 0u8);
    Ok(oup)
}

#[inline(always)]
pub(crate) unsafe fn _decode_base64_avx2_chunks32(
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
    let inp = if inp.len() < 32 {
        inp
    } else {
        let inp_len = inp.len();
        let mut inp_ptr = inp.as_ptr();
        let mut oup_ptr = oup.as_mut_ptr();
        let end_ptr = unsafe { inp_ptr.add(inp_len) };
        let end_ptr_limit = unsafe { end_ptr.sub(32 - 1) };
        //
        #[rustfmt::skip]
        let mm_shuf: __m256i = _mm256_setr_epi8(
            2, 1, 0, 6, 5, 4, 10, 9, 8, 14, 13, 12, -1, -1, -1, -1,
            2, 1, 0, 6, 5, 4, 10, 9, 8, 14, 13, 12, -1, -1, -1, -1,
        );
        let mut cc32 = [0u64; 4];
        //
        while inp_ptr < end_ptr_limit {
            // from ascii to binary
            let a32 = unsafe { std::slice::from_raw_parts(inp_ptr as *const u64, 4) };
            cc32.copy_from_slice(a32);
            ags.ascii_to_binary_64_avx2(&mut cc32)?;
            //
            // 00hhhhhh|00gggggg|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa|
            let mm_in = _mm256_loadu_si256(cc32.as_ptr() as *const __m256i);
            //
            // 0000gggg|gghhhhhh|0000eeee|eeffffff|0000cccc|ccdddddd|0000aaaa|aabbbbbb|
            let mm_t0 = _mm256_maddubs_epi16(mm_in, _mm256_set1_epi32(0x0140_0140));
            // 00000000|eeeeeeff|ffffgggg|gghhhhhh|00000000|aaaaaabb|bbbbcccc|ccdddddd|
            let mm_t1 = _mm256_madd_epi16(mm_t0, _mm256_set1_epi32(0x0001_1000));
            //
            // gghhhhhh|ffffgggg|eeeeeeff|ccdddddd|bbbbcccc|aaaaaabb|
            let mm_out = _mm256_shuffle_epi8(mm_t1, mm_shuf);
            //
            //_mm256_storeu_si256(oup_ptr as *mut __m256i, mm_out);
            //
            let mut out32 = [0u32; 8];
            _mm256_storeu_si256(out32.as_mut_ptr() as *mut __m256i, mm_out);
            *(oup_ptr as *mut u32).add(0) = out32[0];
            *(oup_ptr as *mut u32).add(1) = out32[1];
            *(oup_ptr as *mut u32).add(2) = out32[2];
            *(oup_ptr as *mut u32).add(3) = out32[4];
            *(oup_ptr as *mut u32).add(4) = out32[5];
            *(oup_ptr as *mut u32).add(5) = out32[6];
            //
            oup_ptr = unsafe { oup_ptr.add(12 * 2) };
            inp_ptr = unsafe { inp_ptr.add(32) };
            //_mm_prefetch(inp_ptr.cast::<i8>(), _MM_HINT_T0);
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
