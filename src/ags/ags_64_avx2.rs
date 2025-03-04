use super::super::*;

#[inline(never)]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn _binary_to_ascii_64_avx2(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), EncodeError> {
    assert_eq!(lup.len(), 64);
    unsafe { _binary_to_ascii_64_avx2_chunks32(lup, buf)? };
    Ok(())
}

#[inline(always)]
pub(crate) unsafe fn _binary_to_ascii_64_avx2_chunks32(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), EncodeError> {
    let buf = if buf.len() < 32 {
        buf
    } else {
        let buf_len = buf.len();
        let mut buf_ptr = buf.as_mut_ptr();
        let end_ptr = unsafe { buf_ptr.add(buf_len) };
        let end_ptr_limit = unsafe { end_ptr.sub(32 - 1) };
        //
        while buf_ptr < end_ptr_limit {
            let mut buf2 = [0u64; 4];
            unsafe {
                use std::slice::from_raw_parts;
                buf2.copy_from_slice(from_raw_parts(buf_ptr as *const u64, 4));
                _binary_to_ascii_64_avx2_c32_chunks32(lup, &mut buf2)?;
            }
            unsafe {
                *(buf_ptr as *mut u64) = buf2[0];
                *((buf_ptr as *mut u64).add(1)) = buf2[1];
                *((buf_ptr as *mut u64).add(2)) = buf2[2];
                *((buf_ptr as *mut u64).add(3)) = buf2[3];
            }
            //
            buf_ptr = unsafe { buf_ptr.add(32) };
        }
        let new_buf_len = unsafe { end_ptr.offset_from(buf_ptr) as usize };
        let remaind = unsafe { std::slice::from_raw_parts_mut(buf_ptr, new_buf_len) };
        remaind
    };
    _binary_to_ascii_scalar(lup, buf)?;
    //
    Ok(())
}

#[cfg(target_feature = "sse2")]
#[inline(always)]
//#[target_feature(enable = "avx2")]
pub(crate) unsafe fn _binary_to_ascii_64_avx2_c32(
    lup: &[u8],
    buf: &mut [u64; 4],
) -> Result<(), EncodeError> {
    assert_eq!(lup.len(), 64);
    unsafe { _binary_to_ascii_64_avx2_c32_chunks32(lup, buf)? };
    Ok(())
}

macro_rules! enc_check_error {
    ($err_buf: expr, $buf_ptr: expr) => {
        if $err_buf[0] != 0 || $err_buf[1] != 0 || $err_buf[2] != 0 || $err_buf[3] != 0 {
            // on error
            for i in 0..4 {
                let cc = $err_buf[i];
                for j in 0..8 {
                    if (cc >> (8 * j)) & 0xFF != 0 {
                        return Err(EncodeError::InvalidIndex(*($buf_ptr.add(j + 8 * i))));
                    }
                }
            }
        }
    };
}

#[inline(always)]
pub(crate) unsafe fn _binary_to_ascii_64_avx2_c32_chunks32(
    lup: &[u8],
    buf: &mut [u64; 4],
) -> Result<(), EncodeError> {
    //panic!("PASS: SSE2");
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    unsafe {
        let buf_ptr = buf.as_mut_ptr() as *mut u8;
        //
        let mm_map01 = _mm_loadu_si128(lup.as_ptr() as *const __m128i);
        let mm_map02 = _mm_loadu_si128(lup.as_ptr().add(16) as *const __m128i);
        let mm_map03 = _mm_loadu_si128(lup.as_ptr().add(16 * 2) as *const __m128i);
        let mm_map04 = _mm_loadu_si128(lup.as_ptr().add(16 * 3) as *const __m128i);
        let mm_map1 = _mm256_broadcastsi128_si256(mm_map01);
        let mm_map2 = _mm256_broadcastsi128_si256(mm_map02);
        let mm_map3 = _mm256_broadcastsi128_si256(mm_map03);
        let mm_map4 = _mm256_broadcastsi128_si256(mm_map04);
        let err_buf = [0u64; 4];
        //
        {
            // ..|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa
            let mm_in = _mm256_loadu_si256(buf_ptr as *const __m256i);
            //
            // check error
            let mm_err = _mm256_or_si256(
                _mm256_cmpgt_epi8(_mm256_set1_epi8(0), mm_in),
                _mm256_cmpgt_epi8(mm_in, _mm256_set1_epi8(64 - 1)),
            );
            _mm256_storeu_si256(err_buf.as_ptr() as *mut __m256i, mm_err);
            enc_check_error!(err_buf, buf_ptr);
            //_mm256_prefetch(buf_ptr.cast::<i8>(), _MM_HINT_T0);
            let mm_in_p1 = _mm256_add_epi8(mm_in, _mm256_set1_epi8(1));
            //
            let mm_mask11 = _mm256_cmpgt_epi8(_mm256_set1_epi8(16), mm_in);
            let mm_mask22 = _mm256_cmpgt_epi8(_mm256_set1_epi8(16 * 2), mm_in);
            let mm_mask33 = _mm256_cmpgt_epi8(_mm256_set1_epi8(16 * 3), mm_in);
            let mm_mask44 = _mm256_cmpgt_epi8(_mm256_set1_epi8(16 * 4), mm_in);
            //
            let mm_mask1 = mm_mask11;
            let mm_and1 = _mm256_and_si256(mm_in_p1, mm_mask1);
            let mm_idx1 = _mm256_sub_epi8(mm_and1, _mm256_set1_epi8(1));
            let mm_out1 = _mm256_shuffle_epi8(mm_map1, mm_idx1);
            //
            let mm_mask2 = _mm256_andnot_si256(mm_mask11, mm_mask22);
            let mm_and2 = _mm256_and_si256(mm_in_p1, mm_mask2);
            let mm_idx2 = _mm256_sub_epi8(mm_and2, _mm256_set1_epi8(1));
            let mm_out2 = _mm256_shuffle_epi8(mm_map2, mm_idx2);
            //
            let mm_mask3 = _mm256_andnot_si256(mm_mask22, mm_mask33);
            let mm_and3 = _mm256_and_si256(mm_in_p1, mm_mask3);
            let mm_idx3 = _mm256_sub_epi8(mm_and3, _mm256_set1_epi8(1));
            let mm_out3 = _mm256_shuffle_epi8(mm_map3, mm_idx3);
            //
            let mm_mask4 = _mm256_andnot_si256(mm_mask33, mm_mask44);
            let mm_and4 = _mm256_and_si256(mm_in_p1, mm_mask4);
            let mm_idx4 = _mm256_sub_epi8(mm_and4, _mm256_set1_epi8(1));
            let mm_out4 = _mm256_shuffle_epi8(mm_map4, mm_idx4);
            //
            // make out
            let mm_out = _mm256_or_si256(
                _mm256_or_si256(mm_out1, mm_out2),
                _mm256_or_si256(mm_out3, mm_out4),
            );
            _mm256_storeu_si256(buf_ptr as *mut __m256i, mm_out);
        }
    };
    //
    Ok(())
}

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
