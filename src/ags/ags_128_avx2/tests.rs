#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;
//use rstest::rstest;
//use rstest_reuse::{self, *};

fn gen_data_ags_64() -> (AsciiGraphicSet, Vec<u8>, Vec<u8>) {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut bin_buf = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        bin_buf.push(i);
    }
    let mut asc_buf = Vec::<u8>::with_capacity(64);
    asc_buf.extend_from_slice(&test_utils::_CMAP64);
    (ags, bin_buf, asc_buf)
}

#[test]
fn test_ags_ascii_to_binary_128_avx2_1() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let (ags, bin_buf, asc_buf) = gen_data_ags_64();
    let mut buf = asc_buf.clone();
    let r = unsafe { _ascii_to_binary_128_avx2(&ags.a128map, &mut buf) };
    assert!(r.is_ok());
    assert_eq!(buf, bin_buf);
}

#[test]
fn test_ags_ascii_to_binary_128_avx2_2() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let (ags, bin_buf, asc_buf) = gen_data_ags_64();
    let mut buf = Vec::<u8>::with_capacity(4 * 64);
    buf.extend_from_slice(&asc_buf);
    buf.extend_from_slice(&asc_buf);
    buf.extend_from_slice(&asc_buf);
    buf.extend_from_slice(&asc_buf);
    let r = unsafe { _ascii_to_binary_128_avx2(&ags.a128map, &mut buf) };
    assert!(r.is_ok());
    let mut cor = Vec::<u8>::with_capacity(4 * 64);
    cor.extend_from_slice(&bin_buf);
    cor.extend_from_slice(&bin_buf);
    cor.extend_from_slice(&bin_buf);
    cor.extend_from_slice(&bin_buf);
    assert_eq!(buf, cor);
}

#[test]
fn test_ags_ascii_to_binary_128_avx2_3() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let (ags, _bin_buf, asc_buf0) = gen_data_ags_64();
    for i in 0..64 {
        let mut buf = asc_buf0.clone();
        buf.insert(i, 1);
        let r = unsafe { _ascii_to_binary_128_avx2(&ags.a128map, &mut buf) };
        assert_eq!(r, Err(DecodeError::InvalidByte(1)));
    }
    for i in 0..64 {
        let mut buf = asc_buf0.clone();
        buf.insert(i, b'.');
        let r = unsafe { _ascii_to_binary_128_avx2(&ags.a128map, &mut buf) };
        assert_eq!(r, Err(DecodeError::InvalidByte(46)));
    }
    for i in 0..64 {
        let mut buf = asc_buf0.clone();
        buf.insert(i, 255);
        let r = unsafe { _ascii_to_binary_128_avx2(&ags.a128map, &mut buf) };
        assert_eq!(r, Err(DecodeError::InvalidByte(255)));
    }
}
