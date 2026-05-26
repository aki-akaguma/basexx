use super::super::*;

#[inline(never)]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn _encode_base32_avx2(
    ags: &AsciiGraphicSet,
    inp: &[u8],
) -> Result<String, EncodeError> {
    let oup_sz = 1 + inp.len().div_ceil(5) * 8;
    // encode binary
    let mut oup = vec![0u8; oup_sz];
    let oup_idx = unsafe { _encode_base32_avx2_chunks20(ags, inp, &mut oup[0..])? };
    oup.resize(oup_idx, 0u8);
    // Safety: The encoding process strictly uses ASCII characters from AsciiGraphicSet.
    debug_assert!(std::str::from_utf8(&oup).is_ok());
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
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    #[rustfmt::skip]
    let mm_shuf: __m256i = unsafe { _mm256_set_epi8(
        8, 9, 7, 8, 6, 7, 5, 6, 3, 4, 2, 3, 1, 2, 0, 1,
        8, 9, 7, 8, 6, 7, 5, 6, 3, 4, 2, 3, 1, 2, 0, 1,
    ) };

    let mut i_iter = inp.chunks_exact(20);
    let mut o_iter = oup.chunks_exact_mut(32);
    let mut oup_idx = 0;

    for (i_chunk, o_chunk) in i_iter.by_ref().zip(o_iter.by_ref()) {
        let mut in32 = [0u8; 32];
        in32[0..10].copy_from_slice(&i_chunk[0..10]);
        in32[16..26].copy_from_slice(&i_chunk[10..20]);

        let mut cc32 = [0u64; 4];
        unsafe {
            let mm_in = _mm256_loadu_si256(in32.as_ptr() as *const __m256i);
            let mm_in = _mm256_shuffle_epi8(mm_in, mm_shuf);
            let mm_t0 = _mm256_and_si256(mm_in, _mm256_set1_epi64x(0x03e0_0f80_3e00_f800));
            let mm_t1 = _mm256_mulhi_epu16(mm_t0, _mm256_set1_epi64x(0x0800_0200_0080_0020));
            let mm_t2 = _mm256_and_si256(mm_in, _mm256_set1_epi64x(0x001f_007c_01f0_07c0));
            let mm_t3 = _mm256_mullo_epi16(mm_t2, _mm256_set1_epi64x(0x0100_0040_0010_0004));
            let mm_out = _mm256_or_si256(mm_t1, mm_t3);
            _mm256_storeu_si256(cc32.as_mut_ptr() as *mut __m256i, mm_out);
        }

        ags.binary_to_ascii_32_avx2(&mut cc32)?;
        o_chunk.copy_from_slice(bytemuck::bytes_of(&cc32));
        oup_idx += 32;
    }

    let remaind = i_iter.remainder();
    let oo_idx = _encode_base32_scalar_chunks5(ags, remaind, &mut oup[oup_idx..])?;
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
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    #[rustfmt::skip]
    let mm_shuf: __m256i = unsafe { _mm256_set_epi8(
        -1, -1, -1, -1, -1, -1, 11, 12, 13, 14, 15, 3, 4, 5, 6, 7,
        -1, -1, -1, -1, -1, -1, 11, 12, 13, 14, 15, 3, 4, 5, 6, 7,
    ) };

    let mut i_iter = inp.chunks_exact(32);
    let mut o_iter = oup.chunks_exact_mut(20);
    let mut oup_idx = 0;
    let mut inp_idx = 0;

    for (i_chunk, o_chunk) in i_iter.by_ref().zip(o_iter.by_ref()) {
        let mut cc32: [u64; 4] = bytemuck::pod_read_unaligned(i_chunk);
        ags.ascii_to_binary_32_avx2(&mut cc32)?;

        let mut out32 = [0u8; 32];
        unsafe {
            let mm_in = _mm256_loadu_si256(cc32.as_ptr() as *const __m256i);
            let mm_t0 = _mm256_maddubs_epi16(mm_in, _mm256_set1_epi32(0x0120_0120));
            let mm_t1 = _mm256_madd_epi16(mm_t0, _mm256_set1_epi32(0x0010_4000));
            let mm_t2 = _mm256_slli_epi64(mm_t1, 40);
            let mm_t3 = _mm256_srli_epi64(mm_t1, 12);
            let mm_t4 = _mm256_or_si256(mm_t2, mm_t3);
            let mm_out = _mm256_shuffle_epi8(mm_t4, mm_shuf);
            _mm256_storeu_si256(out32.as_mut_ptr() as *mut __m256i, mm_out);
        }

        o_chunk[0..10].copy_from_slice(&out32[0..10]);
        o_chunk[10..20].copy_from_slice(&out32[16..26]);

        oup_idx += 20;
        inp_idx += 32;
    }

    let remaind = &inp[inp_idx..];
    let oo_idx = _decode_base32_scalar_chunks8(ags, remaind, &mut oup[oup_idx..])?;
    Ok(oup_idx + oo_idx)
}

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
