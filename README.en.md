# zmatrix — Physics-Aware Linear Algebra in Rust

[![Crates.io](https://img.shields.io/crates/v/zmatrix)](https://crates.io/crates/zmatrix)
[![Codecov](https://codecov.io/gh/Treagzhao/zmatrix/branch/master/graph/badge.svg)](https://codecov.io/gh/Treagzhao/zmatrix)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A Rust math library for aerospace, robotics, and physics simulation. Features a compile-time dimensional analysis system and const-generic dense matrices.

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
// let wrong: Area = d / time_delta_from_secs_f64(10.0);  // ❌ Compiler error: length / time is not area
```

> **Physical quantities cannot be matrix elements.**
> `T` in `Matrix<ROWS, COLS, T>` is bounded by `NumericElement` and thus restricted to
> primitive numbers (integers and floats). A single `T` forces the whole matrix to share one
> dimension, which cannot express the heterogeneous matrices engineers actually need
> (state-transition, covariance, Kalman gain, Jacobian). For dimensionless rotation matrices
> use
> [`CosMatrix`](https://docs.rs/zmatrix/latest/zmatrix/spatial_geometry/cos_matrix/struct.CosMatrix.html).

---

## 1. Physical Quantity System

26 physical quantities, each with multiple units. Operations between quantities are dimensionally checked at compile time.

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
| `Inertia` | kg·m², kg·mm² |
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

### Time: `chrono::TimeDelta` only

Every time-taking operation accepts `chrono::TimeDelta`. `std::time::Duration` is **not supported**.

Rationale: `Duration` cannot represent negative spans, so retrograde burns and reversed
time stepping are inexpressible. The dual-track support was removed in `0.4.1`.

```rust
use chrono::TimeDelta;

let dt = TimeDelta::seconds(10);
let dt2 = TimeDelta::milliseconds(500);
let dt3 = time_delta_from_secs_f64(2.5);   // float seconds
let dt4 = time_delta_from_secs_f64(-2.5);  // negative — inexpressible with Duration
```

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

// Impulse–momentum theorem: F·Δt = Δp
let impulse: Momentum = force * dt;              // 20 N × 10 s → 200 kg·m/s

// Rotational kinematics
let angle = Angular::from_deg(180.0);
let omega: AngularVelocity = angle / dt;
let alpha: AngularAcceleration = omega / dt;

// Angular impulse: M·Δt = ΔL
let torque = Torque::from_nm(5.0);
let angular_impulse: AngularMomentum = torque * dt;

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

let mut m1 = Matrix::<2, 3, f64>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
let m2 = Matrix::<2, 3, f64>::new([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);

let sum = m1 + m2;
let diff = m1 - m2;
let scaled = m1.scale(2.0);
let transposed = m1.T();    // transpose

// All-zero / all-one matrices
let zeros = Matrix::<3, 3, f64>::zeros();
let ones = Matrix::<3, 3, f64>::ones();

// Get & set: argument order is (column, row)
let val = m1.get(2, 1).unwrap();   // column 2, row 1
m1.set(2, 1, 99.0).unwrap();
```

### Matrix Product

```rust
let a = Matrix::<2, 3, f64>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
let b = Matrix::<3, 2, f64>::new([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
let c: Matrix<2, 2, f64> = a.product(&b).unwrap();
```

Both operands must share the same element type, which is restricted to primitive numbers
(see the note on physical quantities above).

> Note: `*` is **element-wise** (Hadamard) multiplication. Use `.product()` for matrix
> multiplication.

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

// Dot product: takes a reference, returns the same type
let v1 = Vector3::new(Distance::from_m(1.0), Distance::from_m(0.0), Distance::from_m(0.0));
let v2 = Vector3::new(Distance::from_m(0.0), Distance::from_m(1.0), Distance::from_m(0.0));
let dot: Distance = v1.dot(&v2);

// Cross product: takes a reference, output type must implement VectorQuantity
let c1 = Vector3::new(Coef::new(1.0), Coef::new(0.0), Coef::new(0.0));
let c2 = Vector3::new(Coef::new(0.0), Coef::new(1.0), Coef::new(0.0));
let cross = c1.cross(&c2);

// Skew-symmetric matrices
let skew = v1.skew_symmetric_matrix();    // 3×3
let skew4 = v1.skew_symmetric_matrix_4(); // 4×4
```

> **On the cross-product type bound**: the output type of `cross` is the product of the two
> element types, and that type must implement `VectorQuantity`. Since
> `Distance × Distance = Area` and `Area` is not a vector quantity, `Vector3<Distance>` **cannot**
> be cross-multiplied; dimensionless `Vector3<Coef>` can.

### Vector time arithmetic

The six operations needed for force-domain stepping, all taking `chrono::TimeDelta`:

```rust
let dt = TimeDelta::seconds(2);

// Velocity ÷ time = acceleration vector
let velocity_vec = Vector3::new(
    Velocity::from_m_per_sec(10.0),
    Velocity::from_m_per_sec(4.0),
    Velocity::from_m_per_sec(0.0),
);
let acc: Vector3<Acceleration> = velocity_vec / dt;

// Acceleration × time = velocity increment
let dv: Vector3<Velocity> = acc * dt;

// Force × time = impulse (vector form of the impulse–momentum theorem)
let force_vec = Vector3::new(
    Force::from_newton(1.0),
    Force::from_newton(2.0),
    Force::from_newton(3.0),
);
let impulse_vec: Vector3<Momentum> = force_vec * dt;

// Torque × time = angular impulse
let torque_vec = Vector3::new(
    Torque::from_nm(5.0),
    Torque::from_nm(0.0),
    Torque::from_nm(-1.0),
);
let ang_impulse: Vector3<AngularMomentum> = torque_vec * dt;
```

---

## 4. Rotational Inertia and Attitude Dynamics

Scalar rotational inertia `Inertia` (kg·m²), together with angular velocity, angular
acceleration, angular momentum and torque, closes the whole rigid-body rotational chain
inside the type system.

### Six mutually inverse operations

```rust
let j = Inertia::from_kg_m2(0.5);              // flywheel rotor inertia
let omega = AngularVelocity::from_rad_per_second(10.0);
let torque = Torque::from_nm(3.0);

// Angular momentum: J·ω = L
let l: AngularMomentum = j * omega;             // 5 kg·m²/s

// Solve back for spin rate: L / J = ω
let omega2: AngularVelocity = l / j;

// Angular acceleration: M / J = ω̇
let alpha: AngularAcceleration = torque / j;

// Torque: J·ω̇ = M
let torque2: Torque = j * alpha;

// Solve back for inertia: L / ω = J and M / ω̇ = J, two independent routes
let j1: Inertia = l / omega;
let j2: Inertia = torque / alpha;
```

### Two units

Engineering drawings and CAD usually report inertia in kg·mm² (10⁶ times larger than kg·m²):

```rust
let j_from_drawing = Inertia::from_kg_mm2(500_000.0);
assert_eq!(j_from_drawing.as_kg_m2(), 0.5);   // converted automatically
```

### Full flywheel spin-up

```rust
// Flywheel with J = 0.5 kg·m², 3 N·m applied for 2 seconds
let j = Inertia::from_kg_m2(0.5);
let torque = Torque::from_nm(3.0);
let dt = TimeDelta::seconds(2);

let impulse: AngularMomentum = torque * dt;                    // angular impulse 6 N·m·s
let omega: AngularVelocity = impulse / j;                      // ω = 12 rad/s
let alpha: AngularAcceleration = torque / j;                   // ω̇ = 6 rad/s²
assert_eq!(alpha.as_rad_per_second2() * 2.0, omega.as_rad_per_second());  // both routes agree
```

> **Scalar vs. tensor**: `Inertia` describes rotational inertia about **one specified axis**.
> Describing a rigid body's inertia about any axis requires the 3×3 inertia tensor (with
> products of inertia) — the gyroscopic term `ω × (Jω)` in attitude dynamics arises precisely
> from the tensor's off-diagonal components. Tensor support is not implemented yet.

---

## 5. Spatial Geometry

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

let x_row = cos.get_x_row_vector();
let y_row = cos.get_y_row_vector();
let z_row = cos.get_z_row_vector();
let x_col = cos.get_x_col_vector();
let y_col = cos.get_y_col_vector();
let z_col = cos.get_z_col_vector();

let cos_t = cos.transfer();                 // transpose
let q = cos.to_quaternion();
let euler = cos.to_pry();                   // XZY sequence
let euler = cos.to_rpy();                   // XYZ sequence

// Matrix product: takes references; the vector element type must implement
// VectorQuantity (e.g. Coef)
let cos_a = CosMatrix::unit();
let cos_b = CosMatrix::unit();
let cos_c = cos_a.product(&cos_b);
let dir = Vector3::new(Coef::new(1.0), Coef::new(0.0), Coef::new(0.0));
let vec = cos_c.product_vector(&dir);
```

### Quaternions

```rust
let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);

let norm = q.norm();
let normalized = q.normalize();
let conjugate = q.conjugate();
let inverse = q.inverse();
let cos = q.to_cos_matrix();

let q1 = Quaternion::new(1.0, 0.0, 0.0, 0.0);
let q2 = Quaternion::new(0.0, 1.0, 0.0, 0.0);
let sum = q1 + q2;
let product = q1 * q2;
let quotient = q1 / q2;
```

---

## Dependencies

```toml
[dependencies]
zmatrix = "0.4"
chrono = "0.4"
```

> Time arithmetic uses `chrono::TimeDelta`, so `chrono` must be in scope.
> The helpers `time_delta_from_secs_f64` / `time_delta_to_secs_f64` let you avoid
> importing `chrono` directly.

---

## License

MIT
