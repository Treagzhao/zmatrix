use zmatrix::dense;
use zmatrix::dense::Matrix;

fn main() {
    let m = Matrix::<3, 4, i32>::new([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    println!("{}", m);
    show_add();
    show_sub();
    show_mul();
    show_get();
    show_set();
    show_scale();
    show_matrix_product();
    show_transform();
}

fn show_add() {
    let m1 = Matrix::<3, 3, i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = Matrix::<3, 3, i32>::new([[4, 5, 6], [7, 8, 9], [10, 11, 12]]);
    let m3 = m1 + m2;
    println!("add result:\n{}", m3);
}

fn show_sub() {
    let m1 = Matrix::<3, 3, i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = Matrix::<3, 3, i32>::new([[4, 5, 6], [7, 8, 9], [10, 11, 12]]);
    let m3 = m2 - m1;
    println!("sub result:\n{}", m3);
}

fn show_mul() {
    let m1 = Matrix::<3, 3, i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = Matrix::<3, 3, i32>::new([[4, 5, 6], [7, 8, 9], [10, 11, 12]]);
    let m3 = m2 * m1;
    println!("multi result:\n{}", m3);
}

fn show_get() {
    let m1 = Matrix::<3, 3, i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let value = m1.get(1, 1);
    if let Some(x) = value {
        println!("get value:{}", x);
    }
}

fn show_matrix_product() {
    let m1 = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
    let m2 = Matrix::<3, 4, i32>::new([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    let result = m1.product(&m2).unwrap();
    println!("matrix product:\n{}", result);
}

fn show_transform() {
    let m1 = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
    let m2 = m1.T();
    println!("\ntransform:\n{}", m2);
}

fn show_set() {
    let mut m1 = Matrix::<2, 3, i32>::new([[1, 2, 3], [4, 5, 6]]);
    m1.set(1, 1, 12);
    let v = m1.get(1, 1);
    if let Some(x) = v {
        println!("set value:{}", x);
    }
}

fn show_scale() {
    let m1 = Matrix::<3, 3, i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = m1.scale(3);
    println!("\nscale result:\n{}", m2);
}
