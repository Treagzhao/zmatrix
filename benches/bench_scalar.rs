use criterion::{criterion_group, criterion_main, Criterion};
use zmatrix::dense::Matrix;

// `Matrix<R, C, T>` 的尺寸是编译期常量，无法在运行时遍历尺寸，
// 因此每个尺寸需要一个独立的泛型实例化。
//
// 注意：数据内联存储在栈上，1000x1000 的 f64 约占 8 MB，
// 会超过默认线程栈上限，故此处只压到 100x100。
fn bench_add_scalar<const R: usize, const C: usize>(c: &mut Criterion, label: &str) {
    let matrix = Matrix::<R, C, f64>::random();

    let mut group = c.benchmark_group("Matrix Add Scalar");
    group.bench_with_input(label, &matrix, |b, matrix| {
        b.iter_batched(
            || *matrix, // Matrix 是 Copy，无需 clone
            |m| m + 1.0,
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn benches(c: &mut Criterion) {
    bench_add_scalar::<10, 10>(c, "10x10");
    bench_add_scalar::<100, 100>(c, "100x100");
}

criterion_group!(all, benches);
criterion_main!(all);
