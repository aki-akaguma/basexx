use super::super::*;

#[inline(never)]
#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _binary_to_ascii_32_ssse3(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), EncodeError> {
    assert_eq!(lup.len(), 32);
    _binary_to_ascii_32_ssse3_chunks16(lup, buf)?;
    Ok(())
}

#[inline(always)]
pub(crate) unsafe fn _binary_to_ascii_32_ssse3_chunks16(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), EncodeError> {
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
            buf2.copy_from_slice(std::slice::from_raw_parts(buf_ptr as *const u64, 2));
            _binary_to_ascii_32_ssse3_c16_chunks16(lup, &mut buf2)?;
            *(buf_ptr as *mut u64) = buf2[0];
            *((buf_ptr as *mut u64).add(1)) = buf2[1];
            //
            buf_ptr = unsafe { buf_ptr.add(16) };
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
//#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _binary_to_ascii_32_ssse3_c16(
    lup: &[u8],
    buf: &mut [u64; 2],
) -> Result<(), EncodeError> {
    assert_eq!(lup.len(), 32);
    _binary_to_ascii_32_ssse3_c16_chunks16(lup, buf)?;
    Ok(())
}

macro_rules! enc_check_error {
    ($err_buf: expr, $buf_ptr: expr) => {
        if $err_buf[0] != 0 || $err_buf[1] != 0 {
            // on error
            let c1 = $err_buf[0];
            if c1 & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr));
            }
            if (c1 >> 8) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(1)));
            }
            if (c1 >> (8 * 2)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(2)));
            }
            if (c1 >> (8 * 3)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(3)));
            }
            if (c1 >> (8 * 4)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(4)));
            }
            if (c1 >> (8 * 5)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(5)));
            }
            if (c1 >> (8 * 6)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(6)));
            }
            if (c1 >> (8 * 7)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(7)));
            }
            //
            let c2 = $err_buf[1];
            if c2 & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(8)));
            }
            if (c2 >> 8) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(9)));
            }
            if (c2 >> (8 * 2)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(10)));
            }
            if (c2 >> (8 * 3)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(11)));
            }
            if (c2 >> (8 * 4)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(12)));
            }
            if (c2 >> (8 * 5)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(13)));
            }
            if (c2 >> (8 * 6)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(14)));
            }
            if (c2 >> (8 * 7)) & 0xFF != 0 {
                return Err(EncodeError::InvalidIndex(*$buf_ptr.add(15)));
            }
        }
    };
}

#[inline(always)]
pub(crate) unsafe fn _binary_to_ascii_32_ssse3_c16_chunks16(
    lup: &[u8],
    buf: &mut [u64; 2],
) -> Result<(), EncodeError> {
    //panic!("PASS: SSE2");
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    {
        let buf_ptr = buf.as_mut_ptr() as *mut u8;
        //
        let mm_map1 = _mm_loadu_si128(lup.as_ptr() as *const __m128i);
        let mm_map2 = _mm_loadu_si128(lup.as_ptr().add(16) as *const __m128i);
        let err_buf = [0u64; 2];
        //
        {
            // ..|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa
            let mm_in = _mm_loadu_si128(buf_ptr as *const __m128i);
            //
            // check error
            let mm_err = _mm_or_si128(
                _mm_cmplt_epi8(mm_in, _mm_set1_epi8(0)),
                _mm_cmpgt_epi8(mm_in, _mm_set1_epi8(32 - 1)),
            );
            _mm_storeu_si128(err_buf.as_ptr() as *mut __m128i, mm_err);
            enc_check_error!(err_buf, buf_ptr);
            //_mm_prefetch(buf_ptr.cast::<i8>(), _MM_HINT_T0);
            let mm_in_p1 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            //
            let mm_mask11 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16));
            let mm_mask22 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 2));
            //
            let mm_mask1 = mm_mask11;
            let mm_and1 = _mm_and_si128(mm_in_p1, mm_mask1);
            let mm_idx1 = _mm_sub_epi8(mm_and1, _mm_set1_epi8(1));
            let mm_out1 = _mm_shuffle_epi8(mm_map1, mm_idx1);
            //
            let mm_mask2 = _mm_andnot_si128(mm_mask11, mm_mask22);
            let mm_and2 = _mm_and_si128(mm_in_p1, mm_mask2);
            let mm_idx2 = _mm_sub_epi8(mm_and2, _mm_set1_epi8(1));
            let mm_out2 = _mm_shuffle_epi8(mm_map2, mm_idx2);
            //
            // make out
            let mm_out = _mm_or_si128(mm_out1, mm_out2);
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
