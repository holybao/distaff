use criterion::{ black_box, criterion_group, Criterion };
use distaff::{ field, parallel };

pub fn add(c: &mut Criterion) {
    let x = 20;
    let y = 20;
    c.bench_function("add", |bench| {
        bench.iter(|| field::add(black_box(x), black_box(y)))
    });
}

pub fn mul(c: &mut Criterion) {
    let x = 20;
    let y = 20;
    c.bench_function("mul", |bench| {
        bench.iter(|| field::mul(black_box(x), black_box(y)))
    });
}

pub fn mul_parallel(c: &mut Criterion) {

    let n = (1 << 10) as usize;
    let x = field::rand_vector(n);
    let y = field::rand_vector(n);
    let threads = 2;

    c.bench_function("mul (parallel)", |bench| {
        bench.iter(|| parallel::mul(black_box(&x), black_box(&y), black_box(threads)))
    });
}

pub fn mul_parallel_in_place(c: &mut Criterion) {

    let n = (1 << 10) as usize;
    let x = field::rand_vector(n);
    let mut y = field::rand_vector(n);
    let threads = 2;

    c.bench_function("mul (parallel, in place)", |bench| {
        bench.iter(|| parallel::mul_in_place(black_box(&mut y), black_box(&x), black_box(threads)))
    });
}

pub fn exp(c: &mut Criterion) {
    let x = 20;
    let y = 20;
    c.bench_function("exp", |bench| {
        bench.iter(|| field::exp(black_box(x), black_box(y)))
    });
}

pub fn inv(c: &mut Criterion) {
    let x = 20;
    c.bench_function("inv", |bench| {
        bench.iter(|| field::inv(black_box(x)))
    });
}

criterion_group!(group, add, mul, mul_parallel, mul_parallel_in_place, exp, inv);