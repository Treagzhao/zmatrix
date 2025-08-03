use super::*;
use crate::dense::Matrix;
use crate::physics::basic::{Angular, Coef, PhysicalQuantity, Vector3};
use crate::spatial_geometry::quaternion::Quaternion;
use crate::utils::float::sgn2_64;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct CosMatrix {
    data: [f64; 9],
}

impl Default for CosMatrix {
    fn default() -> Self {
        CosMatrix {
            data: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Add<f64> for CosMatrix {
    type Output = CosMatrix;
    fn add(self, rhs: f64) -> Self::Output {
        &self + rhs
    }
}

impl Add<f64> for &CosMatrix {
    type Output = CosMatrix;
    fn add(self, rhs: f64) -> Self::Output {
        CosMatrix {
            data: [
                self.data[0] + rhs,
                self.data[1] + rhs,
                self.data[2] + rhs,
                self.data[3] + rhs,
                self.data[4] + rhs,
                self.data[5] + rhs,
                self.data[6] + rhs,
                self.data[7] + rhs,
                self.data[8] + rhs,
            ],
        }
    }
}

impl Add<CosMatrix> for f64 {
    type Output = CosMatrix;

    fn add(self, rhs: CosMatrix) -> Self::Output {
        rhs + self
    }
}

impl Sub<f64> for CosMatrix {
    type Output = CosMatrix;

    fn sub(self, rhs: f64) -> Self::Output {
        CosMatrix {
            data: [
                self.data[0] - rhs,
                self.data[1] - rhs,
                self.data[2] - rhs,
                self.data[3] - rhs,
                self.data[4] - rhs,
                self.data[5] - rhs,
                self.data[6] - rhs,
                self.data[7] - rhs,
                self.data[8] - rhs,
            ],
        }
    }
}

impl Sub<CosMatrix> for f64 {
    type Output = CosMatrix;

    fn sub(self, rhs: CosMatrix) -> Self::Output {
        CosMatrix {
            data: [
                self - rhs.data[0],
                self - rhs.data[1],
                self - rhs.data[2],
                self - rhs.data[3],
                self - rhs.data[4],
                self - rhs.data[5],
                self - rhs.data[6],
                self - rhs.data[7],
                self - rhs.data[8],
            ],
        }
    }
}

impl Mul<f64> for CosMatrix {
    type Output = CosMatrix;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<f64> for &CosMatrix {
    type Output = CosMatrix;

    fn mul(self, rhs: f64) -> Self::Output {
        CosMatrix {
            data: [
                self.data[0] * rhs,
                self.data[1] * rhs,
                self.data[2] * rhs,
                self.data[3] * rhs,
                self.data[4] * rhs,
                self.data[5] * rhs,
                self.data[6] * rhs,
                self.data[7] * rhs,
                self.data[8] * rhs,
            ],
        }
    }
}
impl Mul<CosMatrix> for f64 {
    type Output = CosMatrix;

    fn mul(self, rhs: CosMatrix) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for CosMatrix {
    type Output = CosMatrix;

    fn div(self, rhs: f64) -> Self::Output {
        CosMatrix {
            data: [
                self.data[0] / rhs,
                self.data[1] / rhs,
                self.data[2] / rhs,
                self.data[3] / rhs,
                self.data[4] / rhs,
                self.data[5] / rhs,
                self.data[6] / rhs,
                self.data[7] / rhs,
                self.data[8] / rhs,
            ],
        }
    }
}

impl Div<CosMatrix> for f64 {
    type Output = CosMatrix;

    fn div(self, rhs: CosMatrix) -> Self::Output {
        CosMatrix {
            data: [
                self / rhs.data[0],
                self / rhs.data[1],
                self / rhs.data[2],
                self / rhs.data[3],
                self / rhs.data[4],
                self / rhs.data[5],
                self / rhs.data[6],
                self / rhs.data[7],
                self / rhs.data[8],
            ],
        }
    }
}

impl Add<CosMatrix> for CosMatrix {
    type Output = CosMatrix;

    fn add(self, rhs: CosMatrix) -> Self::Output {
        &self + &rhs
    }
}

impl Add<&CosMatrix> for &CosMatrix {
    type Output = CosMatrix;

    fn add(self, rhs: &CosMatrix) -> Self::Output {
        CosMatrix {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
                self.data[3] + rhs.data[3],
                self.data[4] + rhs.data[4],
                self.data[5] + rhs.data[5],
                self.data[6] + rhs.data[6],
                self.data[7] + rhs.data[7],
                self.data[8] + rhs.data[8],
            ],
        }
    }
}

impl Sub<CosMatrix> for CosMatrix {
    type Output = CosMatrix;

    fn sub(self, rhs: CosMatrix) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&CosMatrix> for &CosMatrix {
    type Output = CosMatrix;

    fn sub(self, rhs: &CosMatrix) -> Self::Output {
        CosMatrix {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
                self.data[3] - rhs.data[3],
                self.data[4] - rhs.data[4],
                self.data[5] - rhs.data[5],
                self.data[6] - rhs.data[6],
                self.data[7] - rhs.data[7],
                self.data[8] - rhs.data[8],
            ],
        }
    }
}
impl Mul<&CosMatrix> for &CosMatrix {
    type Output = CosMatrix;

    fn mul(self, rhs: &CosMatrix) -> Self::Output {
        CosMatrix {
            data: [
                self.data[0] * rhs.data[0],
                self.data[1] * rhs.data[1],
                self.data[2] * rhs.data[2],
                self.data[3] * rhs.data[3],
                self.data[4] * rhs.data[4],
                self.data[5] * rhs.data[5],
                self.data[6] * rhs.data[6],
                self.data[7] * rhs.data[7],
                self.data[8] * rhs.data[8],
            ],
        }
    }
}

impl Div<CosMatrix> for &CosMatrix {
    type Output = CosMatrix;

    fn div(self, rhs: CosMatrix) -> Self::Output {
        CosMatrix {
            data: [
                self.data[0] / rhs.data[0],
                self.data[1] / rhs.data[1],
                self.data[2] / rhs.data[2],
                self.data[3] / rhs.data[3],
                self.data[4] / rhs.data[4],
                self.data[5] / rhs.data[5],
                self.data[6] / rhs.data[6],
                self.data[7] / rhs.data[7],
                self.data[8] / rhs.data[8],
            ],
        }
    }
}

impl CosMatrix {
    //返回单位方向余弦阵
    pub fn unit() -> Self {
        let data = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        CosMatrix::new(data)
    }
    pub fn new(data: [[f64; 3]; 3]) -> Self {
        CosMatrix {
            data: [
                data[0][0], data[0][1], data[0][2], data[1][0], data[1][1], data[1][2], data[2][0],
                data[2][1], data[2][2],
            ],
        }
    }

    pub fn get_x_vector(&self) -> Vector3<Coef> {
        Vector3::new(
            Coef::new(self.data[0]),
            Coef::new(self.data[3]),
            Coef::new(self.data[6]),
        )
    }

    pub fn get_y_vector(&self) -> Vector3<Coef> {
        Vector3::new(
            Coef::new(self.data[1]),
            Coef::new(self.data[4]),
            Coef::new(self.data[7]),
        )
    }

    pub fn get_z_vector(&self) -> Vector3<Coef> {
        Vector3::new(
            Coef::new(self.data[2]),
            Coef::new(self.data[5]),
            Coef::new(self.data[8]),
        )
    }

    pub fn get_x_col_vector(&self) -> Vector3<Coef> {
        Vector3::new(
            Coef::new(self.data[0]),
            Coef::new(self.data[3]),
            Coef::new(self.data[6]),
        )
    }

    pub fn get_y_col_vector(&self) -> Vector3<Coef> {
        Vector3::new(
            Coef::new(self.data[1]),
            Coef::new(self.data[4]),
            Coef::new(self.data[7]),
        )
    }

    pub fn get_z_col_vector(&self) -> Vector3<Coef> {
        Vector3::new(
            Coef::new(self.data[2]),
            Coef::new(self.data[5]),
            Coef::new(self.data[8]),
        )
    }

    // 矩阵转置
    pub fn transfer(&self) -> Self {
        let mut data = [0.0; 9];
        data[0] = self.data[0];
        data[1] = self.data[3];
        data[2] = self.data[6];
        data[3] = self.data[1];
        data[4] = self.data[4];
        data[5] = self.data[7];
        data[6] = self.data[2];
        data[7] = self.data[5];
        data[8] = self.data[8];
        CosMatrix { data }
    }

    pub fn to_array(&self) -> [[f64; 3]; 3] {
        [
            [self.data[0], self.data[1], self.data[2]],
            [self.data[3], self.data[4], self.data[5]],
            [self.data[6], self.data[7], self.data[8]],
        ]
    }

    //将方向余弦阵变成四元数
    /**
        这个函数的计算结果是有问题的，我现在的单测通过不了，为了进度，我先把这个函数扔下，等后面有时间再改。
    **/
    pub fn to_quaternion(&self) -> Quaternion {
        let p_mat = self.to_array();
        let a44 = 1.0 + p_mat[0][0] + p_mat[1][1] + p_mat[2][2];
        let b22 = 1.0 - p_mat[0][0] + p_mat[1][1] - p_mat[2][2];
        let c11 = 1.0 + p_mat[0][0] - p_mat[1][1] - p_mat[2][2];
        let mut result = Quaternion::default();
        if a44 > 0.004 {
            let tqi = f64::sqrt(a44) / 2.0;
            result.q1 = 0.25 * (p_mat[1][2] - p_mat[2][1]) / tqi;
            result.q2 = 0.25 * (p_mat[2][0] - p_mat[0][2]) / tqi;
            result.q3 = 0.25 * (p_mat[0][1] - p_mat[1][0]) / tqi;
            result.q0 = tqi;
        } else if b22 > 0.004 {
            let isgn = sgn2_64(p_mat[2][0] - p_mat[0][2]);
            let tqi = f64::sqrt(b22) / 2.0 * isgn as f64;

            result.q1 = 0.25 * (p_mat[1][0] + p_mat[0][1]) / tqi;
            result.q2 = tqi;
            result.q3 = 0.25 * (p_mat[2][1] + p_mat[1][2]) / tqi;
            result.q0 = 0.25 * (p_mat[2][0] - p_mat[0][2]) / tqi;
        } else if c11 > 0.004 {
            let isgn = sgn2_64(p_mat[1][2] - p_mat[2][1]);
            let tqi = f64::sqrt(c11) / 2.0 * isgn as f64;

            result.q1 = tqi;
            result.q2 = 0.25 * (p_mat[1][0] + p_mat[0][1]) / tqi;
            result.q3 = 0.25 * (p_mat[0][2] + p_mat[2][0]) / tqi;
            result.q0 = 0.25 * (p_mat[1][2] - p_mat[2][1]) / tqi;
        } else {
            let d33 = 1.0 - p_mat[0][0] - p_mat[1][1] + p_mat[2][2];

            let isgn = sgn2_64(p_mat[0][1] - p_mat[1][0]);
            let tqi = f64::sqrt(d33) / 2.0 * isgn as f64;

            result.q1 = 0.25 * (p_mat[0][2] + p_mat[2][0]) / tqi;
            result.q2 = 0.25 * (p_mat[1][2] + p_mat[2][1]) / tqi;
            result.q3 = tqi;
            result.q0 = 0.25 * (p_mat[0][1] - p_mat[1][0]) / tqi;
        }
        result.normalize()
    }

    // 与一个列向量进行矩阵乘法，得到一个列向量
    pub fn product_vector<T: PhysicalQuantity + Default>(&self, vec: &Vector3<T>) -> Vector3<T> {
        let vec_arr = vec.to_array();
        let mut result: Vector3<T> = Vector3::default();
        result.x.set_value(
            self.data[0] * vec_arr[0] + self.data[1] * vec_arr[1] + self.data[2] * vec_arr[2],
        );
        result.y.set_value(
            self.data[3] * vec_arr[0] + self.data[4] * vec_arr[1] + self.data[5] * vec_arr[2],
        );
        result.z.set_value(
            self.data[6] * vec_arr[0] + self.data[7] * vec_arr[1] + self.data[8] * vec_arr[2],
        );
        result
    }

    pub fn product(&self, other: &CosMatrix) -> CosMatrix {
        let mut data = [0.0; 9];
        // Row 0
        data[0] = self.data[0] * other.data[0]
            + self.data[1] * other.data[3]
            + self.data[2] * other.data[6];
        data[1] = self.data[0] * other.data[1]
            + self.data[1] * other.data[4]
            + self.data[2] * other.data[7];
        data[2] = self.data[0] * other.data[2]
            + self.data[1] * other.data[5]
            + self.data[2] * other.data[8];
        // Row 1
        data[3] = self.data[3] * other.data[0]
            + self.data[4] * other.data[3]
            + self.data[5] * other.data[6];
        data[4] = self.data[3] * other.data[1]
            + self.data[4] * other.data[4]
            + self.data[5] * other.data[7];
        data[5] = self.data[3] * other.data[2]
            + self.data[4] * other.data[5]
            + self.data[5] * other.data[8];
        // Row 2
        data[6] = self.data[6] * other.data[0]
            + self.data[7] * other.data[3]
            + self.data[8] * other.data[6];
        data[7] = self.data[6] * other.data[1]
            + self.data[7] * other.data[4]
            + self.data[8] * other.data[7];
        data[8] = self.data[6] * other.data[2]
            + self.data[7] * other.data[5]
            + self.data[8] * other.data[8];
        CosMatrix { data }
    }
    // 将余弦阵转换到欧拉角，xzy 转序
    pub fn to_pry(&self) -> Vector3<Angular> {
        let data = self.to_array();
        let y = Angular::atan2(data[2][0], data[2][2]);
        let z = Angular::atan2(data[0][1], data[1][1]);
        let tempf = data[2][1].abs();
        let mut x: Angular = Angular::default();
        if tempf > 1.0 {
            x = Angular::asin(-data[2][1] / tempf);
        } else {
            x = Angular::asin(-data[2][1]);
        }
        Vector3::new(x, y, z)
    }
    // 欧拉角计算，xyz转序
    pub fn to_rpy(&self) -> Vector3<Angular> {
        let mut result: Vector3<Angular> = Vector3::default();
        let aa = self.to_array();
        result.x = Angular::atan2(-aa[2][1], aa[2][2]);
        let tempf: f64 = aa[2][0].abs();
        result.y = if tempf > 1.0 {
            Angular::asin(aa[2][0] / tempf)
        } else {
            Angular::asin(aa[2][0])
        };
        result.z = Angular::atan2(-aa[1][0], aa[0][0]);
        result
    }

    pub fn to_matrix(&self) -> Matrix<3, 3, f64> {
        let data: [[f64; 3]; 3] = self.to_array();
        Matrix::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_cos_matrix_new() {
        let cos = CosMatrix::new([[1.1, 1.2, 1.3], [1.4, 1.5, 1.6], [1.7, 1.8, 1.9]]);
        assert_eq!(cos.data[0], 1.1);
    }
    #[test]
    fn test_cos_matrix_default() {
        let cos = CosMatrix::default();
        assert_eq!(cos.data, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        let cos = CosMatrix::unit();
        assert_eq!(cos.data, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_cos_matrix_get_x_vector() {
        let cos = CosMatrix::new([[1.0, 1.1, 1.2], [1.3, 1.4, 1.5], [1.6, 1.7, 1.8]]);
        let x = cos.get_x_vector();
        assert_relative_eq!(1.0, x.x.get_value());
        assert_relative_eq!(1.3, x.y.get_value());
        assert_relative_eq!(1.6, x.z.get_value());

        let y = cos.get_y_vector();
        assert_relative_eq!(1.1, y.x.get_value());
        assert_relative_eq!(1.4, y.y.get_value());
        assert_relative_eq!(1.7, y.z.get_value());

        let z = cos.get_z_vector();
        assert_relative_eq!(1.2, z.x.get_value());
        assert_relative_eq!(1.5, z.y.get_value());
        assert_relative_eq!(1.8, z.z.get_value());
    }

    #[test]
    fn test_cos_matrix_transfer() {
        let cos = CosMatrix::new([[1.1, 1.2, 1.3], [1.4, 1.5, 1.6], [1.7, 1.8, 1.9]]);
        let cos_trans = cos.transfer();
        let expected = [1.1, 1.4, 1.7, 1.2, 1.5, 1.8, 1.3, 1.6, 1.9];
        assert_eq!(cos_trans.data, expected);
    }

    #[test]
    fn test_cross_matrix_to_array() {
        let cos = CosMatrix::new([[1.1, 1.2, 1.3], [1.4, 1.5, 1.6], [1.7, 1.8, 1.9]]);
        let array = cos.to_array();
        let expected = [1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9];
        for row in 0..3 {
            for col in 0..3 {
                let v = array[row][col];
                let index = (row * 3 + col) as usize;
                let exp = expected[index];
                assert_relative_eq!(v, exp);
            }
        }
    }

    #[test]

    fn test_cos_matrix_to_quaternion() {
        let cos = CosMatrix::new([
            [0.131510392, 0.147510082, 0.980278611],
            [-0.986284316, 0.118965819, 0.114414386],
            [-0.0997423381, -0.981879771, 0.161132157],
        ]);
        let q = cos.to_quaternion();
        assert_relative_eq!(q.q1, 0.46136003293870587);
        assert_relative_eq!(q.q2, -0.4545116814403202);
        assert_relative_eq!(q.q3, 0.47714148384994104);
        assert_relative_eq!(q.q0, 0.5940555999488308);

        let cos = CosMatrix::new([
            [-0.99995, 0.00999, -0.00249],
            [0.00999, 0.00005, -0.99995],
            [0.00249, 0.99995, 0.00005],
        ]);
        let q = cos.to_quaternion();
        assert_relative_eq!(q.q0, 0.002489930281901065);
        assert_relative_eq!(q.q1, 0.009989720287627166);
        assert_relative_eq!(q.q2, 0.9999470014634607);
        assert_relative_eq!(q.q3, 0.0);

        let cos = CosMatrix::new([
            [0.9999, -0.007, -0.001],
            [0.007, -0.0001, -0.9998],
            [0.001, 0.9998, -0.007],
        ]);
        let q = cos.to_quaternion();
        assert_relative_eq!(q.q0, 0.7058925302408472);
        assert_relative_eq!(q.q1, -0.7083012361850654);
        assert_relative_eq!(q.q2, 0.0007084429247700193);
        assert_relative_eq!(q.q3, -0.004959100473390135);
    }

    #[test]
    fn test_cos_matrix_mul_vector_1() {
        let cos = CosMatrix::new([[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [5.0, 2.0, 1.0]]);
        let vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let result = cos.product_vector(&vec);
        assert_relative_eq!(result.x.get_value(), 14.0);
        assert_relative_eq!(result.y.get_value(), 20.0);
        assert_relative_eq!(result.z.get_value(), 12.0);
    }

    #[test]
    fn test_cos_matrix_product() {
        let cos1 = CosMatrix::new([[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [5.0, 2.0, 1.0]]);
        let cos2 = CosMatrix::new([[2.0, 3.0, 4.0], [1.0, 2.0, 3.0], [5.0, 2.0, 1.0]]);
        let result = cos1.product(&cos2);
        let expected = [19.0, 13.0, 13.0, 27.0, 20.0, 21.0, 17.0, 21.0, 27.0];
        assert_eq!(result.data, expected);
    }

    #[test]
    fn test_rpy_f64() {
        let cos = CosMatrix::new([
            [
                0.9999885554813407,
                0.0047363317272977185,
                0.0006609144932516187,
            ],
            [
                -0.0047358448469547315,
                0.9999885069199654,
                -0.0007452599663012316,
            ],
            [
                -0.0006644098300277457,
                0.0007421144721519446,
                0.9999995132277874,
            ],
        ]);
        let tmp_a = Vector3::new(
            Angular::from_rad(-0.0007421146971568069),
            Angular::from_rad(-0.0006644098789106481),
            Angular::from_rad(0.004735863640767091),
        );
        let result = cos.to_rpy();
        assert_relative_eq!(tmp_a.x.as_rad(), result.x.as_rad());
        assert_relative_eq!(tmp_a.y.as_rad(), result.y.as_rad());
        assert_relative_eq!(tmp_a.z.as_rad(), result.z.as_rad());
    }

    #[test]
    fn test_pry_f64() {
        let cos = CosMatrix::new([
            [
                0.9999885554813407,
                0.0047363317272977185,
                0.0006609144932516187,
            ],
            [
                -0.0047358448469547315,
                0.9999885069199654,
                -0.0007452599663012316,
            ],
            [
                -0.0006644098300277457,
                0.0007421144721519446,
                0.9999995132277874,
            ],
        ]);
        let tmp_a = Vector3::new(
            Angular::from_rad(-0.0007421145402698932),
            Angular::from_rad(-0.000664410055678244),
            Angular::from_rad(0.004736350745764036),
        );
        let result = cos.to_pry();
        assert_relative_eq!(tmp_a.x.as_rad(), result.x.as_rad());
        assert_relative_eq!(tmp_a.y.as_rad(), result.y.as_rad());
        assert_relative_eq!(tmp_a.z.as_rad(), result.z.as_rad());
    }

    #[test]
    fn test_cos_matrix_ops_f64() {
        let m = CosMatrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

        // Test Add
        let m_add = &m + 2.0;
        assert_eq!(m_add.data, [3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);

        let m_add = 2.0 + m;
        assert_eq!(m_add.data, [3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);

        // Test Sub
        let m_sub = m - 1.0;
        assert_eq!(m_sub.data, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);

        let m_sub = 10.0 - m;
        assert_eq!(m_sub.data, [9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);

        // Test Mul
        let m_mul = m * 2.0;
        assert_eq!(
            m_mul.data,
            [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0]
        );

        let m_mul = 2.0 * m;
        assert_eq!(
            m_mul.data,
            [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0]
        );

        // Test Div
        let m_div = m / 2.0;
        assert_eq!(m_div.data, [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5]);

        let m_div = 10.0 / m;
        assert_relative_eq!(m_div.data[0], 10.0);
        assert_relative_eq!(m_div.data[1], 5.0);
        assert_relative_eq!(m_div.data[2], 10.0 / 3.0);
        assert_relative_eq!(m_div.data[3], 2.5);
        assert_relative_eq!(m_div.data[4], 2.0);
        assert_relative_eq!(m_div.data[5], 10.0 / 6.0);
        assert_relative_eq!(m_div.data[6], 10.0 / 7.0);
        assert_relative_eq!(m_div.data[7], 1.25);
        assert_relative_eq!(m_div.data[8], 10.0 / 9.0);
    }

    #[test]
    fn test_cos_matrix_ops_matrix() {
        let m1 = CosMatrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let m2 = CosMatrix::new([[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]]);

        // Test Add
        let m_add = m1 + m2;
        assert_eq!(
            m_add.data,
            [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0]
        );

        // Test Sub
        let m_sub = m1 - m2;
        assert_eq!(
            m_sub.data,
            [-8.0, -6.0, -4.0, -2.0, 0.0, 2.0, 4.0, 6.0, 8.0]
        );

        // Test Mul
        let m_mul = &m1 * &m2;
        assert_eq!(
            m_mul.data,
            [9.0, 16.0, 21.0, 24.0, 25.0, 24.0, 21.0, 16.0, 9.0]
        );

        // Test Div
        let m_div = &m1 / m2;
        assert_eq!(
            m_div.data,
            [
                1.0 / 9.0,
                2.0 / 8.0,
                3.0 / 7.0,
                4.0 / 6.0,
                5.0 / 5.0,
                6.0 / 4.0,
                7.0 / 3.0,
                8.0 / 2.0,
                9.0 / 1.0
            ]
        );
    }

    #[test]
    fn test_get_col_vector() {
        let cos = CosMatrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let x = cos.get_x_col_vector();
        assert_relative_eq!(x.x.get_value(), 1.0);
        assert_relative_eq!(x.y.get_value(), 4.0);
        assert_relative_eq!(x.z.get_value(), 7.0);
        let y = cos.get_y_col_vector();
        assert_relative_eq!(y.x.get_value(), 2.0);
        assert_relative_eq!(y.y.get_value(), 5.0);
        assert_relative_eq!(y.z.get_value(), 8.0);
        let z = cos.get_z_col_vector();
        assert_relative_eq!(z.x.get_value(), 3.0);
        assert_relative_eq!(z.y.get_value(), 6.0);
        assert_relative_eq!(z.z.get_value(), 9.0);
    }

    #[test]
    fn test_to_matrix_with_identity_matrix() {
        // Test with identity matrix
        let cos = CosMatrix::unit();
        let matrix = cos.to_matrix();

        // Verify all diagonal elements are 1.0 and others are 0.0
        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    assert_relative_eq!(matrix.get(i, j).unwrap(), 1.0);
                } else {
                    assert_relative_eq!(matrix.get(i, j).unwrap(), 0.0);
                }
            }
        }
    }

    #[test]
    fn test_to_matrix_with_random_values() {
        // Test with arbitrary values
        let test_data = [
            [1.1, 2.2, 3.3],
            [4.4, 5.5, 6.6],
            [7.7, 8.8, 9.9]
        ];
        let cos = CosMatrix::new(test_data);
        let matrix = cos.to_matrix();

        // Verify all elements match exactly
        for i in 0..3 {
            for j in 0..3 {
                assert_relative_eq!(matrix.get(j, i).unwrap(), test_data[i][j]);
            }
        }
    }

    #[test]
    fn test_to_matrix_preserves_transpose_relationship() {
        // Test that matrix conversion preserves transpose relationship
        let test_data = [
            [0.1, -0.2, 0.3],
            [-0.4, 0.5, -0.6],
            [0.7, -0.8, 0.9]
        ];
        let cos = CosMatrix::new(test_data);
        let original_matrix = cos.to_matrix();
        let transposed_matrix = cos.transfer().to_matrix();

        // Verify original_matrix^T equals transposed_matrix
        for i in 0..3 {
            for j in 0..3 {
                assert_relative_eq!(original_matrix.get(i, j).unwrap(), transposed_matrix.get(j, i).unwrap());
            }
        }
    }

    #[test]
    fn test_to_matrix_with_extreme_values() {
        // Test with extreme values (very large and small numbers)
        let test_data = [
            [f64::MAX, f64::MIN, 0.0],
            [0.0, f64::MAX, f64::MIN],
            [f64::MIN, 0.0, f64::MAX]
        ];
        let cos = CosMatrix::new(test_data);
        let matrix = cos.to_matrix();

        // Verify all elements match exactly
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix.get(j,i).unwrap(), test_data[i][j]);
            }
        }
    }

    #[test]
    fn test_to_matrix_with_nan_values() {
        // Test with NaN values (should propagate NaN)
        let test_data = [
            [1.0, f64::NAN, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0]
        ];
        let cos = CosMatrix::new(test_data);
        let matrix = cos.to_matrix();

        // Verify NaN propagates correctly
        assert!(matrix.get(1, 0).unwrap().is_nan());
        assert!(!matrix.get(0, 0).unwrap().is_nan());
    }


}
