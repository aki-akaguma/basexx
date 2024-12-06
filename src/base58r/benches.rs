use super::*;
use crate::test_utils;
use criterion::*;

#[cfg(feature = "rug")]
#[allow(dead_code)]
pub fn bench_base58r_enc(c: &mut Criterion) {
    let v = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP58);
    c.bench_function("base58r_enc", |b| {
        b.iter(|| _encode_base58r(black_box(&ags), black_box(&v)))
    });
}

#[cfg(feature = "rug")]
#[allow(dead_code)]
pub fn bench_base58r_dec(c: &mut Criterion) {
    let v = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP58);
    let a = _encode_base58r(black_box(&ags), black_box(&v)).unwrap();
    c.bench_function("base58r_dec", |b| {
        b.iter(|| _decode_base58r(black_box(&ags), black_box(&a)))
    });
}
