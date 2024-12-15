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
fn test_ags_binary_to_ascii_scalar_1() {
    let (ags, bin_buf, asc_buf) = gen_data_ags_64();
    let mut buf = bin_buf.clone();
    let r = _binary_to_ascii_scalar(&ags.binmap, &mut buf);
    assert!(r.is_ok());
    assert_eq!(buf, asc_buf);
}

#[test]
fn test_ags_binary_to_ascii_scalar_2() {
    let (ags, bin_buf0, asc_buf0) = gen_data_ags_64();
    let mut buf = Vec::<u8>::with_capacity(4 * 64);
    buf.extend_from_slice(&bin_buf0);
    buf.extend_from_slice(&bin_buf0);
    buf.extend_from_slice(&bin_buf0);
    buf.extend_from_slice(&bin_buf0);
    let r = _binary_to_ascii_scalar(&ags.binmap, &mut buf);
    assert!(r.is_ok());
    let mut cor = Vec::<u8>::with_capacity(4 * 64);
    cor.extend_from_slice(&asc_buf0);
    cor.extend_from_slice(&asc_buf0);
    cor.extend_from_slice(&asc_buf0);
    cor.extend_from_slice(&asc_buf0);
    assert_eq!(buf, cor);
}

#[test]
fn test_ags_binary_to_ascii_scalar_3() {
    let (ags, bin_buf0, _asc_buf0) = gen_data_ags_64();
    for i in 0..64 {
        let mut buf = bin_buf0.clone();
        buf.insert(i, 64);
        let r = _binary_to_ascii_scalar(&ags.binmap, &mut buf);
        assert_eq!(r, Err(EncodeError::InvalidIndex(64)));
    }
    for i in 0..64 {
        let mut buf = bin_buf0.clone();
        buf.insert(i, 128);
        let r = _binary_to_ascii_scalar(&ags.binmap, &mut buf);
        assert_eq!(r, Err(EncodeError::InvalidIndex(128)));
    }
    for i in 0..64 {
        let mut buf = bin_buf0.clone();
        buf.insert(i, 255);
        let r = _binary_to_ascii_scalar(&ags.binmap, &mut buf);
        assert_eq!(r, Err(EncodeError::InvalidIndex(255)));
    }
}

#[test]
fn test_ags_ascii_to_binary_scalar_1() {
    let (ags, bin_buf, asc_buf) = gen_data_ags_64();
    let mut buf = asc_buf.clone();
    let r = _ascii_to_binary_scalar(&ags.a128map, &mut buf);
    assert!(r.is_ok());
    assert_eq!(buf, bin_buf);
}

#[test]
fn test_ags_ascii_to_binary_scalar_2() {
    let (ags, bin_buf, asc_buf) = gen_data_ags_64();
    let mut buf = Vec::<u8>::with_capacity(4 * 64);
    buf.extend_from_slice(&asc_buf);
    buf.extend_from_slice(&asc_buf);
    buf.extend_from_slice(&asc_buf);
    buf.extend_from_slice(&asc_buf);
    let r = _ascii_to_binary_scalar(&ags.a128map, &mut buf);
    assert!(r.is_ok());
    let mut cor = Vec::<u8>::with_capacity(4 * 64);
    cor.extend_from_slice(&bin_buf);
    cor.extend_from_slice(&bin_buf);
    cor.extend_from_slice(&bin_buf);
    cor.extend_from_slice(&bin_buf);
    assert_eq!(buf, cor);
}

#[test]
fn test_ags_ascii_to_binary_scalar_3() {
    let (ags, _bin_buf, asc_buf0) = gen_data_ags_64();
    for i in 0..64 {
        let mut buf = asc_buf0.clone();
        buf.insert(i, 1);
        let r = _ascii_to_binary_scalar(&ags.a128map, &mut buf);
        assert_eq!(r, Err(DecodeError::InvalidByte(1)));
    }
    for i in 0..64 {
        let mut buf = asc_buf0.clone();
        buf.insert(i, b'.');
        let r = _ascii_to_binary_scalar(&ags.a128map, &mut buf);
        assert_eq!(r, Err(DecodeError::InvalidByte(46)));
    }
    for i in 0..64 {
        let mut buf = asc_buf0.clone();
        buf.insert(i, 255);
        let r = _ascii_to_binary_scalar(&ags.a128map, &mut buf);
        assert_eq!(r, Err(DecodeError::InvalidByte(255)));
    }
}
