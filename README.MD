# 这是一个矩阵运算的库，能够实现一些简单的矩阵预算的功能
![Codecov](https://codecov.io/gh/Treagzhao/zmatrix/branch/master/graph/badge.svg)

#### 创建一个矩阵
 create a new matrix
``` rust
let m1 = Matrix::<2,3,i32>::new([[1,2,3],[4,5,6]]);
println!("{}",m1);
```

生成一个全0/全1矩阵
 create a new matrix with all 0
``` rust
    let m1 = Matrix::<3,3,i32>::zeros();
    println!("{}", m1);   
    
    let m1 = Matrix::<3,3,i32>::ones();
    println!("{}", m1);   
 ```

生成一个随机矩阵(只有f64类型)
 create a new matrix with random values
``` rust
    let m1 = Matrix::<3,3,f64>::random();
    println!("{}", m1);  
 ```



#### 获取值以及设置值
 get value & set value
``` rust
  let m1 = Matrix::<3,3,i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
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
    let m1 = Matrix::<3,3,i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = Matrix::<3,3,i32>::new([[4, 5, 6], [7, 8, 9], [10, 11, 12]]);
    let m3 = m1 + m2;
    println!("add result:\n{}", m3);
    //减法运算 sub
     let m1 = Matrix::<3,3,i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = Matrix::<3,3,i32>::new([[4, 5, 6], [7, 8, 9], [10, 11, 12]]);
    let m3 = m2 - m1;
    println!("sub result:\n{}", m3);
    //乘法运算 multiple
    let m1 = Matrix::<3,3,i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = Matrix::<3,3,i32>::new([[4, 5, 6], [7, 8, 9], [10, 11, 12]]);
    let m3 = m2 * m1;
    println!("multi result:\n{}", m3);
```
#### 矩阵乘法
 Matrix product
``` rust
    let m1 = Matrix::<2,3,i32>::new([[1, 2, 3], [4, 5, 6]]);
    let m2 = Matrix::<3,4,i32>::new([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    let result = m1.product(m2).unwrap();
    println!("matrix product:\n{}", result);
```

#### 矩阵的数乘
 Scalar multiplication of a matrix
``` rust
    let m1 = Matrix::<3,3,i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = m1.scale(3);
    println!("\nscale result:\n{}", m2);
```

#### 矩阵的操作
 矩阵转置 Matrix transpose
``` rust
    let m1 = Matrix::<2,3,i32>::new([[1, 2, 3], [4, 5, 6]]);
    let m2 = !m1; 
    println!("\ntransform:\n{}", m2);
    
    let m2 = m1.T();
    println!("\ntransform:\n{}", m2);   
```


#### 矩阵的行列式
 matrix determinant
``` rust
    let m1 = Matrix::<2,3,i32>::new([[1, 2, 3], [4, 5, 6]]);
    let m2 = m1.det();
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

| 物理量 | 模块名                 | 支持的单位 |
|----|-----------------------|------------|
| 长度 | `distance`            | 米(m)、千米(km)、光年(light_year) |
| 速度 | `velocity`            | 米/秒(m/s)、千米/小时(km/h)、光速(c) |
| 加速度 | `acceleration`        | 米/秒²(m/s²)、重力加速度(g) |
| 角度 | `angular`             | 弧度(rad)、度(deg) |
| 角速度 | `angular_velocity`    | 弧度/秒(rad/s)、度/秒(deg/s)、弧度/小时(rad/h)、度/小时(deg/h) |
| 系数 | `coef`                | 无量纲系数 |
| 角加速度 | `angular_acceleration`| 弧度/秒²(rad/s²)、度/秒²(deg/s²) |
| 面积 | `area`                | 平方米(m²)、平方千米(km²) |
| 磁感应强度 | `magnetic_induction` | 特斯拉(T)、高斯(G)、毫特斯拉(mT)、微特斯拉(μT)、纳特斯拉(nT) |
| 质量 | `mass`                | 千克(kg)、克(g) |
| 角动量 | `angular_momentum`    | 千克·米²/秒(kg·m²/s)、千克·千米²/秒(kg·km²/s)、牛顿·米·秒(N·m·s)、毫牛顿·米·秒(mN·m·s)、微牛顿·米·秒(μN·m·s)、纳牛顿·米·秒(nN·m·s) |
| 动量 | `momentum`            | 千克·米/秒(kg·m/s)、千克·千米/秒(kg·km/s) |
| 体积 | `volume`              | 立方米(m³)、立方千米(km³) |
| 磁矩 | `magnetic_moment`     | 安培·平方米(A·m²)、焦耳/特斯拉(J/T)及其毫、微、纳单位 |
| 力矩 | `torque`              | 牛顿·米(N·m)及其毫、微、纳、千、兆单位 |
| 力 | `force`                | 牛顿(N)及其毫、微、纳、千、兆单位 |
| 功率 | `power`               | 瓦特(W)、马力(hp)及其毫、微、纳、千、兆单位 |
| 能量 | `energy`              | 焦耳(J)、电子伏特(eV)及其毫、微、纳、千、兆单位 |
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

## 物理量之间的运算关系

库支持符合物理意义的运算关系，所有运算都满足量纲一致性，编译时进行类型检查。

### 支持的运算关系表

| 运算 | 物理量A | 运算符 | 物理量B | 结果 | 物理意义 |
|------|---------|--------|---------|------|----------|
| 距离 | ÷ | 时间 | = | 速度 | 位移除以时间得到速度 |
| 速度 | ÷ | 时间 | = | 加速度 | 速度变化率 |
| 角度 | ÷ | 时间 | = | 角速度 | 角位移变化率 |
| 角速度 | ÷ | 时间 | = | 角加速度 | 角速度变化率 |
| 距离 | × | 距离 | = | 面积 | 长度相乘得到面积 |
| 距离 | × | 面积 | = | 体积 | 长度与面积相乘得到体积 |
| 质量 | × | 速度 | = | 动量 | 质量与速度相乘得到动量 |
| 距离 | × | 动量 | = | 角动量 | 距离与动量叉乘得到角动量 |
| 角动量 | × | 角速度 | = | 力矩 | 角动量与角速度相乘得到力矩 |
| 角速度 | × | 角动量 | = | 力矩 | 满足交换律 |
| 力矩 | ÷ | 角速度 | = | 角动量 | 力矩除以角速度得到角动量 |
| 角动量 | ÷ | 角速度 | = | 转动惯量(系数) | 角动量除以角速度得到转动惯量系数 |
| 磁矩 | × | 磁感应强度 | = | 能量 | 磁矩与磁感应强度相乘得到能量 |
| 力 | × | 距离 | = | 能量 | 力与距离相乘得到能量 |
| 距离 | × | 力 | = | 能量 | 满足交换律 |
| 质量 | × | 加速度 | = | 力 | 质量与加速度相乘得到力 |
| 加速度 | × | 质量 | = | 力 | 满足交换律 |
| 功率 | × | 时间 | = | 能量 | 功率与时间相乘得到能量 |
| 时间 | × | 功率 | = | 能量 | 满足交换律 |
| 力 | × | 速度 | = | 功率 | 力与速度相乘得到功率 |
| 速度 | × | 力 | = | 功率 | 满足交换律 |

### 运算示例

```rust
// 基础运动学运算
let distance = Distance::from_m(1000.0);
let time = Duration::from_secs(10);
let velocity: Velocity = distance / time;
let acceleration: Acceleration = velocity / time;

// 旋转运动学运算
let angular = Angular::from_deg(180.0);
let angular_velocity: AngularVelocity = angular / time;
let angular_acceleration: AngularAcceleration = angular_velocity / time;

// 几何运算
let dis: Distance = Distance::from_m(10.0);
let area: Area = dis * dis; // 10m × 10m = 100m²
let volume: Volume = dis * area; // 10m × 100m² = 1000m³

// 动力学运算
let mass = Mass::from_kg(2.0);
let momentum: Momentum = mass * velocity;
let angular_momentum: AngularMomentum = dis * momentum; // 距离与动量叉乘得到角动量

// 旋转动力学运算
let angular_momentum = AngularMomentum::from_kg_m2_per_second(10.0);
let angular_velocity = AngularVelocity::from_rad_per_second(5.0);
let torque: Torque = angular_momentum * angular_velocity; // 50 N·m

// 满足交换律的运算
let torque2: Torque = angular_velocity * angular_momentum; // 同样是 50 N·m

// 逆运算
let angular_momentum_back: AngularMomentum = torque / angular_velocity; // 10 kg·m²/s
let moment_of_inertia: Coef = angular_momentum / angular_velocity; // 2 kg·m²

// 磁学运算
let magnetic_moment = MagneticMoment::from_am2(1.0); // 1 A·m²
let magnetic_induction = MagneticInduction::from_tesla(2.0); // 2 T
let energy: Energy = magnetic_moment * magnetic_induction; // 2 J

// 力学运算
let force = Force::from_newton(10.0); // 10 N
let distance = Distance::from_m(5.0);  // 5 m
let work: Energy = force * distance;   // 50 J

let mass = Mass::from_kg(2.0);                    // 2 kg
let acceleration = Acceleration::from_m_per_s2(5.0); // 5 m/s²
let force_from_ma: Force = mass * acceleration;   // 10 N

// 功率运算
let power = Power::from_watt(100.0); // 100 W
let time = Duration::from_secs(5); // 5 s
let energy_from_power: Energy = power * time; // 500 J

let force = Force::from_newton(10.0); // 10 N
let velocity = Velocity::from_m_per_sec(5.0); // 5 m/s
let power_from_force: Power = force * velocity; // 50 W
```

### 角动量单位系统

角动量支持多种单位，满足不同精度和领域的计算需求：

```rust
// 传统单位
let l1 = AngularMomentum::from_kg_m2_per_second(100.0); // 100 kg·m²/s
let l2 = AngularMomentum::from_kg_km2_per_second(1.0);  // 1 kg·km²/s

// N·m·s系列单位（与kg·m²/s数值相等）
let l3 = AngularMomentum::from_nms(100.0);              // 100 N·m·s
let l4 = AngularMomentum::from_mill_nms(1000.0);        // 1000 mN·m·s
let l5 = AngularMomentum::from_micro_nms(1000000.0);    // 1000000 μN·m·s
let l6 = AngularMomentum::from_nano_nms(1000000000.0);  // 1000000000 nN·m·s

// 单位转换验证
println!("100 N·m·s = {} kg·m²/s", l3.as_kg_m2_per_second()); // 100
println!("1000 mN·m·s = {} kg·m²/s", l4.as_kg_m2_per_second()); // 1
println!("1000000 μN·m·s = {} kg·m²/s", l5.as_kg_m2_per_second()); // 1
println!("1000000000 nN·m·s = {} kg·m²/s", l6.as_kg_m2_per_second()); // 1

// 不同单位之间的运算
let sum = l3 + l4 + l5 + l6;
println!("总和: {} N·m·s", sum.as_nms()); // 103 N·m·s
```

**物理意义**：
- 1 N·m·s = 1 kg·m²/s（数值相等）
- 这些单位在旋转动力学计算中非常重要
- 支持从纳牛顿·米·秒到千克·平方千米/秒的各种精度

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

# 更新日志

## v0.2.11 - 角动量运算关系和单位系统

### 功能特性
- **角动量与角速度的乘法运算**：实现角动量 × 角速度 = 力矩的关系
- **角动量与角速度的除法运算**：实现角动量 ÷ 角速度 = 转动惯量（系数）的关系
- **力矩与角速度的除法运算**：实现力矩 ÷ 角速度 = 角动量的关系
- **角动量单位系统**：支持N·m·s、mN·m·s、μN·m·s、nN·m·s系列单位
- **力物理量**：新增力(Force)物理量，支持牛顿(N)及其毫、微、纳、千、兆单位
- **力与距离的乘积**：实现力 × 距离 = 能量的关系
- **质量与加速度的乘积**：实现质量 × 加速度 = 力的关系
- **功率物理量**：新增功率(Power)物理量，支持瓦特(W)、马力(hp)及其毫、微、纳、千、兆单位
- **功率与时间的乘积**：实现功率 × 时间 = 能量的关系
- **力与速度的乘积**：实现力 × 速度 = 功率的关系

### 技术特性
- 所有运算都满足数学交换律
- 支持不同单位之间的自动转换
- 编译时类型安全，防止量纲不匹配的错误
- 完整的单元测试覆盖

### 物理意义
- 在旋转动力学中，这些关系非常重要
- 1 N·m·s = 1 kg·m²/s（数值相等）
- 支持从纳牛顿·米·秒到千克·平方千米/秒的各种精度

### 示例
运行 `cargo run --example example_angular_relations` 查看完整示例。
