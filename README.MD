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
// let wrong: Area = d / time_delta_from_secs_f64(10.0);  // ❌ 编译错误：长度÷时间不是面积
```

> **注意：物理量不能作为矩阵元素。**
> `Matrix<ROWS, COLS, T>` 的 `T` 由 `NumericElement` 限定为基础数字量（整数/浮点）。
> 原因：单一 `T` 要求整个矩阵同量纲，而异量纲矩阵（状态转移阵、协方差阵、卡尔曼增益、
> 雅可比）无法这样表达；无量纲的旋转矩阵请用
> [`CosMatrix`](https://docs.rs/zmatrix/latest/zmatrix/spatial_geometry/cos_matrix/struct.CosMatrix.html)。

---

## 一、物理量系统

26 种物理量，每种支持多种单位，物理量之间的运算在编译期进行量纲检查。

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
| 转动惯量 `Inertia` | kg·m², kg·mm² |
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

### 时间：统一使用 `chrono::TimeDelta`

所有涉及时间的运算都接收 `chrono::TimeDelta`，**不支持** `std::time::Duration`。

理由：`Duration` 无法表示负时长（反向制动、时间倒推都无法表达），而这是仿真中的真实需求。此前的双轨支持已在 `0.4.1` 移除。

```rust
use chrono::TimeDelta;

let dt = TimeDelta::seconds(10);
let dt2 = TimeDelta::milliseconds(500);
let dt3 = time_delta_from_secs_f64(2.5);   // 浮点秒
let dt4 = time_delta_from_secs_f64(-2.5);  // 负时长，Duration 表达不了
```

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

// 冲量定理：F·Δt = Δp
let impulse: Momentum = force * time;              // 20 N × 10 s → 200 kg·m/s

// 旋转运动学
let angle = Angular::from_deg(180.0);
let omega: AngularVelocity = angle / time;
let alpha: AngularAcceleration = omega / time;

// 角冲量：M·Δt = ΔL
let torque = Torque::from_nm(5.0);
let angular_impulse: AngularMomentum = torque * time;

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

let mut m1 = Matrix::<2, 3, f64>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
let m2 = Matrix::<2, 3, f64>::new([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);

let sum = m1 + m2;
let diff = m1 - m2;
let scaled = m1.scale(2.0);
let transposed = m1.T();    // 转置

// 全0 / 全1 矩阵
let zeros = Matrix::<3, 3, f64>::zeros();
let ones = Matrix::<3, 3, f64>::ones();

// 获取与设置：参数顺序是 (列, 行)
let val = m1.get(2, 1).unwrap();   // 第 2 列第 1 行
m1.set(2, 1, 99.0).unwrap();
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

// 点乘：接收引用，返回同类型（Distance · Distance 仍是 Distance）
let v1 = Vector3::new(Distance::from_m(1.0), Distance::from_m(0.0), Distance::from_m(0.0));
let v2 = Vector3::new(Distance::from_m(0.0), Distance::from_m(1.0), Distance::from_m(0.0));
let dot: Distance = v1.dot(&v2);

// 叉乘：接收引用，输出类型须实现 VectorQuantity
let c1 = Vector3::new(Coef::new(1.0), Coef::new(0.0), Coef::new(0.0));
let c2 = Vector3::new(Coef::new(0.0), Coef::new(1.0), Coef::new(0.0));
let cross = c1.cross(&c2);

// 反对称矩阵
let skew = v1.skew_symmetric_matrix();    // 3×3
let skew4 = v1.skew_symmetric_matrix_4(); // 4×4
```

> **关于叉乘的类型约束**：`cross` 的输出类型是两元素类型相乘的结果，且该类型必须实现
> `VectorQuantity`。`Distance × Distance = Area`，而 `Area` 不是向量量，因此
> `Vector3<Distance>` **不能叉乘**；无量纲的 `Vector3<Coef>` 可以。

### 向量的时间算术

力域步进需要的六个运算，全部接收 `chrono::TimeDelta`：

```rust
let dt = TimeDelta::seconds(2);

// 速度 ÷ 时间 = 加速度向量
let velocity_vec = Vector3::new(
    Velocity::from_m_per_sec(10.0),
    Velocity::from_m_per_sec(4.0),
    Velocity::from_m_per_sec(0.0),
);
let acc: Vector3<Acceleration> = velocity_vec / dt;

// 加速度 × 时间 = 速度增量
let dv: Vector3<Velocity> = acc * dt;

// 力 × 时间 = 冲量（向量版冲量定理）
let force_vec = Vector3::new(
    Force::from_newton(1.0),
    Force::from_newton(2.0),
    Force::from_newton(3.0),
);
let impulse_vec: Vector3<Momentum> = force_vec * dt;

// 力矩 × 时间 = 角冲量
let torque_vec = Vector3::new(
    Torque::from_nm(5.0),
    Torque::from_nm(0.0),
    Torque::from_nm(-1.0),
);
let ang_impulse: Vector3<AngularMomentum> = torque_vec * dt;
```

---

## 四、转动惯量与姿态动力学

标量转动惯量 `Inertia`（kg·m²）配合角速度、角加速度、角动量、力矩，把刚体转动动力学整条链闭合在类型系统内。

### 六个互逆运算

```rust
let j = Inertia::from_kg_m2(0.5);              // 飞轮转子惯量
let omega = AngularVelocity::from_rad_per_second(10.0);
let torque = Torque::from_nm(3.0);

// 角动量：J·ω = L
let l: AngularMomentum = j * omega;             // 5 kg·m²/s

// 反解转速：L / J = ω
let omega2: AngularVelocity = l / j;

// 角加速度：M / J = ω̇
let alpha: AngularAcceleration = torque / j;

// 力矩：J·ω̇ = M
let torque2: Torque = j * alpha;

// 反解惯量：L / ω = J 与 M / ω̇ = J 两条独立路径
let j1: Inertia = l / omega;
let j2: Inertia = torque / alpha;
```

### 两种单位

工程图纸与 CAD 通常以 kg·mm² 给出转动惯量（比 kg·m² 大 10⁶ 倍）：

```rust
let j_from_drawing = Inertia::from_kg_mm2(500_000.0);
assert_eq!(j_from_drawing.as_kg_m2(), 0.5);   // 自动换算
```

### 飞轮加速全过程

```rust
// J = 0.5 kg·m² 的飞轮，施加 3 N·m 持续 2 秒
let j = Inertia::from_kg_m2(0.5);
let torque = Torque::from_nm(3.0);
let dt = TimeDelta::seconds(2);

let impulse: AngularMomentum = torque * dt;                    // 角冲量 6 N·m·s
let omega: AngularVelocity = impulse / j;                      // ω = 12 rad/s
let alpha: AngularAcceleration = torque / j;                   // ω̇ = 6 rad/s²
assert_eq!(alpha.as_rad_per_second2() * 2.0, omega.as_rad_per_second());  // 两条路径一致
```

> **标量 vs 张量**：`Inertia` 描述绕**单一指定轴**的转动惯性。
> 完整描述刚体在任意轴向上的惯性需要 3×3 转动惯量张量（含惯性积），
> 姿态动力学中的陀螺力矩 `ω × (Jω)` 正是由张量非对角性产生的。张量支持尚未实现。

---

## 五、空间几何

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

// 获取行向量 / 列向量
let x_row = cos.get_x_row_vector();
let y_row = cos.get_y_row_vector();
let z_row = cos.get_z_row_vector();
let x_col = cos.get_x_col_vector();
let y_col = cos.get_y_col_vector();
let z_col = cos.get_z_col_vector();

// 转置
let cos_t = cos.transfer();

// 转换
let q = cos.to_quaternion();
let euler = cos.to_pry();  // XZY 转序
let euler = cos.to_rpy();  // XYZ 转序

// 矩阵乘法：接收引用；要作用的向量元素须实现 VectorQuantity（如 Coef）
let cos_a = CosMatrix::unit();
let cos_b = CosMatrix::unit();
let cos_c = cos_a.product(&cos_b);
let dir = Vector3::new(Coef::new(1.0), Coef::new(0.0), Coef::new(0.0));
let vec = cos_c.product_vector(&dir);
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
let q1 = Quaternion::new(1.0, 0.0, 0.0, 0.0);
let q2 = Quaternion::new(0.0, 1.0, 0.0, 0.0);
let sum = q1 + q2;
let product = q1 * q2;
let quotient = q1 / q2;
```

---

## 依赖

```toml
[dependencies]
zmatrix = "0.4"
chrono = "0.4"
```

> 时间运算使用 `chrono::TimeDelta`，因此需要将 `chrono` 一并加入依赖。
> 也可以只用 `time_delta_from_secs_f64` / `time_delta_to_secs_f64` 两个辅助函数，
> 无需直接 `use chrono`。

---

## License

MIT
