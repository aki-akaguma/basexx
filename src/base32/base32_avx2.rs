use super::super::*;

#[inline(never)]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn _encode_base32_avx2(
    ags: &AsciiGraphicSet,
    inp: &[u8],
) -> Result<String, EncodeError> {
    let oup_sz = 1 + ((inp.len() + 4) / 5) * 8;
    // encode binary
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = unsafe { _encode_base32_avx2_chunks20(ags, inp, &mut oup[0..])? };
    oup.resize(oup_idx, 0u8);
    let string = unsafe { String::from_utf8_unchecked(oup) };
    assert!(string.len() == oup_idx);
    Ok(string)
}

#[inline(always)]
pub(crate) unsafe fn _encode_base32_avx2_chunks20(
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
        let mm_shuf: __m256i = unsafe { _mm256_set_epi8(
            8, 9, 7, 8, 6, 7, 5, 6, 3, 4, 2, 3, 1, 2, 0, 1,
            8, 9, 7, 8, 6, 7, 5, 6, 3, 4, 2, 3, 1, 2, 0, 1,
        ) };
        let mut cc32 = [0u64; 4];
        //
        while inp_ptr < end_ptr_limit {
            // 01234567|01234567|01234567|01234567|01234567
            // aaaaabbb|bbcccccd|ddddeeee|efffffgg|ggghhhhh
            let inp_slice = unsafe { std::slice::from_raw_parts_mut(inp_ptr as *mut u8, 20) };
            let mut in20base = [0u8; 20];
            in20base.copy_from_slice(inp_slice);
            let mut in32 = [0u8; 32];
            in32[0..10].copy_from_slice(&in20base[0..10]);
            in32[16..26].copy_from_slice(&in20base[10..20]);
            unsafe {
                //
                // ggghhhhh|efffffgg|ddddeeee|bbcccccd|aaaaabbb
                let mm_in = _mm256_loadu_si256(in32.as_ptr() as *const __m256i);
                //
                // efffffgg|ggghhhhh|ddddeeee|efffffgg|bbcccccd|ddddeeee|aaaaabbb|bbcccccd|
                let mm_in = _mm256_shuffle_epi8(mm_in, mm_shuf);
                // 000000gg|ggg00000|0000eeee|e0000000|00ccccc0|00000000|aaaaa000|00000000|
                let mm_t0 = _mm256_and_si256(mm_in, _mm256_set1_epi64x(0x03e0_0f80_3e00_f800));
                // 00000000|000ggggg|00000000|000eeeee|00000000|000ccccc|00000000|000aaaaa|
                let mm_t1 = _mm256_mulhi_epu16(mm_t0, _mm256_set1_epi64x(0x0800_0200_0080_0020));
                // 00000000|000hhhhh|00000000|0fffff00|0000000d|dddd0000|00000bbb|bb000000|
                let mm_t2 = _mm256_and_si256(mm_in, _mm256_set1_epi64x(0x001f_007c_01f0_07c0));
                // 000hhhhh|00000000|000fffff|00000000|000ddddd|00000000|000bbbbb|00000000|
                let mm_t3 = _mm256_mullo_epi16(mm_t2, _mm256_set1_epi64x(0x0100_0040_0010_0004));
                //
                // 000hhhhh|000ggggg|000fffff|000eeeee|000ddddd|000ccccc|000bbbbb|000aaaaa|
                let mm_out = _mm256_or_si256(mm_t1, mm_t3);
                //
                _mm256_storeu_si256(cc32.as_mut_ptr() as *mut __m256i, mm_out);
            }
            //
            let c32 = unsafe { std::slice::from_raw_parts_mut(oup_ptr, 32) };
            oup_ptr = unsafe { oup_ptr.add(32) };
            inp_ptr = unsafe { inp_ptr.add(5 * 2 * 2) };
            //_mm_prefetch(inp_ptr.cast::<i8>(), _MM_HINT_T0);
            //
            // from binary to ascii
            let cc32_slice =
                unsafe { std::slice::from_raw_parts_mut(cc32.as_mut_ptr() as *mut u8, 32) };
            ags.binary_to_ascii_32_avx2(&mut cc32)?;
            c32.copy_from_slice(cc32_slice);
        }
        oup_idx = unsafe { oup_ptr.offset_from(oup.as_ptr()) as usize };
        let new_inp_len = unsafe { end_ptr.offset_from(inp_ptr) as usize };
        let remaind = unsafe { std::slice::from_raw_parts(inp_ptr, new_inp_len) };
        remaind
    };
    //
    let oo_idx = _encode_base32_scalar_chunks5(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[inline(never)]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn _decode_base32_avx2(
    ags: &AsciiGraphicSet,
    a: &str,
) -> Result<Vec<u8>, DecodeError> {
    let inp = a.as_bytes();
    let oup_sz = (inp.len() / 8) * 5 + 4;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = unsafe { _decode_base32_avx2_chunks32(ags, inp, &mut oup[0..])? };
    oup.resize(oup_idx, 0u8);
    Ok(oup)
}

#[inline(always)]
pub(crate) unsafe fn _decode_base32_avx2_chunks32(
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
        let mm_shuf: __m256i = unsafe { _mm256_set_epi8(
            -1, -1, -1, -1, -1, -1, 11, 12, 13, 14, 15, 3, 4, 5, 6, 7,
            -1, -1, -1, -1, -1, -1, 11, 12, 13, 14, 15, 3, 4, 5, 6, 7,
        ) };
        let mut cc32 = [0u64; 4];
        //
        while inp_ptr < end_ptr_limit {
            // from ascii to binary
            let a32 = unsafe { std::slice::from_raw_parts(inp_ptr as *const u64, 4) };
            cc32.copy_from_slice(a32);
            ags.ascii_to_binary_32_avx2(&mut cc32)?;
            let mut out32 = [0u8; 32];
            unsafe {
                //
                // 000hhhhh|000ggggg|000fffff|000eeeee|000ddddd|000ccccc|000bbbbb|000aaaaa|
                let mm_in = _mm256_loadu_si256(cc32.as_ptr() as *const __m256i);
                //
                // 000000gg|ggghhhhh|000000ee|eeefffff|000000cc|cccddddd|000000aa|aaabbbbb|
                let mm_t0 = _mm256_maddubs_epi16(mm_in, _mm256_set1_epi32(0x0120_0120)); /* avx2 */
                // 00000000|eeeeefff|ffgggggh|hhhh0000|00000000|aaaaabbb|bbcccccd|dddd0000|
                let mm_t1 = _mm256_madd_epi16(mm_t0, _mm256_set1_epi32(0x0010_4000));
                // aaaaabbb|bbcccccd|dddd0000|
                let mm_t2 = _mm256_slli_epi64(mm_t1, 40);
                // 00000000|00000000|0000eeee|efffffgg|ggghhhhh|00000000|0000aaaa|abbbbbcc|
                let mm_t3 = _mm256_srli_epi64(mm_t1, 12);
                // aaaaabbb|bbcccccd|ddddeeee|efffffgg|ggghhhhh|00000000|0000aaaa|abbbbbcc|
                let mm_t4 = _mm256_or_si256(mm_t2, mm_t3);
                //
                // 00000000|00000000|00000000|ggghhhhh|efffffgg|ddddeeee|bbcccccd|aaaaabbb|
                let mm_out = _mm256_shuffle_epi8(mm_t4, mm_shuf);
                //
                _mm256_storeu_si256(out32.as_mut_ptr() as *mut __m256i, mm_out);
            }
            unsafe {
                use std::slice::from_raw_parts_mut;
                from_raw_parts_mut(oup_ptr, 5).copy_from_slice(&out32[0..5]);
                from_raw_parts_mut(oup_ptr.add(5), 5).copy_from_slice(&out32[5..10]);
                from_raw_parts_mut(oup_ptr.add(5 * 2), 5).copy_from_slice(&out32[16..21]);
                from_raw_parts_mut(oup_ptr.add(5 * 3), 5).copy_from_slice(&out32[21..26]);
            }
            //
            oup_ptr = unsafe { oup_ptr.add(10 * 2) };
            inp_ptr = unsafe { inp_ptr.add(32) };
            //_mm_prefetch(inp_ptr.cast::<i8>(), _MM_HINT_T0);
        }
        oup_idx = unsafe { oup_ptr.offset_from(oup.as_ptr()) as usize };
        let new_inp_len = unsafe { end_ptr.offset_from(inp_ptr) as usize };
        let remaind = unsafe { std::slice::from_raw_parts(inp_ptr, new_inp_len) };
        remaind
    };
    //
    let oo_idx = _decode_base32_scalar_chunks8(ags, inp, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
