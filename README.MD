# zmatrix — Rust 物理计算与矩阵运算库

[![Crates.io](https://img.shields.io/crates/v/zmatrix)](https://crates.io/crates/zmatrix)
[![Codecov](https://codecov.io/gh/Treagzhao/zmatrix/branch/master/graph/badge.svg)](https://codecov.io/gh/Treagzhao/zmatrix)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

面向航空航天、机器人、物理仿真的 Rust 数学库。提供编译期量纲安全的物理量系统，以及编译期定尺寸的泛型矩阵运算。

> 📖 [English README](README.en.md)

---

## 快速了解

```rust
use zmatrix::physics::basic::*;
use zmatrix::dense::Matrix;
use chrono::TimeDelta;

// 物理量自动单位转换
let d = Distance::from_m(1000.0);
println!("{} km", d.as_km()); // 1 km

// 编译期量纲检查 — 物理公式即代码
let v: Velocity = d / time_delta_from_secs_f64(10.0);    // ✅ 正确
let a: Acceleration = v / time_delta_from_secs_f64(5.0);  // ✅ 正确
// let wrong: Area = d / time;  // ❌ 编译错误：量纲不匹配
```

> **注意：物理量不能作为矩阵元素。**
> `Matrix<ROWS, COLS, T>` 的 `T` 由 `NumericElement` 限定为基础数字量（整数/浮点）。
> 原因：单一 `T` 要求整个矩阵同量纲，而异量纲矩阵（状态转移阵、协方差阵、卡尔曼增益、
> 雅可比）无法这样表达；无量纲的旋转矩阵请用
> [`CosMatrix`](https://docs.rs/zmatrix/latest/zmatrix/spatial_geometry/cos_matrix/struct.CosMatrix.html)。

---

## 一、物理量系统

25 种物理量，每种支持多种单位，物理量之间的运算在编译期进行量纲检查。

### 支持的物理量

| 物理量 | 单位 |
|--------|------|
| 长度 `Distance` | m, km, ly |
| 速度 `Velocity` | m/s, km/h, km/s, c |
| 加速度 `Acceleration` | m/s², km/h², g |
| 角度 `Angular` | rad, deg |
| 角速度 `AngularVelocity` | rad/s, deg/s, rad/h, deg/h |
| 角加速度 `AngularAcceleration` | rad/s², deg/s² |
| 面积 `Area` | m², km² |
| 体积 `Volume` | m³, km³ |
| 质量 `Mass` | kg, g |
| 动量 `Momentum` | kg·m/s, kg·km/s |
| 角动量 `AngularMomentum` | kg·m²/s, kg·km²/s, N·m·s 系列 |
| 力 `Force` | N, mN, μN, nN, kN, MN |
| 力矩 `Torque` | N·m, mN·m, μN·m, nN·m, kN·m, MN·m |
| 能量 `Energy` | J, eV, mJ, μJ, nJ, kJ, MJ |
| 功率 `Power` | W, hp, mW, μW, nW, kW, MW |
| 磁感应强度 `MagneticInduction` | T, G, mT, μT, nT |
| 磁矩 `MagneticMoment` | A·m², J/T |
| 磁角速度 `MagneticAngularVelocity` | T·rad/s |
| 电流 `ElectricCurrent` | A, mA, μA, kA |
| 电压 `ElectricPotential` | V, mV, μV, kV |
| 电阻 `ElectricResistance` | Ω, mΩ, kΩ, MΩ |
| 电荷量 `ElectricCharge` | C, mC, μC, nC |
| 电容 `ElectricCapacitance` | F, mF, μF, nF, pF |
| 电导 `ElectricConductance` | S, mS, μS |
| 系数 `Coef` | 无量纲 |

### 物理量运算

```rust
// 运动学
let distance = Distance::from_m(1000.0);
let time = time_delta_from_secs_f64(10.0);
let velocity: Velocity = distance / time;          // 100 m/s
let acceleration: Acceleration = velocity / time;  // 10 m/s²

// 动力学
let mass = Mass::from_kg(2.0);
let force: Force = mass * acceleration;            // F = m × a → 20 N
let energy: Energy = force * distance;             // W = F × d → 20000 J

// 旋转运动学
let angle = Angular::from_deg(180.0);
let omega: AngularVelocity = angle / time;
let alpha: AngularAcceleration = omega / time;

// 电学（欧姆定律与功率）
let current = ElectricCurrent::from_a(2.0);
let resistance = ElectricResistance::from_ohm(5.0);
let voltage: ElectricPotential = current * resistance;   // V = I × R → 10 V
let power: Power = voltage * current;                    // P = V × I → 20 W

// 电能
let charge = ElectricCharge::from_coulomb(3.0);
let energy2: Energy = charge * voltage;                  // E = Q × V → 30 J
```

---

## 二、矩阵运算

基于 const generics 的泛型矩阵，行列在编译期固定。

### 基础操作

```rust
use zmatrix::dense::Matrix;

let m1 = Matrix::<2, 3, f64>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
let m2 = Matrix::<2, 3, f64>::new([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);

let sum = m1 + m2;
let diff = m1 - m2;
let scaled = m1.scale(2.0);
let transposed = m1.T();    // 转置

// 全0 / 全1 矩阵
let zeros = Matrix::<3, 3, f64>::zeros();
let ones = Matrix::<3, 3, f64>::ones();

// 获取与设置
let val = m1.get(1, 2).unwrap();  // (col, row)
m1.set(1, 2, 99.0).unwrap();
```

> 注意：`*` 是**逐元素乘**（Hadamard 积），矩阵乘积请用 `.product()`。

### 矩阵乘积

```rust
let a = Matrix::<2, 3, f64>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
let b = Matrix::<3, 2, f64>::new([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
let c: Matrix<2, 2, f64> = a.product(&b).unwrap();
```

元素类型必须相同，且被限定为基础数字量（见上文关于物理量的说明）。

---

## 三、向量运算

支持多种物理量的三维向量，以及叉乘、点积等运算。

```rust
let displacement = Vector3::new(
    Distance::from_m(10.0),
    Distance::from_m(20.0),
    Distance::from_m(30.0),
);

// 位移向量 ÷ 时间 = 速度向量
let velocity = displacement / time_delta_from_secs_f64(10.0);

// 速度向量 × 质量 = 动量向量
let momentum = velocity * Mass::from_kg(1.0);

// 位移向量 × 动量向量 = 角动量
let angular_momentum = displacement * momentum;

// 叉乘与点乘
let v1 = Vector3::new(Distance::from_m(1.0), Distance::from_m(0.0), Distance::from_m(0.0));
let v2 = Vector3::new(Distance::from_m(0.0), Distance::from_m(1.0), Distance::from_m(0.0));
let cross = v1.cross(v2);
let dot: Area = v1.dot(v2);  // Distance × Distance = Area

// 反对称矩阵
let skew = v1.skew_symmetric_matrix();    // 3×3
let skew4 = v1.skew_symmetric_matrix_4(); // 4×4
```

---

## 四、空间几何

欧拉角、方向余弦矩阵（DCM）、四元数之间的相互转换。

### 欧拉角

```rust
let euler: Vector3<Angular> = Vector3::new(
    Angular::from_deg(10.0),
    Angular::from_deg(20.0),
    Angular::from_deg(30.0),
);
let quat = euler.to_quaternion();
let sin = euler.sin();
let cos = euler.cos();
```

### 方向余弦矩阵

```rust
let cos = CosMatrix::unit();

// 获取行/列向量
let x = cos.get_x_vector();
let y = cos.get_y_vector();
let z = cos.get_z_vector();

// 转置
let cos_t = cos.transfer();

// 转换
let q = cos.to_quaternion();
let euler = cos.to_pry();  // XZY 转序
let euler = cos.to_rpy();  // XYZ 转序

// 矩阵乘法
let cos_c = cos_a.product(cos_b);
let vec = cos_c.product_vector(Vector3::new(1.0, 2.0, 3.0));
```

### 四元数

```rust
let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);

let norm = q.norm();
let normalized = q.normalize();
let conjugate = q.conjugate();
let inverse = q.inverse();

let cos = q.to_cos_matrix();

// 四元数计算
let sum = q1 + q2;
let product = q1 * q2;
let quotient = q1 / q2;
```

---

## 依赖

```toml
[dependencies]
zmatrix = "0.3"
```

---

## License

MIT
