use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use zmatrix::spatial_geometry::quaternion::Quaternion;

fn create_random_quaternion() -> Quaternion {
    let mut rng = rand::rng();
    let x = rng.random_range(-1.0..1.0);
    let y = rng.random_range(-1.0..1.0);
    let z = rng.random_range(-1.0..1.0);
    let w = rng.random_range(-1.0..1.0);
    let q = Quaternion::new(w, x, y, z);
    q.normalize()
}


fn bench_quaternion_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("quaternion_add");
    let mut cases: Vec<(Quaternion, Quaternion)> = Vec::new();
    for i in 0..10 {
        let q1 = create_random_quaternion();
        let q2 = create_random_quaternion();
        cases.push((q1, q2));
    }
    for (i,(q1,q2)) in cases.iter().enumerate() {
        group.bench_with_input(format!("case_{}", i), &(q1,q2), |b, (q1,q2)| {
            b.iter(|| *q1 + *q2 )
        });
    }
}

fn bench_quaternion_multi(c: &mut Criterion) {
    let mut group = c.benchmark_group("quaternion_multi");
    let mut cases: Vec<(Quaternion, Quaternion)> = Vec::new();
    for i in 0..10 {
        let q1 = create_random_quaternion();
        let q2 = create_random_quaternion();
        cases.push((q1, q2));
    }
    for (i,(q1,q2)) in cases.iter().enumerate() {
        group.bench_with_input(format!("case_{}", i), &(q1,q2), |b, (q1,q2)| {
            b.iter(|| *q1 * *q2 )
        });
    }
}

fn bench_quaternion_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("quaternion_div");
    let mut cases: Vec<(Quaternion, Quaternion)> = Vec::new();
    for i in 0..10 {
        let q1 = create_random_quaternion();
        let q2 = create_random_quaternion();
        cases.push((q1, q2));
    }
    for (i,(q1,q2)) in cases.iter().enumerate() {
        group.bench_with_input(format!("case_{}", i), &(q1,q2), |b, (q1,q2)| {
            b.iter(|| *q1 / *q2 )
        });
    }
}

fn bench_quaternion_to_cos(c: &mut Criterion) {
    let mut group = c.benchmark_group("quaternion_to_cos_matrix");
    let mut cases: Vec<Quaternion> = Vec::new();
    for i in 0..10 {
        let q1 = create_random_quaternion();
        cases.push(q1);
    }
    for (i,q1) in cases.iter().enumerate() {
        group.bench_with_input(format!("case_{}", i), &q1, |b, q1| {
            b.iter(|| q1.to_cos_matrix() )
        });
    }
}



criterion_group!(benches,bench_quaternion_add,bench_quaternion_multi,bench_quaternion_div,bench_quaternion_to_cos);
criterion_main!(benches);