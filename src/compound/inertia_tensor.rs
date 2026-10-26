//! 转动惯量张量（3×3）。

use crate::physics::basic::{
    AngularAcceleration, AngularMomentum, AngularVelocity, Inertia, Torque, Vector3,
};
use crate::spatial_geometry::cos_matrix::CosMatrix;

/// 转动惯量张量
///
/// 描述刚体绕**任意方向**的转动惯性。与标量 [`Inertia`] 的关系：
/// 标量是张量沿某条轴的投影，只在绕主轴旋转时两者等价。
///
/// # 为什么需要张量
///
/// 标量的隐含假设是角动量 `L` 与角速度 `ω` 同向（`L = J·ω`，J 是一个数）。
/// 非对称刚体不成立：`ω` 沿 z 轴转，`L` 却会偏出 z 轴，
/// 因为张量有非对角元（惯性积）：
///
/// ```text
/// ω = (0, 0, ω_z)
/// L = J·ω = ( Ixz·ω_z, Iyz·ω_z, Izz·ω_z )
///             ↑        ↑
///           x、y 方向凭空出现角动量
/// ```
///
/// 由此产生的陀螺力矩 `ω × (Jω)` 正是欧拉方程的第二项，
/// 它是姿态动力学不能用标量的根本原因。
///
/// # 六个独立分量
///
/// 张量对称（`J_ij = J_ji`），9 个元素中 3 个重复，实际只需 6 个：
///
/// ```text
///       ┌  Ixx  Ixy  Ixz ┐
/// J  =  │  Ixy  Iyy  Iyz │      对角元：绕单轴的转动惯量
///       └  Ixz  Iyz  Izz ┘      非对角元：惯性积（可取负）
/// ```
///
/// 对称性由构造接口保证：只提供接收 6 个分量的 [`new`](Self::new)，
/// 不提供可变的元素赋值方法，因此不可能构造出不对称的张量。
///
/// # 符号约定
///
/// 内部采用**张量惯例** `J_ij = -∫ xᵢxⱼ dm`（i≠j 时），这是力学教材与
/// 多数仿真软件的做法。部分 CAD 与图纸标注采用工程手册惯例
/// `Ixy = +∫xy dm`，两者**非对角元符号相反**。从外部数据导入时
/// 必须确认来源用哪一套，否则陀螺力矩方向会反。
///
/// # 示例
/// ```
/// use zmatrix::compound::inertia_tensor::InertiaTensor;
/// use zmatrix::physics::basic::{AngularVelocity, Inertia, Vector3};
///
/// // 各向同性：球体近似
/// let j = InertiaTensor::from_isotropic(Inertia::from_kg_m2(2.0));
///
/// // 绕 z 轴转，因惯性积为零，L 与 ω 同向
/// let omega = Vector3::new(
///     AngularVelocity::from_rad_per_second(0.0),
///     AngularVelocity::from_rad_per_second(0.0),
///     AngularVelocity::from_rad_per_second(3.0),
/// );
/// let l = j * omega;
/// assert_eq!(l.z.as_nms(), 6.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InertiaTensor {
    /// 完整 3×3，对称。存 9 个而非 6 个：`Inertia` 是 16 字节，
    /// 省下的 48 字节不值得牺牲行列访问的可读性。
    data: [[Inertia; 3]; 3],
}

impl Default for InertiaTensor {
    fn default() -> Self {
        Self::from_isotropic(Inertia::default())
    }
}

impl InertiaTensor {
    /// 由 6 个独立分量构造。
    ///
    /// 参数顺序为 `Ixx, Iyy, Izz, Ixy, Ixz, Iyz`，前三个是对角元，
    /// 后三个是惯性积。非对角元在内部对称填充。
    pub fn new(
        i_xx: Inertia,
        i_yy: Inertia,
        i_zz: Inertia,
        i_xy: Inertia,
        i_xz: Inertia,
        i_yz: Inertia,
    ) -> Self {
        Self {
            data: [
                [i_xx, i_xy, i_xz],
                [i_xy, i_yy, i_yz],
                [i_xz, i_yz, i_zz],
            ],
        }
    }

    /// 各向同性张量：`J = j·I`。
    ///
    /// 球体、立方体（沿体心轴）以及"只知道一个惯量值"的粗略模型都用它。
    /// 此时 `L` 恒与 `ω` 同向，陀螺力矩为零。
    pub fn from_isotropic(j: Inertia) -> Self {
        Self::from_diagonal([j, j, j])
    }

    /// 由三个主转动惯量构造（主轴坐标系下的对角阵）。
    ///
    /// 惯性积全为零，`L` 与 `ω` 始终同向。这是"已经对角化"的张量，
    /// 姿态动力学里常作为初始状态。
    pub fn from_diagonal(diag: [Inertia; 3]) -> Self {
        Self {
            data: [
                [diag[0], Inertia::default(), Inertia::default()],
                [Inertia::default(), diag[1], Inertia::default()],
                [Inertia::default(), Inertia::default(), diag[2]],
            ],
        }
    }

    /// 取三个对角元 `[Ixx, Iyy, Izz]`。
    ///
    /// **注意**：这些值只代表绕**坐标轴**的转动惯量，不是绕任意轴的。
    /// 绕任意方向 `n` 的值请用 [`about_axis`](Self::about_axis)。
    pub fn diag(&self) -> [Inertia; 3] {
        [self.data[0][0], self.data[1][1], self.data[2][2]]
    }

    /// 取三个惯性积 `[Ixy, Ixz, Iyz]`。
    pub fn products(&self) -> [Inertia; 3] {
        [self.data[0][1], self.data[0][2], self.data[1][2]]
    }

    /// 取第 `(row, col)` 个元素。越界返回 `None`。
    pub fn get(&self, row: usize, col: usize) -> Option<Inertia> {
        if row >= 3 || col >= 3 {
            return None;
        }
        Some(self.data[row][col])
    }

    /// 绕任意方向的等效转动惯量：`nᵀ J n / nᵀ n`。
    ///
    /// 这是张量沿方向 `n` 所张直线的投影，也就是刚体绕该轴旋转时的转动惯量。
    ///
    /// # 内部归一化
    ///
    /// `n` **不需要**是单位向量 —— 方法内部除以 `nᵀn`，因此结果与 `n` 的模长无关。
    /// 直接算 `nᵀJn`（不除）会让结果随 `|n|²` 缩放：
    ///
    /// ```text
    /// n = (0,0,1)  → 3     正确
    /// n = (0,0,2)  → 12    错，是 3 的 4 倍（|n|² = 4）
    /// n = (0,0,10) → 300   错，是 3 的 100 倍
    /// ```
    ///
    /// 这个错误不会报错、不会 panic，只会给出一个看起来正常的数，
    /// 所以归一化放在方法内而不是交给调用方。
    ///
    /// 零向量没有对应方向，返回
    /// [`OperationError`](crate::dense::error::OperationError)。
    ///
    /// # 为什么不能用对角元代替
    ///
    /// 对角元只对坐标轴成立。以张量
    /// `[[2,-1,0],[-1,2,0],[0,0,3]]` 为例，绕 z 轴是 3，
    /// 但绕 `(1,1,0)/√2` 只有 1 —— 惯性积把结果从 2 拉到了 1。
    ///
    /// ```
    /// use zmatrix::compound::inertia_tensor::InertiaTensor;
    /// use zmatrix::physics::basic::{Coef, Inertia, Vector3};
    ///
    /// let j = InertiaTensor::new(
    ///     Inertia::from_kg_m2(2.0), Inertia::from_kg_m2(2.0), Inertia::from_kg_m2(3.0),
    ///     Inertia::from_kg_m2(-1.0), Inertia::default(), Inertia::default(),
    /// );
    /// let s = std::f64::consts::FRAC_1_SQRT_2;
    /// let n = Vector3::new(Coef::new(s), Coef::new(s), Coef::new(0.0));
    /// assert!((j.about_axis(n).unwrap().as_kg_m2() - 1.0).abs() < 1e-9);
    ///
    /// // 非单位向量给出相同结果
    /// let n2 = Vector3::new(Coef::new(1.0), Coef::new(1.0), Coef::new(0.0));
    /// assert!((j.about_axis(n2).unwrap().as_kg_m2() - 1.0).abs() < 1e-9);
    /// ```
    pub fn about_axis(
        &self,
        n: Vector3<crate::physics::basic::Coef>,
    ) -> Result<Inertia, crate::dense::error::OperationError> {
        let a = [n.x.get_value(), n.y.get_value(), n.z.get_value()];

        let norm_sq = a[0] * a[0] + a[1] * a[1] + a[2] * a[2];
        if norm_sq == 0.0 {
            return Err(crate::dense::error::OperationError::new(
                "about_axis: axis vector is zero, no direction to project onto",
            ));
        }

        let mut sum = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                sum += a[i] * self.data[i][j].as_kg_m2() * a[j];
            }
        }
        // 除以 nᵀn：使结果与 |n| 无关
        Ok(Inertia::from_kg_m2(sum / norm_sq))
    }

    /// 迹的三分之一：各向同性等效转动惯量。
    ///
    /// 用于快速估算、以及检查张量是否接近各向同性
    /// （三个主惯量相差越小，各向同性近似越可靠）。
    pub fn isotropic_equivalent(&self) -> Inertia {
        let trace = self.data[0][0].as_kg_m2()
            + self.data[1][1].as_kg_m2()
            + self.data[2][2].as_kg_m2();
        Inertia::from_kg_m2(trace / 3.0)
    }

    /// 坐标变换：`J' = R J Rᵀ`。
    ///
    /// `r` 是从旧坐标系到新坐标系的方向余弦矩阵。
    /// 张量在坐标变换下保持对称性 —— 这是本方法可以安全返回
    /// [`InertiaTensor`] 而非原始 `[[Inertia; 3]; 3]` 的原因。
    ///
    /// # 实现说明
    ///
    /// 分两步算 `RJ` 再乘 `Rᵀ`，中间结果用 `f64` 累加，
    /// 只在最后转回 `Inertia`。这样避免中间量反复做量纲包装。
    pub fn rotate(&self, r: &CosMatrix) -> Self {
        let m = r.to_array();

        // 第一步：rj = R · J
        let mut rj = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += m[i][k] * self.data[k][j].as_kg_m2();
                }
                rj[i][j] = sum;
            }
        }

        // 第二步：result = rj · Rᵀ
        let mut result = [[Inertia::default(); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += rj[i][k] * m[j][k];
                }
                result[i][j] = Inertia::from_kg_m2(sum);
            }
        }

        Self { data: result }
    }
}

// 张量 × 角速度 = 角动量：J·ω = L
//
// 与标量 Inertia 的 `J·ω` 不同，这里可能让 L 偏离 ω 的方向。
impl std::ops::Mul<Vector3<AngularVelocity>> for InertiaTensor {
    type Output = Vector3<AngularMomentum>;

    fn mul(self, rhs: Vector3<AngularVelocity>) -> Self::Output {
        let w = [
            rhs.x.as_rad_per_second(),
            rhs.y.as_rad_per_second(),
            rhs.z.as_rad_per_second(),
        ];
        let mut out = [0.0_f64; 3];
        for i in 0..3 {
            for j in 0..3 {
                out[i] += self.data[i][j].as_kg_m2() * w[j];
            }
        }
        Vector3::new(
            AngularMomentum::from_nms(out[0]),
            AngularMomentum::from_nms(out[1]),
            AngularMomentum::from_nms(out[2]),
        )
    }
}

impl std::ops::Mul<Vector3<AngularVelocity>> for &InertiaTensor {
    type Output = Vector3<AngularMomentum>;

    fn mul(self, rhs: Vector3<AngularVelocity>) -> Self::Output {
        (*self) * rhs
    }
}

impl<'a> std::ops::Mul<&'a Vector3<AngularVelocity>> for InertiaTensor {
    type Output = Vector3<AngularMomentum>;

    fn mul(self, rhs: &'a Vector3<AngularVelocity>) -> Self::Output {
        self * *rhs
    }
}

impl<'a, 'b> std::ops::Mul<&'b Vector3<AngularVelocity>> for &'a InertiaTensor {
    type Output = Vector3<AngularMomentum>;

    fn mul(self, rhs: &'b Vector3<AngularVelocity>) -> Self::Output {
        (*self) * *rhs
    }
}

// 张量 × 角加速度 = 力矩：J·ω̇ = M
impl std::ops::Mul<Vector3<AngularAcceleration>> for InertiaTensor {
    type Output = Vector3<Torque>;

    fn mul(self, rhs: Vector3<AngularAcceleration>) -> Self::Output {
        let a = [
            rhs.x.as_rad_per_second2(),
            rhs.y.as_rad_per_second2(),
            rhs.z.as_rad_per_second2(),
        ];
        let mut out = [0.0_f64; 3];
        for i in 0..3 {
            for j in 0..3 {
                out[i] += self.data[i][j].as_kg_m2() * a[j];
            }
        }
        Vector3::new(
            Torque::from_nm(out[0]),
            Torque::from_nm(out[1]),
            Torque::from_nm(out[2]),
        )
    }
}

impl std::ops::Mul<Vector3<AngularAcceleration>> for &InertiaTensor {
    type Output = Vector3<Torque>;

    fn mul(self, rhs: Vector3<AngularAcceleration>) -> Self::Output {
        (*self) * rhs
    }
}

impl<'a> std::ops::Mul<&'a Vector3<AngularAcceleration>> for InertiaTensor {
    type Output = Vector3<Torque>;

    fn mul(self, rhs: &'a Vector3<AngularAcceleration>) -> Self::Output {
        self * *rhs
    }
}

impl<'a, 'b> std::ops::Mul<&'b Vector3<AngularAcceleration>> for &'a InertiaTensor {
    type Output = Vector3<Torque>;

    fn mul(self, rhs: &'b Vector3<AngularAcceleration>) -> Self::Output {
        (*self) * *rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn unit_inertia_tensor() -> InertiaTensor {
        InertiaTensor::new(
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(3.0),
            Inertia::from_kg_m2(4.0),
            Inertia::from_kg_m2(-0.5),
            Inertia::from_kg_m2(0.25),
            Inertia::from_kg_m2(-0.75),
        )
    }

    #[test]
    fn test_new_symmetric() {
        let j = unit_inertia_tensor();

        // 对角元
        assert_relative_eq!(j.get(0, 0).unwrap().as_kg_m2(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(j.get(1, 1).unwrap().as_kg_m2(), 3.0, epsilon = 1e-12);
        assert_relative_eq!(j.get(2, 2).unwrap().as_kg_m2(), 4.0, epsilon = 1e-12);

        // 非对角元对称填充
        assert_eq!(j.get(0, 1), j.get(1, 0));
        assert_eq!(j.get(0, 2), j.get(2, 0));
        assert_eq!(j.get(1, 2), j.get(2, 1));

        assert_relative_eq!(j.get(0, 1).unwrap().as_kg_m2(), -0.5, epsilon = 1e-12);
        assert_relative_eq!(j.get(0, 2).unwrap().as_kg_m2(), 0.25, epsilon = 1e-12);
        assert_relative_eq!(j.get(1, 2).unwrap().as_kg_m2(), -0.75, epsilon = 1e-12);

        // 越界
        assert!(j.get(3, 0).is_none());
        assert!(j.get(0, 3).is_none());

        // diag 与 products
        assert_relative_eq!(j.diag()[0].as_kg_m2(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(j.products()[0].as_kg_m2(), -0.5, epsilon = 1e-12);
        assert_relative_eq!(j.products()[2].as_kg_m2(), -0.75, epsilon = 1e-12);
    }

    #[test]
    fn test_isotropic() {
        let j = InertiaTensor::from_isotropic(Inertia::from_kg_m2(5.0));
        assert_relative_eq!(j.diag()[0].as_kg_m2(), 5.0, epsilon = 1e-12);
        assert_relative_eq!(j.diag()[2].as_kg_m2(), 5.0, epsilon = 1e-12);
        // 非对角全零
        for p in j.products() {
            assert_relative_eq!(p.as_kg_m2(), 0.0, epsilon = 1e-12);
        }
        assert_relative_eq!(j.isotropic_equivalent().as_kg_m2(), 5.0, epsilon = 1e-12);

        // 默认值
        assert_relative_eq!(InertiaTensor::default().isotropic_equivalent().as_kg_m2(), 0.0);
    }

    #[test]
    fn test_from_diagonal() {
        let j = InertiaTensor::from_diagonal([
            Inertia::from_kg_m2(1.0),
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(3.0),
        ]);
        assert_relative_eq!(j.get(0, 0).unwrap().as_kg_m2(), 1.0, epsilon = 1e-12);
        assert_relative_eq!(j.get(1, 1).unwrap().as_kg_m2(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(j.get(2, 2).unwrap().as_kg_m2(), 3.0, epsilon = 1e-12);
        assert_relative_eq!(j.get(0, 1).unwrap().as_kg_m2(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(j.isotropic_equivalent().as_kg_m2(), 2.0, epsilon = 1e-12);
    }

    #[test]
    fn test_mul_omega_isotropic_keeps_direction() {
        // 各向同性：L 与 ω 同向
        let j = InertiaTensor::from_isotropic(Inertia::from_kg_m2(2.0));
        let omega = Vector3::new(
            AngularVelocity::from_rad_per_second(1.0),
            AngularVelocity::from_rad_per_second(2.0),
            AngularVelocity::from_rad_per_second(3.0),
        );
        let l = j * omega;
        assert_relative_eq!(l.x.as_nms(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(l.y.as_nms(), 4.0, epsilon = 1e-12);
        assert_relative_eq!(l.z.as_nms(), 6.0, epsilon = 1e-12);
    }

    #[test]
    fn test_mul_omega_anisotropic_deviates() {
        // 非对称：ω 纯沿 z 转，L 却出现 x、y 分量
        // 这正是陀螺力矩的来源
        let j = InertiaTensor::new(
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(3.0),
            Inertia::from_kg_m2(4.0),
            Inertia::from_kg_m2(0.0),
            Inertia::from_kg_m2(1.0),  // Ixz
            Inertia::from_kg_m2(0.0),
        );
        let omega = Vector3::new(
            AngularVelocity::from_rad_per_second(0.0),
            AngularVelocity::from_rad_per_second(0.0),
            AngularVelocity::from_rad_per_second(2.0),
        );
        let l = j * omega;

        // Ixz · ω_z = 1.0 × 2 = 2，出现在 x 分量
        assert_relative_eq!(l.x.as_nms(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(l.y.as_nms(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(l.z.as_nms(), 8.0, epsilon = 1e-12);

        // L 与 ω 不共线：叉乘非零
        let cross_z = l.x.as_nms() * omega.y.as_rad_per_second()
            - l.y.as_nms() * omega.x.as_rad_per_second();
        assert_relative_eq!(cross_z, 0.0, epsilon = 1e-12);
        // 但 x-y 平面上确实偏离了
        assert!(l.x.as_nms().abs() > 1e-9);
    }

    #[test]
    fn test_mul_alpha_gives_torque() {
        let j = InertiaTensor::from_diagonal([
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(4.0),
            Inertia::from_kg_m2(6.0),
        ]);
        let alpha = Vector3::new(
            AngularAcceleration::from_rad_per_second2(1.0),
            AngularAcceleration::from_rad_per_second2(2.0),
            AngularAcceleration::from_rad_per_second2(3.0),
        );
        let m = j * alpha;
        assert_relative_eq!(m.x.as_nm(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(m.y.as_nm(), 8.0, epsilon = 1e-12);
        assert_relative_eq!(m.z.as_nm(), 18.0, epsilon = 1e-12);
    }

    #[test]
    fn test_ref_variants() {
        let j = InertiaTensor::from_isotropic(Inertia::from_kg_m2(2.0));
        let omega = Vector3::new(
            AngularVelocity::from_rad_per_second(1.0),
            AngularVelocity::from_rad_per_second(1.0),
            AngularVelocity::from_rad_per_second(1.0),
        );
        assert_eq!((j * omega).x.as_nms(), 2.0);
        assert_eq!((&j * omega).x.as_nms(), 2.0);
        assert_eq!((j * &omega).x.as_nms(), 2.0);
        assert_eq!((&j * &omega).x.as_nms(), 2.0);

        let alpha = Vector3::new(
            AngularAcceleration::from_rad_per_second2(1.0),
            AngularAcceleration::from_rad_per_second2(1.0),
            AngularAcceleration::from_rad_per_second2(1.0),
        );
        assert_eq!((j * alpha).x.as_nm(), 2.0);
        assert_eq!((&j * alpha).x.as_nm(), 2.0);
        assert_eq!((j * &alpha).x.as_nm(), 2.0);
        assert_eq!((&j * &alpha).x.as_nm(), 2.0);
    }

    #[test]
    fn test_about_axis() {
        let j = unit_inertia_tensor();

        // 沿 x 轴：应等于 Ixx
        let ex = Vector3::new(
            crate::physics::basic::Coef::new(1.0),
            crate::physics::basic::Coef::new(0.0),
            crate::physics::basic::Coef::new(0.0),
        );
        assert_relative_eq!(j.about_axis(ex).unwrap().as_kg_m2(), 2.0, epsilon = 1e-12);

        // 沿 y、z 轴
        let ey = Vector3::new(
            crate::physics::basic::Coef::new(0.0),
            crate::physics::basic::Coef::new(1.0),
            crate::physics::basic::Coef::new(0.0),
        );
        assert_relative_eq!(j.about_axis(ey).unwrap().as_kg_m2(), 3.0, epsilon = 1e-12);

        let ez = Vector3::new(
            crate::physics::basic::Coef::new(0.0),
            crate::physics::basic::Coef::new(0.0),
            crate::physics::basic::Coef::new(1.0),
        );
        assert_relative_eq!(j.about_axis(ez).unwrap().as_kg_m2(), 4.0, epsilon = 1e-12);

        // 45° 斜轴：数值必须落在对角元之间，且受惯性积影响
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let n45 = Vector3::new(
            crate::physics::basic::Coef::new(s),
            crate::physics::basic::Coef::new(s),
            crate::physics::basic::Coef::new(0.0),
        );
        // = 0.5*(2 + 2*(-0.5) + 3) = 0.5*4 = 2.0
        assert_relative_eq!(j.about_axis(n45).unwrap().as_kg_m2(), 2.0, epsilon = 1e-12);
    }

    #[test]
    fn test_about_axis_independent_of_magnitude() {
        // 同一个方向，模长不同 → 结果必须相同（内部除以 nᵀn）
        let j = unit_inertia_tensor();
        let coef = crate::physics::basic::Coef::new;

        for k in [1.0, 2.0, 10.0, 1e-3] {
            let n = Vector3::new(coef(0.6 * k), coef(0.8 * k), coef(0.0));
            let v = j.about_axis(n).unwrap().as_kg_m2();
            // 方向 (0.6, 0.8, 0) 固定，结果应与 k 无关
            let n_unit = Vector3::new(coef(0.6), coef(0.8), coef(0.0));
            let v_unit = j.about_axis(n_unit).unwrap().as_kg_m2();
            assert_relative_eq!(v, v_unit, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_about_axis_zero_vector_errors() {
        let j = unit_inertia_tensor();
        let zero = Vector3::new(
            crate::physics::basic::Coef::new(0.0),
            crate::physics::basic::Coef::new(0.0),
            crate::physics::basic::Coef::new(0.0),
        );
        let r = j.about_axis(zero);
        assert!(r.is_err());
        assert!(r.unwrap_err().message.contains("zero"));
    }

    #[test]
    fn test_about_axis_differs_from_diagonal() {
        // 张量 [[2,-1,0],[-1,2,0],[0,0,3]]
        let j = InertiaTensor::new(
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(3.0),
            Inertia::from_kg_m2(-1.0),
            Inertia::default(),
            Inertia::default(),
        );
        let coef = crate::physics::basic::Coef::new;

        // 绕 z 轴：读到 Izz = 3
        let ez = Vector3::new(coef(0.0), coef(0.0), coef(1.0));
        assert_relative_eq!(j.about_axis(ez).unwrap().as_kg_m2(), 3.0, epsilon = 1e-12);

        // 绕 45° 斜轴：只有 1，惯性积把它从 2 拉到 1
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let n45 = Vector3::new(coef(s), coef(s), coef(0.0));
        assert_relative_eq!(j.about_axis(n45).unwrap().as_kg_m2(), 1.0, epsilon = 1e-12);

        // 两者不同，证明不能用对角元代替任意轴
        assert!(
            (j.about_axis(ez).unwrap().as_kg_m2() - j.about_axis(n45).unwrap().as_kg_m2()).abs()
                > 1.0
        );
    }

    #[test]
    fn test_about_axis_matches_diagonal_when_isotropic() {
        // 各向同性时，任意方向的惯量都相同
        let j = InertiaTensor::from_isotropic(Inertia::from_kg_m2(7.0));
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let axes = [
            Vector3::new(
                crate::physics::basic::Coef::new(1.0),
                crate::physics::basic::Coef::new(0.0),
                crate::physics::basic::Coef::new(0.0),
            ),
            Vector3::new(
                crate::physics::basic::Coef::new(s),
                crate::physics::basic::Coef::new(s),
                crate::physics::basic::Coef::new(0.0),
            ),
            Vector3::new(
                crate::physics::basic::Coef::new(1.0 / 3.0_f64.sqrt()),
                crate::physics::basic::Coef::new(1.0 / 3.0_f64.sqrt()),
                crate::physics::basic::Coef::new(1.0 / 3.0_f64.sqrt()),
            ),
        ];
        for n in axes {
            assert_relative_eq!(j.about_axis(n).unwrap().as_kg_m2(), 7.0, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_rotate_identity() {
        let j = unit_inertia_tensor();
        let r = CosMatrix::unit();
        let j2 = j.rotate(&r);
        for i in 0..3 {
            for k in 0..3 {
                assert_relative_eq!(
                    j2.get(i, k).unwrap().as_kg_m2(),
                    j.get(i, k).unwrap().as_kg_m2(),
                    epsilon = 1e-12
                );
            }
        }
    }

    #[test]
    fn test_rotate_preserves_trace_and_symmetry() {
        // 90° 绕 z 轴：交换 x、y 轴，张量应随之交换
        let j = InertiaTensor::from_diagonal([
            Inertia::from_kg_m2(1.0),
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(3.0),
        ]);
        let r = CosMatrix::new([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
        let j2 = j.rotate(&r);

        // x、y 主惯量互换
        assert_relative_eq!(j2.get(0, 0).unwrap().as_kg_m2(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(j2.get(1, 1).unwrap().as_kg_m2(), 1.0, epsilon = 1e-12);
        assert_relative_eq!(j2.get(2, 2).unwrap().as_kg_m2(), 3.0, epsilon = 1e-12);

        // 迹不变
        assert_relative_eq!(
            j2.isotropic_equivalent().as_kg_m2(),
            j.isotropic_equivalent().as_kg_m2(),
            epsilon = 1e-12
        );

        // 对称性保持
        assert_eq!(j2.get(0, 1), j2.get(1, 0));
        assert_eq!(j2.get(0, 2), j2.get(2, 0));
        assert_eq!(j2.get(1, 2), j2.get(2, 1));
    }

    #[test]
    fn test_rotate_double_is_identity_asymmetry() {
        // 转 90° 再转回来，应回到原张量（含惯性积）
        let j = unit_inertia_tensor();
        let r = CosMatrix::new([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
        let rt = r.transfer();
        let back = j.rotate(&r).rotate(&rt);

        for i in 0..3 {
            for k in 0..3 {
                assert_relative_eq!(
                    back.get(i, k).unwrap().as_kg_m2(),
                    j.get(i, k).unwrap().as_kg_m2(),
                    epsilon = 1e-9
                );
            }
        }
    }
}
