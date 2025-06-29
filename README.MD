# 这是一个矩阵运算的库，能够实现一些简单的矩阵预算的功能
![Codecov](https://codecov.io/gh/Treagzhao/zmatrix/branch/master/graph/badge.svg)

#### 创建一个矩阵
 create a new matrix
``` rust
let m1 = Matrix::new(2,3,vec![1,2,3,4,5,6]);
println!("{}",m1);
```

生成一个全0/全1矩阵
 create a new matrix with all 0
``` rust
    let m1 = Matrix::zeros(3, 3);
    println!("{}", m1);   
    
    let m1 = Matrix::ones(3, 3);
    println!("{}", m1);   
 ```

生成一个随机矩阵(只有f64类型)
 create a new matrix with random values
``` rust
    let m1 = Matrix::random(3, 3);
    println!("{}", m1);  
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

#### 矩阵的操作
 矩阵转置 Matrix transpose
``` rust
    let m1 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
    let m2 = !m1; 
    println!("\ntransform:\n{}", m2);
    
    let m2 = m1.T();
    println!("\ntransform:\n{}", m2);   
```
矩阵的水平拼接和垂直拼接
 Matrix horizontal concatenation
``` rust
     let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
     let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
     println!("\nconcat:\n{}", m.hstack(m1));
     
     let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
     let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
     println!("\nconcat:\n{}", m.vstack(m1));    
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
| 磁感应强度 | `magnetic_induction` |
| 质量 | `mass`                |
| 角动量 | `angular_momentum`    |
| 动量 | `momentum`            |
| 体积 | `volume`              |
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

# 向量计算
## 各种物理量向量之间的转换

``` rust
    //位移向量除以时间变成速度向量
    //Displacement vector divided by time yields velocity vector. 
    let a:Vector3<Distance> = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    );
    let v = a / Duration::from_secs(10);
    println!("{:?}", a);

    //速度向量乘以质量，变成动量向量
    // Velocity vector multiplied by mass yields momentum vector. 
    let m = v * Mass::from_kg(1.0);
    println!("{:?}", m);
    // 动量向量，乘以半径向量，变成角动量向量。
    // Momentum vector crossed with radius vector yields angular momentum vector. 
    let am =   a * m ;
    println!("{:?}", am); 
```
各种物理量的向量彼此之间的加减乘除运算，都可以直接通过数学符号实现。

目前支持有限，以后会慢慢补充，欢迎 issue 和 PR。

Vector operations (add/subtract/multiply/divide) for all physical quantities can be directly performed using mathematical symbols.

Current support is limited - more features will be added gradually. Issues and PRs are welcome!

## 向量的常见方法
向量是否是 0 值

``` rust
    let v = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    );
    println!("{}",v.is_zero());
```

将向量转换成数组

``` rust
    let v = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    );
    println!("{:?}",v.to_array()); 
```

通过数组生成向量 & 通过列矩阵生成向量
``` rust
    let v = Vector3::from_array([1.0,2.0,3.0]);
    println!("{:?}",v);
    let v = Vector3::from_matrix(Matrix::new(3,1,vec![1.0,2.0,3.0]).unwrap());
    println!("{:?}",v); 
```

生成反对称矩阵
``` rust
    let v = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    );
    // 3 * 3 反对称矩阵 
    println!("{:?}",v.skew_symmetric_matrix());

    let v = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    );
    //  4 * 4 反对称矩阵
    println!("{:?}",v.skew_symmetric_matrix_4());
```

向量叉乘
``` rust
    let v1 = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    ); 
    let v2 = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    ); 
    // 叉乘的垂直向量 
    let v = v1.cross(v2);
    println!("{:?}",v);
    let v1 = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    );   
    let v2 = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    );
    //叉乘的垂直单位向量
    let v_unit = v1.cross_unit(v2);  
    println!("{:?}",v);  
```

点积

``` rust
    let v1 = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    );
    let v2 = Vector3::new(
        Distance::from_m(10.0),
        Distance::from_m(20.0),
        Distance::from_m(30.0),
    ); 
    let r = v1.dot(v2);
    println!("{:?}",r);  
```
# 空间几何的一些运算库
## 欧拉角
转换成四元数
``` rust
   let euler:Vector3<Angular> = Vector3::new(
        Angular::from_deg(10.0),
        Angular::from_deg(20.0),
        Angular::from_deg(30.0),
    );
    let quat = euler.to_quaternion();
    println!("{:?}",quat); 
``` 

基本的三角函数（待补充完善)
``` rust
   let euler:Vector3<Angular> = Vector3::new(
        Angular::from_deg(10.0),
        Angular::from_deg(20.0),
        Angular::from_deg(30.0),
    );
    let sin = euler.sin();
    let cos = euler.cos();
    println!("sin:{:?} cos:{:?}",sin,cos);
 ```


## 余弦转换矩阵
获取向量
``` rust 
// 获取单位余弦矩阵 
let cos = CosMatrix::unit();
let x = cos.get_x_vector();
let y = cos.get_y_vector();
let z = cos.get_z_vector();
// 获取 x y z 行向量 
println!("x:{:?} y:{:?} z:{:?}",x,y,z);

let x_col = cos.get_col_x_vector();
let y_col = cos.get_col_y_vector();
let z_col = cos.get_col_z_vector();
//获取三个列向量
println!("x:{:?} y:{:?} z:{:?}",x_col,y_col,z_col);          
 ```
转置
``` rust
let cos = CosMatrix::unit(); 
let cos_t = cos.transfer();
println!("{:?}",cos_t);  
 ```
转换到四元数
``` rust
let cos = CosMatrix::unit();
let q = cos.to_quaternion();
println!("{:?}",q);   
  ```
矩阵乘法
``` rust
    let cos_a = CosMatrix::unit();
    let cos_b = CosMatrix::new([[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [5.0, 2.0, 1.0]]);
    // 两个余弦转换矩阵相乘 
    let cos_c = cos_a.product(cos_b);
    // 余弦转换矩阵乘以列向量
    cos_c.product_vector(Vector3::new(1.0, 2.0, 3.0));    
  ```
转换到欧拉角
``` rust
    let cos = CosMatrix::unit();
    let euler = cos.to_pry();
    // xzy 转序 
    println!("{:?}",euler);
    // xyz 转序
    let euler = cos.to_rpy();
    println!("{:?}",euler);
 ```
## 四元数
初始化四元数
``` rust
    let q = Quaternion::new(1.0, 2.0, 3.0, 4.0); 
    println!("{:?}",q);
    // 单位四元数
    let q = Quaternion::default();
    println!("{:?}",q);    
 ```
归一化
``` rust
let q = Quaternion::new(1.0, 2.0, 3.0, 4.0); 
let q1 = q.normalize();
println!("{:?}",q1);  
 ```
模长
``` rust
 let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
 println!("{:?}",q.norm()); 
 ```
共轭四元数
``` rust
let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
let q1 = q.conjugate();
println!("{:?}",q1);   
 ```
四元数的逆
``` rust
let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
let q1 = q.inverse();
println!("{:?}",q1);   
 ```
转换到余弦转换矩阵
``` rust
let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
let cos = q.to_cos_matrix();
println!("{:?}",cos);   
 ```
执行线性变换
``` rust
    let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]).unwrap();
    let q1 = q.linear_transform(m);  
 ```
四元数计算
``` rust
 let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
 let q2 = Quaternion::new(2.0, 3.0, 4.0, 5.0);
 let q_sum = q1 + q2;
 println!("{:?}",q_sum);
 
 let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
 let q2 = Quaternion::new(2.0, 3.0, 4.0, 5.0);
 let q_mul = q1 * q2 ;
 println!("{:?}",q_mul);
 
 let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
 let q2 = Quaternion::new(2.0, 3.0, 4.0, 5.0);
 let q_div = q1 / q2 ;
 println!("{:?}",q_div);             
  ```

