use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use zmatrix::physics::basic::{Coef, Vector3};
use zmatrix::spatial_geometry::cos_matrix::CosMatrix;

pub fn random_3x3_array() -> [[f64; 3]; 3] {
    let mut rng = rand::rng();
    [
        [rng.random(), rng.random(), rng.random()],
        [rng.random(), rng.random(), rng.random()],
        [rng.random(), rng.random(), rng.random()],
    ]
}

pub fn random_3d_array() -> [f64; 3] {
    let mut rng = rand::rng();
    [rng.random(), rng.random(), rng.random()]
}

fn bench_cos_matrix_product_vector(c: &mut Criterion) {
    let mut cases: Vec<([[f64; 3]; 3], [f64; 3])> = Vec::new();
    for i in 0..10 {
        let matrix = random_3x3_array();
        let vector = random_3d_array();
        cases.push((matrix, vector));
    }
    let mut group = c.benchmark_group("bench_cos_matrix_product_vector");
    for (i, (matrix, vector)) in cases.into_iter().enumerate() {
        group.bench_with_input(
            format!("bench_cos_matrix_product_vector {}", i),
            &(CosMatrix::new(matrix), Vector3::<Coef>::from_array(vector)),
            |b, (matrix, vector)| {
                b.iter(|| matrix.product_vector(vector))
            },
        );
    }
}

criterion_group!(benches, bench_cos_matrix_product_vector);
criterion_main!(benches);
