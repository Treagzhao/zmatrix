use criterion::{criterion_group, criterion_main};
use rand::Rng;
use zmatrix::dense::Matrix;

pub fn random_3x3_array() -> [[f64; 3]; 3] {
    let mut rng = rand::rng();
    [
        [rng.random(), rng.random(), rng.random()],
        [rng.random(), rng.random(), rng.random()],
        [rng.random(), rng.random(), rng.random()],
    ]
}

fn bench_matrix_new(c: &mut criterion::Criterion) {
    let mut cases:Vec<[[f64;3];3]> = Vec::new();
    for i in 0..5{
        cases.push(random_3x3_array());
    }
    let mut group = c.benchmark_group("Matrix::new");
    for (i,case) in cases.iter().enumerate() {
        group.bench_with_input(format!("matrix new case {}",i), case, |b, case| {
            b.iter(|| Matrix::<3,3,f64>::new(case.clone()));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_matrix_new);
criterion_main!(benches);