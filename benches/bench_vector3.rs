use criterion::{criterion_group, criterion_main};
use rand::Rng;
use zmatrix::physics::basic::{Coef, Vector3};

fn generate_test_vector3() -> Vector3<Coef> {
    let mut rng = rand::rng();
    let x = rng.random_range(-10.0..10.0);
    let y = rng.random_range(-10.0..10.0);
    let z = rng.random_range(-10.0..10.0);
    Vector3::from_array([x, y, z])
}

fn bench_vector_cross(c: &mut criterion::Criterion) {
    let cases: Vec<(Vector3<Coef>, Vector3<Coef>)> = (0..10)
        .map(|_| return (generate_test_vector3(), generate_test_vector3()))
        .collect();
    let mut group = c.benchmark_group("vector_cross");
    for (i, (v1, v2)) in cases.iter().enumerate() {
        group.bench_with_input(format!("case_{}", i), &(v1, v2), |b, (v1, v2)| {
            b.iter(|| v1.cross(v2))
        });
    }
}

fn bench_vector_cross_unit(c: &mut criterion::Criterion) {
    let cases: Vec<(Vector3<Coef>, Vector3<Coef>)> = (0..10)
        .map(|_| return (generate_test_vector3(), generate_test_vector3()))
        .collect();
    let mut group = c.benchmark_group("vector_cross_unit");
    for (i, (v1, v2)) in cases.iter().enumerate() {
        group.bench_with_input(format!("case_{}", i), &(v1, v2), |b, (v1, v2)| {
            b.iter(|| v1.cross_unit(v2))
        });
    }
}

fn bench_vector_dot(c: &mut criterion::Criterion) {
    let cases: Vec<(Vector3<Coef>, Vector3<Coef>)> = (0..10)
        .map(|_| return (generate_test_vector3(), generate_test_vector3()))
        .collect();
    let mut group = c.benchmark_group("vector_dot");
    for (i, (v1, v2)) in cases.iter().enumerate() {
        group.bench_with_input(format!("case_{}", i), &(v1, v2), |b, (v1, v2)| {
            b.iter(|| v1.dot(v2))
        });
    }
}

fn bench_vector_skew(c: &mut criterion::Criterion) {
    let cases: Vec<Vector3<Coef>> = (0..10)
        .map(|_| return generate_test_vector3())
        .collect();
    let mut group = c.benchmark_group("vector_skew_matrix");
    for (i,v) in cases.iter().enumerate() {
        group.bench_with_input(format!("case_{}", i), &v, |b, v| {
            b.iter(||v.skew_symmetric_matrix())
        });
    }
}

fn bench_vector_skew_4x4(c: &mut criterion::Criterion) {
    let cases: Vec<Vector3<Coef>> = (0..10)
        .map(|_| return generate_test_vector3())
        .collect();
    let mut group = c.benchmark_group("vector_skew_matrix_4x4");
    for (i,v) in cases.iter().enumerate() {
        group.bench_with_input(format!("case_{}", i), &v, |b, v| {
            b.iter(||v.skew_symmetric_matrix_4())
        });
    }
}

criterion_group!(benches, bench_vector_cross,bench_vector_cross_unit,bench_vector_dot,bench_vector_skew,bench_vector_skew_4x4);
criterion_main!(benches);
