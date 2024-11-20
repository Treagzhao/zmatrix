use std::ops::{Add, Mul, Sub};

pub struct ZMatrix<T>
where
    T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Display,
{
    height:u64,
    width:u64,
    vec:Vec<T>,
}