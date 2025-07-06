use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use zmatrix::dense::Matrix; // 替换为你的实际 crate 名

// 1. 准备测试数据
fn setup_matrix(rows: usize, cols: usize) -> Matrix<f64> {
    Matrix::random(rows, cols).unwrap()
}

// 2. 定义基准测试函数
fn bench_sum_row(c: &mut Criterion) {
    let sizes = [(10, 10), (100, 100), (1000, 1000)]; // 测试不同矩阵大小

    let mut group = c.benchmark_group("Matrix Sum Row");
    for (rows, cols) in sizes.iter() {
        let m = setup_matrix(*rows, *cols);

        group.bench_with_input(
            format!("{}x{}", rows, cols), // 测试名称
            &m,                           // 输入数据
            |b, matrix| {
                b.iter(|| {
                    // 3. 执行要测试的函数
                    black_box(matrix.sum_row());
                })
            },
        );
    }
    group.finish();
}

fn bench_sum_column(c: &mut Criterion) {
    let sizes = [(10, 10), (100, 100), (1000, 1000)]; // 测试不同矩阵大小

    let mut group = c.benchmark_group("Matrix Sum Column");
    for (rows, cols) in sizes.iter() {
        let m = setup_matrix(*rows, *cols);

        group.bench_with_input(
            format!("{}x{}", rows, cols), // 测试名称
            &m,                           // 输入数据
            |b, matrix| {
                b.iter(|| {
                    // 3. 执行要测试的函数
                    black_box(matrix.sum_column());
                })
            },
        );
    }
    group.finish();
}


// 4. 注册基准测试组
criterion_group!(benches, bench_sum_row,bench_sum_column);
criterion_main!(benches);
