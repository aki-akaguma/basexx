use basexx::*;
use criterion::*;

mod utils;

fn bench_base56_1(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base56 = Base56::default();
    c.bench_function("base56_1", |b| b.iter(|| base56.encode(black_box(&v))));
}

fn bench_base56_2(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base56 = Base56::default();
    let a = base56.encode(black_box(&v)).unwrap();
    c.bench_function("base56_2", |b| b.iter(|| base56.decode(black_box(&a))));
}

criterion_group!(benches, bench_base56_1, bench_base56_2);
criterion_main!(benches);
