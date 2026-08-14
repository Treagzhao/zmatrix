# zmatrix — Physics-Aware Linear Algebra in Rust

[![Crates.io](https://img.shields.io/crates/v/zmatrix)](https://crates.io/crates/zmatrix)
[![Codecov](https://codecov.io/gh/Treagzhao/zmatrix/branch/master/graph/badge.svg)](https://codecov.io/gh/Treagzhao/zmatrix)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A Rust math library for aerospace, robotics, and physics simulation. Features a compile-time dimensional analysis system and generic matrix operations with typed physical quantities.

> 📖 [中文 README](README.MD)

---

## Quick Look

```rust
use zmatrix::physics::basic::*;
use zmatrix::dense::Matrix;
use chrono::TimeDelta;

// Automatic unit conversion
let d = Distance::from_m(1000.0);
println!("{} km", d.as_km()); // 1 km

// Compile-time dimensional analysis
let v: Velocity = d / time_delta_from_secs_f64(10.0);    // ✅ Correct
let a: Acceleration = v / time_delta_from_secs_f64(5.0);  // ✅ Correct
// let wrong: Area = d / dt;  // ❌ Compiler error: dimension mismatch

// Typed matrix product — output type auto-derived
let rot: Matrix<2, 2, Coef> = Matrix::new([[Coef::new(0.0), Coef::new(-1.0)],
                                            [Coef::new(1.0), Coef::new(0.0)]]);
let pos: Matrix<2, 1, Distance> = Matrix::new([[Distance::from_m(3.0)],
                                                [Distance::from_m(4.0)]]);
let rotated: Matrix<2, 1, Distance> = rot.product(&pos).unwrap();
// Coef × Distance = Distance, derived at compile time ✅
```

---

## 1. Physical Quantity System

25 physical quantities, each with multiple units. Operations between quantities are dimensionally checked at compile time.

### Supported Quantities

| Quantity | Units |
|----------|-------|
| `Distance` | m, km, ly |
| `Velocity` | m/s, km/h, km/s, c |
| `Acceleration` | m/s², km/h², g |
| `Angular` | rad, deg |
| `AngularVelocity` | rad/s, deg/s, rad/h, deg/h |
| `AngularAcceleration` | rad/s², deg/s² |
| `Area` | m², km² |
| `Volume` | m³, km³ |
| `Mass` | kg, g |
| `Momentum` | kg·m/s, kg·km/s |
| `AngularMomentum` | kg·m²/s, kg·km²/s, N·m·s family |
| `Force` | N, mN, μN, nN, kN, MN |
| `Torque` | N·m, mN·m, μN·m, nN·m, kN·m, MN·m |
| `Energy` | J, eV, mJ, μJ, nJ, kJ, MJ |
| `Power` | W, hp, mW, μW, nW, kW, MW |
| `MagneticInduction` | T, G, mT, μT, nT |
| `MagneticMoment` | A·m², J/T |
| `MagneticAngularVelocity` | T·rad/s |
| `ElectricCurrent` | A, mA, μA, kA |
| `ElectricPotential` | V, mV, μV, kV |
| `ElectricResistance` | Ω, mΩ, kΩ, MΩ |
| `ElectricCharge` | C, mC, μC, nC |
| `ElectricCapacitance` | F, mF, μF, nF, pF |
| `ElectricConductance` | S, mS, μS |
| `Coef` | dimensionless |

### Quantity Arithmetic

```rust
// Kinematics
let distance = Distance::from_m(1000.0);
let dt = time_delta_from_secs_f64(10.0);
let velocity: Velocity = distance / dt;          // 100 m/s
let acceleration: Acceleration = velocity / dt;  // 10 m/s²

// Dynamics
let mass = Mass::from_kg(2.0);
let force: Force = mass * acceleration;          // F = m × a → 20 N
let energy: Energy = force * distance;           // W = F × d → 20000 J

// Rotational kinematics
let angle = Angular::from_deg(180.0);
let omega: AngularVelocity = angle / dt;
let alpha: AngularAcceleration = omega / dt;

// Electricity (Ohm's law & power)
let current = ElectricCurrent::from_a(2.0);
let resistance = ElectricResistance::from_ohm(5.0);
let voltage: ElectricPotential = current * resistance;   // V = I × R → 10 V
let power: Power = voltage * current;                    // P = V × I → 20 W

// Electrical energy
let charge = ElectricCharge::from_coulomb(3.0);
let energy2: Energy = charge * voltage;                  // E = Q × V → 30 J
```

---

## 2. Matrix Operations

Generic matrices backed by const generics — rows and columns fixed at compile time.

### Basics

```rust
use zmatrix::dense::Matrix;

let m1 = Matrix::<2, 3, f64>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
let m2 = Matrix::<2, 3, f64>::new([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);

let sum = m1 + m2;
let diff = m1 - m2;
let scaled = m1.scale(2.0);
let transposed = !m1;       // transpose
let transposed = m1.T();    // equivalent

// All-zero / all-one matrices
let zeros = Matrix::<3, 3, f64>::zeros();
let ones = Matrix::<3, 3, f64>::ones();

// Get & set
let val = m1.get(1, 2).unwrap();  // (col, row)
m1.set(1, 2, 99.0).unwrap();
```

### Typed Matrix Product

The `product` method accepts heterogeneous element types and derives the output type automatically:

```rust
// F = m × a
let m: Matrix<1, 2, Mass> = Matrix::new([[Mass::from_kg(2.0), Mass::from_kg(3.0)]]);
let a: Matrix<2, 2, Acceleration> = Matrix::new([
    [Acceleration::from_m_per_s2(5.0), Acceleration::from_m_per_s2(0.0)],
    [Acceleration::from_m_per_s2(0.0), Acceleration::from_m_per_s2(4.0)],
]);
let f: Matrix<1, 2, Force> = m.product(&a).unwrap();  // Mass × Accel = Force

// W = F × d
let f: Matrix<1, 3, Force> = Matrix::new([
    [Force::from_newton(10.0), Force::from_newton(20.0), Force::from_newton(30.0)]
]);
let d: Matrix<3, 1, Distance> = Matrix::new([
    [Distance::from_m(2.0)], [Distance::from_m(0.5)], [Distance::from_m(1.0)]
]);
let w: Matrix<1, 1, Energy> = f.product(&d).unwrap();  // Force × Dist = Energy

// Direction cosine matrix × position vector
let dcm: Matrix<3, 3, Coef> = /* ... */;
let pos: Matrix<3, 1, Distance> = /* ... */;
let rotated: Matrix<3, 1, Distance> = dcm.product(&pos).unwrap();
// Coef × Distance = Distance ✅
```

Dimension mismatches are caught at compile time.

---

## 3. Vector Operations

3D vectors over multiple physical quantity types, with cross/dot products.

```rust
let displacement = Vector3::new(
    Distance::from_m(10.0), Distance::from_m(20.0), Distance::from_m(30.0),
);

// Displacement ÷ time = velocity vector
let velocity = displacement / time_delta_from_secs_f64(10.0);

// Velocity × mass = momentum vector
let momentum = velocity * Mass::from_kg(1.0);

// Displacement × momentum = angular momentum
let angular_momentum = displacement * momentum;

// Cross & dot products
let v1 = Vector3::new(Distance::from_m(1.0), Distance::from_m(0.0), Distance::from_m(0.0));
let v2 = Vector3::new(Distance::from_m(0.0), Distance::from_m(1.0), Distance::from_m(0.0));
let cross = v1.cross(v2);
let dot: Area = v1.dot(v2);  // Distance × Distance = Area

// Skew-symmetric matrices
let skew = v1.skew_symmetric_matrix();    // 3×3
let skew4 = v1.skew_symmetric_matrix_4(); // 4×4
```

---

## 4. Spatial Geometry

Euler angles, direction cosine matrices (DCM), and quaternions with mutual conversions.

### Euler Angles

```rust
let euler: Vector3<Angular> = Vector3::new(
    Angular::from_deg(10.0), Angular::from_deg(20.0), Angular::from_deg(30.0),
);
let quat = euler.to_quaternion();
let sin = euler.sin();
let cos = euler.cos();
```

### Direction Cosine Matrix

```rust
let cos = CosMatrix::unit();

let x = cos.get_x_vector();
let y = cos.get_y_vector();
let z = cos.get_z_vector();

let cos_t = cos.transfer();                 // transpose
let q = cos.to_quaternion();
let euler = cos.to_pry();                   // XZY sequence
let euler = cos.to_rpy();                   // XYZ sequence

let cos_c = cos_a.product(cos_b);
let vec = cos_c.product_vector(Vector3::new(1.0, 2.0, 3.0));
```

### Quaternions

```rust
let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);

let norm = q.norm();
let normalized = q.normalize();
let conjugate = q.conjugate();
let inverse = q.inverse();
let cos = q.to_cos_matrix();

let sum = q1 + q2;
let product = q1 * q2;
let quotient = q1 / q2;
```

---

## Dependencies

```toml
[dependencies]
zmatrix = "0.3"
```

---

## License

MIT
