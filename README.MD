# 这是一个矩阵运算的库，能够实现一些简单的矩阵预算的功能
![Codecov](https://codecov.io/gh/Treagzhao/zmatrix/branch/main/graph/badge.svg)

#### 创建一个矩阵
 create a new matrix
``` rust
let m1 = Matrix::new(2,3,vec![1,2,3,4,5,6]);
println!("{}",m1);
```

#### 获取值以及设置值
 get value & set value
``` rust
  let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
  let value = m1.get(1, 1);
  if let Some(x) = value {
     println!("get value:{}", x); // 5
  }
  
  m1.set(1,1,10);
  let value = m1.get(1, 1);
  if let Some(x) = value {
     println!("get value:{}", x); // 10
  }
```
#### 矩阵加减乘的运算
 Matrix addition, subtraction, and multiplication operations
``` rust
    // 加法运算 add
    let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    let m2 = Matrix::new(3, 3, vec![4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
    let m3 = m1 + m2;
    println!("add result:\n{}", m3);
    //减法运算 sub
     let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    let m2 = Matrix::new(3, 3, vec![4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
    let m3 = m2 - m1;
    println!("sub result:\n{}", m3);
    //乘法运算 multiple
    let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    let m2 = Matrix::new(3, 3, vec![4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
    let m3 = m2 * m1;
    println!("multi result:\n{}", m3);
```
#### 矩阵乘法
 Matrix product
``` rust
    let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
    let m2 = Matrix::new(3, 4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
    let result = m1.product(m2).unwrap();
    println!("matrix product:\n{}", result);
```

#### 矩阵的数乘
 Scalar multiplication of a matrix
``` rust
    let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    let m2 = m1.scale(3);
    println!("\nscale result:\n{}", m2);
```

#### 矩阵的转置
 Matrix transpose
``` rust
    let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
    let m2 = !m1;
    println!("\ntransform:\n{}", m2);
```

#### 矩阵的行列式
 matrix determinant
``` rust
    let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
    let m2 = !m1;
    println!("\ntransform:\n{}", m2);
```

# 物理单位及运算库
解决一部分物理量运算时的单位换算、物理量转换和量纲对齐的问题。

可以让代码向物理公式对齐，而不需要关心具体的单位转换细节。

Resolves unit conversion, physical quantity transformation, and dimensional alignment in physical calculations.

Allows code to align with physical formulas without worrying about unit conversion details.

## 单位对齐
``` rust
fn main(){
    let distance = Distance::from_m(1001.0);
    println!("{:?}", distance.as_km());
    // 1.001 km
    println!("{:?}", distance.as_light_year());
    // 3.262606876811594e-11 light years

    let velocity = Velocity::from_m_per_sec(100.0);
    println!("{:?}", velocity.as_km_per_h());
    // 360.0 km/h
} 
```
以长度单位为例，可以通过from_m() 方法生成一个以米为单位的物理量，然后使用as_km() 方法将其转换为以千米为单位的物理量。

也可以用 as_light_year() 方法将其转换为光年的倍数。

Initialize with from_m(), from_km(), etc.

Convert flexibly via as_\[unit\]() methods.

目前支持的物理量模块：
supported physical quantities:

| 物理量 | 模块名                 |
|----|-----------------------|
| 长度 | `distance`            |
| 速度 | `velocity`            |
| 加速度 | `acceleration`        |
| 角度 | `angular`             |
| 角速度 | `angular_velocity`    |
| 系数 | `coef`                |
| 角加速度 | `angular_acceleration`|
| 面积 | `area`                |
以后会慢慢维护，也欢迎大家提issue和pr。

## 物理量的计算
``` rust
fn calculate() {
    let distance = Distance::from_m(1000.0);
    let time = Duration::from_secs(10);
    let velocity:Velocity = distance / time;
    let acc:Acceleration = velocity / time;

    let ang:Angular = Angular::from_deg(180.0);
    let time =  Duration::from_secs(10);
    let omg:AngularVelocity = ang / time;
    let angular_accel:AngularAcceleration = omg / time;

    let dis:Distance = Distance::from_m(1000.0);
    let area:Area = dis * dis;
    
}
```
符合物理计算关系的物理量之间可以进行加减乘除运算，得到符合物理意义的物理量。

例如距离除以时间得到速度，速度除以时间得到加速度。

一旦两个物理量的量纲不匹配，就会编译报错。避免代码写错导致的bug。

Physical quantities with compatible dimensions can be safely added, subtracted, multiplied, or divided while preserving physical meaning.

Examples:
```
    Distance ÷ Time → Velocity
    
    Velocity ÷ Time → Acceleration
``` 

Compile-Time Safety:

Operations with dimensionally incompatible quantities will trigger compile errors, preventing invalid physics logic at the code level.