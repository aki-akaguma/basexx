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
    let oup_sz = 1 + inp.len().div_ceil(3) * 4;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = unsafe { _encode_base64_avx2_chunks24(ags, inp, &mut oup[0..])? };
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
    #[rustfmt::skip]
    let mm_shuf: __m256i = unsafe { _mm256_set_epi8(
        10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1,
        10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1,
    ) };

    let mut i_iter = inp.chunks_exact(24);
    let mut o_iter = oup.chunks_exact_mut(32);
    let mut oup_idx = 0;

    for (i_chunk, o_chunk) in i_iter.by_ref().zip(o_iter.by_ref()) {
        let mut in32base = [0u32; 8];
        bytemuck::bytes_of_mut(&mut in32base)[0..24].copy_from_slice(i_chunk);

        let mut in32 = [0u32; 8];
        in32[0..3].copy_from_slice(&in32base[0..3]);
        in32[4..7].copy_from_slice(&in32base[3..6]);

        let mut cc32 = [0u64; 4];
        unsafe {
            let mm_in = _mm256_loadu_si256(in32.as_ptr() as *const __m256i);
            let mm_in = _mm256_shuffle_epi8(mm_in, mm_shuf);
            let mm_t0 = _mm256_and_si256(mm_in, _mm256_set1_epi32(0x0fc0_fc00));
            let mm_t1 = _mm256_mulhi_epu16(mm_t0, _mm256_set1_epi32(0x0400_0040));
            let mm_t2 = _mm256_and_si256(mm_in, _mm256_set1_epi32(0x003f_03f0));
            let mm_t3 = _mm256_mullo_epi16(mm_t2, _mm256_set1_epi32(0x0100_0010));
            let mm_out = _mm256_or_si256(mm_t1, mm_t3);
            _mm256_storeu_si256(cc32.as_mut_ptr() as *mut __m256i, mm_out);
        }

        ags.binary_to_ascii_64_avx2(&mut cc32)?;
        o_chunk.copy_from_slice(bytemuck::bytes_of(&cc32));
        oup_idx += 32;
    }

    let remaind = i_iter.remainder();
    let oo_idx = _encode_base64_scalar_chunks6(ags, remaind, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[inline(never)]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn _decode_base64_avx2(
    ags: &AsciiGraphicSet,
    inp: &str,
) -> Result<Vec<u8>, DecodeError> {
    let inp = inp.as_bytes();
    let oup_sz = (inp.len() / 4) * 3 + 2;
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = unsafe { _decode_base64_avx2_chunks32(ags, inp, &mut oup[0..])? };
    oup.resize(oup_idx, 0u8);
    Ok(oup)
}

#[inline(always)]
pub(crate) unsafe fn _decode_base64_avx2_chunks32(
    ags: &AsciiGraphicSet,
    inp: &[u8],
    oup: &mut [u8],
) -> Result<usize, DecodeError> {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    #[rustfmt::skip]
    let mm_shuf: __m256i = unsafe { _mm256_setr_epi8(
        2, 1, 0, 6, 5, 4, 10, 9, 8, 14, 13, 12, -1, -1, -1, -1,
        2, 1, 0, 6, 5, 4, 10, 9, 8, 14, 13, 12, -1, -1, -1, -1,
    ) };

    let mut i_iter = inp.chunks_exact(32);
    let mut o_iter = oup.chunks_exact_mut(24);
    let mut oup_idx = 0;
    let mut inp_idx = 0;

    for (i_chunk, o_chunk) in i_iter.by_ref().zip(o_iter.by_ref()) {
        let mut cc32: [u64; 4] = bytemuck::pod_read_unaligned(i_chunk);
        ags.ascii_to_binary_64_avx2(&mut cc32)?;

        let mut out32 = [0u32; 8];
        unsafe {
            let mm_in = _mm256_loadu_si256(cc32.as_ptr() as *const __m256i);
            let mm_t0 = _mm256_maddubs_epi16(mm_in, _mm256_set1_epi32(0x0140_0140));
            let mm_t1 = _mm256_madd_epi16(mm_t0, _mm256_set1_epi32(0x0001_1000));
            let mm_out = _mm256_shuffle_epi8(mm_t1, mm_shuf);
            _mm256_storeu_si256(out32.as_mut_ptr() as *mut __m256i, mm_out);
        }

        let out_bytes = bytemuck::cast_slice::<u32, u8>(&out32);
        o_chunk[0..12].copy_from_slice(&out_bytes[0..12]);
        o_chunk[12..24].copy_from_slice(&out_bytes[16..28]);

        oup_idx += 24;
        inp_idx += 32;
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
