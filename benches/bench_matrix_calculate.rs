use criterion::{criterion_group, criterion_main, Criterion};
use zmatrix::dense::Matrix;

fn bench_matrix_add_10x10(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix add");
    let a = Matrix::<10, 10, f64>::random();
    let b = Matrix::<10, 10, f64>::random();

    group.bench_function("10x10", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x + y,
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_add_100x100(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix add");
    let a = Matrix::<100, 100, f64>::random();
    let b = Matrix::<100, 100, f64>::random();

    group.bench_function("100x100", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x + y,
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_add_1000x1000(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix add");
    let a = Matrix::<1000, 1000, f64>::random();
    let b = Matrix::<1000, 1000, f64>::random();

    group.bench_function("1000x1000", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x + y,
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_sub_10x10(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix subtract");
    let a = Matrix::<10, 10, f64>::random();
    let b = Matrix::<10, 10, f64>::random();

    group.bench_function("10x10", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x - y,
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_sub_100x100(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix subtract");
    let a = Matrix::<100, 100, f64>::random();
    let b = Matrix::<100, 100, f64>::random();

    group.bench_function("100x100", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x - y,
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_sub_1000x1000(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix subtract");
    let a = Matrix::<1000, 1000, f64>::random();
    let b = Matrix::<1000, 1000, f64>::random();

    group.bench_function("1000x1000", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x - y,
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_mul_10x10(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix multiply");
    let a = Matrix::<10, 10, f64>::random();
    let b = Matrix::<10, 10, f64>::random();

    group.bench_function("10x10", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x * y,
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_mul_100x100(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix multiply");
    let a = Matrix::<100, 100, f64>::random();
    let b = Matrix::<100, 100, f64>::random();

    group.bench_function("100x100", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x * y,
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_mul_1000x1000(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix multiply");
    let a = Matrix::<1000, 1000, f64>::random();
    let b = Matrix::<1000, 1000, f64>::random();

    group.bench_function("1000x1000", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x * y,
            criterion::BatchSize::PerIteration, // Process one iteration at a time
        );
    });
}

fn bench_matrix_product_10x10(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix product");
    let a = Matrix::<10, 10, f64>::random();
    let b = Matrix::<10, 10, f64>::random();

    group.bench_function("10x10", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x.product(&y),
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_product_100x100(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix product");
    let a = Matrix::<100, 100, f64>::random();
    let b = Matrix::<100, 100, f64>::random();

    group.bench_function("100x100", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x.product(&y),
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_matrix_product_1000x1000(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix product");
    let a = Matrix::<1000, 1000, f64>::random();
    let b = Matrix::<1000, 1000, f64>::random();

    group.bench_function("1000x1000", |bencher| {
        bencher.iter_batched(
            || (a.clone(), b.clone()),
            |(x, y)| x.product(&y),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches_add,
    bench_matrix_add_10x10,
    bench_matrix_add_100x100,
   // bench_matrix_add_1000x1000,
);
criterion_group!(
    bences_sub,
    bench_matrix_sub_10x10,
    bench_matrix_sub_100x100,
   // bench_matrix_sub_1000x1000
);
criterion_group!(benches_mul, bench_matrix_mul_10x10, bench_matrix_mul_100x100);
criterion_group!(benches_product, bench_matrix_product_10x10, bench_matrix_product_100x100);
criterion_main!(benches_add, bences_sub, benches_mul, benches_product);
