use super::*;
use crate::constant::FLT64_ZERO;
use crate::dense::Matrix;
use crate::spatial_geometry::cos_matrix::CosMatrix;
use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub q0: f64, // 实部
    pub q1: f64, //x
    pub q2: f64, //y
    pub q3: f64, //z
}

impl Default for Quaternion {
    //生成一个单位四元数
    fn default() -> Self {
        Quaternion {
            q0: 1.0,
            q1: 0.0,
            q2: 0.0,
            q3: 0.0,
        }
    }
}

impl Quaternion {
    //归一化，生成单位四元数
    pub fn normalize(&self) -> Self {
        let mut tmp_norm = f64::sqrt(
            self.q0 * self.q0 + self.q1 * self.q1 + self.q2 * self.q2 + self.q3 * self.q3,
        );
        let mut result = Quaternion::default();
        if tmp_norm > FLT64_ZERO {
            if self.q0 < 0.0 {
                tmp_norm = -tmp_norm;
            }
            result.q0 = self.q0 / tmp_norm;
            result.q1 = self.q1 / tmp_norm;
            result.q2 = self.q2 / tmp_norm;
            result.q3 = self.q3 / tmp_norm;
        } else {
            result.q0 = 1.0;
            result.q1 = 0.0;
            result.q2 = 0.0;
            result.q3 = 0.0;
        }
        result
    }

    pub fn new(q0: f64, q1: f64, q2: f64, q3: f64) -> Self {
        Self { q0, q1, q2, q3 }
    }
    //deprecated 由于跟数组的顺序有关
    pub fn from_array(list: [f64; 4]) -> Self {
        Quaternion::new(list[0], list[1], list[2], list[3])
    }
    // 通过xyz 3个虚部得到四元数
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        let w = (1.0 - x * x - y * y - z * z).sqrt();
        Quaternion::new(w, x, y, z)
    }
    pub fn get_value(&self) -> (f64, f64, f64, f64) {
        (self.q0, self.q1, self.q2, self.q3)
    }
    // 求四元数的模长
    pub fn norm(&self) -> f64 {
        let sum = self.q0 * self.q0 + self.q1 * self.q1 + self.q2 * self.q2 + self.q3 * self.q3;
        sum.sqrt()
    }
    //求共轭四元数
    pub fn conjugate(&self) -> Self {
        Quaternion {
            q0: self.q0,
            q1: -self.q1,
            q2: -self.q2,
            q3: -self.q3,
        }
    }
    // 求四元数的逆
    pub fn inverse(&self) -> Self {
        let norm_square =
            self.q0 * self.q0 + self.q1 * self.q1 + self.q2 * self.q2 + self.q3 * self.q3;
        if norm_square > FLT64_ZERO {
            Quaternion {
                q0: self.q0 / norm_square,
                q1: -self.q1 / norm_square,
                q2: -self.q2 / norm_square,
                q3: -self.q3 / norm_square,
            }
        } else {
            Quaternion::default()
        }
    }
    //四元数到旋转矩阵的转换
    pub fn to_cos_matrix(&self) -> CosMatrix {
        let mut pa: [[f64; 3]; 3] = Default::default();
        pa[0][0] = 1.0 - 2.0 * self.q2 * self.q2 - 2.0 * self.q3 * self.q3;
        pa[0][1] = 2.0 * self.q1 * self.q2 + 2.0 * self.q3 * self.q0;
        pa[0][2] = 2.0 * self.q1 * self.q3 - 2.0 * self.q2 * self.q0;

        pa[1][0] = 2.0 * self.q1 * self.q2 - 2.0 * self.q3 * self.q0;
        pa[1][1] = 1.0 - 2.0 * self.q1 * self.q1 - 2.0 * self.q3 * self.q3;
        pa[1][2] = 2.0 * self.q2 * self.q3 + 2.0 * self.q1 * self.q0;

        pa[2][0] = 2.0 * self.q1 * self.q3 + 2.0 * self.q2 * self.q0;
        pa[2][1] = 2.0 * self.q2 * self.q3 - 2.0 * self.q1 * self.q0;
        pa[2][2] = 1.0 - 2.0 * self.q1 * self.q1 - 2.0 * self.q2 * self.q2;
        CosMatrix::new(pa)
    }

    //对四元数执行线性变换
    pub fn linear_transform(&self, m: Matrix<4, 4, f64>) -> Quaternion {
        let col_vec = Matrix::<4, 1, f64>::new([[self.q1], [self.q2], [self.q3], [self.q0]]);
        let result = m.product(&col_vec).unwrap();
        Quaternion::new(
            result.get(0, 3).unwrap(),
            result.get(0, 0).unwrap(),
            result.get(0, 1).unwrap(),
            result.get(0, 2).unwrap(),
        )
    }

    //这个函数的运算意义不明
    pub fn ksi_matrix(&self) -> Matrix<4, 3, f64> {
        Matrix::<4, 3, f64>::new([
            [self.q0, -self.q3, self.q2],
            [self.q3, self.q0, -self.q1],
            [-self.q2, self.q1, self.q0],
            [-self.q1, -self.q2, -self.q3],
        ])
    }
}

impl Add for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            q0: self.q0 + rhs.q0,
            q1: self.q1 + rhs.q1,
            q2: self.q2 + rhs.q2,
            q3: self.q3 + rhs.q3,
        }
    }
}

impl Add for &Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Self) -> Quaternion {
        Quaternion {
            q0: self.q0 + rhs.q0,
            q1: self.q1 + rhs.q1,
            q2: self.q2 + rhs.q2,
            q3: self.q3 + rhs.q3,
        }
    }
}

impl Div for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: Self) -> Self::Output {
        // 使用 xyzw 分量公式计算（本结构为 wxyz，注意映射）
        let ax = self.q1;
        let ay = self.q2;
        let az = self.q3;
        let aw = self.q0;

        let bx = rhs.q1;
        let by = rhs.q2;
        let bz = rhs.q3;
        let bw = rhs.q0;

        let mut rx = -ax * bw - ay * bz + az * by + aw * bx;
        let mut ry =  ax * bz - ay * bw - az * bx + aw * by;
        let mut rz = -ax * by + ay * bx - az * bw + aw * bz;
        let mut rw =  ax * bx + ay * by + az * bz + aw * bw;

        // 归一化并强制 w>=0
        let norm = f64::sqrt(rw * rw + rx * rx + ry * ry + rz * rz);
        if norm > FLT64_ZERO {
            rw /= norm;
            rx /= norm;
            ry /= norm;
            rz /= norm;
        } else {
            rw = 1.0;
            rx = 0.0;
            ry = 0.0;
            rz = 0.0;
        }
        if rw < 0.0 {
            rw = -rw;
            rx = -rx;
            ry = -ry;
            rz = -rz;
        }
        Quaternion { q0: rw, q1: rx, q2: ry, q3: rz }
    }
}

impl Div for &Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: Self) -> Self::Output {
        // 使用 xyzw 分量公式计算（本结构为 wxyz，注意映射）
        let ax = self.q1;
        let ay = self.q2;
        let az = self.q3;
        let aw = self.q0;

        let bx = rhs.q1;
        let by = rhs.q2;
        let bz = rhs.q3;
        let bw = rhs.q0;

        let mut rx = -ax * bw - ay * bz + az * by + aw * bx;
        let mut ry =  ax * bz - ay * bw - az * bx + aw * by;
        let mut rz = -ax * by + ay * bx - az * bw + aw * bz;
        let mut rw =  ax * bx + ay * by + az * bz + aw * bw;

        // 归一化并强制 w>=0
        let norm = f64::sqrt(rw * rw + rx * rx + ry * ry + rz * rz);
        if norm > FLT64_ZERO {
            rw /= norm;
            rx /= norm;
            ry /= norm;
            rz /= norm;
        } else {
            rw = 1.0;
            rx = 0.0;
            ry = 0.0;
            rz = 0.0;
        }
        if rw < 0.0 {
            rw = -rw;
            rx = -rx;
            ry = -ry;
            rz = -rz;
        }
        Quaternion { q0: rw, q1: rx, q2: ry, q3: rz }
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let q0 = self.q0 * rhs.q0 - self.q1 * rhs.q1 - self.q2 * rhs.q2 - self.q3 * rhs.q3;
        let q1 = self.q0 * rhs.q1 + self.q1 * rhs.q0 + self.q2 * rhs.q3 - self.q3 * rhs.q2;
        let q2 = self.q0 * rhs.q2 - self.q1 * rhs.q3 + self.q2 * rhs.q0 + self.q3 * rhs.q1;
        let q3 = self.q0 * rhs.q3 + self.q1 * rhs.q2 - self.q2 * rhs.q1 + self.q3 * rhs.q0;
        Quaternion::new(q0, q1, q2, q3)
    }
}

impl Mul for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        let q0 = self.q0 * rhs.q0 - self.q1 * rhs.q1 - self.q2 * rhs.q2 - self.q3 * rhs.q3;
        let q1 = self.q0 * rhs.q1 + self.q1 * rhs.q0 + self.q2 * rhs.q3 - self.q3 * rhs.q2;
        let q2 = self.q0 * rhs.q2 - self.q1 * rhs.q3 + self.q2 * rhs.q0 + self.q3 * rhs.q1;
        let q3 = self.q0 * rhs.q3 + self.q1 * rhs.q2 - self.q2 * rhs.q1 + self.q3 * rhs.q0;
        Quaternion::new(q0, q1, q2, q3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_new() {
        let result = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(result.q0, 1.0);
        assert_eq!(result.q1, 2.0);
        assert_eq!(result.q2, 3.0);
        assert_eq!(result.q3, 4.0);

        let result = Quaternion::from_array([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(result.q0, 1.0);
        assert_eq!(result.q1, 2.0);
        assert_eq!(result.q2, 3.0);
        assert_eq!(result.q3, 4.0);
    }

    #[test]
    fn test_from_xyz() {
        let result = Quaternion::from_xyz(0.5, 0.5, 0.5);
        assert_eq!(result.q0, 0.5);
    }
    #[test]
    fn test_unit() {
        let q = Quaternion {
            q0: -1.0,
            q1: 1.0,
            q2: 1.0,
            q3: 1.0,
        };
        let unit = q.normalize();
        assert_relative_eq!(unit.q0, 0.5);
        assert_relative_eq!(unit.q1, -0.5);
        assert_relative_eq!(unit.q2, -0.5);
        assert_relative_eq!(unit.q3, -0.5);

        let q = Quaternion {
            q0: 2.0,
            q1: 4.2,
            q2: 2.3,
            q3: 1.5,
        };
        let unit = q.normalize();
        assert_relative_eq!(unit.q0, 0.37024342251047304, epsilon = 1e-7);
        assert_relative_eq!(unit.q1, 0.7775111872719934, epsilon = 1e-7);
        assert_relative_eq!(unit.q2, 0.425779935887044, epsilon = 1e-7);
        assert_relative_eq!(unit.q3, 0.2776825668828548, epsilon = 1e-7);

        let q = Quaternion {
            q0: 0.0,
            q1: 0.0,
            q2: 0.0,
            q3: 0.0,
        };
        let unit = q.normalize();
        assert_relative_eq!(unit.q0, 1.0);
        assert_relative_eq!(unit.q1, 0.0);
        assert_relative_eq!(unit.q2, 0.0);
        assert_relative_eq!(unit.q3, 0.0);
    }

    #[test]
    fn test_div_specific_case() {
        let b_i_old = Quaternion {
            // 对应旧版 qa（被除数分母）
            q0: 0.9876543210987654,
            q1: 0.12345678901234568,
            q2: 0.2345678901234568,
            q3: 0.3456789012345679,
        };
        let target = Quaternion {
            // 对应旧版 qb（被除数分子）
            q0: 0.9123456789012345,
            q1: 0.2345678901234568,
            q2: 0.3456789012345679,
            q3: 0.4567890123456789,
        };
        let result = target / b_i_old;
        let (q0, q1, q2, q3) = result.get_value();
        println!("final result {:?}",result);
        assert_relative_eq!(q0, 0.982280611992, epsilon = 1e-7);
        assert_relative_eq!(q1, -0.110394701362, epsilon = 1e-7);
        assert_relative_eq!(q2, -0.086304791272, epsilon = 1e-7);
        assert_relative_eq!(q3, -0.124455586076, epsilon = 1e-7);
    }
    #[test]
    fn test_normalize_specific_case() {
        // 输入给定为 (x, y, z, w)
        let x = -0.131382584572_f64;
        let y = -0.102712780237_f64;
        let z = -0.148116677999_f64;
        let w = 1.169028639793_f64;

        // 本库四元数为 (q0, q1, q2, q3) = (w, x, y, z)
        let q = Quaternion::new(w, x, y, z);
        let norm = q.norm();
        assert_relative_eq!(norm, 1.190116763115_f64, epsilon = 1e-7);

        let unit = q.normalize();
        assert_relative_eq!(unit.q1, -0.110394701362_f64, epsilon = 1e-7);
        assert_relative_eq!(unit.q2, -0.086304791272_f64, epsilon = 1e-7);
        assert_relative_eq!(unit.q3, -0.124455586076_f64, epsilon = 1e-7);
        assert_relative_eq!(unit.q0, 0.982280611992_f64, epsilon = 1e-7);
    }

    #[test]
    fn test_norm() {
        let q = Quaternion {
            q0: 1.0,
            q1: 2.0,
            q2: 3.0,
            q3: 4.0,
        };
        let norm = q.norm();
        assert_relative_eq!(norm, 5.477225575051661);
    }

    #[test]
    fn test_conjugate() {
        let q = Quaternion {
            q0: 1.0,
            q1: 2.0,
            q2: 3.0,
            q3: 4.0,
        };
        let conj = q.conjugate();
        assert_relative_eq!(conj.q0, 1.0);
        assert_relative_eq!(conj.q1, -2.0);
        assert_relative_eq!(conj.q2, -3.0);
        assert_relative_eq!(conj.q3, -4.0);
    }

    #[test]
    fn test_inverse() {
        let q = Quaternion::new(2.0, 3.0, 4.0, 5.0);
        let inv = q.inverse();
        assert_relative_eq!(inv.q0, 2.0 / 54.0);
        assert_relative_eq!(inv.q1, -3.0 / 54.0);
        assert_relative_eq!(inv.q2, -4.0 / 54.0);
        assert_relative_eq!(inv.q3, -5.0 / 54.0);

        let q = Quaternion::new(1.23, 4.56, 7.89, 10.11);
        let inv = q.inverse();
        assert_relative_eq!(inv.q0, 1.23 / 186.7707);
        assert_relative_eq!(inv.q1, -4.56 / 186.7707);
        assert_relative_eq!(inv.q2, -7.89 / 186.7707);
        assert_relative_eq!(inv.q3, -10.11 / 186.7707);
    }

    #[test]
    fn test_mul() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(5.0, 6.0, 7.0, 8.0);
        let q3 = q1 * q2;
        assert_relative_eq!(q3.q0, -60.0);
        assert_relative_eq!(q3.q1, 12.0);
        assert_relative_eq!(q3.q2, 30.0);
        assert_relative_eq!(q3.q3, 24.0);

        let q1 = Quaternion::new(1.23, 4.56, 7.89, 10.11);
        let q2 = Quaternion::new(2.34, 5.67, 8.90, 11.22);
        let q3 = q1 * q2;
        assert_relative_eq!(q3.q0, -206.6322);
        assert_relative_eq!(q3.q1, 16.1913);
        assert_relative_eq!(q3.q2, 35.5701);
        assert_relative_eq!(q3.q3, 33.3057);

        let q1_1 = &q1.clone();
        let q2_2 = &q2.clone();
        let q3 = q1_1 * q2_2;
        assert_relative_eq!(q3.q0, -206.6322);
        assert_relative_eq!(q3.q1, 16.1913);
        assert_relative_eq!(q3.q2, 35.5701);
        assert_relative_eq!(q3.q3, 33.3057);
    }

    #[test]
    fn test_div() {
        // 使用分量公式与归一化、w>=0 的一致性校验
        let q1 = Quaternion::new(1.2, -0.4, 0.6, 2.0);
        let r = Quaternion::new(0.5, 1.0, -2.0, 3.0);
        let d1 = q1 / r;
        // 手算预期：按 xyzw 公式计算，再单位化并强制 w>=0
        let ax = q1.q1; let ay = q1.q2; let az = q1.q3; let aw = q1.q0;
        let bx = r.q1;  let by = r.q2;  let bz = r.q3;  let bw = r.q0;
        let mut rx = -ax * bw - ay * bz + az * by + aw * bx;
        let mut ry =  ax * bz - ay * bw - az * bx + aw * by;
        let mut rz = -ax * by + ay * bx - az * bw + aw * bz;
        let mut rw =  ax * bx + ay * by + az * bz + aw * bw;
        let norm = f64::sqrt(rw * rw + rx * rx + ry * ry + rz * rz);
        if norm > FLT64_ZERO { rw/=norm; rx/=norm; ry/=norm; rz/=norm; } else { rw=1.0; rx=0.0; ry=0.0; rz=0.0; }
        if rw < 0.0 { rw=-rw; rx=-rx; ry=-ry; rz=-rz; }
        assert_relative_eq!(d1.q0, rw, epsilon = 1e-12);
        assert_relative_eq!(d1.q1, rx, epsilon = 1e-12);
        assert_relative_eq!(d1.q2, ry, epsilon = 1e-12);
        assert_relative_eq!(d1.q3, rz, epsilon = 1e-12);

        // 单位四元数校验：同一四元数相除得到单位
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q3 = q1 / q2;
        assert_relative_eq!(q3.q0, 1.0);
        assert_relative_eq!(q3.q1, 0.0);
        assert_relative_eq!(q3.q2, 0.0);
        assert_relative_eq!(q3.q3, 0.0);

        // 引用版本一致性
        let q1_1 = &q1.clone();
        let q2_2 = &q2.clone();
        let q3 = q1_1 / q2_2;
        assert_relative_eq!(q3.q0, 1.0);
        assert_relative_eq!(q3.q1, 0.0);
        assert_relative_eq!(q3.q2, 0.0);
        assert_relative_eq!(q3.q3, 0.0);
    }

    #[test]
    fn test_default() {
        let q = Quaternion::default();
        assert_relative_eq!(q.q0, 1.0);
        assert_relative_eq!(q.q1, 0.0);
        assert_relative_eq!(q.q2, 0.0);
        assert_relative_eq!(q.q3, 0.0);
    }

    #[test]
    fn test_to_cos_matrix() {
        let q = Quaternion::new(0.199075058, 0.0910919681, -0.969775259, -0.107737795);
        let cos = q.to_cos_matrix();
        let expected_pa: [[f64; 3]; 3] = [
            [-0.904143035, -0.219573289, 0.366488039],
            [-0.133781672, 0.960189641, 0.245231181],
            [-0.405744255, 0.172694623, -0.897523642],
        ];
        let actual = cos.to_array();
        for (row, list) in actual.iter().enumerate() {
            for (col, value) in list.iter().enumerate() {
                assert_relative_eq!(*value, expected_pa[row][col], epsilon = 1e-7);
            }
        }
    }

    #[test]
    fn test_linear_transform() {
        let q = Quaternion::new(0.0791359246, -0.641401172, 0.746174634, 0.159892201);
        let m = Matrix::<4, 4, f64>::new([
            [
                0.9999949337802935,
                -0.0029879893040530093,
                0.0010973906667888372,
                -8.209128966888257e-6,
            ],
            [
                0.0029879893040530093,
                0.9999949337802935,
                -8.209128966888257e-6,
                -0.0010973906667888372,
            ],
            [
                -0.0010973906667888372,
                8.209128966888257e-6,
                0.9999949337802935,
                -0.0029879893040530093,
            ],
            [
                8.209128966888257e-6,
                0.0010973906667888372,
                0.0029879893040530093,
                0.9999949337802935,
            ],
        ]);
        let result = q.linear_transform(m);
        let expected = Quaternion::new(
            0.08042685960061452,
            -0.6434526697740315,
            0.744166198273059,
            0.16036492675833178,
        );
        assert_relative_eq!(result.q0, expected.q0, epsilon = 1e-7);
        assert_relative_eq!(result.q1, expected.q1, epsilon = 1e-7);
        assert_relative_eq!(result.q2, expected.q2, epsilon = 1e-7);
        assert_relative_eq!(result.q3, expected.q3, epsilon = 1e-7);
    }

    #[test]
    fn test_ksi_matrix() {
        let q = Quaternion::new(0.081541, -0.644067, 0.743605, 0.15994);
        let result = q.ksi_matrix();
        let expected = [
            [0.081541, -0.15994, 0.743605],
            [0.15994, 0.081541, 0.644067],
            [-0.743605, -0.644067, 0.081541],
            [0.644067, -0.743605, -0.15994],
        ];
        for (row, list) in expected.iter().enumerate() {
            for (col, exp) in list.iter().enumerate() {
                assert_relative_eq!(*exp, result.get(col, row).unwrap(), epsilon = 1e-7);
            }
        }
    }

    #[test]
    fn test_add() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(5.0, 6.0, 7.0, 8.0);
        let q3 = q1 + q2;
        assert_relative_eq!(q3.q0, 6.0);
        assert_relative_eq!(q3.q1, 8.0);
        assert_relative_eq!(q3.q2, 10.0);
        assert_relative_eq!(q3.q3, 12.0);

        let q1_1 = &q1.clone();
        let q2_1 = &q2.clone();
        let q3_1 = q1_1 + q2_1;
        assert_relative_eq!(q3_1.q0, 6.0);
        assert_relative_eq!(q3_1.q1, 8.0);
        assert_relative_eq!(q3_1.q2, 10.0);
        assert_relative_eq!(q3_1.q3, 12.0);
    }

    #[test]
    fn test_get_value() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let (q0, q1, q2, q3) = q.get_value();
        assert_relative_eq!(q0, 1.0);
        assert_relative_eq!(q1, 2.0);
        assert_relative_eq!(q2, 3.0);
        assert_relative_eq!(q3, 4.0);
    }
}
