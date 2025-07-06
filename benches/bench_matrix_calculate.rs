use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use zmatrix::dense::Matrix;

fn setup_same_size_matrix(rows: usize, cols: usize) -> (Matrix<f64>, Matrix<f64>) {
    (
        Matrix::random(rows, cols).unwrap(),
        Matrix::random(rows, cols).unwrap(),
    )
}

fn bench_matrix_add(c: &mut Criterion) {
    let sizes = [(10, 10), (100, 100), (1000, 1000)]; // 测试不同矩阵大小

    let mut group = c.benchmark_group("Matrix plus matrix");
    for (rows, cols) in sizes.iter() {
        let (a, b) = setup_same_size_matrix(*rows, *cols);

        group.bench_with_input(
            format!("{}x{}", rows, cols),
            &(a, b),
            |b, (f, t)| {
                b.iter_batched(
                    || (f.clone(), t.clone()),  // 每次迭代前克隆
                    |(f_clone, t_clone)| f_clone + t_clone,
                    criterion::BatchSize::SmallInput
                )
            },
        );
    }
}


fn bench_matrix_product(c: &mut Criterion) {
    let sizes = [(10, 10), (100, 100), (1000, 1000)]; // 测试不同矩阵大小
    let mut group = c.benchmark_group("Matrix product matrix");
    for (rows, cols) in sizes.iter() {
        let (a, b) = setup_same_size_matrix(*rows, *cols);

        group.bench_with_input(
            format!("{}x{}", rows, cols),
            &(a, b),
            |b, (f, t)| {
                b.iter_batched(
                    || (f.clone(), t.clone()),  // 每次迭代前克隆
                    |(f_clone, t_clone)| f_clone.product(&t_clone),
                    criterion::BatchSize::SmallInput
                )
            },
        );
    }
}

criterion_group!(benches, bench_matrix_add,bench_matrix_product);
criterion_main!(benches);