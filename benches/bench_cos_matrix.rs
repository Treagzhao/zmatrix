use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use zmatrix::dense::Matrix;
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
            |b, (matrix, vector)| b.iter(|| matrix.product_vector(vector)),
        );
    }
}

fn bench_cos_matrix_from_matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_cos_matrix_from_matrix");
    for i in 0..5{
        let arr = random_3x3_array();
        let matrix = Matrix::<3,3,f64>::new(arr);
        group.bench_with_input(format!("bench_cos_matrix_from_matrix {}", i), &matrix, |b, m| {
            b.iter(|| {
                CosMatrix::from_matrix(m);
            });
        });
    }
    group.finish();
}

fn bench_cos_matrix_transfer(c: &mut Criterion) {
    let mut cases: Vec<([[f64; 3]; 3], [f64; 3])> = Vec::new();
    for i in 0..10 {
        let matrix = random_3x3_array();
        let vector = random_3d_array();
        cases.push((matrix, vector));
    }
    let mut group = c.benchmark_group("bench_cos_matrix_transfer");
    for (i, (matrix, vector)) in cases.into_iter().enumerate() {
        group.bench_with_input(
            format!("bench_cos_matrix_transfer {}", i),
            &(CosMatrix::new(matrix), Vector3::<Coef>::from_array(vector)),
            |b, (matrix, vector)| b.iter(|| matrix.transfer()),
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
    let mut cases: Vec<([[f64; 3]; 3], [[f64; 3]; 3])> = Vec::new();
    for i in 0..10 {
        let matrix = random_3x3_array();
        let matrix2 = random_3x3_array();
        cases.push((matrix, matrix2));
    }
    let mut group = c.benchmark_group("cos_matrix_add");
    for (i, (matrix1, matrix2)) in cases.into_iter().enumerate() {
        group.bench_with_input(
            format!("bench_cos_matrix_add {}", i),
            &(CosMatrix::new(matrix1), CosMatrix::new(matrix2)),
            |b, (cos1, cos2)| {
                b.iter(|| *cos1 + *cos2);
            },
        );
    }
}

fn bench_cos_matrix_multi_f64(c: &mut Criterion) {
    let mut cases: Vec<[[f64; 3]; 3]> = Vec::new();
    for i in 0..10 {
        let matrix = random_3x3_array();
        cases.push(matrix);
    }
    let mut group = c.benchmark_group("cos_matrix_multi_f64");
    for (i, matrix) in cases.into_iter().enumerate() {
        group.bench_with_input(
            format!("bench_multi {}", i),
            &CosMatrix::new(matrix),
            |b, cos| {
                b.iter(|| cos * 2.0);
            },
        );
    }
}

criterion_group!(
    benches,
    bench_cos_matrix_product_vector,
    bench_cos_matrix_product,
    bench_cos_matrix_add,
    bench_cos_matrix_transfer,
    bench_cos_matrix_multi_f64,
    bench_cos_matrix_from_matrix,
);
criterion_main!(benches);
