//! 转动惯量张量（3×3）。

use crate::physics::basic::{
    AngularAcceleration, AngularMomentum, AngularVelocity, Distance, Inertia, Mass, Torque,
    Vector3,
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

    /// 由角动量解角速度：`ω = J⁻¹ · L`。
    ///
    /// 姿态动力学每拍都要算这个。`J` 对称正定时有唯一解。
    ///
    /// # 为什么不能逐分量做标量除法
    ///
    /// 惯性积不为零时，`J⁻¹L` 的每个分量都依赖 `L` 的全部分量。
    /// 逐分量写 `(Lx/Jxx, Ly/Jyy, Lz/Jzz)` 不满足 `J·ω = L`：
    ///
    /// ```text
    /// J = [[120, 8, 0], [8, 130, 0], [0, 0, 45]],  L = (60, 0, 90)
    ///
    /// 逐分量   ω = (0.500000,  0.000000, 2.0)   反代 J·ω = (60, 4.0, 90)  ← 多出 4
    /// 正确解   ω = (0.502060, -0.030896, 2.0)   反代 J·ω = (60, 0.0, 90)  ✓
    /// ```
    ///
    /// 每拍凭空注入 4 N·m·s 的角动量，90 分钟轨道周期内会累积成可观测的姿态漂移。
    ///
    /// # 算法：Cholesky
    ///
    /// 惯量张量对称正定是**物理固有性质**，不是可选假设，所以用 Cholesky
    /// 而不是通用 LU：运算量减半、无需选主元。分解过程中对角元 `L_ii² ≤ 0`
    /// 就等价于非正定，**正定校验是免费的副产品**。
    ///
    /// 非正定意味着输入的张量在物理上不可能存在（负质量分布、惯量积超限），
    /// 返回 [`OperationError`](crate::dense::error::OperationError) 而不是
    /// 静默返回零向量。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::compound::inertia_tensor::InertiaTensor;
    /// use zmatrix::physics::basic::{AngularMomentum, Inertia, Vector3};
    ///
    /// let j = InertiaTensor::new(
    ///     Inertia::from_kg_m2(120.0), Inertia::from_kg_m2(130.0), Inertia::from_kg_m2(45.0),
    ///     Inertia::from_kg_m2(8.0), Inertia::default(), Inertia::default(),
    /// );
    /// let l = Vector3::new(
    ///     AngularMomentum::from_nms(60.0),
    ///     AngularMomentum::from_nms(0.0),
    ///     AngularMomentum::from_nms(90.0),
    /// );
    /// let w = j.solve_omega(l).unwrap();
    ///
    /// // 反代验证 J·ω 应还原 L
    /// let back = j * w;
    /// assert!((back.x.as_nms() - 60.0).abs() < 1e-9);
    /// assert!(back.y.as_nms().abs() < 1e-9);
    /// assert!((back.z.as_nms() - 90.0).abs() < 1e-9);
    /// ```
    pub fn solve_omega(
        &self,
        l: Vector3<AngularMomentum>,
    ) -> Result<Vector3<AngularVelocity>, crate::dense::error::OperationError> {
        let b = [
            l.x.as_nms(),
            l.y.as_nms(),
            l.z.as_nms(),
        ];
        let s = self.cholesky_solve(b)?;
        Ok(Vector3::new(
            AngularVelocity::from_rad_per_second(s[0]),
            AngularVelocity::from_rad_per_second(s[1]),
            AngularVelocity::from_rad_per_second(s[2]),
        ))
    }

    /// 由力矩解角加速度：`ω̇ = J⁻¹ · τ`。
    ///
    /// 与 [`solve_omega`](Self::solve_omega) 同一套 Cholesky 分解，
    /// 只是右端项换成力矩、输出换成角加速度。量纲核对：
    /// `(kg·m²)⁻¹ × N·m = 1/s²` ✓
    ///
    /// 控制律设计与激励响应分析用。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::compound::inertia_tensor::InertiaTensor;
    /// use zmatrix::physics::basic::{Inertia, Torque, Vector3};
    ///
    /// let j = InertiaTensor::from_diagonal([
    ///     Inertia::from_kg_m2(2.0),
    ///     Inertia::from_kg_m2(4.0),
    ///     Inertia::from_kg_m2(8.0),
    /// ]);
    /// let tau = Vector3::new(
    ///     Torque::from_nm(6.0),
    ///     Torque::from_nm(8.0),
    ///     Torque::from_nm(16.0),
    /// );
    /// let alpha = j.solve_alpha(tau).unwrap();
    /// assert!((alpha.x.as_rad_per_second2() - 3.0).abs() < 1e-12);
    /// assert!((alpha.y.as_rad_per_second2() - 2.0).abs() < 1e-12);
    /// assert!((alpha.z.as_rad_per_second2() - 2.0).abs() < 1e-12);
    /// ```
    pub fn solve_alpha(
        &self,
        tau: Vector3<Torque>,
    ) -> Result<Vector3<AngularAcceleration>, crate::dense::error::OperationError> {
        let b = [
            tau.x.as_nm(),
            tau.y.as_nm(),
            tau.z.as_nm(),
        ];
        let s = self.cholesky_solve(b)?;
        Ok(Vector3::new(
            AngularAcceleration::from_rad_per_second2(s[0]),
            AngularAcceleration::from_rad_per_second2(s[1]),
            AngularAcceleration::from_rad_per_second2(s[2]),
        ))
    }

    /// Cholesky 分解求解 `J · x = b`（`J` 为 3×3 对称正定）。
    ///
    /// 分解 `J = L·Lᵀ`（`L` 下三角），再前代 + 回代。
    /// 只用 `f64` 运算，量纲由调用方在入口/出口处包装。
    fn cholesky_solve(
        &self,
        b: [f64; 3],
    ) -> Result<[f64; 3], crate::dense::error::OperationError> {
        let a = [
            [
                self.data[0][0].as_kg_m2(),
                self.data[0][1].as_kg_m2(),
                self.data[0][2].as_kg_m2(),
            ],
            [
                self.data[1][0].as_kg_m2(),
                self.data[1][1].as_kg_m2(),
                self.data[1][2].as_kg_m2(),
            ],
            [
                self.data[2][0].as_kg_m2(),
                self.data[2][1].as_kg_m2(),
                self.data[2][2].as_kg_m2(),
            ],
        ];

        // --- 分解：J = L·Lᵀ ---
        let mut lo = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for j in 0..=i {
                let mut sum = a[i][j];
                for k in 0..j {
                    sum -= lo[i][k] * lo[j][k];
                }
                if i == j {
                    // 对角元必须为正，否则非正定
                    if !(sum > 0.0) {
                        return Err(crate::dense::error::OperationError::new(
                            "cholesky_solve: inertia tensor is not positive definite \
                             (this cannot describe a real mass distribution)",
                        ));
                    }
                    lo[i][j] = sum.sqrt();
                } else {
                    lo[i][j] = sum / lo[j][j];
                }
            }
        }

        // --- 前代：L·y = b ---
        let mut y = [0.0_f64; 3];
        for i in 0..3 {
            let mut sum = b[i];
            for k in 0..i {
                sum -= lo[i][k] * y[k];
            }
            y[i] = sum / lo[i][i];
        }

        // --- 回代：Lᵀ·x = y ---
        let mut x = [0.0_f64; 3];
        for i in (0..3).rev() {
            let mut sum = y[i];
            for k in (i + 1)..3 {
                sum -= lo[k][i] * x[k];
            }
            x[i] = sum / lo[i][i];
        }

        Ok(x)
    }

    /// 主轴对角化：求三个主惯量与主轴方向。
    ///
    /// 返回 `(主惯量 [J1, J2, J3], 主轴方向)`，其中特征值按**升序**排列。
    ///
    /// # 主轴方向矩阵的方向约定（重要）
    ///
    /// 返回的 `CosMatrix` 的**每一列**是一个主轴在本体坐标系中的方向向量，
    /// 即它表示**本体坐标系 → 主轴坐标系**的转换。满足：
    ///
    /// ```text
    /// diag(J1, J2, J3) = Rᵀ · J · R
    /// ```
    ///
    /// 换言之 `R` 的列是主轴方向，`R` 的行是主轴基在本体系下的分量。
    /// 若你要把某个本体系向量转到主轴系，用 `Rᵀ`（即 `R.transfer()`）；
    /// 从主轴系转回本体系，用 `R`。
    ///
    /// **这个方向搞反是姿态动力学里的经典错误**，而且不报错、只是结果转置了。
    ///
    /// # 算法：Jacobi 旋转
    ///
    /// 3×3 对称阵的特征值有解析公式（特征多项式求根），但**对近乎重根的情形数值不稳定**：
    /// 判别式接近零时开方会放大舍入误差。Jacobi 迭代对对称阵无条件收敛、
    /// 精度好、代码量也不大，所以选它。
    ///
    /// 迭代终止条件：非对角元平方和小于 `1e-30 × 对角元平方和`（相对判据）。
    ///
    /// # 退化情形
    ///
    /// 两个或三个主惯量相等时主轴**不唯一**（任何绕对称轴的正交基都成立）。
    /// 此时返回迭代得到的任意一组正交基，不报错——因为物理上确实有无穷多解，
    /// 报错反而是错的。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::compound::inertia_tensor::InertiaTensor;
    /// use zmatrix::physics::basic::Inertia;
    ///
    /// // 已是对角的张量，主轴就是坐标轴
    /// let j = InertiaTensor::from_diagonal([
    ///     Inertia::from_kg_m2(45.0),
    ///     Inertia::from_kg_m2(120.0),
    ///     Inertia::from_kg_m2(130.0),
    /// ]);
    /// let (principal, _) = j.diagonalize();
    /// assert!((principal[0].as_kg_m2() - 45.0).abs() < 1e-9);
    /// assert!((principal[1].as_kg_m2() - 120.0).abs() < 1e-9);
    /// assert!((principal[2].as_kg_m2() - 130.0).abs() < 1e-9);
    /// ```
    pub fn diagonalize(&self) -> ([Inertia; 3], CosMatrix) {
        // 对称化：数值上 J 应已对称，但 accumulate 的误差可能引入微小不对称
        let mut a = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                a[i][j] = 0.5
                    * (self.data[i][j].as_kg_m2() + self.data[j][i].as_kg_m2());
            }
        }

        // 累积旋转矩阵，初始为单位阵
        let mut v = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            v[i][i] = 1.0;
        }

        // Jacobi 迭代
        for _sweep in 0..64 {
            // 非对角元平方和
            let off = a[0][1] * a[0][1] + a[0][2] * a[0][2] + a[1][2] * a[1][2];
            let diag = a[0][0] * a[0][0] + a[1][1] * a[1][1] + a[2][2] * a[2][2];
            if off <= 1e-30 * diag || off == 0.0 {
                break;
            }

            // 逐对消除三个非对角元
            for &(p, q) in &[(0usize, 1usize), (0, 2), (1, 2)] {
                if a[p][q].abs() < 1e-300 {
                    continue;
                }

                // 计算旋转角：使 a[p][q] 归零
                let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                let t = if theta >= 0.0 {
                    1.0 / (theta + (1.0 + theta * theta).sqrt())
                } else {
                    -1.0 / (-theta + (1.0 + theta * theta).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;

                // 应用旋转到 a：A' = Gᵀ A G
                for k in 0..3 {
                    let akp = a[k][p];
                    let akq = a[k][q];
                    a[k][p] = c * akp - s * akq;
                    a[k][q] = s * akp + c * akq;
                }
                for k in 0..3 {
                    let apk = a[p][k];
                    let aqk = a[q][k];
                    a[p][k] = c * apk - s * aqk;
                    a[q][k] = s * apk + c * aqk;
                }
                // 强制对称（消除累积误差）
                a[p][q] = 0.0;
                a[q][p] = 0.0;

                // 累积到特征向量矩阵：V' = V G
                for k in 0..3 {
                    let vkp = v[k][p];
                    let vkq = v[k][q];
                    v[k][p] = c * vkp - s * vkq;
                    v[k][q] = s * vkp + c * vkq;
                }
            }
        }

        // 按特征值升序排序，同时重排特征向量列
        let mut idx = [0usize, 1, 2];
        idx.sort_by(|&i, &j| {
            a[i][i]
                .partial_cmp(&a[j][j])
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let principal = [
            Inertia::from_kg_m2(a[idx[0]][idx[0]]),
            Inertia::from_kg_m2(a[idx[1]][idx[1]]),
            Inertia::from_kg_m2(a[idx[2]][idx[2]]),
        ];

        // R 的第 j 列 = 第 idx[j] 个特征向量
        let mut r = [[0.0_f64; 3]; 3];
        for col in 0..3 {
            for row in 0..3 {
                r[row][col] = v[row][idx[col]];
            }
        }

        (principal, CosMatrix::new(r))
    }

    /// 平行轴定理：把质心系惯量搬到偏移 `d` 处。
    ///
    /// `J' = J_cm + m · (|d|² · I − d ⊗ d)`
    ///
    /// 展开成矩阵形式：
    ///
    /// ```text
    ///          ⎡ dy²+dz²    −dx·dy     −dx·dz ⎤
    /// J' = J + ⎢ −dx·dy   dx²+dz²     −dy·dz ⎥ · m
    ///          ⎣ −dx·dz    −dy·dz    dx²+dy² ⎦
    /// ```
    ///
    /// 注意非对角元的**负号**：这与张量定义 `J_ij = -∫xᵢxⱼ dm` 一致，
    /// 也与 [`new`](Self::new) 期望的符号约定一致（直接传惯性积的值，
    /// 内部再按张量定义使用）。
    ///
    /// # 方向
    ///
    /// `offset` 是**从质心指向新参考点**的位移。若你知道的是新点相对质心的位置，
    /// 直接传入；若要反推质心惯量，传入取负的位移（定理可逆）。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::compound::inertia_tensor::InertiaTensor;
    /// use zmatrix::physics::basic::{Distance, Inertia, Mass, Vector3};
    ///
    /// // 质点 m=2 kg，位于 x=3 m。绕原点 z 轴：J = m·d² = 18
    /// let j = InertiaTensor::from_isotropic(Inertia::default());
    /// let offset = Vector3::new(
    ///     Distance::from_m(3.0),
    ///     Distance::from_m(0.0),
    ///     Distance::from_m(0.0),
    /// );
    /// let shifted = j.parallel_axis(Mass::from_kg(2.0), offset);
    /// assert!((shifted.get(2, 2).unwrap().as_kg_m2() - 18.0).abs() < 1e-12);
    /// ```
    pub fn parallel_axis(&self, mass: Mass, offset: Vector3<Distance>) -> Self {
        let m = mass.as_kg();
        let d = [
            offset.x.as_m(),
            offset.y.as_m(),
            offset.z.as_m(),
        ];

        let d_sq = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];

        let mut out = self.data;
        for i in 0..3 {
            for j in 0..3 {
                // δ_ij·|d|² − d_i·d_j
                let delta = if i == j { d_sq } else { 0.0 };
                let term = (delta - d[i] * d[j]) * m;
                out[i][j] = Inertia::from_kg_m2(out[i][j].as_kg_m2() + term);
            }
        }

        Self { data: out }
    }

    /// 由工程裸值数组构造：`[Ixx, Iyy, Izz, Ixy, Ixz, Iyz]`。
    ///
    /// # 为什么需要这个入口
    ///
    /// `new` 接受的是**惯性积的值**，内部按张量定义 `J_ij = -∫xᵢxⱼ dm` 使用。
    /// 而工程手册、CAD 导出、以及 aods 的 `mass_sim_para.J_kgmm2[9]` 用的是
    /// **裸值（+Ixy 工程约定）**，两者**非对角元反号**。
    ///
    /// 如果让每个调用点自己记「这里要取负」，一定会有人漏掉。把符号约定
    /// 收敛到这一个函数里，调用方只需要知道「我手上是工程裸值」。
    ///
    /// # 与 `new` 的区别
    ///
    /// ```text
    /// new(Ixy)                → 直接按张量约定使用
    /// from_engineering_array  → 内部对非对角元取负后再交给 new
    /// ```
    ///
    /// 数值上 `from_engineering_array([.., ixy, ..])` 等价于 `new(.., -ixy, ..)`。
    ///
    /// # 单位
    /// 统一按 `kg·m²` 解释。工程图纸常给 `kg·mm²`，请先自行乘 `1e-6`，
    /// 或用分量逐个 [`Inertia::from_kg_mm2`](crate::physics::basic::Inertia::from_kg_mm2)。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::compound::inertia_tensor::InertiaTensor;
    /// use zmatrix::physics::basic::Inertia;
    ///
    /// // 工程裸值：Ixy = +8（表示质量在 xy 平面里"正相关"分布）
    /// let j = InertiaTensor::from_engineering_array([120.0, 130.0, 45.0, 8.0, 0.0, 0.0]);
    ///
    /// // 与手写取负的 new 等价
    /// let j2 = InertiaTensor::new(
    ///     Inertia::from_kg_m2(120.0), Inertia::from_kg_m2(130.0), Inertia::from_kg_m2(45.0),
    ///     Inertia::from_kg_m2(-8.0), Inertia::default(), Inertia::default(),
    /// );
    /// assert!((j.get(0, 1).unwrap().as_kg_m2() - j2.get(0, 1).unwrap().as_kg_m2()).abs() < 1e-12);
    /// assert!((j.get(0, 1).unwrap().as_kg_m2() - (-8.0)).abs() < 1e-12);
    /// ```
    pub fn from_engineering_array(
        values: [f64; 6],
    ) -> Self {
        Self::new(
            Inertia::from_kg_m2(values[0]),
            Inertia::from_kg_m2(values[1]),
            Inertia::from_kg_m2(values[2]),
            // 工程裸值 → 张量约定：非对角元取负
            Inertia::from_kg_m2(-values[3]),
            Inertia::from_kg_m2(-values[4]),
            Inertia::from_kg_m2(-values[5]),
        )
    }

    /// 导出为工程裸值数组：`[Ixx, Iyy, Izz, Ixy, Ixz, Iyz]`（+Ixy 约定）。
    ///
    /// 与 [`from_engineering_array`](Self::from_engineering_array) 互逆，
    /// 用于 FFI 回写或与 CAD 对照。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::compound::inertia_tensor::InertiaTensor;
    ///
    /// let v = [120.0, 130.0, 45.0, 8.0, -3.0, 2.0];
    /// let j = InertiaTensor::from_engineering_array(v);
    /// let back = j.to_engineering_array();
    /// for i in 0..6 {
    ///     assert!((back[i] - v[i]).abs() < 1e-12);
    /// }
    /// ```
    pub fn to_engineering_array(&self) -> [f64; 6] {
        [
            self.data[0][0].as_kg_m2(),
            self.data[1][1].as_kg_m2(),
            self.data[2][2].as_kg_m2(),
            // 张量约定 → 工程裸值：非对角元取负
            -self.data[0][1].as_kg_m2(),
            -self.data[0][2].as_kg_m2(),
            -self.data[1][2].as_kg_m2(),
        ]
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
    use crate::physics::basic::Coef;

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

    // ===== solve_omega / solve_alpha =====

    /// 带惯性积的非对称张量：Ixx=120, Iyy=130, Izz=45, Ixy=8
    fn asymmetric_tensor() -> InertiaTensor {
        InertiaTensor::new(
            Inertia::from_kg_m2(120.0),
            Inertia::from_kg_m2(130.0),
            Inertia::from_kg_m2(45.0),
            Inertia::from_kg_m2(8.0),
            Inertia::default(),
            Inertia::default(),
        )
    }

    #[test]
    fn test_solve_omega_diagonal() {
        // 对角张量：解就是逐分量除法
        let j = InertiaTensor::from_diagonal([
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(4.0),
            Inertia::from_kg_m2(8.0),
        ]);
        let l = Vector3::new(
            AngularMomentum::from_nms(6.0),
            AngularMomentum::from_nms(8.0),
            AngularMomentum::from_nms(16.0),
        );
        let w = j.solve_omega(l).unwrap();
        assert_relative_eq!(w.x.as_rad_per_second(), 3.0, epsilon = 1e-12);
        assert_relative_eq!(w.y.as_rad_per_second(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(w.z.as_rad_per_second(), 2.0, epsilon = 1e-12);
    }

    #[test]
    fn test_solve_omega_round_trips_through_mul() {
        // J·(J⁻¹L) 必须精确还原 L —— 这是解算正确性的判据
        let j = asymmetric_tensor();
        let l = Vector3::new(
            AngularMomentum::from_nms(60.0),
            AngularMomentum::from_nms(0.0),
            AngularMomentum::from_nms(90.0),
        );

        let w = j.solve_omega(l).unwrap();
        let back = j * w;

        assert_relative_eq!(back.x.as_nms(), 60.0, epsilon = 1e-9);
        assert_relative_eq!(back.y.as_nms(), 0.0, epsilon = 1e-9);
        assert_relative_eq!(back.z.as_nms(), 90.0, epsilon = 1e-9);
    }

    #[test]
    fn test_solve_omega_differs_from_componentwise_division() {
        // 这是 solve 存在的理由：逐分量除法不满足 J·ω = L
        let j = asymmetric_tensor();
        let l = Vector3::new(
            AngularMomentum::from_nms(60.0),
            AngularMomentum::from_nms(0.0),
            AngularMomentum::from_nms(90.0),
        );

        // 逐分量除法（错误做法）
        let wrong = Vector3::new(
            AngularVelocity::from_rad_per_second(60.0 / 120.0),
            AngularVelocity::from_rad_per_second(0.0 / 130.0),
            AngularVelocity::from_rad_per_second(90.0 / 45.0),
        );
        // 反代：y 分量凭空多出 4 N·m·s
        let wrong_back = j * wrong;
        assert_relative_eq!(wrong_back.y.as_nms(), 4.0, epsilon = 1e-9);

        // 正确解反代精确还原
        let right = j.solve_omega(l).unwrap();
        let right_back = j * right;
        assert_relative_eq!(right_back.y.as_nms(), 0.0, epsilon = 1e-9);

        // 两个解确实不同
        assert!((right.x.as_rad_per_second() - wrong.x.as_rad_per_second()).abs() > 1e-3);
        assert!((right.y.as_rad_per_second() - wrong.y.as_rad_per_second()).abs() > 1e-3);
    }

    #[test]
    fn test_solve_alpha() {
        let j = InertiaTensor::from_diagonal([
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(4.0),
            Inertia::from_kg_m2(8.0),
        ]);
        let tau = Vector3::new(
            Torque::from_nm(6.0),
            Torque::from_nm(8.0),
            Torque::from_nm(16.0),
        );
        let a = j.solve_alpha(tau).unwrap();
        assert_relative_eq!(a.x.as_rad_per_second2(), 3.0, epsilon = 1e-12);
        assert_relative_eq!(a.y.as_rad_per_second2(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(a.z.as_rad_per_second2(), 2.0, epsilon = 1e-12);

        // 反代 J·ω̇ 还原 τ
        let back = j * a;
        assert_relative_eq!(back.x.as_nm(), 6.0, epsilon = 1e-9);
        assert_relative_eq!(back.y.as_nm(), 8.0, epsilon = 1e-9);
        assert_relative_eq!(back.z.as_nm(), 16.0, epsilon = 1e-9);
    }

    #[test]
    fn test_solve_rejects_non_positive_definite() {
        // 对角元为负：物理上不存在的张量（负质量分布）
        let bad = InertiaTensor::from_diagonal([
            Inertia::from_kg_m2(-1.0),
            Inertia::from_kg_m2(1.0),
            Inertia::from_kg_m2(1.0),
        ]);
        assert!(bad
            .solve_omega(Vector3::new(
                AngularMomentum::from_nms(1.0),
                AngularMomentum::default(),
                AngularMomentum::default(),
            ))
            .is_err());

        // 惯性积超限导致非正定：[[1, 2, 0],[2, 1, 0],[0,0,1]]
        // 2x2 块的行列式 = 1-4 = -3 < 0
        let bad2 = InertiaTensor::new(
            Inertia::from_kg_m2(1.0),
            Inertia::from_kg_m2(1.0),
            Inertia::from_kg_m2(1.0),
            Inertia::from_kg_m2(2.0),
            Inertia::default(),
            Inertia::default(),
        );
        assert!(bad2
            .solve_alpha(Vector3::new(
                Torque::from_nm(1.0),
                Torque::default(),
                Torque::default(),
            ))
            .is_err());
    }

    #[test]
    fn test_solve_singular_is_error() {
        // 零张量：奇异
        let zero = InertiaTensor::from_isotropic(Inertia::default());
        assert!(zero
            .solve_omega(Vector3::new(
                AngularMomentum::from_nms(1.0),
                AngularMomentum::default(),
                AngularMomentum::default(),
            ))
            .is_err());
    }

    #[test]
    fn test_solve_matches_inverse_product() {
        // 用 J⁻¹ 显式乘 L 也应得到同一结果（通过 solve 解单位阵列得到 J⁻¹）
        let j = asymmetric_tensor();
        let l = Vector3::new(
            AngularMomentum::from_nms(12.0),
            AngularMomentum::from_nms(-5.0),
            AngularMomentum::from_nms(8.0),
        );
        let w = j.solve_omega(l).unwrap();

        // 逐列解单位阵得到 J⁻¹ 的三列，再组合
        let inv_col0 = j
            .cholesky_solve([1.0, 0.0, 0.0])
            .unwrap();
        let inv_col1 = j
            .cholesky_solve([0.0, 1.0, 0.0])
            .unwrap();
        let inv_col2 = j
            .cholesky_solve([0.0, 0.0, 1.0])
            .unwrap();

        let b = [12.0, -5.0, 8.0];
        let manual = [
            inv_col0[0] * b[0] + inv_col1[0] * b[1] + inv_col2[0] * b[2],
            inv_col0[1] * b[0] + inv_col1[1] * b[1] + inv_col2[1] * b[2],
            inv_col0[2] * b[0] + inv_col1[2] * b[1] + inv_col2[2] * b[2],
        ];

        assert_relative_eq!(w.x.as_rad_per_second(), manual[0], epsilon = 1e-12);
        assert_relative_eq!(w.y.as_rad_per_second(), manual[1], epsilon = 1e-12);
        assert_relative_eq!(w.z.as_rad_per_second(), manual[2], epsilon = 1e-12);
    }

    #[test]
    fn test_solve_isotropic_is_scalar_division() {
        // 各向同性张量：张量解必须与标量除法一致
        let j_iso = 5.0;
        let j = InertiaTensor::from_isotropic(Inertia::from_kg_m2(j_iso));
        let l = Vector3::new(
            AngularMomentum::from_nms(10.0),
            AngularMomentum::from_nms(20.0),
            AngularMomentum::from_nms(-15.0),
        );
        let w = j.solve_omega(l).unwrap();

        assert_relative_eq!(w.x.as_rad_per_second(), 10.0 / j_iso, epsilon = 1e-12);
        assert_relative_eq!(w.y.as_rad_per_second(), 20.0 / j_iso, epsilon = 1e-12);
        assert_relative_eq!(w.z.as_rad_per_second(), -15.0 / j_iso, epsilon = 1e-12);
    }

    // ===== diagonalize =====

    #[test]
    fn test_diagonalize_already_diagonal() {
        let j = InertiaTensor::from_diagonal([
            Inertia::from_kg_m2(45.0),
            Inertia::from_kg_m2(120.0),
            Inertia::from_kg_m2(130.0),
        ]);
        let (p, _) = j.diagonalize();

        // 升序排列
        assert_relative_eq!(p[0].as_kg_m2(), 45.0, epsilon = 1e-9);
        assert_relative_eq!(p[1].as_kg_m2(), 120.0, epsilon = 1e-9);
        assert_relative_eq!(p[2].as_kg_m2(), 130.0, epsilon = 1e-9);
    }

    #[test]
    fn test_diagonalize_isotropic() {
        // 各向同性：三个主惯量相等，主轴任意
        let j = InertiaTensor::from_isotropic(Inertia::from_kg_m2(7.0));
        let (p, _) = j.diagonalize();
        for v in p {
            assert_relative_eq!(v.as_kg_m2(), 7.0, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_diagonalize_satisfies_rtjr() {
        // 核心验证：Rᵀ JR 必须是对角阵，且对角元等于返回的主惯量
        let j = asymmetric_tensor();
        let (principal, r) = j.diagonalize();

        let rot = r.to_array();
        // m = Rᵀ J R
        let mut jr = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for k in 0..3 {
                let mut sum = 0.0;
                for l in 0..3 {
                    sum += j.data[i][l].as_kg_m2() * rot[l][k];
                }
                jr[i][k] = sum;
            }
        }
        let mut out = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for k in 0..3 {
                let mut sum = 0.0;
                for l in 0..3 {
                    sum += rot[l][i] * jr[l][k];
                }
                out[i][k] = sum;
            }
        }

        // 对角元应等于主惯量
        for i in 0..3 {
            assert_relative_eq!(out[i][i], principal[i].as_kg_m2(), epsilon = 1e-8);
        }
        // 非对角元应为零
        assert!(out[0][1].abs() < 1e-8, "off-diag[0][1] = {}", out[0][1]);
        assert!(out[0][2].abs() < 1e-8, "off-diag[0][2] = {}", out[0][2]);
        assert!(out[1][2].abs() < 1e-8, "off-diag[1][2] = {}", out[1][2]);
    }

    #[test]
    fn test_diagonalize_preserves_trace() {
        // 迹在正交变换下不变：三个主惯量之和 = 原张量迹
        let j = asymmetric_tensor();
        let trace = j.data[0][0].as_kg_m2()
            + j.data[1][1].as_kg_m2()
            + j.data[2][2].as_kg_m2();
        let (p, _) = j.diagonalize();
        let sum = p[0].as_kg_m2() + p[1].as_kg_m2() + p[2].as_kg_m2();
        assert_relative_eq!(sum, trace, epsilon = 1e-8);
    }

    #[test]
    fn test_diagonalize_rotation_is_orthonormal() {
        // R 的列必须正交归一
        let j = asymmetric_tensor();
        let (_, r) = j.diagonalize();
        let m = r.to_array();

        for c1 in 0..3 {
            for c2 in 0..3 {
                let mut dot = 0.0;
                for row in 0..3 {
                    dot += m[row][c1] * m[row][c2];
                }
                let expected = if c1 == c2 { 1.0 } else { 0.0 };
                assert_relative_eq!(dot, expected, epsilon = 1e-8);
            }
        }
    }

    #[test]
    fn test_diagonalize_default_axes_give_principal_inertia() {
        // 用 about_axis 交叉验证：绕某根主轴方向测得的等效惯量
        // 应等于对应的主惯量
        let j = asymmetric_tensor();
        let (principal, r) = j.diagonalize();
        let m = r.to_array();

        for col in 0..3 {
            let axis = Vector3::new(
                Coef::new(m[0][col]),
                Coef::new(m[1][col]),
                Coef::new(m[2][col]),
            );
            let about = j.about_axis(axis).unwrap();
            assert_relative_eq!(about.as_kg_m2(), principal[col].as_kg_m2(), epsilon = 1e-8);
        }
    }

    #[test]
    fn test_get_out_of_bounds() {
        let j = asymmetric_tensor();
        assert!(j.get(3, 0).is_none());
        assert!(j.get(0, 3).is_none());
        assert!(j.get(2, 2).is_some());
    }

    #[test]
    fn test_diagonalize_negative_theta_branch() {
        // theta = (a_qq - a_pp)/(2·a_pq)
        // 取 a_pp > a_qq 且 a_pq > 0，使 theta < 0，覆盖 Jacobi 的负分支
        let j = InertiaTensor::new(
            Inertia::from_kg_m2(5.0),
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(3.0),
            Inertia::from_kg_m2(1.0),
            Inertia::default(),
            Inertia::default(),
        );
        let (p, r) = j.diagonalize();

        // 迹不变
        assert_relative_eq!(p[0].as_kg_m2() + p[1].as_kg_m2() + p[2].as_kg_m2(), 10.0, epsilon = 1e-9);

        // RᵀJR 仍是对角阵
        let rot = r.to_array();
        let mut jr = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for k in 0..3 {
                let mut sum = 0.0;
                for l in 0..3 {
                    sum += j.data[i][l].as_kg_m2() * rot[l][k];
                }
                jr[i][k] = sum;
            }
        }
        let mut out = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for k in 0..3 {
                let mut sum = 0.0;
                for l in 0..3 {
                    sum += rot[l][i] * jr[l][k];
                }
                out[i][k] = sum;
            }
        }
        assert!(out[0][1].abs() < 1e-8);
        assert!(out[1][2].abs() < 1e-8);
    }

    #[test]
    fn test_diagonalize_known_eigenvalues() {
        // [[2,-1,0],[-1,2,0],[0,0,3]] 的特征值是 1, 3, 3
        let j = InertiaTensor::new(
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(2.0),
            Inertia::from_kg_m2(3.0),
            Inertia::from_kg_m2(-1.0),
            Inertia::default(),
            Inertia::default(),
        );
        let (p, _) = j.diagonalize();
        assert_relative_eq!(p[0].as_kg_m2(), 1.0, epsilon = 1e-9);
        assert_relative_eq!(p[1].as_kg_m2(), 3.0, epsilon = 1e-9);
        assert_relative_eq!(p[2].as_kg_m2(), 3.0, epsilon = 1e-9);
    }

    // ===== parallel_axis =====

    #[test]
    fn test_parallel_axis_point_mass() {
        // 质点 2 kg 位于 x=3：绕 z 轴 J = m·d² = 18
        let zero = InertiaTensor::from_isotropic(Inertia::default());
        let shifted = zero.parallel_axis(
            Mass::from_kg(2.0),
            Vector3::new(
                Distance::from_m(3.0),
                Distance::from_m(0.0),
                Distance::from_m(0.0),
            ),
        );
        // 绕 z：d² = 9，2×9 = 18
        assert_relative_eq!(shifted.get(2, 2).unwrap().as_kg_m2(), 18.0, epsilon = 1e-12);
        // 绕 x：d 沿 x，贡献为零
        assert_relative_eq!(shifted.get(0, 0).unwrap().as_kg_m2(), 0.0, epsilon = 1e-12);
        // 绕 y：d² 里的 x 分量贡献 18
        assert_relative_eq!(shifted.get(1, 1).unwrap().as_kg_m2(), 18.0, epsilon = 1e-12);
    }

    #[test]
    fn test_parallel_axis_diagonal_form() {
        // 位移 (a, b, 0)，质量 m：
        //   Jxx = m·b², Jyy = m·a², Jzz = m(a²+b²), Jxy = -m·a·b
        let zero = InertiaTensor::from_isotropic(Inertia::default());
        let a = 2.0;
        let b = 3.0;
        let m = 5.0;
        let shifted = zero.parallel_axis(
            Mass::from_kg(m),
            Vector3::new(
                Distance::from_m(a),
                Distance::from_m(b),
                Distance::from_m(0.0),
            ),
        );

        assert_relative_eq!(shifted.get(0, 0).unwrap().as_kg_m2(), m * b * b, epsilon = 1e-12);
        assert_relative_eq!(shifted.get(1, 1).unwrap().as_kg_m2(), m * a * a, epsilon = 1e-12);
        assert_relative_eq!(
            shifted.get(2, 2).unwrap().as_kg_m2(),
            m * (a * a + b * b),
            epsilon = 1e-12
        );
        // 非对角元为负
        assert_relative_eq!(shifted.get(0, 1).unwrap().as_kg_m2(), -m * a * b, epsilon = 1e-12);
    }

    #[test]
    fn test_parallel_axis_zero_offset_is_identity() {
        let j = asymmetric_tensor();
        let shifted = j.parallel_axis(
            Mass::from_kg(100.0),
            Vector3::new(
                Distance::from_m(0.0),
                Distance::from_m(0.0),
                Distance::from_m(0.0),
            ),
        );
        for i in 0..3 {
            for k in 0..3 {
                assert_relative_eq!(
                    shifted.get(i, k).unwrap().as_kg_m2(),
                    j.get(i, k).unwrap().as_kg_m2(),
                    epsilon = 1e-12
                );
            }
        }
    }

    #[test]
    fn test_parallel_axis_is_additive() {
        // 定理对张量是线性叠加：J_cm + 位移项
        let j_cm = asymmetric_tensor();
        let m = 10.0;
        let d = Vector3::new(
            Distance::from_m(1.5),
            Distance::from_m(-2.0),
            Distance::from_m(0.5),
        );

        let combined = j_cm.parallel_axis(Mass::from_kg(m), d);
        let offset_only =
            InertiaTensor::from_isotropic(Inertia::default()).parallel_axis(Mass::from_kg(m), d);

        for i in 0..3 {
            for k in 0..3 {
                assert_relative_eq!(
                    combined.get(i, k).unwrap().as_kg_m2(),
                    j_cm.get(i, k).unwrap().as_kg_m2() + offset_only.get(i, k).unwrap().as_kg_m2(),
                    epsilon = 1e-12
                );
            }
        }
    }

    // ===== from_engineering_array / to_engineering_array =====

    #[test]
    fn test_engineering_array_sign_convention() {
        // 工程裸值 Ixy = +8 → 张量存储应为 -8
        let j = InertiaTensor::from_engineering_array([120.0, 130.0, 45.0, 8.0, 0.0, 0.0]);
        assert_relative_eq!(j.get(0, 1).unwrap().as_kg_m2(), -8.0, epsilon = 1e-12);
        assert_relative_eq!(j.get(1, 0).unwrap().as_kg_m2(), -8.0, epsilon = 1e-12);

        // 与手写取负的 new 一致
        let j2 = InertiaTensor::new(
            Inertia::from_kg_m2(120.0),
            Inertia::from_kg_m2(130.0),
            Inertia::from_kg_m2(45.0),
            Inertia::from_kg_m2(-8.0),
            Inertia::default(),
            Inertia::default(),
        );
        assert_relative_eq!(
            j.get(0, 1).unwrap().as_kg_m2(),
            j2.get(0, 1).unwrap().as_kg_m2(),
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_engineering_array_round_trip() {
        let v = [120.0, 130.0, 45.0, 8.0, -3.0, 2.0];
        let j = InertiaTensor::from_engineering_array(v);
        let back = j.to_engineering_array();
        for i in 0..6 {
            assert_relative_eq!(back[i], v[i], epsilon = 1e-12);
        }
    }

    #[test]
    fn test_engineering_array_diagonal_unaffected() {
        // 对角元不参与符号翻转
        let j = InertiaTensor::from_engineering_array([1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
        assert_relative_eq!(j.get(0, 0).unwrap().as_kg_m2(), 1.0, epsilon = 1e-12);
        assert_relative_eq!(j.get(1, 1).unwrap().as_kg_m2(), 2.0, epsilon = 1e-12);
        assert_relative_eq!(j.get(2, 2).unwrap().as_kg_m2(), 3.0, epsilon = 1e-12);
    }

    #[test]
    fn test_engineering_array_affects_rotation() {
        // 符号约定错了会影响物理结果：验证用工程数组构造的 J
        // 与手写 new(-ixy) 的 J·ω 完全一致
        let j1 = InertiaTensor::from_engineering_array([120.0, 130.0, 45.0, 8.0, 0.0, 0.0]);
        let j2 = InertiaTensor::new(
            Inertia::from_kg_m2(120.0),
            Inertia::from_kg_m2(130.0),
            Inertia::from_kg_m2(45.0),
            Inertia::from_kg_m2(-8.0),
            Inertia::default(),
            Inertia::default(),
        );
        let w = Vector3::new(
            AngularVelocity::from_rad_per_second(0.0),
            AngularVelocity::from_rad_per_second(0.0),
            AngularVelocity::from_rad_per_second(1.0),
        );
        let l1 = j1 * w;
        let l2 = j2 * w;
        assert_relative_eq!(l1.x.as_nms(), l2.x.as_nms(), epsilon = 1e-15);
        assert_relative_eq!(l1.y.as_nms(), l2.y.as_nms(), epsilon = 1e-15);
    }
}
