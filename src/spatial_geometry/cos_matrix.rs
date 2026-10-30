use crate::dense::Matrix;
use crate::physics::basic::vector::angular::RotationSeq;
use crate::physics::basic::{Angular, Coef, Vector3, VectorQuantity};
use crate::spatial_geometry::quaternion::Quaternion;
use crate::utils::float::sgn;
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

    pub fn from_matrix(m: &Matrix<3, 3, f64>) -> Self {
        let mut data: [f64; 9] = [0.0; 9];
        for row in 0..3 {
            for col in 0..3 {
                data[row * 3 + col] = m.get(col, row).unwrap();
            }
        }
        CosMatrix { data }
    }

    /// 设置指定行的向量值
    /// # Panics
    /// 如果行索引不在0-2范围内会panic
    pub fn set_row_vector_by_array(&mut self, row: usize, data: [f64; 3]) {
        assert!(row < 3, "Row index out of bounds");
        let start = row * 3;
        self.data[start] = data[0];
        self.data[start + 1] = data[1];
        self.data[start + 2] = data[2];
    }

    /// 设置指定列的向量值
    /// # Panics
    /// 如果列索引不在0-2范围内会panic
    pub fn set_col_vector_by_array(&mut self, col: usize, data: [f64; 3]) {
        assert!(col < 3, "Column index out of bounds");
        self.data[col] = data[0];
        self.data[col + 3] = data[1];
        self.data[col + 6] = data[2];
    }

    pub fn get_x_row_vector(&self) -> Vector3<Coef> {
        Vector3::new(
            Coef::new(self.data[0]), // data[0][0] = 第0行第0列
            Coef::new(self.data[1]), // data[0][1] = 第0行第1列
            Coef::new(self.data[2]), // data[0][2] = 第0行第2列
        )
    }

    pub fn get_y_row_vector(&self) -> Vector3<Coef> {
        Vector3::new(
            Coef::new(self.data[3]), // data[1][0] = 第1行第0列
            Coef::new(self.data[4]), // data[1][1] = 第1行第1列
            Coef::new(self.data[5]), // data[1][2] = 第1行第2列
        )
    }

    pub fn get_z_row_vector(&self) -> Vector3<Coef> {
        Vector3::new(
            Coef::new(self.data[6]), // data[2][0] = 第2行第0列
            Coef::new(self.data[7]), // data[2][1] = 第2行第1列
            Coef::new(self.data[8]), // data[2][2] = 第2行第2列
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
            let isgn = sgn(p_mat[2][0] - p_mat[0][2]);
            let tqi = f64::sqrt(b22) / 2.0 * isgn as f64;

            result.q1 = 0.25 * (p_mat[1][0] + p_mat[0][1]) / tqi;
            result.q2 = tqi;
            result.q3 = 0.25 * (p_mat[2][1] + p_mat[1][2]) / tqi;
            result.q0 = 0.25 * (p_mat[2][0] - p_mat[0][2]) / tqi;
        } else if c11 > 0.004 {
            let isgn = sgn(p_mat[1][2] - p_mat[2][1]);
            let tqi = f64::sqrt(c11) / 2.0 * isgn as f64;

            result.q1 = tqi;
            result.q2 = 0.25 * (p_mat[1][0] + p_mat[0][1]) / tqi;
            result.q3 = 0.25 * (p_mat[0][2] + p_mat[2][0]) / tqi;
            result.q0 = 0.25 * (p_mat[1][2] - p_mat[2][1]) / tqi;
        } else {
            let d33 = 1.0 - p_mat[0][0] - p_mat[1][1] + p_mat[2][2];

            let isgn = sgn(p_mat[0][1] - p_mat[1][0]);
            let tqi = f64::sqrt(d33) / 2.0 * isgn as f64;

            result.q1 = 0.25 * (p_mat[0][2] + p_mat[2][0]) / tqi;
            result.q2 = 0.25 * (p_mat[1][2] + p_mat[2][1]) / tqi;
            result.q3 = tqi;
            result.q0 = 0.25 * (p_mat[0][1] - p_mat[1][0]) / tqi;
        }
        result.normalize()
    }

    // 与一个列向量进行矩阵乘法，得到一个列向量
    pub fn product_vector<T: VectorQuantity + Default>(&self, vec: &Vector3<T>) -> Vector3<T> {
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
    // 将余弦阵转换到欧拉角，YXZ 转序
    //
    // 对应 Pitch(Y)-Roll(X)-Yaw(Z)。返回值按轴序：`.x` 是绕 X 的角、
    // `.y` 绕 Y、`.z` 绕 Z —— 与 `to_euler(YXZ)` 完全一致（有测试钉住）。
    //
    // 注意：这里的「YXZ」是轴的先后顺序，与 `to_cos_matrix` 的「步骤序」
    // 角度分配不同，两者往返时需要重排角度。
    pub fn to_pry(&self) -> Vector3<Angular> {
        let data = self.to_array();
        let y = Angular::atan2(data[2][0], data[2][2]);
        let z = Angular::atan2(data[0][1], data[1][1]);
        let tempf = data[2][1].abs();
        let x: Angular = if tempf > 1.0 {
            Angular::asin(-data[2][1] / tempf)
        } else {
            Angular::asin(-data[2][1])
        };
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

    /// 按给定旋转顺序把方向余弦矩阵分解为欧拉角。
    ///
    /// 返回值的 `x` / `y` / `z` 分量是**绕对应轴的转角**，与 `seq` 的写入顺序无关。
    /// 这一点与 [`Vector3::<Angular>::to_cos_matrix`] 不同：那个函数按**步骤顺序**
    /// 取角度（`.x` 是第一步的转角）。两者只在序列以 X 开头时等价，
    /// 做往返验证时需要按 `seq` 重排角度。
    ///
    /// # Panics
    ///
    /// `seq` 不是恰好三个轴时 panic（`RotationSeq::value()` 返回 `Err`）。
    /// 三个轴有重复时也 panic —— 例如 `X-X-X` 不是合法的欧拉序列，
    /// 雅可比矩阵退化，没有唯一的分解。
    ///
    /// 这两种输入在构造阶段就能避免：`x()` / `y()` / `z()` 各返回 `Result`，
    /// 第三次之后的调用会返回 `Err`。本函数不做二次校验。
    pub fn to_euler(&self, seq: &RotationSeq) -> Vector3<Angular> {
        // 获取旋转顺序的编码值
        let seq_tag = seq.value().unwrap();

        // 解析旋转顺序的三个轴。
        //
        // tag 的每个 2 bit 段由 `RotationSeq::set_tag` 写入，只可能是
        // `TAG_X`/`TAG_Y`/`TAG_Z`（1/2/3），所以这里的取值本身就已经是
        // 需要的 1/2/3 编码，不需要再逐位 match。非法的 0 或重复轴
        // 由下面 `ijk` 的兜底分支统一处理。
        let i = ((seq_tag >> 4) & 0x3) as u32;
        let j = ((seq_tag >> 2) & 0x3) as u32;
        let k = (seq_tag & 0x3) as u32;

        // 组合成三位数字编码，如 ZYX → 321
        let ijk = i * 100 + j * 10 + k;

        // 全程 f64。
        //
        // 历史注意：本函数曾在内部转成 f32 再转回 f64 —— 那是为了匹配已删除的
        // `dcm2angle` 的参数类型。该中转会让中间角（asin 入参）在接近 ±1 时
        // 被 f32 精度（约 1e-7）截断，导致中间角在趋近万向锁时退化到精确的
        // ±π/2。实测 89.99° 的解会算成 −90.000°，偏差 1.7e-4 rad。
        let m = self.to_array();

        let mut euler: [f64; 3] = [0.0; 3];
        match ijk {
            321 => {
                euler[0] = f64::atan2(m[1][2], m[2][2]);
                euler[1] = safe_asin(0.0 - m[0][2]);
                euler[2] = f64::atan2(m[0][1], m[0][0]);
            }
            312 => {
                euler[0] = safe_asin(0.0 + m[1][2]);
                euler[1] = f64::atan2(0.0 - m[0][2], m[2][2]);
                euler[2] = f64::atan2(0.0 - m[1][0], m[1][1]);
            }
            123 => {
                euler[0] = f64::atan2(0.0 - m[2][1], m[2][2]);
                euler[1] = safe_asin(m[2][0]);
                euler[2] = f64::atan2(0.0 - m[1][0], m[0][0]);
            }
            132 => {
                euler[0] = f64::atan2(m[1][2], m[1][1]);
                euler[1] = f64::atan2(m[2][0], m[0][0]);
                euler[2] = safe_asin(0.0 - m[1][0]);
            }
            213 => {
                euler[0] = safe_asin(0.0 - m[2][1]);
                euler[1] = f64::atan2(m[2][0], m[2][2]);
                euler[2] = f64::atan2(m[0][1], m[1][1]);
            }
            231 => {
                euler[0] = f64::atan2(0.0 - m[2][1], m[1][1]);
                euler[1] = f64::atan2(0.0 - m[0][2], m[0][0]);
                euler[2] = safe_asin(m[0][1]);
            }
            _ => panic!("Unsupported rotation sequence: {}", ijk),
        };

        // 将计算得到的欧拉角转换为Angular类型并返回
        Vector3::new(
            Angular::from_rad(euler[0]),
            Angular::from_rad(euler[1]),
            Angular::from_rad(euler[2]),
        )
    }
}

/// `asin` 的安全版本：入参先钳制到 `[-1, 1]`。
///
/// 合法 DCM 的元素恒在 `[-1, 1]` 内，但连续矩阵乘法会引入舍入漂移
/// （实测 1000 次累乘后可能出现 `1.00000000000005`）。直接传给 `asin`
/// 会得到 NaN，且这个 NaN 会一路污染到姿态输出。
///
/// 超出范围的部分按边界值处理：`|x| > 1` 在几何上等价于「已经越过万向锁」，
/// 返回 `±π/2` 是唯一合理答案（这也是 `to_pry` / `to_rpy` 原本就有的处理）。
fn safe_asin(x: f64) -> f64 {
    if x > 1.0 {
        std::f64::consts::FRAC_PI_2
    } else if x < -1.0 {
        -std::f64::consts::FRAC_PI_2
    } else {
        x.asin()
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
    fn test_cos_matrix_get_x_row_vector() {
        let cos = CosMatrix::new([[1.0, 1.1, 1.2], [1.3, 1.4, 1.5], [1.6, 1.7, 1.8]]);
        let x = cos.get_x_row_vector();
        assert_relative_eq!(1.0, x.x.get_value());
        assert_relative_eq!(1.1, x.y.get_value());
        assert_relative_eq!(1.2, x.z.get_value());

        let y = cos.get_y_row_vector();
        assert_relative_eq!(1.3, y.x.get_value());
        assert_relative_eq!(1.4, y.y.get_value());
        assert_relative_eq!(1.5, y.z.get_value());

        let z = cos.get_z_row_vector();
        assert_relative_eq!(1.6, z.x.get_value());
        assert_relative_eq!(1.7, z.y.get_value());
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

        assert_relative_eq!(q.q0, 0.59405565, epsilon = 1e-6);
        assert_relative_eq!(q.q1, 0.46136004, epsilon = 1e-6);
        assert_relative_eq!(q.q2, -0.45451167, epsilon = 1e-6);
        assert_relative_eq!(q.q3, 0.4771415, epsilon = 1e-6);

        let cos = CosMatrix::new([
            [-0.99995, 0.00999, -0.00249],
            [0.00999, 0.00005, -0.99995],
            [0.00249, 0.99995, 0.00005],
        ]);
        let q = cos.to_quaternion();
        assert_relative_eq!(q.q0, 0.0024899303, epsilon = 1e-6);
        assert_relative_eq!(q.q1, 0.00998972, epsilon = 1e-6);
        assert_relative_eq!(q.q2, 0.99994695, epsilon = 1e-6);
        assert_relative_eq!(q.q3, 0.0, epsilon = 1e-6);

        let cos = CosMatrix::new([
            [0.9999, -0.007, -0.001],
            [0.007, -0.0001, -0.9998],
            [0.001, 0.9998, -0.007],
        ]);
        let q = cos.to_quaternion();
        assert_relative_eq!(q.q0, 0.70589256, epsilon = 1e-6);
        assert_relative_eq!(q.q1, -0.70830125, epsilon = 1e-6);
        assert_relative_eq!(q.q2, 0.00070844294, epsilon = 1e-6);
        assert_relative_eq!(q.q3, -0.004959101, epsilon = 1e-6);

        let cos = CosMatrix::new([
            [1.0, 0.0, 0.0],
            [0.0, -0.999998477, -0.00174532837],
            [0.0, 0.00174532837, -0.999998477],
        ]);
        let q = cos.to_quaternion();
        assert_relative_eq!(q.q0, 0.0008726645, epsilon = 1e-6);
        assert_relative_eq!(q.q1, -0.99999964, epsilon = 1e-6);
        assert_relative_eq!(q.q2, -0.0, epsilon = 1e-6);
        assert_relative_eq!(q.q3, -0.0, epsilon = 1e-6);

        let cos = CosMatrix::new([
            [-0.999998477, -0.00174532837, 0.0],
            [0.00174532837, -0.999998477, 0.0],
            [0.0, 0.0, 1.0],
        ]);
        let q = cos.to_quaternion();
        assert_relative_eq!(q.q0, 0.0008726646, epsilon = 1e-6);
        assert_relative_eq!(q.q1, -0.0, epsilon = 1e-6);
        assert_relative_eq!(q.q2, -0.0, epsilon = 1e-6);
        assert_relative_eq!(q.q3, -0.99999964, epsilon = 1e-6);
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

    // to_pry / to_rpy 的 `if tempf > 1.0` 防御分支：|元素| > 1 时先把 asin 的入参
    // 归一化到 [-1, 1]，避免 asin 返回 NaN。合法 DCM 的 |元素| 恒 ≤ 1，所以只有
    // 退化矩阵（数值漂移或未经校验的输入）才走这条路径。
    #[test]
    fn test_pry_rpy_degenerate_guard() {
        // to_pry: x = asin(-data[2][1])，取 data[2][1] = 1.5 触发 tempf > 1.0
        let cos = CosMatrix::new([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.5, 1.0],
        ]);
        let x = cos.to_pry().x.as_rad();
        assert!(!x.is_nan(), "asin 入参未归一化会得到 NaN");
        assert_relative_eq!(x, (-1.0f64).asin(), epsilon = 1e-12); // -π/2

        // to_rpy: y = asin(data[2][0])，取 data[2][0] = 1.5 触发守卫
        let cos = CosMatrix::new([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.5, 0.0, 1.0],
        ]);
        let y = cos.to_rpy().y.as_rad();
        assert!(!y.is_nan(), "asin 入参未归一化会得到 NaN");
        assert_relative_eq!(y, 1.0f64.asin(), epsilon = 1e-12); // π/2

        // 负向同样要走归一化分支：data[2][1] = -2.0 → asin(2.0/2.0) = asin(1) = π/2
        let cos = CosMatrix::new([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -2.0, 1.0],
        ]);
        let x = cos.to_pry().x.as_rad();
        assert_relative_eq!(x, 1.0f64.asin(), epsilon = 1e-12);
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
        let test_data = [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6], [7.7, 8.8, 9.9]];
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
        let test_data = [[0.1, -0.2, 0.3], [-0.4, 0.5, -0.6], [0.7, -0.8, 0.9]];
        let cos = CosMatrix::new(test_data);
        let original_matrix = cos.to_matrix();
        let transposed_matrix = cos.transfer().to_matrix();

        // Verify original_matrix^T equals transposed_matrix
        for i in 0..3 {
            for j in 0..3 {
                assert_relative_eq!(
                    original_matrix.get(i, j).unwrap(),
                    transposed_matrix.get(j, i).unwrap()
                );
            }
        }
    }

    #[test]
    fn test_to_matrix_with_extreme_values() {
        // Test with extreme values (very large and small numbers)
        let test_data = [
            [f64::MAX, f64::MIN, 0.0],
            [0.0, f64::MAX, f64::MIN],
            [f64::MIN, 0.0, f64::MAX],
        ];
        let cos = CosMatrix::new(test_data);
        let matrix = cos.to_matrix();

        // Verify all elements match exactly
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix.get(j, i).unwrap(), test_data[i][j]);
            }
        }
    }

    #[test]
    fn test_to_matrix_with_nan_values() {
        // Test with NaN values (should propagate NaN)
        let test_data = [[1.0, f64::NAN, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let cos = CosMatrix::new(test_data);
        let matrix = cos.to_matrix();

        // Verify NaN propagates correctly
        assert!(matrix.get(1, 0).unwrap().is_nan());
        assert!(!matrix.get(0, 0).unwrap().is_nan());
    }

    #[test]
    fn test_from_matrix_with_values() {
        // Test with arbitrary values
        let test_data = [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6], [7.7, 8.8, 9.9]];
        let matrix = Matrix::new(test_data);
        let cos_matrix = CosMatrix::from_matrix(&matrix);
        // Verify values are correctly transposed and stored
        for row in 0..3 {
            for col in 0..3 {
                assert_relative_eq!(cos_matrix.data[row * 3 + col], test_data[row][col]);
            }
        }
    }

    #[test]
    fn test_set_row_vector_by_array() {
        let mut cos = CosMatrix::unit();
        // Set first row
        cos.set_row_vector_by_array(0, [1.1, 2.2, 3.3]);
        assert_relative_eq!(cos.data[0], 1.1);
        assert_relative_eq!(cos.data[1], 2.2);
        assert_relative_eq!(cos.data[2], 3.3);
        // Other rows should remain unchanged
        assert_relative_eq!(cos.data[3], 0.0);
        assert_relative_eq!(cos.data[4], 1.0);
        assert_relative_eq!(cos.data[5], 0.0);
        assert_relative_eq!(cos.data[6], 0.0);
        assert_relative_eq!(cos.data[7], 0.0);
        assert_relative_eq!(cos.data[8], 1.0);

        // Set last row
        cos.set_row_vector_by_array(2, [4.4, 5.5, 6.6]);
        assert_relative_eq!(cos.data[6], 4.4);
        assert_relative_eq!(cos.data[7], 5.5);
        assert_relative_eq!(cos.data[8], 6.6);
    }

    #[test]
    #[should_panic(expected = "Row index out of bounds")]
    fn test_set_row_vector_by_array_out_of_bounds() {
        let mut cos = CosMatrix::unit();
        cos.set_row_vector_by_array(3, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_set_col_vector_by_array() {
        let mut cos = CosMatrix::unit();
        // Set first column
        cos.set_col_vector_by_array(0, [1.1, 2.2, 3.3]);
        assert_relative_eq!(cos.data[0], 1.1);
        assert_relative_eq!(cos.data[3], 2.2);
        assert_relative_eq!(cos.data[6], 3.3);
        // Other columns should remain unchanged
        assert_relative_eq!(cos.data[1], 0.0);
        assert_relative_eq!(cos.data[2], 0.0);
        assert_relative_eq!(cos.data[4], 1.0);
        assert_relative_eq!(cos.data[5], 0.0);
        assert_relative_eq!(cos.data[7], 0.0);
        assert_relative_eq!(cos.data[8], 1.0);

        // Set last column
        cos.set_col_vector_by_array(2, [4.4, 5.5, 6.6]);
        assert_relative_eq!(cos.data[2], 4.4);
        assert_relative_eq!(cos.data[5], 5.5);
        assert_relative_eq!(cos.data[8], 6.6);
    }

    #[test]
    #[should_panic(expected = "Column index out of bounds")]
    fn test_set_col_vector_by_array_out_of_bounds() {
        let mut cos = CosMatrix::unit();
        cos.set_col_vector_by_array(3, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn specific_cos_matrix_to_quaternion() {
        let cos = CosMatrix::new([
            [-0.302833493, 0.428414506, 0.851324196],
            [-0.456789010, -0.789012340, 0.234567890],
            [0.820272150, -0.337628264, 0.461693357],
        ]);
        let quat = cos.to_quaternion();
        let (q0, q1, q2, q3) = quat.get_value();
        println!("q0: {}, q1: {}, q2: {}, q3: {}", q0, q1, q2, q3);
    }

    #[test]
    fn test_to_euler() {
        use crate::physics::basic::vector::angular::RotationSeq;
        use approx::assert_relative_eq;

        println!("=== 测试to_euler方法 ===");

        // 测试1: 单位矩阵，预期欧拉角全为0
        println!("\n测试1: 单位矩阵");
        let identity_matrix: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let identity = CosMatrix::new(identity_matrix);

        // 测试所有支持的旋转顺序
        let rotation_orders = [
            (321, "ZYX"),
            (312, "ZXY"),
            (123, "XYZ"),
            (132, "XZY"),
            (213, "YXZ"),
            (231, "YZX"),
        ];

        for (order_code, order_name) in rotation_orders.iter() {
            // 创建对应的RotationSeq
            let mut seq = RotationSeq::default();
            match order_code {
                321 => {
                    seq.z().unwrap();
                    seq.y().unwrap();
                    seq.x().unwrap();
                }
                312 => {
                    seq.z().unwrap();
                    seq.x().unwrap();
                    seq.y().unwrap();
                }
                123 => {
                    seq.x().unwrap();
                    seq.y().unwrap();
                    seq.z().unwrap();
                }
                132 => {
                    seq.x().unwrap();
                    seq.z().unwrap();
                    seq.y().unwrap();
                }
                213 => {
                    seq.y().unwrap();
                    seq.x().unwrap();
                    seq.z().unwrap();
                }
                231 => {
                    seq.y().unwrap();
                    seq.z().unwrap();
                    seq.x().unwrap();
                }
                _ => panic!("Unsupported rotation order: {}", order_code),
            }

            let euler = identity.to_euler(&seq);
            println!(
                "旋转顺序 {}: 欧拉角 = [{:.6}, {:.6}, {:.6}]",
                order_name,
                euler.x.as_rad(),
                euler.y.as_rad(),
                euler.z.as_rad()
            );

            // 单位矩阵的欧拉角应该全为0
            assert_relative_eq!(euler.x.as_rad(), 0.0, epsilon = 1e-6);
            assert_relative_eq!(euler.y.as_rad(), 0.0, epsilon = 1e-6);
            assert_relative_eq!(euler.z.as_rad(), 0.0, epsilon = 1e-6);
        }

        // 测试2: 已知旋转角度的矩阵（与dcm2angle测试用例相同）
        println!("\n测试2: 已知旋转角度的矩阵（Z-Y-X顺序）");
        let test_matrix_321: [[f64; 3]; 3] = [
            [0.5, 0.5, 0.7071],
            [-0.5, 0.5, 0.7071],
            [-0.7071, 0.7071, 0.0],
        ];
        let cos_321 = CosMatrix::new(test_matrix_321);

        // 测试所有支持的旋转顺序
        for (order_code, order_name) in rotation_orders.iter() {
            // 创建对应的RotationSeq
            let mut seq = RotationSeq::default();
            match order_code {
                321 => {
                    seq.z().unwrap();
                    seq.y().unwrap();
                    seq.x().unwrap();
                }
                312 => {
                    seq.z().unwrap();
                    seq.x().unwrap();
                    seq.y().unwrap();
                }
                123 => {
                    seq.x().unwrap();
                    seq.y().unwrap();
                    seq.z().unwrap();
                }
                132 => {
                    seq.x().unwrap();
                    seq.z().unwrap();
                    seq.y().unwrap();
                }
                213 => {
                    seq.y().unwrap();
                    seq.x().unwrap();
                    seq.z().unwrap();
                }
                231 => {
                    seq.y().unwrap();
                    seq.z().unwrap();
                    seq.x().unwrap();
                }
                _ => panic!("Unsupported rotation order: {}", order_code),
            }

            let euler = cos_321.to_euler(&seq);

            println!(
                "旋转顺序 {}: to_euler = [{:.6}, {:.6}, {:.6}]",
                order_name,
                euler.x.as_rad(),
                euler.y.as_rad(),
                euler.z.as_rad()
            );

            // 验证结果合理性，确保角度在合理范围内
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.x.as_rad()));
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.y.as_rad()));
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.z.as_rad()));
        }

        // 测试3: 仅绕X轴旋转90度的矩阵（与dcm2angle测试用例相同）
        println!("\n测试3: 仅绕X轴旋转90度");
        let x_rot_90: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]];
        let cos_x_rot = CosMatrix::new(x_rot_90);

        for (order_code, order_name) in rotation_orders.iter() {
            // 创建对应的RotationSeq
            let mut seq = RotationSeq::default();
            match order_code {
                321 => {
                    seq.z().unwrap();
                    seq.y().unwrap();
                    seq.x().unwrap();
                }
                312 => {
                    seq.z().unwrap();
                    seq.x().unwrap();
                    seq.y().unwrap();
                }
                123 => {
                    seq.x().unwrap();
                    seq.y().unwrap();
                    seq.z().unwrap();
                }
                132 => {
                    seq.x().unwrap();
                    seq.z().unwrap();
                    seq.y().unwrap();
                }
                213 => {
                    seq.y().unwrap();
                    seq.x().unwrap();
                    seq.z().unwrap();
                }
                231 => {
                    seq.y().unwrap();
                    seq.z().unwrap();
                    seq.x().unwrap();
                }
                _ => panic!("Unsupported rotation order: {}", order_code),
            }

            let euler = cos_x_rot.to_euler(&seq);

            println!(
                "旋转顺序 {}: to_euler = [{:.6}, {:.6}, {:.6}]",
                order_name,
                euler.x.as_rad(),
                euler.y.as_rad(),
                euler.z.as_rad()
            );

            // 验证结果合理性，确保角度在合理范围内
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.x.as_rad()));
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.y.as_rad()));
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.z.as_rad()));
        }

        // 测试4: 仅绕Y轴旋转90度的矩阵（与dcm2angle测试用例相同）
        println!("\n测试4: 仅绕Y轴旋转90度");
        let y_rot_90: [[f64; 3]; 3] = [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]];
        let cos_y_rot = CosMatrix::new(y_rot_90);

        for (order_code, order_name) in rotation_orders.iter() {
            // 创建对应的RotationSeq
            let mut seq = RotationSeq::default();
            match order_code {
                321 => {
                    seq.z().unwrap();
                    seq.y().unwrap();
                    seq.x().unwrap();
                }
                312 => {
                    seq.z().unwrap();
                    seq.x().unwrap();
                    seq.y().unwrap();
                }
                123 => {
                    seq.x().unwrap();
                    seq.y().unwrap();
                    seq.z().unwrap();
                }
                132 => {
                    seq.x().unwrap();
                    seq.z().unwrap();
                    seq.y().unwrap();
                }
                213 => {
                    seq.y().unwrap();
                    seq.x().unwrap();
                    seq.z().unwrap();
                }
                231 => {
                    seq.y().unwrap();
                    seq.z().unwrap();
                    seq.x().unwrap();
                }
                _ => panic!("Unsupported rotation order: {}", order_code),
            }

            let euler = cos_y_rot.to_euler(&seq);

            println!(
                "旋转顺序 {}: to_euler = [{:.6}, {:.6}, {:.6}]",
                order_name,
                euler.x.as_rad(),
                euler.y.as_rad(),
                euler.z.as_rad()
            );

            // 验证结果合理性，确保角度在合理范围内
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.x.as_rad()));
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.y.as_rad()));
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.z.as_rad()));
        }

        // 测试5: 仅绕Z轴旋转90度的矩阵（与dcm2angle测试用例相同）
        println!("\n测试5: 仅绕Z轴旋转90度");
        let z_rot_90: [[f64; 3]; 3] = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        let cos_z_rot = CosMatrix::new(z_rot_90);

        for (order_code, order_name) in rotation_orders.iter() {
            // 创建对应的RotationSeq
            let mut seq = RotationSeq::default();
            match order_code {
                321 => {
                    seq.z().unwrap();
                    seq.y().unwrap();
                    seq.x().unwrap();
                }
                312 => {
                    seq.z().unwrap();
                    seq.x().unwrap();
                    seq.y().unwrap();
                }
                123 => {
                    seq.x().unwrap();
                    seq.y().unwrap();
                    seq.z().unwrap();
                }
                132 => {
                    seq.x().unwrap();
                    seq.z().unwrap();
                    seq.y().unwrap();
                }
                213 => {
                    seq.y().unwrap();
                    seq.x().unwrap();
                    seq.z().unwrap();
                }
                231 => {
                    seq.y().unwrap();
                    seq.z().unwrap();
                    seq.x().unwrap();
                }
                _ => panic!("Unsupported rotation order: {}", order_code),
            }

            let euler = cos_z_rot.to_euler(&seq);

            println!(
                "旋转顺序 {}: to_euler = [{:.6}, {:.6}, {:.6}]",
                order_name,
                euler.x.as_rad(),
                euler.y.as_rad(),
                euler.z.as_rad()
            );

            // 验证结果合理性，确保角度在合理范围内
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.x.as_rad()));
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.y.as_rad()));
            assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&euler.z.as_rad()));
        }

        println!("\n=== 所有to_euler测试通过！ ===");
    }

    // ===== to_euler 精度与健壮性回归 =====

    fn seq_of(s: &str) -> crate::physics::basic::vector::angular::RotationSeq {
        use crate::physics::basic::vector::angular::RotationSeq;
        let mut q = RotationSeq::default();
        for ch in s.chars() {
            match ch {
                'X' => q.x().unwrap(),
                'Y' => q.y().unwrap(),
                _ => q.z().unwrap(),
            };
        }
        q
    }

    /// 用库自带的正向函数构造 DCM，再反解，验证往返精度。
    ///
    /// # 两个方向的角度约定不同（易踩）
    ///
    /// - `to_cos_matrix`：**步骤序**。返回的 `Angular.x` 是「第 1 步绕该步轴的角」，
    ///   `.y` 是第 2 步，`.z` 是第 3 步。ZYX 时 `.x` 就是绕 Z 的角。
    /// - `to_euler`：**轴序**。返回的 `.x` 恒为绕 X 轴的角，与序列位置无关。
    ///
    /// 所以往返时必须把轴键角按 `seq` 重排成步骤序喂给 `to_cos_matrix`。
    /// 序列以 X 开头时两者恰好相同，其余序列不等价——这也是本测试用
    /// 重排而非直接用同一组角的原因。
    fn round_trip_angles(seq_name: &str, per_axis: [f64; 3]) -> [f64; 3] {
        use crate::physics::basic::vector::angular::RotationHand;
        use crate::physics::basic::{Angular, Vector3};

        let step: [f64; 3] = {
            let mut s = [0.0_f64; 3];
            for (t, ch) in seq_name.chars().enumerate() {
                let idx = match ch {
                    'X' => 0,
                    'Y' => 1,
                    _ => 2,
                };
                s[t] = per_axis[idx];
            }
            s
        };
        let ang = Vector3::new(
            Angular::from_rad(step[0]),
            Angular::from_rad(step[1]),
            Angular::from_rad(step[2]),
        );
        let m = ang
            .to_cos_matrix(seq_of(seq_name), RotationHand::Left)
            .unwrap();
        let e = m.to_euler(&seq_of(seq_name));
        [e.x.as_rad(), e.y.as_rad(), e.z.as_rad()]
    }

    /// 六转序在非退化角下必须精确往返。
    #[test]
    fn test_to_euler_round_trip_all_sequences() {
        let per_axis: [f64; 3] = [
            10.0_f64.to_radians(),
            (-35.0_f64).to_radians(),
            20.0_f64.to_radians(),
        ];
        for name in ["ZYX", "ZXY", "XYZ", "XZY", "YXZ", "YZX"] {
            let got = round_trip_angles(name, per_axis);
            for i in 0..3 {
                assert_relative_eq!(got[i], per_axis[i], epsilon = 1e-12);
            }
        }
    }

    /// 万向锁附近必须保持 f64 精度。
    ///
    /// 回归对象：`to_euler` 曾经内部转成 `f32` 再转回 `f64`（为匹配已删除的
    /// `dcm2angle` 参数类型）。f32 只有约 7 位十进制精度，当中间角趋近 90° 时
    /// `asin` 入参是 `1 − ε`，ε 小于 f32 epsilon 就被舍入成 1，导致解退化到
    /// 精确的 ±π/2。实测 89.99° 的误差曾达 1.75e-4 rad（约 0.01°）。
    #[test]
    fn test_to_euler_precision_near_gimbal_lock() {
        // 中间轴（第二个轴）是万向锁敏感轴
        let middle_of = |name: &str| -> usize {
            match name.chars().nth(1).unwrap() {
                'X' => 0,
                'Y' => 1,
                _ => 2,
            }
        };

        for mid_deg in [30.0_f64, 85.0, 89.9, 89.99] {
            for name in ["ZYX", "ZXY", "XYZ", "XZY", "YXZ", "YZX"] {
                let mi = middle_of(name);
                let mut per_axis: [f64; 3] = [
                    10.0_f64.to_radians(),
                    (-30.0_f64).to_radians(),
                    20.0_f64.to_radians(),
                ];
                per_axis[mi] = (-mid_deg).to_radians();

                let got = round_trip_angles(name, per_axis);
                let err = (0..3)
                    .map(|i| (got[i] - per_axis[i]).abs())
                    .fold(0.0_f64, f64::max);
                assert!(
                    err < 1e-10,
                    "{} 中间角 {:.2}° 误差 {:.3e} 过大，解={:?}，期望={:?}",
                    name,
                    mid_deg,
                    err,
                    got,
                    per_axis
                );
            }
        }
    }

    /// `safe_asin` 双向钳制都要覆盖。
    #[test]
    fn test_safe_asin_clamps_both_directions() {
        let half_pi = std::f64::consts::FRAC_PI_2;
        // 正常区间原样返回
        assert_relative_eq!(safe_asin(0.0), 0.0, epsilon = 1e-15);
        assert_relative_eq!(safe_asin(1.0), half_pi, epsilon = 1e-15);
        assert_relative_eq!(safe_asin(-1.0), -half_pi, epsilon = 1e-15);
        // 正向超界 → +π/2
        assert_relative_eq!(safe_asin(1.0 + 1e-9), half_pi, epsilon = 1e-15);
        assert_relative_eq!(safe_asin(5.0), half_pi, epsilon = 1e-15);
        // 负向超界 → -π/2
        assert_relative_eq!(safe_asin(-1.0 - 1e-9), -half_pi, epsilon = 1e-15);
        assert_relative_eq!(safe_asin(-5.0), -half_pi, epsilon = 1e-15);
        // NaN 原样传播（不吞掉）
        assert!(safe_asin(f64::NAN).is_nan());
    }

    /// `asin` 入参略超 1 时不能返回 NaN。
    ///
    /// 合法 DCM 元素恒在 `[-1, 1]`，但连续矩阵乘法会引入舍入漂移
    /// （实测 1000 次累乘后可出现 `1.00000000000005`）。原先 `to_euler`
    /// 直接调用 `asin`，此时返回 NaN 并污染整个姿态输出；
    /// `to_pry` / `to_rpy` 本来就有守卫，两者行为不一致。
    #[test]
    fn test_to_euler_no_nan_on_out_of_range_input() {
        for over in [1e-12_f64, 1e-9, 1e-7, 1e-4] {
            // 构造让 321 分支 asin 入参为 -(1+over) 的矩阵
            let m = [
                [1.0, 0.0, 1.0 + over],
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 1.0],
            ];
            let e = CosMatrix::new(m).to_euler(&seq_of("ZYX"));
            let got = [e.x.as_rad(), e.y.as_rad(), e.z.as_rad()];
            assert!(
                got.iter().all(|v| v.is_finite()),
                "超出 {:.0e} 时出现非有限值: {:?}",
                over,
                got
            );
            // 越过万向锁时应钳制到 ±π/2。
            // 先把值取出来再断言：把 `got[1]` 直接写进 assert 的消息参数，
            // 那个索引表达式只在断言失败时求值，永远不计入覆盖。
            let mid = got[1];
            let mid_err = (mid.abs() - std::f64::consts::FRAC_PI_2).abs();
            assert!(mid_err < 1e-9, "中间角应钳制到 ±π/2，实际 {}", mid);
        }
    }

    /// `to_euler` 对非法序列 panic，而不是返回错误结果。
    ///
    /// 函数签名返回 `Vector3<Angular>` 而非 `Result`，所以无法传回错误。
    /// 这些 panic 是既有行为；本测试把「哪种输入会 panic」钉住，
    /// 免得以后有人以为它会对任意 `RotationSeq` 都有定义。
    #[test]
    #[should_panic(expected = "Unsupported rotation sequence")]
    fn test_to_euler_panics_on_repeated_axis() {
        let m = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        // X-X-X：三个轴都合法，但重复序列的分解不唯一
        let mut seq = crate::physics::basic::vector::angular::RotationSeq::default();
        seq.x().unwrap();
        seq.x().unwrap();
        seq.x().unwrap();
        let _ = CosMatrix::new(m).to_euler(&seq);
    }

    /// `to_euler` 对轴数不足的序列 panic。
    #[test]
    #[should_panic]
    fn test_to_euler_panics_on_incomplete_sequence() {
        let m = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let mut seq = crate::physics::basic::vector::angular::RotationSeq::default();
        seq.x().unwrap();
        seq.y().unwrap(); // 只有两个轴
        let _ = CosMatrix::new(m).to_euler(&seq);
    }

    /// `to_pry` / `to_rpy` 与 `to_euler` 对应转序必须完全一致。
    ///
    /// 实测结论：`to_rpy` ≡ `to_euler(XYZ)`，`to_pry` ≡ `to_euler(YXZ)`。
    /// 注意 `to_pry` 原注释写的是「xzy」，与实现不符——
    /// YXZ 才对应 Pitch(Y)-Roll(X)-Yaw(Z) 这个名字。本测试钉住实际行为。
    #[test]
    fn test_pry_rpy_agree_with_to_euler() {
        let m = [[0.36_f64, 0.48, -0.8], [-0.8, 0.6, 0.0], [0.48, 0.64, 0.6]];
        let c = CosMatrix::new(m);

        let rpy = c.to_rpy();
        let exyz = c.to_euler(&seq_of("XYZ"));
        assert_relative_eq!(rpy.x.as_rad(), exyz.x.as_rad(), epsilon = 1e-12);
        assert_relative_eq!(rpy.y.as_rad(), exyz.y.as_rad(), epsilon = 1e-12);
        assert_relative_eq!(rpy.z.as_rad(), exyz.z.as_rad(), epsilon = 1e-12);

        let pry = c.to_pry();
        let eyxz = c.to_euler(&seq_of("YXZ"));
        assert_relative_eq!(pry.x.as_rad(), eyxz.x.as_rad(), epsilon = 1e-12);
        assert_relative_eq!(pry.y.as_rad(), eyxz.y.as_rad(), epsilon = 1e-12);
        assert_relative_eq!(pry.z.as_rad(), eyxz.z.as_rad(), epsilon = 1e-12);
    }
}
