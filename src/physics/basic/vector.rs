mod acceleration;
mod angular;
mod angular_acceleration;
mod angular_momentum;
mod angular_velocity;
mod coef;
mod distance;
mod force;
mod magnetic_angular_velocity;
mod magnetic_induction;
mod magnetic_moment;
mod megnetic_induction;
mod momentum;
mod torque;
mod velocity;

use super::*;
use crate::dense::Matrix;
use crate::utils::float;
use std::ops::{Add, Mul, Sub};

// 将值限制在 [-1, 1] 区间内，供角度计算使用
pub(crate) fn clamp_to_unit_interval(v: f64) -> f64 {
    if v > 1.0 {
        1.0
    } else if v < -1.0 {
        -1.0
    } else {
        v
    }
}

impl<T: VectorQuantity + Default> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }

    pub fn to_array(&self) -> [f64; 3] {
        [
            self.x.default_unit_value(),
            self.y.default_unit_value(),
            self.z.default_unit_value(),
        ]
    }

    pub fn from_array(arr: [f64; 3]) -> Self {
        let mut v = Self::default();
        v.x.set_value(arr[0]);
        v.y.set_value(arr[1]);
        v.z.set_value(arr[2]);
        v
    }

    pub fn from_col_matrix(m: &Matrix<3, 1, f64>) -> Self {
        let mut result = Self::default();
        result.x.set_value(m.get(0, 0).unwrap());
        result.y.set_value(m.get(0, 1).unwrap());
        result.z.set_value(m.get(0, 2).unwrap());
        result
    }
    // 3*3 的反对称矩阵
    pub fn skew_symmetric_matrix(&self) -> Matrix<3, 3, f64> {
        let data = [
            [
                0.0,
                -self.z.default_unit_value(),
                self.y.default_unit_value(),
            ],
            [
                self.z.default_unit_value(),
                0.0,
                -self.x.default_unit_value(),
            ],
            [
                -self.y.default_unit_value(),
                self.x.default_unit_value(),
                0.0,
            ],
        ];
        Matrix::new(data)
    }

    //获取4*4的反对称矩阵
    pub fn skew_symmetric_matrix_4(&self) -> Matrix<4, 4, f64> {
        let pw = self.to_array();
        let data = [
            [0.0, pw[2], -pw[1], pw[0]],
            [-pw[2], 0.0, pw[0], pw[1]],
            [pw[1], -pw[0], 0.0, pw[2]],
            [-pw[0], -pw[1], -pw[2], 0.0],
        ];
        Matrix::new(data)
    }
    // // 通过两个向量叉乘返回与平面垂直的单位向量
    // pub fn cross_unit(&self, rhs: &Vector3<T>) -> Vector3<T> {
    //     let result = self.cross(rhs);
    //     result.normalize()
    // }
    //
    // //通过两个向量叉乘返回与平面垂直的向量
    // pub fn cross(&self, rhs: &Vector3<T>) -> Vector3<T> {
    //     let crsf3 = [
    //         self.y.default_unit_value() * rhs.z.default_unit_value()
    //             - self.z.default_unit_value() * rhs.y.default_unit_value(), // 1维值叉乘值
    //         self.z.default_unit_value() * rhs.x.default_unit_value()
    //             - self.x.default_unit_value() * rhs.z.default_unit_value(), // 2维值叉乘值
    //         self.x.default_unit_value() * rhs.y.default_unit_value()
    //             - self.y.default_unit_value() * rhs.x.default_unit_value(), // 3维值叉乘值
    //     ];
    //     let mut result: Vector3<T> = Vector3::default();
    //     result.x.set_value(crsf3[0]);
    //     result.y.set_value(crsf3[1]);
    //     result.z.set_value(crsf3[2]);
    //     result
    // }

    //向量点积
    pub fn dot(&self, rhs: &Vector3<T>) -> T {
        let mut result = T::default();
        result.set_value(
            self.x.default_unit_value() * rhs.x.default_unit_value()
                + self.y.default_unit_value() * rhs.y.default_unit_value()
                + self.z.default_unit_value() * rhs.z.default_unit_value(),
        );
        result
    }

    // 计算两个三维向量的夹角，返回 Angular。
    // 对于任意可作为三维向量分量的物理量（同/不同量纲均可），
    // 该实现使用无量化的默认单位值进行计算，不影响量纲系统。
    pub fn angle_with<B>(&self, rhs: &Vector3<B>) -> Angular
    where
        B: VectorQuantity + Default,
    {
        let ax = self.x.default_unit_value();
        let ay = self.y.default_unit_value();
        let az = self.z.default_unit_value();
        let bx = rhs.x.default_unit_value();
        let by = rhs.y.default_unit_value();
        let bz = rhs.z.default_unit_value();

        let dot = ax * bx + ay * by + az * bz;
        let a_norm = (ax * ax + ay * ay + az * az).sqrt();
        let b_norm = (bx * bx + by * by + bz * bz).sqrt();

        if a_norm == 0.0 || b_norm == 0.0 {
            return Angular::from_rad(f64::NAN);
        }

        let cos_theta = clamp_to_unit_interval(dot / (a_norm * b_norm));
        Angular::acos(cos_theta)
    }
}

impl<T: VectorQuantity + Default> Vector3<T> {
    pub fn to_col_matrix(&self) -> Matrix<3, 1, f64> {
        Matrix::new([
            [self.x.default_unit_value()],
            [self.y.default_unit_value()],
            [self.z.default_unit_value()],
        ])
    }

    pub fn norm_square(&self) -> f64 {
        let sum = self.x.default_unit_value() * self.x.default_unit_value()
            + self.y.default_unit_value() * self.y.default_unit_value()
            + self.z.default_unit_value() * self.z.default_unit_value();
        sum
    }

    pub fn norm(&self) -> T {
        let sum = self.x.default_unit_value() * self.x.default_unit_value()
            + self.y.default_unit_value() * self.y.default_unit_value()
            + self.z.default_unit_value() * self.z.default_unit_value();
        let mut t = T::default();
        t.set_value(sum.sqrt());
        t
    }
    // 归一化
    pub fn normalize(&self) -> Vector3<T> {
        let norm = self.norm();
        let mut result = Self::default();
        if norm.is_zero() {
            result.x.set_value(0.0);
            result.y.set_value(0.0);
            result.z.set_value(0.0);
            return result;
        }
        result
            .x
            .set_value(self.x.default_unit_value() / norm.default_unit_value());
        result
            .y
            .set_value(self.y.default_unit_value() / norm.default_unit_value());
        result
            .z
            .set_value(self.z.default_unit_value() / norm.default_unit_value());
        result
    }
}

impl<T: VectorQuantity + Mul<f64, Output = T> + Default> Mul<f64> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: VectorQuantity + Mul<Coef, Output = T> + Default> Mul<Coef> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Coef) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: VectorQuantity + Sub<T, Output = T> + Default> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self { x, y, z }
    }
}
impl<T: VectorQuantity + Add<T, Output = T> + Default> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self { x, y, z }
    }
}

// 引用-引用 与 混合引用 的加减法重载（不消耗操作数）
impl<'a, 'b, T> Add<&'b Vector3<T>> for &'a Vector3<T>
where
    T: VectorQuantity + Add<T, Output = T> + Default + Copy,
{
    type Output = Vector3<T>;

    fn add(self, rhs: &'b Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a, T> Add<&'a Vector3<T>> for Vector3<T>
where
    T: VectorQuantity + Add<T, Output = T> + Default + Copy,
{
    type Output = Vector3<T>;

    fn add(self, rhs: &'a Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a, T> Add<Vector3<T>> for &'a Vector3<T>
where
    T: VectorQuantity + Add<T, Output = T> + Default + Copy,
{
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a, 'b, T> Sub<&'b Vector3<T>> for &'a Vector3<T>
where
    T: VectorQuantity + Sub<T, Output = T> + Default + Copy,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: &'b Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a, T> Sub<&'a Vector3<T>> for Vector3<T>
where
    T: VectorQuantity + Sub<T, Output = T> + Default + Copy,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: &'a Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a, T> Sub<Vector3<T>> for &'a Vector3<T>
where
    T: VectorQuantity + Sub<T, Output = T> + Default + Copy,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: VectorQuantity + Div<f64, Output = T> + Default> Div<f64> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: VectorQuantity + Div<Coef, Output = T> + Default> Div<Coef> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: Coef) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: VectorQuantity + Mul<f64, Output = T> + Default> Mul<Vector3<T>> for f64 {
    type Output = Vector3<T>;
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        rhs * self
    }
}

impl<T: VectorQuantity + Div<f64, Output = T> + Default> Div<Vector3<T>> for f64
where
    f64: Div<T, Output = T>,
{
    type Output = Vector3<T>;
    fn div(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl<T> Vector3<T>
where
    T: VectorQuantity + Default + Copy,
{
    pub fn cross<B, F>(&self, rhs: &Vector3<B>) -> Vector3<F>
    where
        T: Mul<B, Output = F> + Sub<T, Output = T>,
        B: VectorQuantity + Default + Copy,
        F: VectorQuantity + Default + Copy + Sub<F, Output = F>,
    {
        let x = self.y.default_unit_value() * rhs.z.default_unit_value()
            - self.z.default_unit_value() * rhs.y.default_unit_value();
        let y = self.z.default_unit_value() * rhs.x.default_unit_value()
            - self.x.default_unit_value() * rhs.z.default_unit_value();
        let z = self.x.default_unit_value() * rhs.y.default_unit_value()
            - self.y.default_unit_value() * rhs.x.default_unit_value();
        let mut f_x = F::default();
        let mut f_y = F::default();
        let mut f_z = F::default();
        f_x.set_value(x);
        f_y.set_value(y);
        f_z.set_value(z);
        Vector3::new(f_x, f_y, f_z)
    }

    pub fn cross_unit<B, F>(&self, rhs: &Vector3<B>) -> Vector3<F>
    where
        T: Mul<B, Output = F> + Sub<T, Output = T>,
        B: VectorQuantity + Default + Copy,
        F: VectorQuantity + Default + Copy + Sub<F, Output = F>,
    {
        let result = self.cross(rhs);
        println!("{:?}", result.to_array());
        result.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::basic::MagneticInduction;
    use approx::assert_relative_eq;
    #[test]
    fn test_new() {
        let vec = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        assert_eq!(vec.x, Distance::from_m(1.0));
        assert_eq!(vec.y, Distance::from_m(2.0));
        assert_eq!(vec.z, Distance::from_m(3.0));
    }

    #[test]
    fn test_mul() {
        let vec = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        let vec2 = vec * 2.0;
        assert_eq!(vec2.x, Distance::from_m(2.0));
        assert_eq!(vec2.y, Distance::from_m(4.0));
        assert_eq!(vec2.z, Distance::from_m(6.0));

        let vec = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        let vec2 = vec * Coef::new(2.0);
        assert_eq!(vec2.x, Distance::from_m(2.0));
        assert_eq!(vec2.y, Distance::from_m(4.0));
        assert_eq!(vec2.z, Distance::from_m(6.0));
    }

    #[test]
    fn test_sub() {
        let vec = Vector3::new(
            Distance::from_m(10.0),
            Distance::from_m(6.0),
            Distance::from_m(2.0),
        );
        let vec2 = Vector3::new(
            Distance::from_m(8.0),
            Distance::from_m(3.0),
            Distance::from_m(0.0),
        );
        let vec3 = vec - vec2;
        assert_eq!(vec3.x, Distance::from_m(2.0));
        assert_eq!(vec3.y, Distance::from_m(3.0));
        assert_eq!(vec3.z, Distance::from_m(2.0));
    }

    #[test]
    fn test_add() {
        let vec = Vector3::new(
            Distance::from_m(10.0),
            Distance::from_m(6.0),
            Distance::from_m(2.0),
        );
        let vec2 = Vector3::new(
            Distance::from_m(8.0),
            Distance::from_m(3.0),
            Distance::from_m(0.0),
        );
        let vec3 = vec + vec2;
        assert_eq!(vec3.x, Distance::from_m(18.0));
        assert_eq!(vec3.y, Distance::from_m(9.0));
        assert_eq!(vec3.z, Distance::from_m(2.0));
    }

    #[test]
    fn test_div() {
        let vec = Vector3::new(
            Distance::from_m(10.0),
            Distance::from_m(6.0),
            Distance::from_m(2.0),
        );
        let vec2 = vec / 2.0;
        assert_eq!(vec2.x, Distance::from_m(5.0));
        assert_eq!(vec2.y, Distance::from_m(3.0));
        assert_eq!(vec2.z, Distance::from_m(1.0));

        let vec = Vector3::new(
            Distance::from_m(10.0),
            Distance::from_m(6.0),
            Distance::from_m(2.0),
        );
        let vec2 = vec / Coef::new(2.0);
        assert_eq!(vec2.x, Distance::from_m(5.0));
        assert_eq!(vec2.y, Distance::from_m(3.0));
        assert_eq!(vec2.z, Distance::from_m(1.0));
    }

    #[test]
    fn test_is_zero() {
        let vec = Vector3::new(
            Distance::from_m(0.0),
            Distance::from_m(0.0),
            Distance::from_m(0.0),
        );
        assert!(vec.is_zero());
    }

    #[test]
    fn test_to_matrix() {
        let vec = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        let m = vec.to_col_matrix();
        assert_eq!(m.size(), (3, 1));
        assert_eq!(m.get(0, 0).unwrap(), 1.0);
        assert_eq!(m.get(0, 1).unwrap(), 2.0);
        assert_eq!(m.get(0, 2).unwrap(), 3.0);
    }

    #[test]
    fn test_norm() {
        let vec = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        let distance = vec.norm();
        assert_relative_eq!(distance.as_m(), 14.0f64.sqrt());
    }

    #[test]
    fn test_to_array() {
        let vec = Vector3::new(
            Distance::from_km(1.0),
            Distance::from_km(2.0),
            Distance::from_km(3.0),
        );
        let array = vec.to_array();
        assert_eq!(array[0], 1000.0);
        assert_eq!(array[1], 2000.0);
        assert_eq!(array[2], 3000.0);
    }

    #[test]
    fn test_normalization_vector_f3() {
        let expected = [0.101015, 0.404061, 0.909137];
        let mut input = [1f32, 4f32, 9f32];
        let vec = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(4.0),
            Distance::from_m(9.0),
        );
        let result = vec.normalize();
        for i in 0..3 {
            assert_relative_eq!(result.to_array()[i], expected[i], epsilon = 1e-6);
        }
    }

    #[test]
    fn test_skew_symmetric_matrix_4() {
        let vec: Vector3<Angular> = Vector3::new(
            Angular::from_rad(0.00985752232),
            Angular::from_rad(0.0102456128),
            Angular::from_rad(0.00993415248),
        );
        let result = vec.skew_symmetric_matrix_4();
        let expected = [
            0.0,
            0.00993415248,
            -0.0102456128,
            0.00985752232,
            -0.00993415248,
            0.0,
            0.00985752232,
            0.0102456128,
            0.0102456128,
            -0.00985752232,
            0.0,
            0.00993415248,
            -0.00985752232,
            -0.0102456128,
            -0.00993415248,
            0.0,
        ];
        for y in 0..4 {
            for x in 0..4 {
                assert_relative_eq!(
                    result.get(x, y).unwrap(),
                    expected[(y * 4 + x) as usize],
                    epsilon = 1e-6
                );
            }
        }
    }

    #[test]
    fn test_skew_symmetric_matrix() {
        //[-8.20512651e-6, -0.00109738705, -0.00298800273]
        let v = Vector3::new(
            AngularVelocity::from_rad_per_second(-8.20512651e-6),
            AngularVelocity::from_rad_per_second(-0.00109738705),
            AngularVelocity::from_rad_per_second(-0.00298800273),
        );
        let result = v.skew_symmetric_matrix();
        let (height, width) = result.size();
        assert_eq!(height, 3);
        assert_eq!(width, 3);
        let expected = [
            [0.0, 0.0029880027, -0.001097387],
            [-0.0029880027, 0.0, 8.2051265e-6],
            [0.001097387, -8.2051265e-6, 0.0],
        ];
        for (row, list) in expected.into_iter().enumerate() {
            for (col, val) in list.into_iter().enumerate() {
                assert_relative_eq!(result.get(col, row).unwrap(), val, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_from_col_matrix() {
        let m = Matrix::<3, 1, f64>::new([[1.0], [2.0], [3.0]]);
        let result: Vector3<Coef> = Vector3::from_col_matrix(&m);
        assert_relative_eq!(result.x.get_value(), 1.0, epsilon = 1e-6);
        assert_relative_eq!(result.y.get_value(), 2.0, epsilon = 1e-6);
        assert_relative_eq!(result.z.get_value(), 3.0, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_cross_unit3f() {
        let vx1 = Vector3::new(Coef::new(1.0), Coef::new(3.0), Coef::new(4.0)); //  [1., 3., 4.];
        let vy2 = Vector3::new(Coef::new(2.0), Coef::new(8.0), Coef::new(9.0));

        let result = vx1.cross_unit(&vy2);

        let expected = [-0.91287093, -0.1825742, 0.36514837];
        assert_relative_eq!(result.x.get_value(), expected[0], epsilon = 0.00001);
        assert_relative_eq!(result.y.get_value(), expected[1], epsilon = 0.00001);
        assert_relative_eq!(result.z.get_value(), expected[2], epsilon = 0.00001);

        let vx1 = Vector3::new(Coef::new(1.0), Coef::new(3.0), Coef::new(4.0)); //  [1., 3., 4.];
        let vy2 = Vector3::new(Coef::new(2.0), Coef::new(6.0), Coef::new(8.0));

        let result = vx1.cross_unit(&vy2);

        let expected = [0.0, 0.0, 0.0];
        assert_relative_eq!(result.x.get_value(), expected[0], epsilon = 0.00001);
        assert_relative_eq!(result.y.get_value(), expected[1], epsilon = 0.00001);
        assert_relative_eq!(result.z.get_value(), expected[2], epsilon = 0.00001);


    }

    #[test]
    fn test_vector_ref_ops() {
        let a: Vector3<Coef> = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let b: Vector3<Coef> = Vector3::new(Coef::new(4.0), Coef::new(5.0), Coef::new(6.0));
        // &a + &b
        let s = &a + &b;
        assert_relative_eq!(s.x.get_value(), 5.0);
        assert_relative_eq!(s.y.get_value(), 7.0);
        assert_relative_eq!(s.z.get_value(), 9.0);

        // &a - &b
        let d = &a - &b;
        assert_relative_eq!(d.x.get_value(), -3.0);
        assert_relative_eq!(d.y.get_value(), -3.0);
        assert_relative_eq!(d.z.get_value(), -3.0);

        // (&a).cross(&b) 不消耗 a
        let c: Vector3<Coef> = (&a).cross(&b);
        let arr = c.to_array();
        assert_relative_eq!(arr[0], -3.0);
        assert_relative_eq!(arr[1], 6.0);
        assert_relative_eq!(arr[2], -3.0);

        // 混合引用形式
        let s2 = &a + b;
        assert_relative_eq!(s2.x.get_value(), 5.0);
        let s3 = a + &b;
        assert_relative_eq!(s3.y.get_value(), 7.0);
        let d2 = &a - b;
        assert_relative_eq!(d2.z.get_value(), -3.0);
        let d3 = a - &b;
        assert_relative_eq!(d3.x.get_value(), -3.0);
    }

    #[test]
    fn test_from_array() {
        let arr = [1.0, 2.0, 3.0];
        let v: Vector3<Coef> = Vector3::from_array(arr);
        assert_eq!(v.x.get_value(), 1.0);
        assert_eq!(v.y.get_value(), 2.0);
        assert_eq!(v.z.get_value(), 3.0);
    }

    #[test]
    fn test_dot() {
        let a: Vector3<Coef> = Vector3::new(Coef::new(1.0), Coef::new(3.0), Coef::new(4.0));
        let b: Vector3<Coef> = Vector3::new(Coef::new(2.0), Coef::new(6.0), Coef::new(8.0));
        let result = a.dot(&b);
        assert_eq!(result.get_value(), 52.0);
    }

    #[test]
    fn test_angle_with_basic_cases() {
        // 同向
        let a: Vector3<Coef> = Vector3::new(Coef::new(1.0), Coef::new(0.0), Coef::new(0.0));
        let b: Vector3<Coef> = Vector3::new(Coef::new(2.0), Coef::new(0.0), Coef::new(0.0));
        let theta = a.angle_with(&b);
        assert_relative_eq!(theta.as_rad(), 0.0, epsilon = 1e-10);

        // 反向
        let a: Vector3<Coef> = Vector3::new(Coef::new(1.0), Coef::new(0.0), Coef::new(0.0));
        let b: Vector3<Coef> = Vector3::new(Coef::new(-2.0), Coef::new(0.0), Coef::new(0.0));
        let theta = a.angle_with(&b);
        assert_relative_eq!(theta.as_deg(), 180.0, epsilon = 1e-8);

        // 正交
        let a: Vector3<Coef> = Vector3::new(Coef::new(1.0), Coef::new(0.0), Coef::new(0.0));
        let b: Vector3<Coef> = Vector3::new(Coef::new(0.0), Coef::new(2.0), Coef::new(0.0));
        let theta = a.angle_with(&b);
        assert_relative_eq!(theta.as_deg(), 90.0, epsilon = 1e-8);

        // 任一零向量 -> NaN
        let a: Vector3<Coef> = Vector3::new(Coef::new(0.0), Coef::new(0.0), Coef::new(0.0));
        let b: Vector3<Coef> = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let theta = a.angle_with(&b);
        assert!(theta.as_rad().is_nan());
    }

    #[test]
    fn test_angle_with_cross_dimension() {
        // 不同量纲也允许计算夹角（使用默认单位值进行计算）
        let a: Vector3<Distance> = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(0.0),
            Distance::from_m(0.0),
        );
        let b: Vector3<MagneticInduction> = Vector3::new(
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_tesla(0.0),
        );
        let theta = a.angle_with(&b);
        assert_relative_eq!(theta.as_deg(), 90.0, epsilon = 1e-8);
    }

    #[test]
    fn test_clamp_to_unit_interval_branches() {
        // 命中 > 1.0 分支
        assert_relative_eq!(clamp_to_unit_interval(1.0000001), 1.0, epsilon = 0.0);
        // 命中 < -1.0 分支
        assert_relative_eq!(clamp_to_unit_interval(-1.0000001), -1.0, epsilon = 0.0);
        // 中间值保持不变
        assert_relative_eq!(clamp_to_unit_interval(0.5), 0.5, epsilon = 0.0);
        assert_relative_eq!(clamp_to_unit_interval(-0.5), -0.5, epsilon = 0.0);
        // 边界值
        assert_relative_eq!(clamp_to_unit_interval(1.0), 1.0, epsilon = 0.0);
        assert_relative_eq!(clamp_to_unit_interval(-1.0), -1.0, epsilon = 0.0);
    }

    #[test]
    fn test_vector3_magnetic_induction_add_sub() {
        // 创建两个MagneticInduction向量
        let vec1 = Vector3::new(
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_gauss(100.0),
            MagneticInduction::from_mill_tesla(500.0),
        );

        let vec2 = Vector3::new(
            MagneticInduction::from_tesla(2.0),
            MagneticInduction::from_gauss(200.0),
            MagneticInduction::from_mill_tesla(1000.0),
        );

        // 测试加法
        let vec_add = vec1 + vec2;
        assert_relative_eq!(vec_add.x.as_tesla(), 3.0, epsilon = 1e-8);
        assert_relative_eq!(vec_add.y.as_gauss(), 300.0, epsilon = 1e-8);
        assert_relative_eq!(vec_add.z.as_milli_tesla(), 1500.0, epsilon = 1e-8);

        // 测试减法
        let vec_sub = vec2 - vec1;
        assert_relative_eq!(vec_sub.x.as_tesla(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(vec_sub.y.as_gauss(), 100.0, epsilon = 1e-8);
        assert_relative_eq!(vec_sub.z.as_milli_tesla(), 500.0, epsilon = 1e-8);

        // 测试不同类型单位的混合运算
        let vec3 = Vector3::new(
            MagneticInduction::from_micro_tesla(1000000.0), // 1 Tesla
            MagneticInduction::from_kilo_gauss(1.0),        // 1000 Gauss
            MagneticInduction::from_nano_tesla(1000000000.0), // 1 Tesla
        );

        let vec_mixed = vec1 + vec3;
        assert_relative_eq!(vec_mixed.x.as_tesla(), 2.0, epsilon = 1e-8);
        assert_relative_eq!(vec_mixed.y.as_gauss(), 1100.0, epsilon = 1e-8);
        assert_relative_eq!(vec_mixed.z.as_tesla(), 1.5, epsilon = 1e-8);
    }

    #[test]
    fn test_vector3_f64_operations() {
        let v = Vector3::new(
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_tesla(2.0),
            MagneticInduction::from_tesla(3.0),
        );

        // 测试 Vector3<T> * f64
        let result1 = v * 2.0;
        assert_relative_eq!(result1.x.as_tesla(), 2.0);
        assert_relative_eq!(result1.y.as_tesla(), 4.0);
        assert_relative_eq!(result1.z.as_tesla(), 6.0);

        // 测试 Vector3<T> / f64
        let result2 = v / 2.0;
        assert_relative_eq!(result2.x.as_tesla(), 0.5);
        assert_relative_eq!(result2.y.as_tesla(), 1.0);
        assert_relative_eq!(result2.z.as_tesla(), 1.5);

        // 测试 f64 * Vector3<T> (现在应该支持了)
        let result3 = 2.0 * v;
        assert_relative_eq!(result3.x.as_tesla(), 2.0);
        assert_relative_eq!(result3.y.as_tesla(), 4.0);
        assert_relative_eq!(result3.z.as_tesla(), 6.0);

        // 测试 f64 / Vector3<T> (现在应该支持了)
        let result4 = 6.0 / v;
        assert_relative_eq!(result4.x.as_tesla(), 6.0);
        assert_relative_eq!(result4.y.as_tesla(), 3.0);
        assert_relative_eq!(result4.z.as_tesla(), 2.0);

        // 验证正向操作正常工作
        assert!(result1.x.as_tesla() == 2.0);
        assert!(result2.x.as_tesla() == 0.5);
    }

    #[test]

    fn test_partial_eq() {
        let vec1 = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );

        let vec2 = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        let vec3 = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(4.0),
        );

        // 测试相等
        assert_eq!(vec1, vec2);

        // 测试不相等
        assert_ne!(vec1, vec3);
        assert_ne!(vec2, vec3);

        // 测试自反性
        assert_eq!(vec1, vec1);
        assert_eq!(vec2, vec2);
        assert_eq!(vec3, vec3);
    }

    fn test_norm_square_basic() {
        // 测试基本功能：计算向量的模平方
        let vec = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        let result = vec.norm_square();
        assert_relative_eq!(result, 14.0);
    }
    #[test]
    fn test_norm_square_zero_vector() {
        // 测试零向量的模平方
        let vec = Vector3::new(
            Distance::from_m(0.0),
            Distance::from_m(0.0),
            Distance::from_m(0.0),
        );
        let result = vec.norm_square();
        assert_relative_eq!(result, 0.0);
    }

    #[test]
    fn test_norm_square_negative_components() {
        // 测试含负分量的向量模平方
        let vec = Vector3::new(
            Distance::from_m(-1.0),
            Distance::from_m(-2.0),
            Distance::from_m(-3.0),
        );
        let result = vec.norm_square();
        assert_relative_eq!(result, 14.0);
    }
}
