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

36 physical quantities, each with multiple units. Operations between quantities are dimensionally checked at compile time.

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
| **`Temperature`** | K, °C, °F, °R |
| **`TemperatureDifference`** | ΔK, Δ°C, Δ°F, Δ°R |
| **`HeatFlux`** | W/m², kW/m², mW/cm² |
| **`ThermalConductivity`** | W/(m·K), kW/(m·K), mW/(cm·K) |
| **`HeatCapacity`** | J/K, kJ/K |
| **`ThermalConductance`** | W/K, mW/K |
| **`Density`** | kg/m³, g/cm³, g/L |
| **`Pressure`** | Pa, hPa, kPa, MPa, bar, atm, mmHg |
| **`MassFlux`** | kg/(m²·s), g/(m²·s), g/(cm²·s) |
| **`MassFlowRate`** | kg/s, g/s, kg/h, t/h |
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

### Systems of equations and determinants

```rust
let m = Matrix::<3, 3, f64>::new([
    [4.0, 7.0, 2.0],
    [3.0, 6.0, 1.0],
    [2.0, 5.0, 3.0],
]);

// Solve A·x = b (LU with partial pivoting)
let x: [f64; 3] = m.solve([1.0, 2.0, 3.0])?;

// Determinant: LU is O(N³), the permutation version is O(N!)
let d = m.det_via_lu()?;      // recommended; clearly faster from N ≥ 6
let d2 = m.det()?;            // generic version, small N only

// Explicit inverse (not recommended, see below)
let inv = m.inverse()?;
```

> **`solve` is `f64`-only.** Solving needs division, and integer division truncates
> silently (`5/2 → 2`); Gaussian elimination on `Matrix<i32>` would return plausible
> nonsense. `NumericElement` cannot distinguish "types with real division" from
> "types with truncating division", so only `f64` gets it.
>
> **`inverse` is numerically worse than `solve`**: inverting then multiplying adds
> rounding error and an extra matrix multiply. Solving once is enough for nearly every
> case; it exists because some algorithms genuinely need `A⁻¹` itself.
>
> **Partial pivoting is mandatory**: without it, a pivot near zero amplifies rounding
> error and even a non-singular matrix yields a badly distorted solution.

`Matrix` keeps a **plain-numeric** scope, with no dimensioned version. For `J⁻¹·L` use
`InertiaTensor::solve_omega`, which provides dimensional checking and positive-definiteness
validation.

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
// Cross product: the output type is decided by the CrossProduct trait
let c1 = Vector3::new(Coef::new(1.0), Coef::new(0.0), Coef::new(0.0));
let c2 = Vector3::new(Coef::new(0.0), Coef::new(1.0), Coef::new(0.0));
let cross: Vector3<Coef> = c1.cross(&c2);   // (0, 0, 1)

// Torque: r × F
let r = Vector3::new(Distance::from_m(1.0), Distance::from_m(0.0), Distance::from_m(0.0));
let f = Vector3::new(Force::from_newton(0.0), Force::from_newton(2.0), Force::from_newton(0.0));
let tau: Vector3<Torque> = r.cross(&f);      // 2 N·m along +z

// Skew-symmetric matrices
let skew = v1.skew_symmetric_matrix();    // 3×3
let skew4 = v1.skew_symmetric_matrix_4(); // 4×4
```

> **The cross-product output type comes from `CrossProduct`, not `Mul`.**
>
> Earlier versions derived it from `T: Mul<B, Output = F>`, which made the most natural
> operation unexpressible: `m × B`. Scalar `MagneticMoment * MagneticInduction` is already
> taken by the **dot product** (yielding `Energy`), and Rust forbids two `Mul` impls with
> different `Output`. `Distance × Force` fails the same way (`Mul` yields work).
>
> A dedicated trait replaces it:
>
> ```text
> pub trait CrossProduct<B> {
>     type Out: VectorQuantity + Default;
> }
> ```
>
> Implemented combinations:
>
> | `T` | `B` | `Out` | Meaning |
> |---|---|---|---|
> | `Distance` | `Force` | `Torque` | torque `τ = r × F` |
> | `AngularVelocity` | `AngularMomentum` | `Torque` | gyroscopic torque `ω × L` (Euler's second term) |
> | `MagneticMoment` | `MagneticInduction` | `Torque` | magnetic torque `τ = m × B` |
> | `MagneticInduction` | `AngularVelocity` | `MagneticAngularVelocity` | magnetic angular velocity |
> | `Coef` | `Coef` | `Coef` | direction-vector cross product |
>
> **Undefined combinations fail to compile** rather than silently producing the wrong type.
> Reversed orders (`F × r`, `B × m`) are not provided — they are the same quantity negated,
> and writing `-tau` states the intent.

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
> Full description of a rigid body about any axis needs the 3×3 tensor (with products of
> inertia) — see the next chapter.

---

## 5. Inertia Tensor and the `compound` Module

`compound` holds **structures assembled from physical quantities**, as opposed to
`physics::basic` (single quantities):

| Module | Contents | Element type |
|---|---|---|
| `physics::basic` | single quantities (one value + unit) | — |
| `spatial_geometry` | pure geometry, dimensionless | all `f64` |
| **`compound`** | structures assembled from quantities | dimensioned |

Dependencies are one-way: `compound → physics::basic`, `compound → spatial_geometry`.
The reverse is not allowed.

### `InertiaTensor`

```rust
use zmatrix::compound::inertia_tensor::InertiaTensor;

// Six independent components. Symmetry is guaranteed by the interface —
// an asymmetric tensor cannot be constructed.
let j = InertiaTensor::new(
    Inertia::from_kg_m2(120.0),   // Ixx
    Inertia::from_kg_m2(130.0),   // Iyy
    Inertia::from_kg_m2(45.0),    // Izz
    Inertia::from_kg_m2(0.0),     // Ixy
    Inertia::from_kg_m2(8.0),     // Ixz
    Inertia::from_kg_m2(0.0),     // Iyz
);

// Principal axes / isotropic
let j2 = InertiaTensor::from_diagonal([
    Inertia::from_kg_m2(1.0),
    Inertia::from_kg_m2(2.0),
    Inertia::from_kg_m2(3.0),
]);
let j3 = InertiaTensor::from_isotropic(Inertia::from_kg_m2(2.0));

// Angular momentum: J·ω = L. For an asymmetric tensor, L deviates from ω.
let omega = Vector3::new(
    AngularVelocity::from_rad_per_second(0.0),
    AngularVelocity::from_rad_per_second(0.0),
    AngularVelocity::from_rad_per_second(1.0),
);
let l: Vector3<AngularMomentum> = j * omega;

// Torque: J·ω̇ = M
let alpha = Vector3::new(
    AngularAcceleration::from_rad_per_second2(1.0),
    AngularAcceleration::from_rad_per_second2(0.0),
    AngularAcceleration::from_rad_per_second2(0.0),
);
let m: Vector3<Torque> = j * alpha;

// Inertia about an arbitrary axis: nᵀJn / nᵀn
// (normalized internally, n need not be a unit vector)
let dir = Vector3::new(Coef::new(1.0), Coef::new(1.0), Coef::new(0.0));
let about: Inertia = j.about_axis(dir)?;

// Isotropic equivalent: trace/3
let iso = j.isotropic_equivalent();

// Coordinate transform: R·J·Rᵀ
let rotated = j.rotate(&CosMatrix::unit());
```

> **`about_axis` cannot be replaced by a diagonal element.** For
> `[[2,-1,0],[-1,2,0],[0,0,3]]`, the z-axis value is 3 but the `(1,1,0)/√2` axis gives 1 —
> products of inertia pull it down from 2. Normalization happens inside the method, so a
> non-unit vector gives the same answer; a zero vector returns an error.

### Solving: J⁻¹·L and J⁻¹·τ

Attitude dynamics needs `ω = J⁻¹L` every tick. **Component-wise scalar division is wrong** —
with non-zero products of inertia, every component of `J⁻¹L` depends on all components of `L`:

```text
J = [[120, 8, 0], [8, 130, 0], [0, 0, 45]],  L = (60, 0, 90)

component-wise  ω = (0.500000,  0.000000, 2.0)   J·ω = (60, 4.0, 90)  ← 4 invented
correct         ω = (0.502060, -0.030896, 2.0)   J·ω = (60, 0.0, 90)  ✓
```

That injects 4 N·m·s of fictitious angular momentum every tick, accumulating into visible
attitude drift over a 90-minute orbit.

```rust
let j = InertiaTensor::from_engineering_array([120.0, 130.0, 45.0, 8.0, 0.0, 0.0]);
let l = Vector3::new(
    AngularMomentum::from_nms(60.0),
    AngularMomentum::from_nms(0.0),
    AngularMomentum::from_nms(90.0),
);
let tau = Vector3::new(
    Torque::from_nm(0.0),
    Torque::from_nm(13.0),
    Torque::from_nm(0.0),
);

// ω = J⁻¹·L (rad/s)
let omega = j.solve_omega(l)?;

// ω̇ = J⁻¹·τ (rad/s²)
let alpha = j.solve_alpha(tau)?;

// Round-trip: J·ω must restore L
let back = j * omega;
assert!((back.x.as_nms() - 60.0).abs() < 1e-9);
```

**Cholesky, not LU**, because symmetric positive-definiteness is a *physical* property of an
inertia tensor:

- half the operations of general LU, no pivoting needed
- during decomposition `L_ii² ≤ 0` is exactly the non-PD condition, so **the
  positive-definiteness check comes free**
- non-PD input (negative mass distribution, products of inertia out of range) returns
  `OperationError` rather than silently returning a zero vector

### Principal-axis diagonalization

```rust
let j = InertiaTensor::from_engineering_array([120.0, 130.0, 45.0, 8.0, 0.0, 0.0]);
let (principal, rot) = j.diagonalize();
// principal: [Inertia; 3], ascending
// rot:       CosMatrix, columns = principal axis directions in the body frame
assert!((principal[0].as_kg_m2() + principal[1].as_kg_m2() + principal[2].as_kg_m2()
    - 295.0).abs() < 1e-8);
```

Satisfies `diag(J1,J2,J3) = Rᵀ · J · R`.

> **The direction convention matters**: `rot`'s **columns** are the axis directions,
> representing a **body → principal** transform. To take a body-frame vector into the
> principal frame, use `rot.transfer()` (i.e. `Rᵀ`); to go back, use `rot`.
> **Getting this backwards is a classic attitude-dynamics bug, and it does not error —
> the result is just transposed.**

**Jacobi sweeps rather than the closed-form cubic roots**: the analytic roots are unstable
for nearly-degenerate eigenvalues, where a near-zero discriminant amplifies rounding error.
Jacobi converges unconditionally for symmetric matrices.

Degenerate cases (two or three equal principal inertias) leave the axes non-unique; an
arbitrary orthonormal basis is returned rather than an error, because infinitely many
solutions genuinely exist.

### Parallel-axis theorem and the FFI sign convention

```rust
// J' = J_cm + m·(|d|²·I − d⊗d)
let j = InertiaTensor::from_engineering_array([120.0, 130.0, 45.0, 8.0, 0.0, 0.0]);
let offset = Vector3::new(
    Distance::from_m(1.5),
    Distance::from_m(0.0),
    Distance::from_m(0.0),
);
let shifted = j.parallel_axis(Mass::from_kg(100.0), offset);
// about z: 45 + 100×1.5² = 270
assert!((shifted.get(2, 2).unwrap().as_kg_m2() - 270.0).abs() < 1e-9);
```

**The sign convention is a hard constraint at the FFI boundary.** The tensor definition
`J_ij = -∫xᵢxⱼ dm` means off-diagonal entries have the **opposite sign** to engineering raw
values (the `+Ixy` convention). aods' `mass_sim_para.J_kgmm2[9]` follows the engineering
convention; passing it straight to `new` flips every product of inertia — silently.

So there is one entry point that owns the convention:

```rust
// engineering raw [Ixx, Iyy, Izz, Ixy, Ixz, Iyz] → off-diagonals negated internally
let j = InertiaTensor::from_engineering_array([120.0, 130.0, 45.0, 8.0, 0.0, 0.0]);

// write back to FFI
let raw: [f64; 6] = j.to_engineering_array();
```

**Do not re-derive "negate here" at each call site** — someone will forget.

### Gyroscopic torque and Euler's equation

The point of the tensor: `L` can deviate from `ω`, so `ω × L` is nonzero.

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

// Euler: M = J·ω̇ + ω × (Jω)
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

**For rotation about a principal axis `ω × L` is zero** — which is why an
isotropic body admits a scalar model. With `Ixx ≠ Izz` and both axes turning,
the term is necessarily nonzero and no scalar model can express it.

### Magnetic dipole: torque and potential energy

Two quantities, and their **sign conventions are easy to get wrong**:

| Quantity | Formula | Type | Access |
|---|---|---|---|
| torque | `τ = m × B` | `Vector3<Torque>` | `cross` or `magnetic_torque` |
| potential energy | `U = -m · B` | `Energy` | `potential_energy` (**negative**) |

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

// Torque: both forms are equivalent
let tau: Vector3<Torque> = m.cross(&b);
let tau2 = m.magnetic_torque(&b);

// Potential energy: aligned gives -3.6e-4 J (minimum), anti-aligned +3.6e-4 J (maximum)
let u = m.potential_energy(&b);
```

> **The `m * B` operator is deliberately not provided.** The dot product's physical
> meaning is potential energy `U = -m·B`, which carries a minus sign; as `Mul`, `m * b`
> would yield `+m·B` — opposite to the real quantity, and silently so. A named function
> can state the sign in its name and documentation; an operator cannot.
>
> Relations: `U = -|m||B|cosθ`, `|τ| = |m||B|sinθ`. The zero-torque orientations are
> exactly the extrema of the potential, i.e. the stable and unstable equilibria.

---

## 6. Temperature and Thermal Quantities

### Temperature and temperature difference must be separate types

Celsius and kelvin are related by an **affine** transform with an offset
(`T_K = T_C + 273.15`), while differences scale linearly with no offset:

```text
20 °C  = 293.15 K          temperature has an offset
Δ20 °C = 20 K              difference does not
```

Collapsing them into one type has two consequences:

- `(t1 - t2).as_celsius()` returns nonsense like `-253.15`
- `G * T` and `G * ΔT` both typecheck (`W/K × K = W`) but differ by tens of times, with
  **dimensional analysis unable to tell them apart**

So `Temperature` and `TemperatureDifference` are separate, and these operations are
deliberately absent:

| Operation | Allowed | Why |
|---|---|---|
| `T - T → ΔT` | ✅ | the only meaningful subtraction |
| `T + ΔT → T` | ✅ | heating / cooling (commutative) |
| `T / T → Coef` | ✅ | temperature ratio, e.g. Carnot efficiency |
| `T + T` | ❌ | sum of two temperatures is not a temperature |
| `T × T` | ❌ | output would be K², which has no use here |
| `T × f64` | ❌ | misleading from Celsius: 20 °C doubled gives 313 °C |

```rust
let t1 = Temperature::from_celsius(30.0);
let t2 = Temperature::from_celsius(20.0);
let dt: TemperatureDifference = t1 - t2;    // 10 K
let heated = t2 + dt;                       // 30 °C
let ratio: Coef = t1 / t2;                  // based on kelvin, not 30/20
```

> Temperature is a scalar, so `Vector3<Temperature>` is unconstructable at compile time.

### Heat flux and Fourier's law

**Heat flux density** `HeatFlux` (W/m²) is thermal power per unit area. Distinct from two
easily confused quantities:

| Quantity | Unit | Meaning | Type |
|---|---|---|---|
| Heat Q | J | accumulated energy | `Energy` |
| Thermal power Q̇ | W | energy per time | `Power` |
| Heat flux q | W/m² | energy per time per area | `HeatFlux` |

```rust
let q = HeatFlux::from_w_per_m2(1361.0);        // solar constant
let p: Power = q * Area::from_m2(2.0);          // 2722 W

// Datasheets often use mW/cm²: 1 mW/cm² = 10 W/m² (not 1e-3)
let q2 = HeatFlux::from_mw_per_cm2(1.0);
assert_eq!(q2.as_w_per_m2(), 10.0);
```

**Thermal conductivity** `ThermalConductivity` (W/(m·K)) is a material property.
Fourier's law:

```rust
let k = ThermalConductivity::from_w_per_m_k(237.0);   // aluminium
let dt = TemperatureDifference::from_celsius(20.0);
let thickness = Distance::from_m(0.01);

let q: HeatFlux = k.heat_flux(dt, thickness);          // 474 kW/m²

// Inverse — calibrate k from a measured heat flux
let k2 = ThermalConductivity::from_flux(q, dt, thickness);
```

> Takes `TemperatureDifference`, not `Temperature`. Passing an absolute temperature is a
> compile error — otherwise you get a heat flux measured from 0 K.

### The two elements of a lumped-parameter thermal network

$$C\frac{dT}{dt} = G\,\Delta T + \dot{Q}$$

| Symbol | Type | Unit | Meaning |
|---|---|---|---|
| C | `HeatCapacity` | J/K | node storage |
| G | `ThermalConductance` | W/K | coupling between nodes |
| ΔT | `TemperatureDifference` | K | temperature difference |
| Q̇ | `Power` | W | external source |

```rust
// Storage: Q = C·ΔT
let c = HeatCapacity::from_j_per_k(9000.0);
let q: Energy = c * TemperatureDifference::from_celsius(20.0);   // 180 kJ

// Inverse: ΔT = Q/C
let dt: TemperatureDifference = q / c;

// Transmission: Q̇ = G·ΔT
let g = ThermalConductance::from_w_per_k(12.0);
let p: Power = g * TemperatureDifference::from_celsius(20.0);    // 240 W
```

> **Four similar names, two layers** (all distinct):
>
> | Layer | Storage | Transmission |
> |---|---|---|
> | Material property | specific heat capacity c, J/(kg·K) | thermal conductivity k, W/(m·K) |
> | Whole object | **heat capacity C, J/K** | **thermal conductance G, W/K** |
>
> Relations: `C = c·m`, `G = k·A/L`.

---

## 7. Fluid Quantities: Density, Pressure, Mass Flux

The two main paths of a force model: **aerodynamic drag** `F = ½ρv²·C_D·A` and
**solar radiation pressure** `F = P·A·C_R`.

### Why the dimensional chain is split in three

`ρv²` has dimensions `kg/m³ × m/s × m/s = kg/(m·s²) = Pa`. Without an intermediate
quantity you would need either a ternary rule "density × velocity × velocity → pressure"
or a named function that skips `ρv` entirely.

But `ρv` (mass flux) is a physical quantity in its own right — momentum equations,
boundary layers, and ablation all use it:

```rust
let rho = Density::from_kg_per_m3(1.225);     // sea-level air
let v = Velocity::from_m_per_sec(200.0);

let mf: MassFlux = rho * v;                   // ρv  = 245 kg/(m²·s)
let q: Pressure = mf * v;                     // ρv² = 49000 Pa (no ½)

// full dynamic pressure (with ½)
let q_half = rho.dynamic_pressure(v);         // 24500 Pa
```

| Operation | Result | Meaning |
|---|---|---|
| `Density × Velocity` | `MassFlux` | mass flux `ρv` |
| `MassFlux × Velocity` | `Pressure` | `ρv²` (**no ½**) |
| `MassFlux / Velocity` | `Density` | density from flux |
| `Pressure / Velocity` | `MassFlux` | flux from dynamic pressure |

> **Operators never carry physical coefficients.** `ρ * v * v` gives `ρv²`; the `½` only
> comes from the named methods `dynamic_pressure` / `drag_force`. Same stance as
> `potential_energy` carrying its minus sign: operators handle dimensions, physical
> coefficients belong to names.

### The two force-model paths

```rust
// Aerodynamic drag: sea level, 200 m/s, C_D = 1.2, 2 m² frontal area
let rho = Density::from_kg_per_m3(1.225);
let v = Velocity::from_m_per_sec(200.0);
let drag = rho.drag_force(v, Coef::new(1.2), Area::from_m2(2.0));   // 58800 N

// Solar radiation pressure (1 AU; C_R is the reflectivity coefficient)
let f_solar = Pressure::from_pa(4.56e-6) * Area::from_m2(10.0) * Coef::new(1.5);
```

### Three unit-conversion traps

```text
1 g/cm³     = 1000   kg/m³      density, materials
1 g/L       = 1      kg/m³      density, gases
1 g/(cm²·s) = 10     kg/(m²·s)  mass flux

1 bar = 1e5    Pa
1 atm = 101325 Pa               differs by 1.3%, do not mix
```

**Both groups are off by 1000× or 1.3%, and the spellings differ by a couple of characters.**

- Density: `g/cm³` and `g/L` differ by 1000×. Materials use the former, gases the latter;
  pick wrong and you are three orders of magnitude off.
- Mass flux: `g/(cm²·s)` has a factor of **10**, not `1e-3` — because the denominator is
  cm² rather than m². Same trap as `HeatFlux`'s `mW/cm² = 10 W/m²`.
- Pressure: `bar` and `atm` differ by 1.325%. In vacuum systems and seal pressure
  differentials that becomes a real error.

> One more precision detail: `760 mmHg` at the conventional value is 101325.014 Pa —
> **not exactly 1 atm**. `Torr` is what equals `1/760 atm`. Engineering treats them as
> equal; high-precision vacuum work should not.

### Relations to other quantities

```rust
// density × volume = mass
let m: Mass = Density::from_kg_per_m3(2700.0) * Volume::from_m3(0.05);   // 135 kg aluminium

// pressure × volume = energy (isobaric work)
let w: Energy = Pressure::from_atm(1.0) * Volume::from_m3(1.0);          // 101325 J

// force ÷ area = pressure
let p: Pressure = Force::from_newton(1000.0) / Area::from_m2(2.0);
```

---

## 8. Spatial Geometry

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

## Not yet implemented

Ordered by current priority. **Deliberately out of scope** items are listed last so they do
not get re-litigated.

### Linear algebra (done)

`dense::Matrix` now has `det` / `det_via_lu` / `solve` / `inverse` / `lu_decompose`,
all `f64`-only (see "Systems of equations and determinants" above).

`InertiaTensor` now has `solve_omega` / `solve_alpha` (Cholesky) / `diagonalize` (Jacobi) /
`parallel_axis` / `from_engineering_array` / `to_engineering_array`.

Remaining gaps:

| Missing | Impact |
|---|---|
| eigenvalue solving for `N > 3` | flexible-mode analysis; only the 3×3 `diagonalize` exists |
| sparse matrices | large-N steady-state thermal networks would exhaust memory with dense LU |

**The dimensional constraint still holds**: `J⁻¹` needs `(kg·m²)⁻¹` and `det` needs
`(kg·m²)³`, neither of which exists as a quantity. So `Matrix` stays `f64`-only —
which is exactly why it keeps its plain-numeric scope.

### Thermal quantities

| Missing | Unit | Notes |
|---|---|---|
| `SpecificHeatCapacity` | J/(kg·K) | material property, `C = c·m` |
| `HeatTransferCoefficient` | W/(m²·K) | surface convection, `q = h·ΔT`. Rare in vacuum, low priority |
| `ThermalResistance` | K/W | `R = 1/G`, clearer for series paths |
| Radiation helpers | — | `σT⁴` has no place in the type system; functions only: `radiative_flux(ε, T)` |

**Deliberately not planned**:

- `Entropy` (J/K) — unused in satellite thermal work; wait for a concrete need
- `ThermalDiffusivity` (m²/s) — derivable as `α = k/(ρc)`, redundant
- `Heat`, `InternalEnergy` — duplicate `Energy`
- `SpecificHeatRatio` γ — dimensionless quotient, `Coef` suffices; for solids `cp ≈ cv` and
  `γ ≈ 1`, so it carries no information
- `Emissivity` — use `Coef` for now; no value-range constraints exist anywhere in this crate

### Fluid quantities

| Missing | Unit | Notes |
|---|---|---|
| `DynamicViscosity` | Pa·s | Reynolds number `Re = ρvL/μ` |
| `KinematicViscosity` | m²/s | `ν = μ/ρ` |
| `SpecificGasConstant` | J/(kg·K) | ideal gas `P = ρRT` |
| `SpeedOfSound` | m/s | Mach number; `a = √(γRT)` carries γ, so it belongs to a named function |

### `compound` module

| Missing | Notes |
|---|---|
| `InertiaTensor` from a mass distribution | build directly from a point-mass table `Σ mᵢ(r²I − rrᵀ)` |
| `ThermalNetwork` container | see below |

**Explicit stepping only**: implicit stepping needs a time-stepping framework (Newton
iteration plus convergence criteria). `Matrix::solve` now unblocks *steady-state* solving,
but the framework is out of scope.

**Planned `ThermalNetwork` scope** (deliberately narrow):

```text
add_node(capacity, temperature)     add a node
add_link(i, j, conductance)         add a link
set_source(i, power)                attach a heat source
step(dt) -> Result<(), _>           advance one explicit step
temperature(i) -> Option<Temperature>
```

`step()` will check the explicit stability criterion `Δt < 2·Cᵢ/ΣGᵢⱼ` and return an error —
without that check, a diverging solution still produces plausible-looking curves early on,
and by the time it blows up exponentially you are already reading wrong conclusions.

### Other

- eigenvalue solving for `N > 3` (flexible-mode analysis; only the 3×3 `diagonalize` exists)
- sparse matrices (large-N steady-state thermal networks would exhaust memory with dense LU)
- Quantity-to-quantity operators are still incomplete; additions land when a concrete case
  needs them
- The unit system covers a practical engineering subset, not the full SI prefix hierarchy

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
