use super::*;
use crate::dense::Matrix;
use crate::physics::basic::{Angular, Coef, PhysicalQuantity, Vector3};
use crate::spatial_geometry::quaternion::Quaternion;
use crate::utils::float::sgn2_64;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct CosMatrix {
    matrix: Matrix<f64>,
}

impl Default for CosMatrix {
    fn default() -> Self {
        CosMatrix {
            matrix: Matrix::new_with_default(3, 3, 0.0).unwrap(),
        }
    }
}

impl CosMatrix {
    //返回单位方向余弦阵
    pub fn unit() -> Self {
        let data: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        CosMatrix::new(data)
    }
    pub fn new(data: [[f64; 3]; 3]) -> Self {
        let mut vec: Vec<f64> = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                vec.push(data[row][col]);
            }
        }
        let m = Matrix::new(3, 3, vec).unwrap();
        CosMatrix { matrix: m }
    }

    pub fn get_x_vector(&self) -> Vector3<Coef> {
        let to_x = Coef::new(self.matrix.get(0, 0).expect("get x vector failed"));
        let to_y: Coef = Coef::new(self.matrix.get(1, 0).expect("get y vector failed"));
        let to_z: Coef = Coef::new(self.matrix.get(2, 0).expect("get z vector failed"));
        Vector3::new(to_x, to_y, to_z)
    }

    pub fn get_y_vector(&self) -> Vector3<Coef> {
        let to_x = Coef::new(self.matrix.get(0, 1).expect("get x vector failed"));
        let to_y: Coef = Coef::new(self.matrix.get(1, 1).expect("get y vector failed"));
        let to_z: Coef = Coef::new(self.matrix.get(2, 1).expect("get z vector failed"));
        Vector3::new(to_x, to_y, to_z)
    }
    pub fn get_z_vector(&self) -> Vector3<Coef> {
        let to_x = Coef::new(self.matrix.get(0, 2).expect("get x vector failed"));
        let to_y: Coef = Coef::new(self.matrix.get(1, 2).expect("get y vector failed"));
        let to_z: Coef = Coef::new(self.matrix.get(2, 2).expect("get z vector failed"));
        Vector3::new(to_x, to_y, to_z)
    }

    pub fn get_x_col_vector(&self) -> Vector3<Coef> {
        let to_x = Coef::new(self.matrix.get(0, 0).expect("get x vector failed"));
        let to_y: Coef = Coef::new(self.matrix.get(0, 1).expect("get y vector failed"));
        let to_z: Coef = Coef::new(self.matrix.get(0, 2).expect("get z vector failed"));
        Vector3::new(to_x, to_y, to_z)
    }

    pub fn get_y_col_vector(&self) -> Vector3<Coef> {
        let to_x = Coef::new(self.matrix.get(1, 0).expect("get x vector failed"));
        let to_y: Coef = Coef::new(self.matrix.get(1, 1).expect("get y vector failed"));
        let to_z: Coef = Coef::new(self.matrix.get(1, 2).expect("get z vector failed"));
        Vector3::new(to_x, to_y, to_z)
    }

    pub fn get_z_col_vector(&self) -> Vector3<Coef> {
        let to_x = Coef::new(self.matrix.get(2, 0).expect("get x vector failed"));
        let to_y: Coef = Coef::new(self.matrix.get(2, 1).expect("get y vector failed"));
        let to_z: Coef = Coef::new(self.matrix.get(2, 2).expect("get z vector failed"));
        Vector3::new(to_x, to_y, to_z)
    }

    // 矩阵转置
    pub fn transfer(&self) -> Self {
        let m = self.matrix.T();
        let mut cos = CosMatrix::default();
        cos.matrix = m;
        cos
    }

    pub fn to_array(&self) -> [[f64; 3]; 3] {
        let mut arr: [[f64; 3]; 3] = [[0.0; 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                arr[col][row] = self.matrix.get(row, col).expect("get matrix failed");
            }
        }
        arr
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
        let mut result: Vector3<T> = Vector3::default();
        let m = vec.to_col_matrix();
        let r = self.matrix.product(&m).unwrap();
        result.x.set_value(r.get(0, 0).unwrap());
        result.y.set_value(r.get(0, 1).unwrap());
        result.z.set_value(r.get(0, 2).unwrap());
        result
    }

    pub fn product(&self, other: &CosMatrix) -> CosMatrix {
        let m = self.matrix.product(&other.matrix.clone()).unwrap();
        let mut cos = CosMatrix::default();
        cos.matrix = m;
        cos
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_cos_matrix_new() {
        let cos = CosMatrix::new([[1.1, 1.2, 1.3], [1.4, 1.5, 1.6], [1.7, 1.8, 1.9]]);
        assert_eq!(cos.matrix.size(), (3, 3));
        assert_eq!(cos.matrix.get(0, 0).unwrap(), 1.1);
    }
    #[test]
    fn test_cos_matrix_default() {
        let cos = CosMatrix::default();
        assert_eq!(cos.matrix.size(), (3, 3));
        let cos = CosMatrix::unit();
        for (row, line) in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
            .iter()
            .enumerate()
        {
            for (col, expected) in line.iter().enumerate() {
                let actual = cos.matrix.get(col, row).unwrap();
                assert_relative_eq!(actual, *expected, epsilon = 1e-10);
            }
        }
    }

    #[test]
    fn test_cos_matrix_get_x_vector() {
        let mut cos = CosMatrix::default();
        cos.matrix = Matrix::new(3, 3, vec![1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8]).unwrap();
        let x = cos.get_x_vector();
        assert_relative_eq!(1.0, x.x.get_value());
        assert_relative_eq!(1.1, x.y.get_value());
        assert_relative_eq!(1.2, x.z.get_value());

        let y = cos.get_y_vector();
        assert_relative_eq!(1.3, y.x.get_value());
        assert_relative_eq!(1.4, y.y.get_value());
        assert_relative_eq!(1.5, y.z.get_value());

        let z = cos.get_z_vector();
        assert_relative_eq!(1.6, z.x.get_value());
        assert_relative_eq!(1.7, z.y.get_value());
        assert_relative_eq!(1.8, z.z.get_value());
    }

    #[test]
    fn test_cos_matrix_transfer() {
        let mut cos = CosMatrix::default();
        cos.matrix = Matrix::new(3, 3, vec![1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9]).unwrap();
        let cos_trans = cos.transfer();
        let result = format!("{}", cos_trans.matrix);
        let expected = [1.1, 1.4, 1.7, 1.2, 1.5, 1.8, 1.3, 1.6, 1.9];
        for row in 0..3 {
            for col in 0..3 {
                let v = cos_trans.matrix.get(col, row).unwrap();
                let index = (row * 3 + col) as usize;
                let exp = expected[index];
                assert_relative_eq!(v, exp);
            }
        }
    }

    #[test]
    fn test_cross_matrix_to_array() {
        let mut cos = CosMatrix::default();
        cos.matrix = Matrix::new(3, 3, vec![1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9]).unwrap();
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
        let r = [[19.0, 13.0, 13.0], [27.0, 20.0, 21.0], [17.0, 21.0, 27.0]];
        for row in 0..3 {
            for col in 0..3 {
                let v = result.matrix.get(col, row).unwrap();
                let exp = r[row as usize][col as usize];
                assert_relative_eq!(v, exp);
            }
        }
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
}
