use super::super::*;

#[inline(never)]
#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _binary_to_ascii_64_ssse3(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), EncodeError> {
    assert_eq!(lup.len(), 64);
    _binary_to_ascii_64_ssse3_chunks16(lup, buf)?;
    Ok(())
}

#[cfg(target_feature = "sse2")]
#[inline(always)]
//#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _binary_to_ascii_64_ssse3_c16(
    lup: &[u8],
    buf: &mut [u64; 2],
) -> Result<(), EncodeError> {
    assert_eq!(lup.len(), 64);
    _binary_to_ascii_64_ssse3_c16_chunks16(lup, buf)?;
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
pub(crate) unsafe fn _binary_to_ascii_64_ssse3_c16_chunks16(
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
        let mm_map3 = _mm_loadu_si128(lup.as_ptr().add(16 * 2) as *const __m128i);
        let mm_map4 = _mm_loadu_si128(lup.as_ptr().add(16 * 3) as *const __m128i);
        let err_buf = [0u64; 2];
        //
        {
            // ..|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa
            let mm_in = _mm_loadu_si128(buf_ptr as *const __m128i);
            //
            // check error
            let mm_check1 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(0));
            let mm_check2 = _mm_cmpgt_epi8(mm_in, _mm_set1_epi8(64 - 1));
            let mm_check = _mm_or_si128(mm_check1, mm_check2);
            _mm_storeu_si128(err_buf.as_ptr() as *mut __m128i, mm_check);
            enc_check_error!(err_buf, buf_ptr);
            //
            let mm_mask1 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16));
            let mm_add1 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and1 = _mm_and_si128(mm_add1, mm_mask1);
            let mm_idx1 = _mm_sub_epi8(mm_and1, _mm_set1_epi8(1));
            let mm_out1 = _mm_shuffle_epi8(mm_map1, mm_idx1);
            //
            let mm_mask11 = _mm_andnot_si128(mm_mask1, _mm_set1_epi8(-1));
            let mm_mask22 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 2));
            let mm_mask2 = _mm_and_si128(mm_mask11, mm_mask22);
            let mm_add2 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and2 = _mm_and_si128(mm_add2, mm_mask2);
            let mm_idx2 = _mm_sub_epi8(mm_and2, _mm_set1_epi8(1));
            let mm_out2 = _mm_shuffle_epi8(mm_map2, mm_idx2);
            //
            let mm_mask22 = _mm_andnot_si128(mm_mask22, _mm_set1_epi8(-1));
            let mm_mask33 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 3));
            let mm_mask3 = _mm_and_si128(mm_mask22, mm_mask33);
            let mm_add3 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and3 = _mm_and_si128(mm_add3, mm_mask3);
            let mm_idx3 = _mm_sub_epi8(mm_and3, _mm_set1_epi8(1));
            let mm_out3 = _mm_shuffle_epi8(mm_map3, mm_idx3);
            //
            let mm_mask4 = _mm_andnot_si128(mm_mask33, _mm_set1_epi8(-1));
            let mm_add4 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and4 = _mm_and_si128(mm_add4, mm_mask4);
            let mm_idx4 = _mm_sub_epi8(mm_and4, _mm_set1_epi8(1));
            let mm_out4 = _mm_shuffle_epi8(mm_map4, mm_idx4);
            //
            let mm_out = _mm_or_si128(
                _mm_or_si128(mm_out1, mm_out2),
                _mm_or_si128(mm_out3, mm_out4),
            );
            _mm_storeu_si128(buf_ptr as *mut __m128i, mm_out);
        }
    };
    //
    Ok(())
}

#[inline(always)]
pub(crate) unsafe fn _binary_to_ascii_64_ssse3_chunks16(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), EncodeError> {
    //panic!("PASS: SSE2");
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    let buf = if buf.len() < 16 {
        buf
    } else {
        let buf_len = buf.len();
        let mut buf_ptr = buf.as_mut_ptr();
        let end_ptr = unsafe { buf_ptr.add(buf_len) };
        let end_ptr_limit = unsafe { end_ptr.sub(16 - 1) };
        //
        let mm_map1 = _mm_loadu_si128(lup.as_ptr() as *const __m128i);
        let mm_map2 = _mm_loadu_si128(lup.as_ptr().add(16) as *const __m128i);
        let mm_map3 = _mm_loadu_si128(lup.as_ptr().add(16 * 2) as *const __m128i);
        let mm_map4 = _mm_loadu_si128(lup.as_ptr().add(16 * 3) as *const __m128i);
        let err_buf = [0u64; 2];
        //
        while buf_ptr < end_ptr_limit {
            // ..|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa
            let mm_in = _mm_loadu_si128(buf_ptr as *const __m128i);
            //
            // check error
            let mm_check1 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(0));
            let mm_check2 = _mm_cmpgt_epi8(mm_in, _mm_set1_epi8(64 - 1));
            let mm_check = _mm_or_si128(mm_check1, mm_check2);
            _mm_storeu_si128(err_buf.as_ptr() as *mut __m128i, mm_check);
            enc_check_error!(err_buf, buf_ptr);
            //
            let mm_mask1 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16));
            let mm_add1 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and1 = _mm_and_si128(mm_add1, mm_mask1);
            let mm_idx1 = _mm_sub_epi8(mm_and1, _mm_set1_epi8(1));
            let mm_out1 = _mm_shuffle_epi8(mm_map1, mm_idx1);
            //
            let mm_mask11 = _mm_andnot_si128(mm_mask1, _mm_set1_epi8(-1));
            let mm_mask22 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 2));
            let mm_mask2 = _mm_and_si128(mm_mask11, mm_mask22);
            let mm_add2 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and2 = _mm_and_si128(mm_add2, mm_mask2);
            let mm_idx2 = _mm_sub_epi8(mm_and2, _mm_set1_epi8(1));
            let mm_out2 = _mm_shuffle_epi8(mm_map2, mm_idx2);
            //
            let mm_mask22 = _mm_andnot_si128(mm_mask22, _mm_set1_epi8(-1));
            let mm_mask33 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 3));
            let mm_mask3 = _mm_and_si128(mm_mask22, mm_mask33);
            let mm_add3 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and3 = _mm_and_si128(mm_add3, mm_mask3);
            let mm_idx3 = _mm_sub_epi8(mm_and3, _mm_set1_epi8(1));
            let mm_out3 = _mm_shuffle_epi8(mm_map3, mm_idx3);
            //
            let mm_mask4 = _mm_andnot_si128(mm_mask33, _mm_set1_epi8(-1));
            let mm_add4 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and4 = _mm_and_si128(mm_add4, mm_mask4);
            let mm_idx4 = _mm_sub_epi8(mm_and4, _mm_set1_epi8(1));
            let mm_out4 = _mm_shuffle_epi8(mm_map4, mm_idx4);
            //
            let mm_out = _mm_or_si128(
                _mm_or_si128(mm_out1, mm_out2),
                _mm_or_si128(mm_out3, mm_out4),
            );
            _mm_storeu_si128(buf_ptr as *mut __m128i, mm_out);
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

#[inline(never)]
#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _ascii_to_binary_64_ssse3(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), DecodeError> {
    assert_eq!(lup.len(), 128);
    _ascii_to_binary_64_ssse3_chunks16(lup, buf)?;
    Ok(())
}

#[cfg(target_feature = "sse2")]
#[inline(always)]
//#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn _ascii_to_binary_64_ssse3_c16(
    lup: &[u8],
    buf: &mut [u64; 2],
) -> Result<(), DecodeError> {
    assert_eq!(lup.len(), 128);
    _ascii_to_binary_64_ssse3_c16_chunks16(lup, buf)?;
    Ok(())
}

macro_rules! dec_check_error {
    ($err_buf: expr, $buf_ptr: expr) => {
        if $err_buf[0] != 0 || $err_buf[1] != 0 {
            // on error
            let c1 = $err_buf[0];
            if c1 & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr));
            }
            if (c1 >> 8) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(1)));
            }
            if (c1 >> (8 * 2)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(2)));
            }
            if (c1 >> (8 * 3)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(3)));
            }
            if (c1 >> (8 * 4)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(4)));
            }
            if (c1 >> (8 * 5)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(5)));
            }
            if (c1 >> (8 * 6)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(6)));
            }
            if (c1 >> (8 * 7)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(7)));
            }
            //
            let c2 = $err_buf[1];
            if c2 & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(8)));
            }
            if (c2 >> 8) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(9)));
            }
            if (c2 >> (8 * 2)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(10)));
            }
            if (c2 >> (8 * 3)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(11)));
            }
            if (c2 >> (8 * 4)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(12)));
            }
            if (c2 >> (8 * 5)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(13)));
            }
            if (c2 >> (8 * 6)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(14)));
            }
            if (c2 >> (8 * 7)) & 0xFF != 0 {
                return Err(DecodeError::InvalidByte(*$buf_ptr.add(15)));
            }
        }
    };
}

#[inline(always)]
pub(crate) unsafe fn _ascii_to_binary_64_ssse3_c16_chunks16(
    lup: &[u8],
    buf: &mut [u64; 2],
) -> Result<(), DecodeError> {
    //panic!("PASS: SSE2");
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    {
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
            let mm_idx1 = _mm_or_si128(
                _mm_cmplt_epi8(mm_in, _mm_set1_epi8(33)),
                _mm_cmpeq_epi8(mm_in, _mm_set1_epi8(127)),
            );
            let mm_err = _mm_cmplt_epi8(mm_idx1, _mm_set1_epi8(0));
            _mm_storeu_si128(err_buf.as_ptr() as *mut __m128i, mm_err);
            dec_check_error!(err_buf, buf_ptr);
            //
            let mm_mask22 = _mm_andnot_si128(mm_idx1, _mm_set1_epi8(-1));
            let mm_mask33 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 3));
            let mm_mask3 = _mm_and_si128(mm_mask22, mm_mask33);
            let mm_add3 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and3 = _mm_and_si128(mm_add3, mm_mask3);
            let mm_idx3 = _mm_sub_epi8(mm_and3, _mm_set1_epi8(1));
            let mm_out3 = _mm_shuffle_epi8(mm_map3, mm_idx3);
            //
            let mm_mask33 = _mm_andnot_si128(mm_mask3, _mm_set1_epi8(-1));
            let mm_mask44 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 4));
            let mm_mask4 = _mm_and_si128(mm_mask33, mm_mask44);
            let mm_add4 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and4 = _mm_and_si128(mm_add4, mm_mask4);
            let mm_idx4 = _mm_sub_epi8(mm_and4, _mm_set1_epi8(1));
            let mm_out4 = _mm_shuffle_epi8(mm_map4, mm_idx4);
            //
            let mm_mask44 = _mm_andnot_si128(mm_mask44, _mm_set1_epi8(-1));
            let mm_mask55 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 5));
            let mm_mask5 = _mm_and_si128(mm_mask44, mm_mask55);
            let mm_add5 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and5 = _mm_and_si128(mm_add5, mm_mask5);
            let mm_idx5 = _mm_sub_epi8(mm_and5, _mm_set1_epi8(1));
            let mm_out5 = _mm_shuffle_epi8(mm_map5, mm_idx5);
            //
            let mm_mask55 = _mm_andnot_si128(mm_mask55, _mm_set1_epi8(-1));
            let mm_mask66 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 6));
            let mm_mask6 = _mm_and_si128(mm_mask55, mm_mask66);
            let mm_add6 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and6 = _mm_and_si128(mm_add6, mm_mask6);
            let mm_idx6 = _mm_sub_epi8(mm_and6, _mm_set1_epi8(1));
            let mm_out6 = _mm_shuffle_epi8(mm_map6, mm_idx6);
            //
            let mm_mask66 = _mm_andnot_si128(mm_mask66, _mm_set1_epi8(-1));
            let mm_mask77 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 7));
            let mm_mask7 = _mm_and_si128(mm_mask66, mm_mask77);
            let mm_add7 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and7 = _mm_and_si128(mm_add7, mm_mask7);
            let mm_idx7 = _mm_sub_epi8(mm_and7, _mm_set1_epi8(1));
            let mm_out7 = _mm_shuffle_epi8(mm_map7, mm_idx7);
            //
            let mm_mask8 = _mm_andnot_si128(mm_mask77, _mm_set1_epi8(-1));
            let mm_add8 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and8 = _mm_and_si128(mm_add8, mm_mask8);
            let mm_idx8 = _mm_sub_epi8(mm_and8, _mm_set1_epi8(1));
            let mm_out8 = _mm_shuffle_epi8(mm_map8, mm_idx8);
            //
            // make out
            let mm_out_34 = _mm_or_si128(mm_out3, mm_out4);
            let mm_out_58 = _mm_or_si128(
                _mm_or_si128(mm_out5, mm_out6),
                _mm_or_si128(mm_out7, mm_out8),
            );
            let mm_out = _mm_or_si128(mm_out_34, mm_out_58);
            let mm_err = _mm_cmplt_epi8(mm_out, _mm_set1_epi8(0));
            _mm_storeu_si128(err_buf.as_ptr() as *mut __m128i, mm_err);
            dec_check_error!(err_buf, buf_ptr);
            _mm_storeu_si128(buf_ptr as *mut __m128i, mm_out);
        }
    };
    //
    Ok(())
}

#[inline(always)]
pub(crate) unsafe fn _ascii_to_binary_64_ssse3_chunks16(
    lup: &[u8],
    buf: &mut [u8],
) -> Result<(), DecodeError> {
    //panic!("PASS: SSE2");
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    //
    let buf = if buf.len() < 16 {
        buf
    } else {
        let buf_len = buf.len();
        let mut buf_ptr = buf.as_mut_ptr();
        let end_ptr = unsafe { buf_ptr.add(buf_len) };
        let end_ptr_limit = unsafe { end_ptr.sub(16 - 1) };
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
        while buf_ptr < end_ptr_limit {
            // ..|00ffffff|00eeeeee|00dddddd|00cccccc|00bbbbbb|00aaaaaa
            let mm_in = _mm_loadu_si128(buf_ptr as *const __m128i);
            //
            let mm_idx1 = _mm_or_si128(
                _mm_cmplt_epi8(mm_in, _mm_set1_epi8(33)),
                _mm_cmpeq_epi8(mm_in, _mm_set1_epi8(127)),
            );
            let mm_err = _mm_cmplt_epi8(mm_idx1, _mm_set1_epi8(0));
            _mm_storeu_si128(err_buf.as_ptr() as *mut __m128i, mm_err);
            dec_check_error!(err_buf, buf_ptr);
            //
            let mm_mask22 = _mm_andnot_si128(mm_idx1, _mm_set1_epi8(-1));
            let mm_mask33 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 3));
            let mm_mask3 = _mm_and_si128(mm_mask22, mm_mask33);
            let mm_add3 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and3 = _mm_and_si128(mm_add3, mm_mask3);
            let mm_idx3 = _mm_sub_epi8(mm_and3, _mm_set1_epi8(1));
            let mm_out3 = _mm_shuffle_epi8(mm_map3, mm_idx3);
            //
            let mm_mask33 = _mm_andnot_si128(mm_mask3, _mm_set1_epi8(-1));
            let mm_mask44 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 4));
            let mm_mask4 = _mm_and_si128(mm_mask33, mm_mask44);
            let mm_add4 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and4 = _mm_and_si128(mm_add4, mm_mask4);
            let mm_idx4 = _mm_sub_epi8(mm_and4, _mm_set1_epi8(1));
            let mm_out4 = _mm_shuffle_epi8(mm_map4, mm_idx4);
            //
            let mm_mask44 = _mm_andnot_si128(mm_mask44, _mm_set1_epi8(-1));
            let mm_mask55 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 5));
            let mm_mask5 = _mm_and_si128(mm_mask44, mm_mask55);
            let mm_add5 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and5 = _mm_and_si128(mm_add5, mm_mask5);
            let mm_idx5 = _mm_sub_epi8(mm_and5, _mm_set1_epi8(1));
            let mm_out5 = _mm_shuffle_epi8(mm_map5, mm_idx5);
            //
            let mm_mask55 = _mm_andnot_si128(mm_mask55, _mm_set1_epi8(-1));
            let mm_mask66 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 6));
            let mm_mask6 = _mm_and_si128(mm_mask55, mm_mask66);
            let mm_add6 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and6 = _mm_and_si128(mm_add6, mm_mask6);
            let mm_idx6 = _mm_sub_epi8(mm_and6, _mm_set1_epi8(1));
            let mm_out6 = _mm_shuffle_epi8(mm_map6, mm_idx6);
            //
            let mm_mask66 = _mm_andnot_si128(mm_mask66, _mm_set1_epi8(-1));
            let mm_mask77 = _mm_cmplt_epi8(mm_in, _mm_set1_epi8(16 * 7));
            let mm_mask7 = _mm_and_si128(mm_mask66, mm_mask77);
            let mm_add7 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and7 = _mm_and_si128(mm_add7, mm_mask7);
            let mm_idx7 = _mm_sub_epi8(mm_and7, _mm_set1_epi8(1));
            let mm_out7 = _mm_shuffle_epi8(mm_map7, mm_idx7);
            //
            let mm_mask8 = _mm_andnot_si128(mm_mask77, _mm_set1_epi8(-1));
            let mm_add8 = _mm_add_epi8(mm_in, _mm_set1_epi8(1));
            let mm_and8 = _mm_and_si128(mm_add8, mm_mask8);
            let mm_idx8 = _mm_sub_epi8(mm_and8, _mm_set1_epi8(1));
            let mm_out8 = _mm_shuffle_epi8(mm_map8, mm_idx8);
            //
            // make out
            let mm_out_34 = _mm_or_si128(mm_out3, mm_out4);
            let mm_out_58 = _mm_or_si128(
                _mm_or_si128(mm_out5, mm_out6),
                _mm_or_si128(mm_out7, mm_out8),
            );
            let mm_out = _mm_or_si128(mm_out_34, mm_out_58);
            let mm_err = _mm_cmplt_epi8(mm_out, _mm_set1_epi8(0));
            _mm_storeu_si128(err_buf.as_ptr() as *mut __m128i, mm_err);
            dec_check_error!(err_buf, buf_ptr);
            _mm_storeu_si128(buf_ptr as *mut __m128i, mm_out);
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

#[cfg(all(test, not(feature = "bench")))]
mod tests;

#[cfg(all(test, feature = "ubench"))]
mod benches;
#[cfg(all(test, feature = "ubench"))]
#[allow(unused_imports)]
pub(crate) use benches::*;
