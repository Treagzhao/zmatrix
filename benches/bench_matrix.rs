use criterion::{criterion_group, criterion_main};
use rand::Rng;
use zmatrix::dense::Matrix;

pub fn random_9_array() -> Vec<f64> {
    let mut rng = rand::rng();
    (0..9).map(|_|{
        rng.random_range(-10.0..10.0)
    }).collect()
}

fn bench_matrix_new(c: &mut criterion::Criterion) {
    let mut cases:Vec<Vec<f64>> = Vec::new();
    for i in 0..5{
        cases.push(random_9_array());
    }
    let mut group = c.benchmark_group("Matrix::new");
    for (i,case) in cases.iter().enumerate() {
        group.bench_with_input(format!("matrix new case {}",i), case, |b, case| {
            b.iter(|| Matrix::new(3,3,case.clone()));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_matrix_new);
criterion_main!(benches);