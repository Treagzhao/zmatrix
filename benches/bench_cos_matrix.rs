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


pub fn random_f64_array_9() -> [f64; 9] {
    let mut rng = rand::thread_rng();
    let mut arr = [0.0; 9];
    for item in &mut arr {
        *item = rng.random();
    }
    arr
}
fn bench_cos_matrix_product(c: &mut Criterion) {
    let mut group = c.benchmark_group("cos_matrix_product");
    let arr = random_f64_array_9();
    let cos1 = CosMatrix::new([
        [arr[0], arr[1], arr[2]],
        [arr[3], arr[4], arr[5]],
        [arr[6], arr[7], arr[8]],
    ]);
    let arr = random_f64_array_9();
    let cos2 = CosMatrix::new([
        [arr[0], arr[1], arr[2]],
        [arr[3], arr[4], arr[5]],
        [arr[6], arr[7], arr[8]],
    ]);
    group.bench_with_input("cos matrix product", &(cos1, cos2), |b, (cos1, cos2)| {
        b.iter(|| cos1.product(cos2));
    });
}

fn bench_cos_matrix_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("cos_matrix_add");
    let arr = random_f64_array_9();
    let cod = CosMatrix::new([
        [arr[0], arr[1], arr[2]],
        [arr[3], arr[4], arr[5]],
        [arr[6], arr[7], arr[8]],
    ]);
}

criterion_group!(benches,bench_cos_matrix_product_vector, bench_cos_matrix_product);
criterion_main!(benches);
