use basexx::*;
use criterion::*;

mod utils;

const _CMAP64: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

fn bench_ags_enc(c: &mut Criterion) {
    let mut v = utils::read_ascii_t1();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let _ = ags.ascii_to_binary(&mut v);
    c.bench_function("ags_enc", |b| {
        b.iter(|| {
            let mut v = v.clone();
            let _ = ags.binary_to_ascii(&mut v);
        })
    });
}

fn bench_ags_dec(c: &mut Criterion) {
    let v = utils::read_ascii_t1();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    c.bench_function("ags_dec", |b| {
        b.iter(|| {
            let mut v = v.clone();
            let _ = ags.ascii_to_binary(&mut v);
        })
    });
}

criterion_group!(benches, bench_ags_enc, bench_ags_dec);
criterion_main!(benches);
