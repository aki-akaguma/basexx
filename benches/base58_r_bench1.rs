#[cfg(feature = "rug")]
use basexx::*;
use criterion::*;

#[cfg(feature = "rug")]
mod utils;

#[cfg(feature = "rug")]
fn bench_base58_1(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base58 = Base58R::default();
    c.bench_function("base58_r_1", |b| b.iter(|| base58.encode(black_box(&v))));
}

#[cfg(feature = "rug")]
fn bench_base58_2(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base58 = Base58R::default();
    let a = base58.encode(black_box(&v)).unwrap();
    c.bench_function("base58_r_2", |b| b.iter(|| base58.decode(black_box(&a))));
}

#[cfg(feature = "rug")]
criterion_group!(benches, bench_base58_1, bench_base58_2);

#[cfg(not(feature = "rug"))]
fn bench_dummy(_c: &mut Criterion) {}

#[cfg(not(feature = "rug"))]
criterion_group!(benches, bench_dummy);

criterion_main!(benches);
