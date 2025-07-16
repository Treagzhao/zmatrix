use criterion::{criterion_group, criterion_main};
use std::hint::black_box;
use zmatrix::dense::Matrix;

fn init_material(rows: usize, col: usize) -> Matrix<f64> {
    Matrix::random(rows, col).unwrap()
}

fn bench_addition(c: &mut criterion::Criterion) {
    let sizes = [(10, 10), (100, 100), (1000, 1000)]; // 测试不同矩阵大小

    let mut group = c.benchmark_group("Matrix Add Scalar");
    for (rows, cols) in sizes {
        let matrix = init_material(rows, cols);

        group.bench_with_input(
            format!("{}x{}", rows, cols), // 测试名称
            &matrix,                      // 输入数据
            |b, matrix| {
                b.iter_batched(
                    || matrix.clone(), // 每次迭代前克隆
                    |m| m + 1.0,
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_addition);
criterion_main!(benches);
