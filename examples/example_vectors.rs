use std::time::Duration;
use zmatrix::physics::basic::{Distance, Mass, Vector3};

fn main() {
    //位移向量除以时间变成速度向量
    let a:Vector3<Distance> = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    );
    let v = a / Duration::from_secs(10);
    println!("{:?}", a);

    //速度向量乘以质量，变成动量向量
    let m = v * Mass::from_kg(1.0);
    println!("{:?}", m);
    // 动量向量，乘以半径向量，变成角动量向量。
    let am =   a * m ;
    println!("{:?}", am);


}