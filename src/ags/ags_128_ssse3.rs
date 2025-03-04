use super::super::*;

#[inline(never)]
#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _ascii_to_binary_128_ssse3(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), DecodeError> {
    assert_eq!(lup.len(), 128);
    unsafe { _ascii_to_binary_128_ssse3_chunks16(lup, buf)? };
    Ok(())
}

#[inline(always)]
pub(crate) unsafe fn _ascii_to_binary_128_ssse3_chunks16(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), DecodeError> {
    let buf = if buf.len() < 16 {
        buf
    } else {
        let buf_len = buf.len();
        let mut buf_ptr = buf.as_mut_ptr();
        let end_ptr = unsafe { buf_ptr.add(buf_len) };
        let end_ptr_limit = unsafe { end_ptr.sub(16 - 1) };
        //
        while buf_ptr < end_ptr_limit {
            let mut buf2 = [0u64; 2];
            unsafe {
                use std::slice::from_raw_parts;
                buf2.copy_from_slice(from_raw_parts(buf_ptr as *const u64, 2));
                _ascii_to_binary_128_ssse3_c16_chunks16(lup, &mut buf2)?;
            }
            unsafe {
                *(buf_ptr as *mut u64) = buf2[0];
                *((buf_ptr as *mut u64).add(1)) = buf2[1];
            }
            //
            buf_ptr = unsafe { buf_ptr.add(16) };
        }
        let new_buf_len = unsafe { end_ptr.offset_from(buf_ptr) as usize };
        let remaind = unsafe { std::slice::from_raw_parts_mut(buf_ptr, new_buf_len) };
        remaind
    };
    _ascii_to_binary_scalar(lup, buf)?;
    //
    Ok(())
}

#[cfg(target_feature = "sse2")]
#[inline(always)]
//#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _ascii_to_binary_128_ssse3_c16(
    lup: &[u8],
    buf: &mut [u64; 2],
) -> Result<(), DecodeError> {
    assert_eq!(lup.len(), 128);
    unsafe { _ascii_to_binary_128_ssse3_c16_chunks16(lup, buf)? };
    Ok(())
}

macro_rules! dec_check_error {
    ($err_buf: expr, $buf_ptr: expr) => {
        if $err_buf[0] != 0 || $err_buf[1] != 0 {
            // on error
            for i in 0..2 {
                let cc = $err_buf[i];
                for j in 0..8 {
                    if (cc >> (8 * j)) & 0xFF != 0 {
                        return Err(DecodeError::InvalidByte(*($buf_ptr.add(j + 8 * i))));
                    }
                }
            }
        }
    };
}

#[inline(always)]
pub(crate) unsafe fn _ascii_to_binary_128_ssse3_c16_chunks16(
    lup: &[u8],
    buf: &mut [u64; 2],
) -> Result<(), DecodeError> {
    //panic!("PASS: SSE2");
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    unsafe {
        let buf_ptr = buf.as_mut_ptr() as *mut u8;
        //
        // ASCII graphic: U+0021 '!' ..= U+007E '~': 33..=126
        // skip range: 0..=31
        let mm_map3 = _mm_loadu_si128(lup.as_ptr().add(16 * 2) as *const __m128i);
        let mm_map4 = _mm_loadu_si128(lup.as_ptr().add(16 * 3) as *const __m128i);
        let mm_map5 = _mm_loadu_si128(lup.as_ptr().add(16 * 4) as *const __m128i);
        let mm_map6 = _mm_loadu_si128(lup.as_ptr().add(16 * 5) as *const __m128i);
        let mm_map7 = _mm_loadu_si128(lup.as_ptr().add(16 * 6) as *const __m128i);
        let mm_map8 = _mm_loadu_si128(lup.as_ptr().add(16 * 7) as *const __m128i);
        let err_buf = [0u64; 2];
        //
        {
            // ..|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa
            let mm_in = _mm_loadu_si128(buf_ptr as *const __m128i);
            //
            // check error
            let mm_err = _mm_or_si128(
                _mm_cmplt_epi8(mm_in, _mm_set1_epi8(33)),
                _mm_cmpeq_epi8(mm_in, _mm_set1_epi8(127)),
            );
            _mm_storeu_si128(err_buf.as_ptr() as *mut __m128i, mm_err);
            dec_check_error!(err_buf, buf_ptr);
            //_mm_prefetch(buf_ptr.cast::<i8>(), _MM_HINT_T0);
            let mm_in_p1 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            //
            let mm_mask33 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 3));
            let mm_mask44 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 4));
            let mm_mask55 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 5));
            let mm_mask66 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 6));
            let mm_mask77 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 7));
            let mm_mask88 = _mm_set1_epi8(-1);
            //
            let mm_mask3 = mm_mask33;
            let mm_and3 = _mm_and_si128(mm_in_p1, mm_mask3);
            let mm_idx3 = _mm_sub_epi8(mm_and3, _mm_set1_epi8(1));
            let mm_out3 = _mm_shuffle_epi8(mm_map3, mm_idx3);
            //
            let mm_mask4 = _mm_andnot_si128(mm_mask33, mm_mask44);
            let mm_and4 = _mm_and_si128(mm_in_p1, mm_mask4);
            let mm_idx4 = _mm_sub_epi8(mm_and4, _mm_set1_epi8(1));
            let mm_out4 = _mm_shuffle_epi8(mm_map4, mm_idx4);
            //
            let mm_mask5 = _mm_andnot_si128(mm_mask44, mm_mask55);
            let mm_and5 = _mm_and_si128(mm_in_p1, mm_mask5);
            let mm_idx5 = _mm_sub_epi8(mm_and5, _mm_set1_epi8(1));
            let mm_out5 = _mm_shuffle_epi8(mm_map5, mm_idx5);
            //
            let mm_mask6 = _mm_andnot_si128(mm_mask55, mm_mask66);
            let mm_and6 = _mm_and_si128(mm_in_p1, mm_mask6);
            let mm_idx6 = _mm_sub_epi8(mm_and6, _mm_set1_epi8(1));
            let mm_out6 = _mm_shuffle_epi8(mm_map6, mm_idx6);
            //
            let mm_mask7 = _mm_andnot_si128(mm_mask66, mm_mask77);
            let mm_and7 = _mm_and_si128(mm_in_p1, mm_mask7);
            let mm_idx7 = _mm_sub_epi8(mm_and7, _mm_set1_epi8(1));
            let mm_out7 = _mm_shuffle_epi8(mm_map7, mm_idx7);
            //
            let mm_mask8 = _mm_andnot_si128(mm_mask77, mm_mask88);
            let mm_and8 = _mm_and_si128(mm_in_p1, mm_mask8);
            let mm_idx8 = _mm_sub_epi8(mm_and8, _mm_set1_epi8(1));
            let mm_out8 = _mm_shuffle_epi8(mm_map8, mm_idx8);
            //
            // make out
            let mm_out = _mm_or_si128(
                _mm_or_si128(
                    _mm_or_si128(mm_out3, mm_out4),
                    _mm_or_si128(mm_out5, mm_out6),
                ),
                _mm_or_si128(mm_out7, mm_out8),
            );
            let mm_err = _mm_cmplt_epi8(mm_out, _mm_set1_epi8(0));
            _mm_storeu_si128(err_buf.as_ptr() as *mut __m128i, mm_err);
            dec_check_error!(err_buf, buf_ptr);
            _mm_storeu_si128(buf_ptr as *mut __m128i, mm_out);
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
