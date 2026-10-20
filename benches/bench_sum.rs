use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use zmatrix::dense::Matrix;

// `Matrix<R, C, T>` 的尺寸是编译期常量，无法像旧版那样在运行时循环遍历尺寸，
// 因此每个尺寸需要一个独立的泛型实例化。
//
// 注意：`Matrix` 的数据是内联数组 `[[T; COLS]; ROWS]`，1000x1000 的 f64 约占
// 8 MB 栈空间，会超过默认线程栈上限，故此处只压到 100x100。
fn bench_sum_row<const R: usize, const C: usize>(c: &mut Criterion, label: &str) {
    let m = Matrix::<R, C, f64>::random();

    let mut group = c.benchmark_group("Matrix Sum Row");
    group.bench_with_input(label, &m, |b, matrix| {
        b.iter(|| {
            black_box(matrix.sum_row());
        })
    });
    group.finish();
}

fn bench_sum_column<const R: usize, const C: usize>(c: &mut Criterion, label: &str) {
    let m = Matrix::<R, C, f64>::random();

    let mut group = c.benchmark_group("Matrix Sum Column");
    group.bench_with_input(label, &m, |b, matrix| {
        b.iter(|| {
            black_box(matrix.sum_column());
        })
    });
    group.finish();
}

fn benches(c: &mut Criterion) {
    bench_sum_row::<10, 10>(c, "10x10");
    bench_sum_row::<100, 100>(c, "100x100");

    bench_sum_column::<10, 10>(c, "10x10");
    bench_sum_column::<100, 100>(c, "100x100");
}

criterion_group!(all, benches);
criterion_main!(all);
