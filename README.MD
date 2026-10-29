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

35 种物理量，每种支持多种单位，物理量之间的运算在编译期进行量纲检查。

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
| **温度 `Temperature`** | K, °C, °F, °R |
| **温差 `TemperatureDifference`** | ΔK, Δ°C, Δ°F, Δ°R |
| **热流密度 `HeatFlux`** | W/m², kW/m², mW/cm² |
| **导热系数 `ThermalConductivity`** | W/(m·K), kW/(m·K), mW/(cm·K) |
| **热容 `HeatCapacity`** | J/K, kJ/K |
| **热导 `ThermalConductance`** | W/K, mW/K |
| **密度 `Density`** | kg/m³, g/cm³, g/L |
| **压力 `Pressure`** | Pa, hPa, kPa, MPa, bar, atm, mmHg |
| **质量通量 `MassFlux`** | kg/(m²·s), g/(m²·s), g/(cm²·s) |
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

// 叉乘：输出类型由 CrossProduct trait 决定
let c1 = Vector3::new(Coef::new(1.0), Coef::new(0.0), Coef::new(0.0));
let c2 = Vector3::new(Coef::new(0.0), Coef::new(1.0), Coef::new(0.0));
let cross: Vector3<Coef> = c1.cross(&c2);   // (0, 0, 1)

// 力矩：r × F
let r = Vector3::new(Distance::from_m(1.0), Distance::from_m(0.0), Distance::from_m(0.0));
let f = Vector3::new(Force::from_newton(0.0), Force::from_newton(2.0), Force::from_newton(0.0));
let tau: Vector3<Torque> = r.cross(&f);      // 2 N·m 沿 +z

// 反对称矩阵
let skew = v1.skew_symmetric_matrix();    // 3×3
let skew4 = v1.skew_symmetric_matrix_4(); // 4×4
```

> **叉乘的输出类型由 `CrossProduct` 决定，不再依赖 `Mul`。**
>
> 早期版本以 `T: Mul<B, Output = F>` 推导输出类型，导致最常用的 `m × B` 无法表达 ——
> 因为 `MagneticMoment * MagneticInduction` 已被**点乘**占用（输出 `Energy`），
> Rust 不允许同一 `Mul` 有两个 `Output`。`Distance × Force` 同理（`Mul` 给出功）。
>
> 现在改用专用 trait：
>
> ```text
> pub trait CrossProduct<B> {
>     type Out: VectorQuantity + Default;
> }
> ```
>
> 已实现的组合：
>
> | `T` | `B` | `Out` | 含义 |
> |---|---|---|---|
> | `Distance` | `Force` | `Torque` | 力矩 `τ = r × F` |
> | `AngularVelocity` | `AngularMomentum` | `Torque` | 陀螺力矩 `ω × L`（欧拉方程第二项） |
> | `MagneticMoment` | `MagneticInduction` | `Torque` | 磁力矩 `τ = m × B` |
> | `MagneticInduction` | `AngularVelocity` | `MagneticAngularVelocity` | 磁角速度 |
> | `Coef` | `Coef` | `Coef` | 方向向量叉乘 |
>
> **没有定义的组合会在编译期报错**，而不是悄悄给出错误类型。反序（`F × r`、`B × m`）
> 不提供 —— 那是同一个量的相反数，写 `-tau` 更清楚。

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
> 完整描述刚体在任意轴向上的惯性需要 3×3 张量（含惯性积），见下一章。

---

## 五、转动惯量张量与 `compound` 模块

`compound` 放**由物理量组装而成的结构**，与 `physics::basic`（单一物理量）区分：

| 模块 | 放什么 | 元素类型 |
|---|---|---|
| `physics::basic` | 单一物理量（一个值 + 单位） | — |
| `spatial_geometry` | 纯几何、无量纲 | 全 `f64` |
| **`compound`** | 由物理量组装的结构 | 带量纲 |

依赖方向单向：`compound → physics::basic`、`compound → spatial_geometry`，反向不允许。

### `InertiaTensor`

```rust
use zmatrix::compound::inertia_tensor::InertiaTensor;

// 由 6 个独立分量构造（对称性由接口保证，无法构造出非对称张量）
let j = InertiaTensor::new(
    Inertia::from_kg_m2(120.0),   // Ixx
    Inertia::from_kg_m2(130.0),   // Iyy
    Inertia::from_kg_m2(45.0),    // Izz
    Inertia::from_kg_m2(0.0),     // Ixy
    Inertia::from_kg_m2(8.0),     // Ixz
    Inertia::from_kg_m2(0.0),     // Iyz
);

// 主轴系 / 各向同性
let j2 = InertiaTensor::from_diagonal([
    Inertia::from_kg_m2(1.0),
    Inertia::from_kg_m2(2.0),
    Inertia::from_kg_m2(3.0),
]);
let j3 = InertiaTensor::from_isotropic(Inertia::from_kg_m2(2.0));

// 角动量：J·ω = L。非对称张量下 L 会偏离 ω 的方向
let omega = Vector3::new(
    AngularVelocity::from_rad_per_second(0.0),
    AngularVelocity::from_rad_per_second(0.0),
    AngularVelocity::from_rad_per_second(1.0),
);
let l: Vector3<AngularMomentum> = j * omega;

// 力矩：J·ω̇ = M
let alpha = Vector3::new(
    AngularAcceleration::from_rad_per_second2(1.0),
    AngularAcceleration::from_rad_per_second2(0.0),
    AngularAcceleration::from_rad_per_second2(0.0),
);
let m: Vector3<Torque> = j * alpha;

// 绕任意轴的等效惯量：nᵀJn / nᵀn（内部归一化，n 无需是单位向量）
let dir = Vector3::new(Coef::new(1.0), Coef::new(1.0), Coef::new(0.0));
let about: Inertia = j.about_axis(dir)?;

// 各向同性等效：迹/3
let iso = j.isotropic_equivalent();

// 坐标变换：R·J·Rᵀ
let rotated = j.rotate(&CosMatrix::unit());
```

> **`about_axis` 不能用对角元代替。** 以 `[[2,-1,0],[-1,2,0],[0,0,3]]` 为例，
> 绕 z 轴是 3，绕 `(1,1,0)/√2` 只有 1 —— 惯性积把结果从 2 拉到了 1。
> 归一化在方法内部完成，传入非单位向量结果不变；零向量返回错误。

### 陀螺力矩与欧拉方程

张量的意义在于：`L` 可以偏离 `ω`，于是 `ω × L` 不为零。

```rust
let j = InertiaTensor::new(
    Inertia::from_kg_m2(120.0), Inertia::from_kg_m2(130.0), Inertia::from_kg_m2(45.0),
    Inertia::from_kg_m2(0.0), Inertia::from_kg_m2(0.0), Inertia::from_kg_m2(0.0),
);

let omega = Vector3::new(
    AngularVelocity::from_rad_per_second(0.5),
    AngularVelocity::from_rad_per_second(0.0),
    AngularVelocity::from_rad_per_second(2.0),
);

// 欧拉方程：M = J·ω̇ + ω × (Jω)
let l: Vector3<AngularMomentum> = j * omega;          // L = (60, 0, 90)
let gyro: Vector3<Torque> = omega.cross(&l);          // ω × L = (0, 75, 0)

let alpha = Vector3::new(
    AngularAcceleration::from_rad_per_second2(0.0),
    AngularAcceleration::from_rad_per_second2(0.1),
    AngularAcceleration::from_rad_per_second2(0.0),
);
let inertial: Vector3<Torque> = j * alpha;            // J·ω̇ = (0, 13, 0)
let total = inertial + gyro;                          // M = (0, 88, 0)
```

**纯绕主轴旋转时 `ω × L` 为零** —— 这就是各向同性刚体可以用标量模型的原因。
`Ixx ≠ Izz` 且两轴同时转动时该项必不为零，标量模型无法表达。

### 磁偶极子：力矩与势能

磁矩在外场中有两个量，**符号约定容易搞错**：

| 量 | 公式 | 类型 | 获取方式 |
|---|---|---|---|
| 力矩 | `τ = m × B` | `Vector3<Torque>` | `cross` 或 `magnetic_torque` |
| 势能 | `U = -m · B` | `Energy` | `potential_energy`（**带负号**） |

```rust
let m = Vector3::new(
    MagneticMoment::from_am2(12.0),
    MagneticMoment::from_am2(0.0),
    MagneticMoment::from_am2(0.0),
);
let b = Vector3::new(
    MagneticInduction::from_tesla(3e-5),
    MagneticInduction::from_tesla(0.0),
    MagneticInduction::from_tesla(0.0),
);

// 力矩：两种写法等价
let tau: Vector3<Torque> = m.cross(&b);
let tau2 = m.magnetic_torque(&b);

// 势能：平行取向 -3.6e-4 J（最低），反平行 +3.6e-4 J（最高）
let u = m.potential_energy(&b);
```

> **`m * B` 这个运算符不提供。** 点乘的物理意义是势能 `U = -m·B`，带负号；
> 若做成 `Mul`，`m * b` 会给出 `+m·B`，与真实物理量反号 —— 而且不会报错。
> 命名函数可以在名字和文档里把负号说清楚，运算符做不到。
>
> 关系：`U = -|m||B|cosθ`，`|τ| = |m||B|sinθ`。两者都是零或极值的取向，
> 恰好对应稳定与不稳定平衡。

---

## 六、温度与热学量

### 温度与温差必须是两个类型

摄氏与开尔文是**带偏移**的仿射换算（`T_K = T_C + 273.15`），而温差的换算是纯比例、无偏移：

```
20 °C  = 293.15 K          ← 温度有偏移
Δ20 °C = 20 K              ← 温差没有
```

合并成一个类型会有两个后果：

- `(t1 - t2).as_celsius()` 返回 `-253.15` 这类荒谬值
- `G * T` 与 `G * ΔT` 量纲**都成立**（`W/K × K = W`），但数值差几十倍，**量纲检查无能为力**

因此 `Temperature` 与 `TemperatureDifference` 独立成型，且刻意没有这些运算：

| 运算 | 是否允许 | 理由 |
|---|---|---|
| `T - T → ΔT` | ✅ | 唯一有意义的温度减法 |
| `T + ΔT → T` | ✅ | 加热 / 冷却（可交换） |
| `T / T → Coef` | ✅ | 温度比，如 Carnot 效率 |
| `T + T` | ❌ | 两个位置温度之和不是温度 |
| `T × T` | ❌ | 输出应是 K²，无此量纲 |
| `T × f64` | ❌ | 从摄氏构造时反直觉（20 °C 乘 2 得 313 °C） |

```rust
let t1 = Temperature::from_celsius(30.0);
let t2 = Temperature::from_celsius(20.0);
let dt: TemperatureDifference = t1 - t2;    // 10 K
let heated = t2 + dt;                       // 30 °C
let ratio: Coef = t1 / t2;                  // 基于开尔文，非 30/20
```

> 温度是标量，`Vector3<Temperature>` 在编译期即不可构造。

### 热流密度与傅里叶定律

**热流密度** `HeatFlux`（W/m²）是单位面积的热功率，注意与两个易混量区分：

| 量 | 单位 | 含义 | 本库类型 |
|---|---|---|---|
| 热量 Q | J | 累计能量 | `Energy` |
| 热功率 Q̇ | W | 单位时间的能量 | `Power` |
| 热流密度 q | W/m² | 单位时间、单位面积 | `HeatFlux` |

```rust
let q = HeatFlux::from_w_per_m2(1361.0);        // 太阳常数
let p: Power = q * Area::from_m2(2.0);          // 2722 W

// 手册常给 mW/cm²：1 mW/cm² = 10 W/m²（不是 1e-3）
let q2 = HeatFlux::from_mw_per_cm2(1.0);
assert_eq!(q2.as_w_per_m2(), 10.0);
```

**导热系数** `ThermalConductivity`（W/(m·K)）是材料属性，傅里叶定律：

```rust
let k = ThermalConductivity::from_w_per_m_k(237.0);   // 铝
let dt = TemperatureDifference::from_celsius(20.0);
let thickness = Distance::from_m(0.01);

let q: HeatFlux = k.heat_flux(dt, thickness);          // 474 kW/m²

// 反推（从实测热流标定材料参数）
let k2 = ThermalConductivity::from_flux(q, dt, thickness);
```

> 参数是 `TemperatureDifference` 而非 `Temperature`，传绝对温度会编译错误 —— 否则算出的是
> 从 0 K 量起的巨大热流。

### 集总参数热网络的两个元素

$$C\frac{dT}{dt} = G\,\Delta T + \dot{Q}$$

| 符号 | 类型 | 单位 | 含义 |
|---|---|---|---|
| C | `HeatCapacity` | J/K | 节点储热能力 |
| G | `ThermalConductance` | W/K | 节点间耦合强度 |
| ΔT | `TemperatureDifference` | K | 温差 |
| Q̇ | `Power` | W | 外部热源 |

```rust
// 储热：Q = C·ΔT
let c = HeatCapacity::from_j_per_k(9000.0);
let q: Energy = c * TemperatureDifference::from_celsius(20.0);   // 180 kJ

// 反解温升：ΔT = Q/C
let dt: TemperatureDifference = q / c;

// 传热：Q̇ = G·ΔT
let g = ThermalConductance::from_w_per_k(12.0);
let p: Power = g * TemperatureDifference::from_celsius(20.0);    // 240 W
```

> **四个近名量，两层含义**（中文只差一个字，务必区分）：
>
> | 层级 | 储热 | 传热 |
> |---|---|---|
> | 材料属性 | 比热容 c (J/(kg·K)) | 导热系数 k (W/(m·K)) |
> | 整体属性 | **热容 C (J/K)** | **热导 G (W/K)** |
>
> 规律：带「比」或「系数」的是材料属性，不带的是整体属性。
> 关系：`C = c·m`、`G = k·A/L`。

---

## 七、流体量：密度、压力、质量通量

力模型的两条主路径：**大气阻力** `F = ½ρv²·C_D·A` 和**光压** `F = P·A·C_R`。

### 量纲链为什么要拆成三步

`ρv²` 的量纲是 `kg/m³ × m/s × m/s = kg/(m·s²) = Pa`。若不给中间量，
就只能写一条「密度 × 速度 × 速度 → 压力」的三元特殊规则，或者跳过 `ρv` 直接给命名函数。

而 `ρv`（质量通量）本身是独立的物理量，动量方程、边界层、烧蚀都要用：

```rust
let rho = Density::from_kg_per_m3(1.225);     // 海平面空气
let v = Velocity::from_m_per_sec(200.0);

let mf: MassFlux = rho * v;                   // ρv  = 245 kg/(m²·s)
let q: Pressure = mf * v;                     // ρv² = 49000 Pa（不含 ½）

// 完整动压（含 ½）
let q_half = rho.dynamic_pressure(v);         // 24500 Pa
```

| 运算 | 结果 | 含义 |
|---|---|---|
| `Density × Velocity` | `MassFlux` | 质量通量 `ρv` |
| `MassFlux × Velocity` | `Pressure` | `ρv²`（**不含 ½**） |
| `MassFlux / Velocity` | `Density` | 由通量反推密度 |
| `Pressure / Velocity` | `MassFlux` | 由动压反推通量 |

> **运算符永远不含物理系数。** `ρ * v * v` 给 `ρv²`，`½` 只能靠命名方法
> `dynamic_pressure` / `drag_force` 补上。这和 `potential_energy` 带负号是同一个取向：
> 运算符只管量纲，物理系数归名字。

### 力模型两条路径

```rust
// 大气阻力：海平面 200 m/s，C_D = 1.2，迎风 2 m²
let rho = Density::from_kg_per_m3(1.225);
let v = Velocity::from_m_per_sec(200.0);
let drag = rho.drag_force(v, Coef::new(1.2), Area::from_m2(2.0));   // 58800 N

// 光压（1 AU，C_R 是反射系数）
let f_solar = Pressure::from_pa(4.56e-6) * Area::from_m2(10.0) * Coef::new(1.5);
```

### 单位换算的三个坑

```text
1 g/cm³     = 1000   kg/m³      ← 密度，材料
1 g/L       = 1      kg/m³      ← 密度，气体
1 g/(cm²·s) = 10     kg/(m²·s)  ← 质量通量

1 bar = 1e5    Pa
1 atm = 101325 Pa               ← 差 1.3%，不能混用
```

**两组都差 1000 倍或 1.3%，而纸面上只差几个字符。**

- 密度：`g/cm³` 和 `g/L` 差 1000 倍。材料用前者、气体用后者，用错就错三个数量级。
- 质量通量：`g/(cm²·s)` 的因子是 **10**，不是 `1e-3`——因为分母是 cm² 而非 m²。
  这和 `HeatFlux` 的 `mW/cm² = 10 W/m²` 是同一个陷阱。
- 压力：`bar` 与 `atm` 差 1.325%。真空系统、密封压差这类场景会变成实质错误。

> 另有精度细节：`760 mmHg` 按约定值算 = 101325.014 Pa，**不严格等于 1 atm**。
> `Torr` 才定义为 `1/760 atm`。工程上视为相等，高精度真空计算需注意。

### 与其他量的关系

```rust
// 密度 × 体积 = 质量
let m: Mass = Density::from_kg_per_m3(2700.0) * Volume::from_m3(0.05);   // 135 kg 铝件

// 压力 × 体积 = 能量（等压做功）
let w: Energy = Pressure::from_atm(1.0) * Volume::from_m3(1.0);          // 101325 J

// 力 ÷ 面积 = 压力
let p: Pressure = Force::from_newton(1000.0) / Area::from_m2(2.0);
```

---

## 八、空间几何

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

## 尚未实现的账

按当前优先级排列。**已明确不做**的也列在最后，免得反复讨论。

### 线性代数缺口（阻塞项）

`dense::Matrix` 目前**只有加减、数乘、转置、矩阵乘积**，缺少解方程能力：

| 缺什么 | 阻塞了什么 |
|---|---|
| `det()` | 判奇异、特征值 |
| `inverse()` | 显式求逆 |
| `solve()` / LU 分解 | **稳态热网络求解 `G·T = Q̇`**、任何线性方程组 |

这是**热网络稳态分析的前置条件**，也是 `InertiaTensor` 求逆（解 `J·ω = L`）的前置条件。
量纲上 `J⁻¹` 需要 `(kg·m²)⁻¹`、`det` 需要 `(kg·m²)³`，这两个量纲都不存在，
所以即便做了求解器，也只能在 `f64` 层面提供，不能做成带量纲的接口。

### 热学量

| 待做 | 单位 | 说明 |
|---|---|---|
| `SpecificHeatCapacity` 比热容 | J/(kg·K) | 材料属性，`C = c·m` |
| `HeatTransferCoefficient` 传热系数 | W/(m²·K) | 表面对流，`q = h·ΔT`。真空环境很少用，优先级低 |
| `ThermalResistance` 热阻 | K/W | `R = 1/G`，串联分析更直观 |
| 辐射辅助函数 | — | `σT⁴` 在类型系统里没有落点，只能做成函数：`radiative_flux(ε, T)` |

**已明确不做**：

- `Entropy` 熵（J/K）—— 卫星热分析不用，等有明确需求
- `ThermalDiffusivity` 热扩散率（m²/s）—— `α = k/(ρc)` 可推，重复
- `Heat` 热量、`InternalEnergy` 内能 —— 与 `Energy` 重复
- `SpecificHeatRatio` 比热容比 γ —— 无量纲商，`Coef` 够用；且固体 `cp≈cv`、`γ≈1` 无信息量
- `Emissivity` 发射率 —— 先用 `Coef`，不引入值域约束（库里无先例）

### 流体量

| 待做 | 单位 | 说明 |
|---|---|---|
| `MassFlowRate` 质量流量 | kg/s | `MassFlux × Area` 的落点，目前缺这个类型所以该运算未提供 |
| `DynamicViscosity` 动力粘度 | Pa·s | 雷诺数 `Re = ρvL/μ` 需要 |
| `KinematicViscosity` 运动粘度 | m²/s | `ν = μ/ρ` |
| `SpecificGasConstant` 气体常数 | J/(kg·K) | 理想气体 `P = ρRT` 需要 |
| `SpeedOfSound` 声速 | m/s | 马赫数需要；`a = √(γRT)` 含 γ，属命名函数场景 |

**注意**：`MassFlux × Area` 现在**不可用**（编译错误），因为 `MassFlowRate` 不存在。
需要时先补类型，不要退回裸 `f64`。

### `compound` 模块

| 待做 | 说明 |
|---|---|
| `InertiaTensor` 平行轴定理 | `J' = J + m(d²I - ddᵀ)`，需 `Mass` + `Distance` |
| `InertiaTensor` 主轴对角化 | Jacobi 迭代，把惯性积消为零 |
| `InertiaTensor` 求逆 / 行列式 | 受上面线性代数缺口阻塞 |
| `ThermalNetwork` 热网络容器 | 见下 |

**`ThermalNetwork` 计划范围**（刻意收窄）：

```text
add_node(capacity, temperature)     加节点
add_link(i, j, conductance)         加连接
set_source(i, power)                加热源
step(dt) -> Result<(), _>           显式推进一歩
temperature(i) -> Option<Temperature>
```

**只做显式推进**：隐式推进与稳态求解都依赖线性方程组，被上面的缺口阻塞。
`step()` 会检查显式稳定性判据 `Δt < 2·Cᵢ/ΣGᵢⱼ` 并返回错误 ——
不检查的话发散初期的曲线仍然"看起来合理"，等指数爆炸时已经在看错误结论了。

### 其他

- `Matrix` 的 `det` / `inverse` / `solve`（见上）
- 物理量之间的乘除法覆盖仍不完整，遇到具体场景再补
- 单位系统仅覆盖工程常用子集，不是完整的 SI 词头体系

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
