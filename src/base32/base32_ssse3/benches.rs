use super::*;
use crate::test_utils;
use criterion::*;

#[allow(dead_code)]
pub fn bench_base32_ssse3_enc(c: &mut Criterion) {
    let v = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP32);
    c.bench_function("base32_ssse3_enc", |b| {
        b.iter(|| unsafe { _encode_base32_ssse3(black_box(&ags), black_box(&v)) })
    });
}

#[allow(dead_code)]
pub fn bench_base32_ssse3_dec(c: &mut Criterion) {
    let v = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP32);
    let a = unsafe { _encode_base32_ssse3(black_box(&ags), black_box(&v)).unwrap() };
    c.bench_function("base32_ssse3_dec", |b| {
        b.iter(|| unsafe { _decode_base32_ssse3(black_box(&ags), black_box(&a)) })
    });
}
