use super::*;
use crate::test_utils;
use criterion::*;

#[allow(dead_code)]
pub fn bench_base64g_enc(c: &mut Criterion) {
    let v = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    c.bench_function("base64g_enc", |b| {
        b.iter(|| _encode_base64g(black_box(&ags), black_box(&v)))
    });
}

#[allow(dead_code)]
pub fn bench_base64g_dec(c: &mut Criterion) {
    let v = test_utils::read_t4_data();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let a = _encode_base64g(black_box(&ags), black_box(&v)).unwrap();
    c.bench_function("base64g_dec", |b| {
        b.iter(|| _decode_base64g(black_box(&ags), black_box(&a)))
    });
}
