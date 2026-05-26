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
    // Safety: The encoding process strictly uses ASCII characters from AsciiGraphicSet.
    debug_assert!(std::str::from_utf8(&oup).is_ok());
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
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    let mm_shuf: __m128i =
        unsafe { _mm_set_epi8(10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1) };

    let mut i_iter = inp.chunks_exact(12);
    let mut o_iter = oup.chunks_exact_mut(16);
    let mut oup_idx = 0;

    for (i_chunk, o_chunk) in i_iter.by_ref().zip(o_iter.by_ref()) {
        let mut in16base = [0u64; 2];
        bytemuck::bytes_of_mut(&mut in16base)[0..12].copy_from_slice(i_chunk);

        let mut cc16 = [0u64; 2];
        unsafe {
            let mm_in = _mm_loadu_si128(in16base.as_ptr() as *const __m128i);
            let mm_in = _mm_shuffle_epi8(mm_in, mm_shuf);
            let mm_t0 = _mm_and_si128(mm_in, _mm_set1_epi32(0x0fc0_fc00));
            let mm_t1 = _mm_mulhi_epu16(mm_t0, _mm_set1_epi32(0x0400_0040));
            let mm_t2 = _mm_and_si128(mm_in, _mm_set1_epi32(0x003f_03f0));
            let mm_t3 = _mm_mullo_epi16(mm_t2, _mm_set1_epi32(0x0100_0010));
            let mm_out = _mm_or_si128(mm_t1, mm_t3);
            _mm_storeu_si128(cc16.as_mut_ptr() as *mut __m128i, mm_out);
        }

        ags.binary_to_ascii_64_ssse3(&mut cc16)?;
        o_chunk.copy_from_slice(bytemuck::bytes_of(&cc16));
        oup_idx += 16;
    }

    let remaind = i_iter.remainder();
    let oo_idx = _encode_base64_scalar_chunks6(ags, remaind, &mut oup[oup_idx..])?;
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
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    let mm_shuf: __m128i =
        unsafe { _mm_setr_epi8(2, 1, 0, 6, 5, 4, 10, 9, 8, 14, 13, 12, -1, -1, -1, -1) };

    let mut i_iter = inp.chunks_exact(16);
    let mut o_iter = oup.chunks_exact_mut(12);
    let mut oup_idx = 0;
    let mut inp_idx = 0;

    for (i_chunk, o_chunk) in i_iter.by_ref().zip(o_iter.by_ref()) {
        let mut cc16: [u64; 2] = bytemuck::pod_read_unaligned(i_chunk);
        ags.ascii_to_binary_64_ssse3(&mut cc16)?;

        let mut out16 = [0u64; 2];
        unsafe {
            let mm_in = _mm_loadu_si128(cc16.as_ptr() as *const __m128i);
            let mm_t0 = _mm_maddubs_epi16(mm_in, _mm_set1_epi32(0x0140_0140));
            let mm_t1 = _mm_madd_epi16(mm_t0, _mm_set1_epi32(0x0001_1000));
            let mm_out = _mm_shuffle_epi8(mm_t1, mm_shuf);
            _mm_storeu_si128(out16.as_mut_ptr() as *mut __m128i, mm_out);
        }

        o_chunk.copy_from_slice(&bytemuck::bytes_of(&out16)[0..12]);

        oup_idx += 12;
        inp_idx += 16;
    }

    let remaind = &inp[inp_idx..];
    let oo_idx = _decode_base64_scalar_chunks4(ags, remaind, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
