mod angular;
mod angular_velocity;
mod angular_acceleration;
mod distance;
mod velocity;
mod coef;

use std::ops::{Add, Mul, Sub};
use crate::dense::Matrix;
use crate::utils::float;
use super::*;

impl<T: PhysicalQuantity + Default> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.x.default_unit_value(), self.y.default_unit_value(), self.z.default_unit_value()]
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
        result.y.set_value(m.get(1, 0).unwrap());
        result.z.set_value(m.get(2, 0).unwrap());
        result
    }
    // 3*3 的反对称矩阵
    pub fn skew_symmetric_matrix(&self) -> Matrix<3, 3, f64> {
        let data = [
            0.0, -self.z.default_unit_value(), self.y.default_unit_value(),
            self.z.default_unit_value(), 0.0, -self.x.default_unit_value(),
            -self.y.default_unit_value(), self.x.default_unit_value(), 0.0,
        ];
        Matrix::new(data)
    }

    //获取4*4的反对称矩阵
    pub fn skew_symmetric_matrix_4(&self) -> Matrix<4, 4, f64> {
        let pw = self.to_array();
        let data = [
            0.0, pw[2], -pw[1], pw[0],
            -pw[2], 0.0, pw[0], pw[1],
            pw[1], -pw[0], 0.0, pw[2],
            -pw[0], -pw[1], -pw[2], 0.0
        ];
        Matrix::new(data)
    }
    // 通过两个向量叉乘返回与平面垂直的单位向量
    pub fn cross_unit(&self, rhs: &Vector3<T>) -> Vector3<T> {
        let result = self.cross(rhs);
        result.normalize()
    }

    //通过两个向量叉乘返回与平面垂直的向量
    pub fn cross(&self, rhs: &Vector3<T>) -> Vector3<T> {
        let crsf3 = [
            self.y.default_unit_value() * rhs.z.default_unit_value() - self.z.default_unit_value() * rhs.y.default_unit_value(), // 1维值叉乘值
            self.z.default_unit_value() * rhs.x.default_unit_value() - self.x.default_unit_value() * rhs.z.default_unit_value(), // 2维值叉乘值
            self.x.default_unit_value() * rhs.y.default_unit_value() - self.y.default_unit_value() * rhs.x.default_unit_value(), // 3维值叉乘值
        ];
        let mut result: Vector3<T> = Vector3::default();
        result.x.set_value(crsf3[0]);
        result.y.set_value(crsf3[1]);
        result.z.set_value(crsf3[2]);
        result
    }

    //向量点积
    pub fn dot(&self, rhs: &Vector3<T>) -> T {
        let mut result = T::default();
        result.set_value(self.x.default_unit_value() * rhs.x.default_unit_value()
            + self.y.default_unit_value() * rhs.y.default_unit_value()
            + self.z.default_unit_value() * rhs.z.default_unit_value());
        result
    }
}


impl<T: PhysicalQuantity + Default> Vector3<T> {
    pub fn to_col_matrix(&self) -> Matrix<3,1,f64> {
        Matrix::new([self.x.default_unit_value(), self.y.default_unit_value(), self.z.default_unit_value()])
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
        result.x.set_value(self.x.default_unit_value() / norm.default_unit_value());
        result.y.set_value(self.y.default_unit_value() / norm.default_unit_value());
        result.z.set_value(self.z.default_unit_value() / norm.default_unit_value());
        result
    }
}

impl<T: PhysicalQuantity + Mul<f64, Output=T> + Default> Mul<f64> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: PhysicalQuantity + Mul<Coef, Output=T> + Default> Mul<Coef> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Coef) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: PhysicalQuantity + Sub<T, Output=T> + Default> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self {
            x,
            y,
            z,
        }
    }
}
impl<T: PhysicalQuantity + Add<T, Output=T> + Default> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self {
            x,
            y,
            z,
        }
    }
}

impl<T: PhysicalQuantity + Div<f64, Output=T> + Default> Div<f64> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: PhysicalQuantity + Div<Coef, Output=T> + Default> Div<Coef> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: Coef) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_new() {
        let vec = Vector3::new(Distance::from_m(1.0), Distance::from_m(2.0), Distance::from_m(3.0));
        assert_eq!(vec.x, Distance::from_m(1.0));
        assert_eq!(vec.y, Distance::from_m(2.0));
        assert_eq!(vec.z, Distance::from_m(3.0));
    }

    #[test]
    fn test_mul() {
        let vec = Vector3::new(Distance::from_m(1.0), Distance::from_m(2.0), Distance::from_m(3.0));
        let vec2 = vec * 2.0;
        assert_eq!(vec2.x, Distance::from_m(2.0));
        assert_eq!(vec2.y, Distance::from_m(4.0));
        assert_eq!(vec2.z, Distance::from_m(6.0));

        let vec = Vector3::new(Distance::from_m(1.0), Distance::from_m(2.0), Distance::from_m(3.0));
        let vec2 = vec * Coef::new(2.0);
        assert_eq!(vec2.x, Distance::from_m(2.0));
        assert_eq!(vec2.y, Distance::from_m(4.0));
        assert_eq!(vec2.z, Distance::from_m(6.0));
    }


    #[test]
    fn test_sub() {
        let vec = Vector3::new(Distance::from_m(10.0), Distance::from_m(6.0), Distance::from_m(2.0));
        let vec2 = Vector3::new(Distance::from_m(8.0), Distance::from_m(3.0), Distance::from_m(0.0));
        let vec3 = vec - vec2;
        assert_eq!(vec3.x, Distance::from_m(2.0));
        assert_eq!(vec3.y, Distance::from_m(3.0));
        assert_eq!(vec3.z, Distance::from_m(2.0));
    }

    #[test]
    fn test_add() {
        let vec = Vector3::new(Distance::from_m(10.0), Distance::from_m(6.0), Distance::from_m(2.0));
        let vec2 = Vector3::new(Distance::from_m(8.0), Distance::from_m(3.0), Distance::from_m(0.0));
        let vec3 = vec + vec2;
        assert_eq!(vec3.x, Distance::from_m(18.0));
        assert_eq!(vec3.y, Distance::from_m(9.0));
        assert_eq!(vec3.z, Distance::from_m(2.0));
    }

    #[test]
    fn test_div() {
        let vec = Vector3::new(Distance::from_m(10.0), Distance::from_m(6.0), Distance::from_m(2.0));
        let vec2 = vec / 2.0;
        assert_eq!(vec2.x, Distance::from_m(5.0));
        assert_eq!(vec2.y, Distance::from_m(3.0));
        assert_eq!(vec2.z, Distance::from_m(1.0));

        let vec = Vector3::new(Distance::from_m(10.0), Distance::from_m(6.0), Distance::from_m(2.0));
        let vec2 = vec / Coef::new(2.0);
        assert_eq!(vec2.x, Distance::from_m(5.0));
        assert_eq!(vec2.y, Distance::from_m(3.0));
        assert_eq!(vec2.z, Distance::from_m(1.0));
    }

    #[test]
    fn test_is_zero() {
        let vec = Vector3::new(Distance::from_m(0.0), Distance::from_m(0.0), Distance::from_m(0.0));
        assert!(vec.is_zero());
    }

    #[test]
    fn test_to_matrix() {
        let vec = Vector3::new(Distance::from_m(1.0), Distance::from_m(2.0), Distance::from_m(3.0));
        let m = vec.to_col_matrix();
        assert_eq!(m.size(), (3, 1));
        assert_eq!(m.get(0, 0).unwrap(), 1.0);
        assert_eq!(m.get(0, 1).unwrap(), 2.0);
        assert_eq!(m.get(0, 2).unwrap(), 3.0);
    }

    #[test]
    fn test_norm() {
        let vec = Vector3::new(Distance::from_m(1.0), Distance::from_m(2.0), Distance::from_m(3.0));
        let distance = vec.norm();
        assert_relative_eq!(distance.as_m(), 14.0f64.sqrt());
    }

    #[test]
    fn test_to_array() {
        let vec = Vector3::new(Distance::from_km(1.0), Distance::from_km(2.0), Distance::from_km(3.0));
        let array = vec.to_array();
        assert_eq!(array[0], 1000.0);
        assert_eq!(array[1], 2000.0);
        assert_eq!(array[2], 3000.0);
    }

    #[test]
    fn test_normalization_vector_f3() {
        let expected = [0.101015, 0.404061, 0.909137];
        let mut input = [1f32, 4f32, 9f32];
        let vec = Vector3::new(Distance::from_m(1.0), Distance::from_m(4.0), Distance::from_m(9.0));
        let result = vec.normalize();
        for i in 0..3 {
            assert_relative_eq!(result.to_array()[i], expected[i],epsilon = 1e-6);
        }
    }

    #[test]
    fn test_skew_symmetric_matrix_4() {
        let vec: Vector3<Angular> = Vector3::new(Angular::from_rad(0.00985752232), Angular::from_rad(0.0102456128), Angular::from_rad(0.00993415248));
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
            0.0];
        for y in 0..4 {
            for x in 0..4 {
                assert_relative_eq!(result.get(x, y).unwrap(), expected[(y * 4 + x) as usize], epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_skew_symmetric_matrix() {
        //[-8.20512651e-6, -0.00109738705, -0.00298800273]
        let v = Vector3::new(AngularVelocity::from_rad_per_second(-8.20512651e-6), AngularVelocity::from_rad_per_second(-0.00109738705), AngularVelocity::from_rad_per_second(-0.00298800273));
        let result = v.skew_symmetric_matrix();
        let (height, width) = result.size();
        assert_eq!(height, 3);
        assert_eq!(width, 3);
        let expected = [[0.0, 0.0029880027, -0.001097387], [-0.0029880027, 0.0, 8.2051265e-6], [0.001097387, -8.2051265e-6, 0.0]];
        for (row, list) in expected.into_iter().enumerate() {
            for (col, val) in list.into_iter().enumerate() {
                assert_relative_eq!(result.get(col, row).unwrap(), val, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_from_col_matrix() {
        let m = Matrix::<3, 1, f64>::new([1.0, 2.0, 3.0]);
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
}
