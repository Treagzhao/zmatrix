use crate::physics::basic::{
    MagneticMoment, MagneticMomentType, Vector3, Coef, MagneticInduction, Torque,
};
use crate::utils::float;

impl Vector3<MagneticMoment> {
    /// 磁偶极子在外磁场中所受的磁力矩：`τ = m × B`。
    ///
    /// 方向遵循右手定则：力矩倾向于把磁矩转向与外场平行。
    /// 量纲核对：`A·m² × T = A·m² × N/(A·m) = N·m` ✓
    ///
    /// # 与 `cross` 的关系
    ///
    /// 等价的通用写法是 [`Vector3::cross`]：
    ///
    /// ```text
    /// let tau: Vector3<Torque> = m.cross(&b);      // 通用入口
    /// let tau = m.magnetic_torque(&b);             // 本方法
    /// ```
    ///
    /// 两者数值完全相同。本方法的价值在于**名字即物理含义**，
    /// 且不需要调用方写类型标注。选哪个看场景：
    /// 一长串向量运算里用 `cross` 更连贯，单点计算用本方法更易读。
    ///
    /// # 为什么没有「反向叉乘」
    ///
    /// `B × m = -(m × B)`，是同一个物理量的相反数，不是另一个物理量。
    /// 因此不提供 `Vector3<MagneticInduction>` 上的对应方法 —— 两个方法给出
    /// 同一个力矩但符号相反，只会制造符号错误。真需要反向时写 `-tau`。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::physics::basic::{MagneticInduction, MagneticMoment, Torque, Vector3};
    ///
    /// // 磁矩沿 +x，外场沿 +y
    /// let m = Vector3::new(
    ///     MagneticMoment::from_am2(2.0),
    ///     MagneticMoment::from_am2(0.0),
    ///     MagneticMoment::from_am2(0.0),
    /// );
    /// let b = Vector3::new(
    ///     MagneticInduction::from_tesla(0.0),
    ///     MagneticInduction::from_tesla(3.0),
    ///     MagneticInduction::from_tesla(0.0),
    /// );
    ///
    /// // x × y = +z，大小 2 × 3 = 6 N·m
    /// let tau = m.magnetic_torque(&b);
    /// assert!((tau.z.as_nm() - 6.0).abs() < 1e-12);
    ///
    /// // 与通用 cross 入口结果一致
    /// let tau2: Vector3<Torque> = m.cross(&b);
    /// assert!((tau.z.as_nm() - tau2.z.as_nm()).abs() < 1e-15);
    /// ```
    pub fn magnetic_torque(&self, b: &Vector3<MagneticInduction>) -> Vector3<Torque> {
        // 委托给通用叉乘，避免两份展开公式各自漂移
        self.cross(b)
    }

    /// 磁矩向量与外场向量的**势能**：`U = -m · B`。
    ///
    /// 注意是**负值**，与标量版本
    /// [`MagneticMoment::potential_energy`](crate::physics::basic::MagneticMoment::potential_energy)
    /// 一致。平行取向势能最低，反平行最高。
    ///
    /// 力矩 `τ = m × B` 见 [`magnetic_torque`](Self::magnetic_torque) 或
    /// [`cross`](crate::physics::basic::Vector3::cross)。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::physics::basic::{MagneticInduction, MagneticMoment, Vector3};
    ///
    /// let m = Vector3::new(
    ///     MagneticMoment::from_am2(2.0),
    ///     MagneticMoment::from_am2(0.0),
    ///     MagneticMoment::from_am2(0.0),
    /// );
    /// // 平行：U = -6 J
    /// let b = Vector3::new(
    ///     MagneticInduction::from_tesla(3.0),
    ///     MagneticInduction::from_tesla(0.0),
    ///     MagneticInduction::from_tesla(0.0),
    /// );
    /// assert!((m.potential_energy(&b).as_joule() + 6.0).abs() < 1e-12);
    ///
    /// // 反平行：U = +6 J
    /// let b2 = Vector3::new(
    ///     MagneticInduction::from_tesla(-3.0),
    ///     MagneticInduction::from_tesla(0.0),
    ///     MagneticInduction::from_tesla(0.0),
    /// );
    /// assert!((m.potential_energy(&b2).as_joule() - 6.0).abs() < 1e-12);
    /// ```
    pub fn potential_energy(
        &self,
        b: &Vector3<MagneticInduction>,
    ) -> crate::physics::basic::Energy {
        crate::physics::basic::Energy::from_joule(
            -(self.x.as_am2() * b.x.as_tesla()
                + self.y.as_am2() * b.y.as_tesla()
                + self.z.as_am2() * b.z.as_tesla()),
        )
    }

    pub fn limit_float(&self, threshold: f64, magnetic_moment_type: MagneticMomentType) -> Self {
        let (x, y, z) = match magnetic_moment_type {
            MagneticMomentType::AM2 => (
                float::limit_float(self.x.as_am2(), threshold),
                float::limit_float(self.y.as_am2(), threshold),
                float::limit_float(self.z.as_am2(), threshold),
            ),
            MagneticMomentType::MillAM2 => (
                float::limit_float(self.x.as_mill_am2(), threshold),
                float::limit_float(self.y.as_mill_am2(), threshold),
                float::limit_float(self.z.as_mill_am2(), threshold),
            ),
            MagneticMomentType::MicroAM2 => (
                float::limit_float(self.x.as_micro_am2(), threshold),
                float::limit_float(self.y.as_micro_am2(), threshold),
                float::limit_float(self.z.as_micro_am2(), threshold),
            ),
            MagneticMomentType::NanoAM2 => (
                float::limit_float(self.x.as_nano_am2(), threshold),
                float::limit_float(self.y.as_nano_am2(), threshold),
                float::limit_float(self.z.as_nano_am2(), threshold),
            ),
            MagneticMomentType::JPerTesla => (
                float::limit_float(self.x.as_j_per_tesla(), threshold),
                float::limit_float(self.y.as_j_per_tesla(), threshold),
                float::limit_float(self.z.as_j_per_tesla(), threshold),
            ),
            MagneticMomentType::MillJPerTesla => (
                float::limit_float(self.x.as_mill_j_per_tesla(), threshold),
                float::limit_float(self.y.as_mill_j_per_tesla(), threshold),
                float::limit_float(self.z.as_mill_j_per_tesla(), threshold),
            ),
            MagneticMomentType::MicroJPerTesla => (
                float::limit_float(self.x.as_micro_j_per_tesla(), threshold),
                float::limit_float(self.y.as_micro_j_per_tesla(), threshold),
                float::limit_float(self.z.as_micro_j_per_tesla(), threshold),
            ),
            MagneticMomentType::NanoJPerTesla => (
                float::limit_float(self.x.as_nano_j_per_tesla(), threshold),
                float::limit_float(self.y.as_nano_j_per_tesla(), threshold),
                float::limit_float(self.z.as_nano_j_per_tesla(), threshold),
            ),
        };
        match magnetic_moment_type {
            MagneticMomentType::AM2 => Self { x: MagneticMoment::from_am2(x), y: MagneticMoment::from_am2(y), z: MagneticMoment::from_am2(z) },
            MagneticMomentType::MillAM2 => Self { x: MagneticMoment::from_mill_am2(x), y: MagneticMoment::from_mill_am2(y), z: MagneticMoment::from_mill_am2(z) },
            MagneticMomentType::MicroAM2 => Self { x: MagneticMoment::from_micro_am2(x), y: MagneticMoment::from_micro_am2(y), z: MagneticMoment::from_micro_am2(z) },
            MagneticMomentType::NanoAM2 => Self { x: MagneticMoment::from_nano_am2(x), y: MagneticMoment::from_nano_am2(y), z: MagneticMoment::from_nano_am2(z) },
            MagneticMomentType::JPerTesla => Self { x: MagneticMoment::from_j_per_tesla(x), y: MagneticMoment::from_j_per_tesla(y), z: MagneticMoment::from_j_per_tesla(z) },
            MagneticMomentType::MillJPerTesla => Self { x: MagneticMoment::from_mill_j_per_tesla(x), y: MagneticMoment::from_mill_j_per_tesla(y), z: MagneticMoment::from_mill_j_per_tesla(z) },
            MagneticMomentType::MicroJPerTesla => Self { x: MagneticMoment::from_micro_j_per_tesla(x), y: MagneticMoment::from_micro_j_per_tesla(y), z: MagneticMoment::from_micro_j_per_tesla(z) },
            MagneticMomentType::NanoJPerTesla => Self { x: MagneticMoment::from_nano_j_per_tesla(x), y: MagneticMoment::from_nano_j_per_tesla(y), z: MagneticMoment::from_nano_j_per_tesla(z) },
        }
    }

    pub fn to_vector3_coef(&self, magnetic_moment_type: MagneticMomentType) -> Vector3<Coef> {
        match magnetic_moment_type {
            MagneticMomentType::AM2 => {
                let (x, y, z) = (self.x.as_am2(), self.y.as_am2(), self.z.as_am2());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticMomentType::MillAM2 => {
                let (x, y, z) = (self.x.as_mill_am2(), self.y.as_mill_am2(), self.z.as_mill_am2());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticMomentType::MicroAM2 => {
                let (x, y, z) = (self.x.as_micro_am2(), self.y.as_micro_am2(), self.z.as_micro_am2());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticMomentType::NanoAM2 => {
                let (x, y, z) = (self.x.as_nano_am2(), self.y.as_nano_am2(), self.z.as_nano_am2());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticMomentType::JPerTesla => {
                let (x, y, z) = (self.x.as_j_per_tesla(), self.y.as_j_per_tesla(), self.z.as_j_per_tesla());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticMomentType::MillJPerTesla => {
                let (x, y, z) = (self.x.as_mill_j_per_tesla(), self.y.as_mill_j_per_tesla(), self.z.as_mill_j_per_tesla());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticMomentType::MicroJPerTesla => {
                let (x, y, z) = (self.x.as_micro_j_per_tesla(), self.y.as_micro_j_per_tesla(), self.z.as_micro_j_per_tesla());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticMomentType::NanoJPerTesla => {
                let (x, y, z) = (self.x.as_nano_j_per_tesla(), self.y.as_nano_j_per_tesla(), self.z.as_nano_j_per_tesla());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, magnetic_moment_type: MagneticMomentType) -> Vector3<MagneticMoment> {
        let [x, y, z] = coef.to_array();
        match magnetic_moment_type {
            MagneticMomentType::AM2 => {
                Vector3::new(
                    MagneticMoment::from_am2(x),
                    MagneticMoment::from_am2(y),
                    MagneticMoment::from_am2(z),
                )
            }
            MagneticMomentType::MillAM2 => {
                Vector3::new(
                    MagneticMoment::from_mill_am2(x),
                    MagneticMoment::from_mill_am2(y),
                    MagneticMoment::from_mill_am2(z),
                )
            }
            MagneticMomentType::MicroAM2 => {
                Vector3::new(
                    MagneticMoment::from_micro_am2(x),
                    MagneticMoment::from_micro_am2(y),
                    MagneticMoment::from_micro_am2(z),
                )
            }
            MagneticMomentType::NanoAM2 => {
                Vector3::new(
                    MagneticMoment::from_nano_am2(x),
                    MagneticMoment::from_nano_am2(y),
                    MagneticMoment::from_nano_am2(z),
                )
            }
            MagneticMomentType::JPerTesla => {
                Vector3::new(
                    MagneticMoment::from_j_per_tesla(x),
                    MagneticMoment::from_j_per_tesla(y),
                    MagneticMoment::from_j_per_tesla(z),
                )
            }
            MagneticMomentType::MillJPerTesla => {
                Vector3::new(
                    MagneticMoment::from_mill_j_per_tesla(x),
                    MagneticMoment::from_mill_j_per_tesla(y),
                    MagneticMoment::from_mill_j_per_tesla(z),
                )
            }
            MagneticMomentType::MicroJPerTesla => {
                Vector3::new(
                    MagneticMoment::from_micro_j_per_tesla(x),
                    MagneticMoment::from_micro_j_per_tesla(y),
                    MagneticMoment::from_micro_j_per_tesla(z),
                )
            }
            MagneticMomentType::NanoJPerTesla => {
                Vector3::new(
                    MagneticMoment::from_nano_j_per_tesla(x),
                    MagneticMoment::from_nano_j_per_tesla(y),
                    MagneticMoment::from_nano_j_per_tesla(z),
                )
            }
        }
    }

    pub fn from_array_with_unit(array: [f64; 3], magnetic_moment_type: MagneticMomentType) -> Vector3<MagneticMoment> {
        let [x, y, z] = array;
        match magnetic_moment_type {
            MagneticMomentType::AM2 => {
                Vector3::new(
                    MagneticMoment::from_am2(x),
                    MagneticMoment::from_am2(y),
                    MagneticMoment::from_am2(z),
                )
            }
            MagneticMomentType::MillAM2 => {
                Vector3::new(
                    MagneticMoment::from_mill_am2(x),
                    MagneticMoment::from_mill_am2(y),
                    MagneticMoment::from_mill_am2(z),
                )
            }
            MagneticMomentType::MicroAM2 => {
                Vector3::new(
                    MagneticMoment::from_micro_am2(x),
                    MagneticMoment::from_micro_am2(y),
                    MagneticMoment::from_micro_am2(z),
                )
            }
            MagneticMomentType::NanoAM2 => {
                Vector3::new(
                    MagneticMoment::from_nano_am2(x),
                    MagneticMoment::from_nano_am2(y),
                    MagneticMoment::from_nano_am2(z),
                )
            }
            MagneticMomentType::JPerTesla => {
                Vector3::new(
                    MagneticMoment::from_j_per_tesla(x),
                    MagneticMoment::from_j_per_tesla(y),
                    MagneticMoment::from_j_per_tesla(z),
                )
            }
            MagneticMomentType::MillJPerTesla => {
                Vector3::new(
                    MagneticMoment::from_mill_j_per_tesla(x),
                    MagneticMoment::from_mill_j_per_tesla(y),
                    MagneticMoment::from_mill_j_per_tesla(z),
                )
            }
            MagneticMomentType::MicroJPerTesla => {
                Vector3::new(
                    MagneticMoment::from_micro_j_per_tesla(x),
                    MagneticMoment::from_micro_j_per_tesla(y),
                    MagneticMoment::from_micro_j_per_tesla(z),
                )
            }
            MagneticMomentType::NanoJPerTesla => {
                Vector3::new(
                    MagneticMoment::from_nano_j_per_tesla(x),
                    MagneticMoment::from_nano_j_per_tesla(y),
                    MagneticMoment::from_nano_j_per_tesla(z),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_limit_float() {
        let v = Vector3::new(
            MagneticMoment::from_am2(1.0),
            MagneticMoment::from_am2(-5.0),
            MagneticMoment::from_am2(3.0),
        );
        let result = v.limit_float(3.0, MagneticMomentType::AM2);
        assert_relative_eq!(result.x.as_am2(), 1.0);
        assert_relative_eq!(result.y.as_am2(), -3.0);
        assert_relative_eq!(result.z.as_am2(), 3.0);

        let result = v.limit_float(1e10, MagneticMomentType::MillAM2);
        assert_relative_eq!(result.x.as_mill_am2(), 1000.0);
        assert_relative_eq!(result.y.as_mill_am2(), -5000.0);
        assert_relative_eq!(result.z.as_mill_am2(), 3000.0);

        let result = v.limit_float(1e10, MagneticMomentType::MicroAM2);
        assert_relative_eq!(result.x.as_micro_am2(), 1000000.0);
        assert_relative_eq!(result.y.as_micro_am2(), -5000000.0);
        assert_relative_eq!(result.z.as_micro_am2(), 3000000.0);

        let result = v.limit_float(1e10, MagneticMomentType::NanoAM2);
        assert_relative_eq!(result.x.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(result.y.as_nano_am2(), -5000000000.0);
        assert_relative_eq!(result.z.as_nano_am2(), 3000000000.0);

        let result = v.limit_float(1e10, MagneticMomentType::JPerTesla);
        assert_relative_eq!(result.x.as_j_per_tesla(), 1.0);
        assert_relative_eq!(result.y.as_j_per_tesla(), -5.0);
        assert_relative_eq!(result.z.as_j_per_tesla(), 3.0);

        let result = v.limit_float(1e10, MagneticMomentType::MillJPerTesla);
        assert_relative_eq!(result.x.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(result.y.as_mill_j_per_tesla(), -5000.0);
        assert_relative_eq!(result.z.as_mill_j_per_tesla(), 3000.0);

        let result = v.limit_float(1e10, MagneticMomentType::MicroJPerTesla);
        assert_relative_eq!(result.x.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(result.y.as_micro_j_per_tesla(), -5000000.0);
        assert_relative_eq!(result.z.as_micro_j_per_tesla(), 3000000.0);

        let result = v.limit_float(1e10, MagneticMomentType::NanoJPerTesla);
        assert_relative_eq!(result.x.as_nano_j_per_tesla(), 1000000000.0);
        assert_relative_eq!(result.y.as_nano_j_per_tesla(), -5000000000.0);
        assert_relative_eq!(result.z.as_nano_j_per_tesla(), 3000000000.0);
    }

    #[test]
    fn test_to_vector3_coef_am2() {
        let magnetic_moment_vec = Vector3::new(
            MagneticMoment::from_am2(1.0),
            MagneticMoment::from_am2(2.0),
            MagneticMoment::from_am2(3.0),
        );
        let coef_vec = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::AM2);
        assert_relative_eq!(coef_vec.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_mill_am2() {
        let magnetic_moment_vec = Vector3::new(
            MagneticMoment::from_mill_am2(1000.0),
            MagneticMoment::from_mill_am2(2000.0),
            MagneticMoment::from_mill_am2(3000.0),
        );
        let coef_vec = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::MillAM2);
        assert_relative_eq!(coef_vec.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_micro_am2() {
        let magnetic_moment_vec = Vector3::new(
            MagneticMoment::from_micro_am2(1000000.0),
            MagneticMoment::from_micro_am2(2000000.0),
            MagneticMoment::from_micro_am2(3000000.0),
        );
        let coef_vec = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::MicroAM2);
        assert_relative_eq!(coef_vec.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_nano_am2() {
        let magnetic_moment_vec = Vector3::new(
            MagneticMoment::from_nano_am2(1000000000.0),
            MagneticMoment::from_nano_am2(2000000000.0),
            MagneticMoment::from_nano_am2(3000000000.0),
        );
        let coef_vec = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::NanoAM2);
        assert_relative_eq!(coef_vec.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3000000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_j_per_tesla() {
        let magnetic_moment_vec = Vector3::new(
            MagneticMoment::from_j_per_tesla(1.0),
            MagneticMoment::from_j_per_tesla(2.0),
            MagneticMoment::from_j_per_tesla(3.0),
        );
        let coef_vec = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::JPerTesla);
        assert_relative_eq!(coef_vec.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_mill_j_per_tesla() {
        let magnetic_moment_vec = Vector3::new(
            MagneticMoment::from_mill_j_per_tesla(1000.0),
            MagneticMoment::from_mill_j_per_tesla(2000.0),
            MagneticMoment::from_mill_j_per_tesla(3000.0),
        );
        let coef_vec = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::MillJPerTesla);
        assert_relative_eq!(coef_vec.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_micro_j_per_tesla() {
        let magnetic_moment_vec = Vector3::new(
            MagneticMoment::from_micro_j_per_tesla(1000000.0),
            MagneticMoment::from_micro_j_per_tesla(2000000.0),
            MagneticMoment::from_micro_j_per_tesla(3000000.0),
        );
        let coef_vec = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::MicroJPerTesla);
        assert_relative_eq!(coef_vec.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_nano_j_per_tesla() {
        let magnetic_moment_vec = Vector3::new(
            MagneticMoment::from_nano_j_per_tesla(1000000000.0),
            MagneticMoment::from_nano_j_per_tesla(2000000000.0),
            MagneticMoment::from_nano_j_per_tesla(3000000000.0),
        );
        let coef_vec = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::NanoJPerTesla);
        assert_relative_eq!(coef_vec.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3000000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_am2() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_vector_coef(coef_vec, MagneticMomentType::AM2);
        assert_relative_eq!(magnetic_moment_vec.x.as_am2(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.y.as_am2(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.z.as_am2(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_mill_am2() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_vector_coef(coef_vec, MagneticMomentType::MillAM2);
        assert_relative_eq!(magnetic_moment_vec.x.as_mill_am2(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.y.as_mill_am2(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.z.as_mill_am2(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_micro_am2() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_vector_coef(coef_vec, MagneticMomentType::MicroAM2);
        assert_relative_eq!(magnetic_moment_vec.x.as_micro_am2(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.y.as_micro_am2(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.z.as_micro_am2(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_nano_am2() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_vector_coef(coef_vec, MagneticMomentType::NanoAM2);
        assert_relative_eq!(magnetic_moment_vec.x.as_nano_am2(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.y.as_nano_am2(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.z.as_nano_am2(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_j_per_tesla() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_vector_coef(coef_vec, MagneticMomentType::JPerTesla);
        assert_relative_eq!(magnetic_moment_vec.x.as_j_per_tesla(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.y.as_j_per_tesla(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.z.as_j_per_tesla(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_mill_j_per_tesla() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_vector_coef(coef_vec, MagneticMomentType::MillJPerTesla);
        assert_relative_eq!(magnetic_moment_vec.x.as_mill_j_per_tesla(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.y.as_mill_j_per_tesla(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.z.as_mill_j_per_tesla(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_micro_j_per_tesla() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_vector_coef(coef_vec, MagneticMomentType::MicroJPerTesla);
        assert_relative_eq!(magnetic_moment_vec.x.as_micro_j_per_tesla(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.y.as_micro_j_per_tesla(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.z.as_micro_j_per_tesla(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_nano_j_per_tesla() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_vector_coef(coef_vec, MagneticMomentType::NanoJPerTesla);
        assert_relative_eq!(magnetic_moment_vec.x.as_nano_j_per_tesla(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.y.as_nano_j_per_tesla(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_moment_vec.z.as_nano_j_per_tesla(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_magnetic_moment_comprehensive_as_methods() {
        // 测试所有单位类型之间的转换
        let magnetic_moment_vec = Vector3::new(
            MagneticMoment::from_am2(1.0),
            MagneticMoment::from_am2(2.0),
            MagneticMoment::from_am2(3.0),
        );

        // 从 AM2 转换到所有单位类型
        let coef_am2 = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::AM2);
        let coef_mill_am2 = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::MillAM2);
        let coef_micro_am2 = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::MicroAM2);
        let coef_nano_am2 = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::NanoAM2);
        let coef_j_per_tesla = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::JPerTesla);
        let coef_mill_j_per_tesla = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::MillJPerTesla);
        let coef_micro_j_per_tesla = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::MicroJPerTesla);
        let coef_nano_j_per_tesla = magnetic_moment_vec.to_vector3_coef(MagneticMomentType::NanoJPerTesla);

        // 验证转换结果
        assert_relative_eq!(coef_am2.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_mill_am2.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_micro_am2.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_nano_am2.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_mill_j_per_tesla.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_micro_j_per_tesla.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_nano_j_per_tesla.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 MillAM2 转换到所有单位类型
        let mill_am2_vec = Vector3::new(
            MagneticMoment::from_mill_am2(1000.0),
            MagneticMoment::from_mill_am2(2000.0),
            MagneticMoment::from_mill_am2(3000.0),
        );

        let coef_from_mill_am2 = mill_am2_vec.to_vector3_coef(MagneticMomentType::AM2);
        let coef_from_mill_am2_mill = mill_am2_vec.to_vector3_coef(MagneticMomentType::MillAM2);
        let coef_from_mill_am2_micro = mill_am2_vec.to_vector3_coef(MagneticMomentType::MicroAM2);
        let coef_from_mill_am2_nano = mill_am2_vec.to_vector3_coef(MagneticMomentType::NanoAM2);
        let coef_from_mill_am2_j_per_tesla = mill_am2_vec.to_vector3_coef(MagneticMomentType::JPerTesla);
        let coef_from_mill_am2_mill_j_per_tesla = mill_am2_vec.to_vector3_coef(MagneticMomentType::MillJPerTesla);
        let coef_from_mill_am2_micro_j_per_tesla = mill_am2_vec.to_vector3_coef(MagneticMomentType::MicroJPerTesla);
        let coef_from_mill_am2_nano_j_per_tesla = mill_am2_vec.to_vector3_coef(MagneticMomentType::NanoJPerTesla);

        assert_relative_eq!(coef_from_mill_am2.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_am2_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_am2_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_am2_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_am2_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_am2_mill_j_per_tesla.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_am2_micro_j_per_tesla.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_am2_nano_j_per_tesla.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 MicroAM2 转换到所有单位类型
        let micro_am2_vec = Vector3::new(
            MagneticMoment::from_micro_am2(1000000.0),
            MagneticMoment::from_micro_am2(2000000.0),
            MagneticMoment::from_micro_am2(3000000.0),
        );

        let coef_from_micro_am2 = micro_am2_vec.to_vector3_coef(MagneticMomentType::AM2);
        let coef_from_micro_am2_mill = micro_am2_vec.to_vector3_coef(MagneticMomentType::MillAM2);
        let coef_from_micro_am2_micro = micro_am2_vec.to_vector3_coef(MagneticMomentType::MicroAM2);
        let coef_from_micro_am2_nano = micro_am2_vec.to_vector3_coef(MagneticMomentType::NanoAM2);
        let coef_from_micro_am2_j_per_tesla = micro_am2_vec.to_vector3_coef(MagneticMomentType::JPerTesla);
        let coef_from_micro_am2_mill_j_per_tesla = micro_am2_vec.to_vector3_coef(MagneticMomentType::MillJPerTesla);
        let coef_from_micro_am2_micro_j_per_tesla = micro_am2_vec.to_vector3_coef(MagneticMomentType::MicroJPerTesla);
        let coef_from_micro_am2_nano_j_per_tesla = micro_am2_vec.to_vector3_coef(MagneticMomentType::NanoJPerTesla);

        assert_relative_eq!(coef_from_micro_am2.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_am2_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_am2_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_am2_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_am2_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_am2_mill_j_per_tesla.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_am2_micro_j_per_tesla.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_am2_nano_j_per_tesla.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 NanoAM2 转换到所有单位类型
        let nano_am2_vec = Vector3::new(
            MagneticMoment::from_nano_am2(1000000000.0),
            MagneticMoment::from_nano_am2(2000000000.0),
            MagneticMoment::from_nano_am2(3000000000.0),
        );

        let coef_from_nano_am2 = nano_am2_vec.to_vector3_coef(MagneticMomentType::AM2);
        let coef_from_nano_am2_mill = nano_am2_vec.to_vector3_coef(MagneticMomentType::MillAM2);
        let coef_from_nano_am2_micro = nano_am2_vec.to_vector3_coef(MagneticMomentType::MicroAM2);
        let coef_from_nano_am2_nano = nano_am2_vec.to_vector3_coef(MagneticMomentType::NanoAM2);
        let coef_from_nano_am2_j_per_tesla = nano_am2_vec.to_vector3_coef(MagneticMomentType::JPerTesla);
        let coef_from_nano_am2_mill_j_per_tesla = nano_am2_vec.to_vector3_coef(MagneticMomentType::MillJPerTesla);
        let coef_from_nano_am2_micro_j_per_tesla = nano_am2_vec.to_vector3_coef(MagneticMomentType::MicroJPerTesla);
        let coef_from_nano_am2_nano_j_per_tesla = nano_am2_vec.to_vector3_coef(MagneticMomentType::NanoJPerTesla);

        assert_relative_eq!(coef_from_nano_am2.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_am2_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_am2_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_am2_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_am2_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_am2_mill_j_per_tesla.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_am2_micro_j_per_tesla.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_am2_nano_j_per_tesla.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 JPerTesla 转换到所有单位类型
        let j_per_tesla_vec = Vector3::new(
            MagneticMoment::from_j_per_tesla(1.0),
            MagneticMoment::from_j_per_tesla(2.0),
            MagneticMoment::from_j_per_tesla(3.0),
        );

        let coef_from_j_per_tesla = j_per_tesla_vec.to_vector3_coef(MagneticMomentType::AM2);
        let coef_from_j_per_tesla_mill = j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MillAM2);
        let coef_from_j_per_tesla_micro = j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MicroAM2);
        let coef_from_j_per_tesla_nano = j_per_tesla_vec.to_vector3_coef(MagneticMomentType::NanoAM2);
        let coef_from_j_per_tesla_j_per_tesla = j_per_tesla_vec.to_vector3_coef(MagneticMomentType::JPerTesla);
        let coef_from_j_per_tesla_mill_j_per_tesla = j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MillJPerTesla);
        let coef_from_j_per_tesla_micro_j_per_tesla = j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MicroJPerTesla);
        let coef_from_j_per_tesla_nano_j_per_tesla = j_per_tesla_vec.to_vector3_coef(MagneticMomentType::NanoJPerTesla);

        assert_relative_eq!(coef_from_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_j_per_tesla_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_j_per_tesla_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_j_per_tesla_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_j_per_tesla_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_j_per_tesla_mill_j_per_tesla.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_j_per_tesla_micro_j_per_tesla.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_j_per_tesla_nano_j_per_tesla.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 MillJPerTesla 转换到所有单位类型
        let mill_j_per_tesla_vec = Vector3::new(
            MagneticMoment::from_mill_j_per_tesla(1000.0),
            MagneticMoment::from_mill_j_per_tesla(2000.0),
            MagneticMoment::from_mill_j_per_tesla(3000.0),
        );

        let coef_from_mill_j_per_tesla = mill_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::AM2);
        let coef_from_mill_j_per_tesla_mill = mill_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MillAM2);
        let coef_from_mill_j_per_tesla_micro = mill_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MicroAM2);
        let coef_from_mill_j_per_tesla_nano = mill_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::NanoAM2);
        let coef_from_mill_j_per_tesla_j_per_tesla = mill_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::JPerTesla);
        let coef_from_mill_j_per_tesla_mill_j_per_tesla = mill_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MillJPerTesla);
        let coef_from_mill_j_per_tesla_micro_j_per_tesla = mill_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MicroJPerTesla);
        let coef_from_mill_j_per_tesla_nano_j_per_tesla = mill_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::NanoJPerTesla);

        assert_relative_eq!(coef_from_mill_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_j_per_tesla_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_j_per_tesla_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_j_per_tesla_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_j_per_tesla_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_j_per_tesla_mill_j_per_tesla.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_j_per_tesla_micro_j_per_tesla.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_j_per_tesla_nano_j_per_tesla.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 MicroJPerTesla 转换到所有单位类型
        let micro_j_per_tesla_vec = Vector3::new(
            MagneticMoment::from_micro_j_per_tesla(1000000.0),
            MagneticMoment::from_micro_j_per_tesla(2000000.0),
            MagneticMoment::from_micro_j_per_tesla(3000000.0),
        );

        let coef_from_micro_j_per_tesla = micro_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::AM2);
        let coef_from_micro_j_per_tesla_mill = micro_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MillAM2);
        let coef_from_micro_j_per_tesla_micro = micro_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MicroAM2);
        let coef_from_micro_j_per_tesla_nano = micro_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::NanoAM2);
        let coef_from_micro_j_per_tesla_j_per_tesla = micro_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::JPerTesla);
        let coef_from_micro_j_per_tesla_mill_j_per_tesla = micro_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MillJPerTesla);
        let coef_from_micro_j_per_tesla_micro_j_per_tesla = micro_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MicroJPerTesla);
        let coef_from_micro_j_per_tesla_nano_j_per_tesla = micro_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::NanoJPerTesla);

        assert_relative_eq!(coef_from_micro_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_j_per_tesla_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_j_per_tesla_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_j_per_tesla_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_j_per_tesla_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_j_per_tesla_mill_j_per_tesla.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_j_per_tesla_micro_j_per_tesla.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_j_per_tesla_nano_j_per_tesla.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 NanoJPerTesla 转换到所有单位类型
        let nano_j_per_tesla_vec = Vector3::new(
            MagneticMoment::from_nano_j_per_tesla(1000000000.0),
            MagneticMoment::from_nano_j_per_tesla(2000000000.0),
            MagneticMoment::from_nano_j_per_tesla(3000000000.0),
        );

        let coef_from_nano_j_per_tesla = nano_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::AM2);
        let coef_from_nano_j_per_tesla_mill = nano_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MillAM2);
        let coef_from_nano_j_per_tesla_micro = nano_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MicroAM2);
        let coef_from_nano_j_per_tesla_nano = nano_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::NanoAM2);
        let coef_from_nano_j_per_tesla_j_per_tesla = nano_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::JPerTesla);
        let coef_from_nano_j_per_tesla_mill_j_per_tesla = nano_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MillJPerTesla);
        let coef_from_nano_j_per_tesla_micro_j_per_tesla = nano_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::MicroJPerTesla);
        let coef_from_nano_j_per_tesla_nano_j_per_tesla = nano_j_per_tesla_vec.to_vector3_coef(MagneticMomentType::NanoJPerTesla);

        assert_relative_eq!(coef_from_nano_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_j_per_tesla_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_j_per_tesla_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_j_per_tesla_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_j_per_tesla_j_per_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_j_per_tesla_mill_j_per_tesla.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_j_per_tesla_micro_j_per_tesla.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_j_per_tesla_nano_j_per_tesla.x.get_value(), 1000000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_array_with_unit() {
        // 测试从数组和单位类型创建磁矩向量
        let array = [1.0, 2.0, 3.0];
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_array_with_unit(array, MagneticMomentType::AM2);
        
        assert_relative_eq!(magnetic_moment_vec.x.as_am2(), 1.0);
        assert_relative_eq!(magnetic_moment_vec.y.as_am2(), 2.0);
        assert_relative_eq!(magnetic_moment_vec.z.as_am2(), 3.0);
        
        // 测试不同单位类型
        let array = [1000.0, 2000.0, 3000.0];
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_array_with_unit(array, MagneticMomentType::MillAM2);
        
        assert_relative_eq!(magnetic_moment_vec.x.as_mill_am2(), 1000.0);
        assert_relative_eq!(magnetic_moment_vec.y.as_mill_am2(), 2000.0);
        assert_relative_eq!(magnetic_moment_vec.z.as_mill_am2(), 3000.0);
        
        // 测试微安平方米单位
        let array = [1000000.0, 2000000.0, 3000000.0];
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_array_with_unit(array, MagneticMomentType::MicroAM2);
        
        assert_relative_eq!(magnetic_moment_vec.x.as_micro_am2(), 1000000.0);
        assert_relative_eq!(magnetic_moment_vec.y.as_micro_am2(), 2000000.0);
        assert_relative_eq!(magnetic_moment_vec.z.as_micro_am2(), 3000000.0);
        
        // 测试纳安平方米单位
        let array = [1000000000.0, 2000000000.0, 3000000000.0];
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_array_with_unit(array, MagneticMomentType::NanoAM2);
        
        assert_relative_eq!(magnetic_moment_vec.x.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(magnetic_moment_vec.y.as_nano_am2(), 2000000000.0);
        assert_relative_eq!(magnetic_moment_vec.z.as_nano_am2(), 3000000000.0);
        
        // 测试焦耳每特斯拉单位
        let array = [1.0, 2.0, 3.0];
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_array_with_unit(array, MagneticMomentType::JPerTesla);
        
        assert_relative_eq!(magnetic_moment_vec.x.as_j_per_tesla(), 1.0);
        assert_relative_eq!(magnetic_moment_vec.y.as_j_per_tesla(), 2.0);
        assert_relative_eq!(magnetic_moment_vec.z.as_j_per_tesla(), 3.0);
        
        // 测试毫焦耳每特斯拉单位
        let array = [1000.0, 2000.0, 3000.0];
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_array_with_unit(array, MagneticMomentType::MillJPerTesla);
        
        assert_relative_eq!(magnetic_moment_vec.x.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(magnetic_moment_vec.y.as_mill_j_per_tesla(), 2000.0);
        assert_relative_eq!(magnetic_moment_vec.z.as_mill_j_per_tesla(), 3000.0);
        
        // 测试微焦耳每特斯拉单位
        let array = [1000000.0, 2000000.0, 3000000.0];
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_array_with_unit(array, MagneticMomentType::MicroJPerTesla);
        
        assert_relative_eq!(magnetic_moment_vec.x.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(magnetic_moment_vec.y.as_micro_j_per_tesla(), 2000000.0);
        assert_relative_eq!(magnetic_moment_vec.z.as_micro_j_per_tesla(), 3000000.0);
        
        // 测试纳焦耳每特斯拉单位
        let array = [1000000000.0, 2000000000.0, 3000000000.0];
        let magnetic_moment_vec = Vector3::<MagneticMoment>::from_array_with_unit(array, MagneticMomentType::NanoJPerTesla);
        
        assert_relative_eq!(magnetic_moment_vec.x.as_nano_j_per_tesla(), 1000000000.0);
        assert_relative_eq!(magnetic_moment_vec.y.as_nano_j_per_tesla(), 2000000000.0);
        assert_relative_eq!(magnetic_moment_vec.z.as_nano_j_per_tesla(), 3000000000.0);
    }

    fn m_along_x(v: f64) -> Vector3<MagneticMoment> {
        Vector3::new(
            MagneticMoment::from_am2(v),
            MagneticMoment::from_am2(0.0),
            MagneticMoment::from_am2(0.0),
        )
    }

    fn b_along_y(v: f64) -> Vector3<MagneticInduction> {
        Vector3::new(
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(v),
            MagneticInduction::from_tesla(0.0),
        )
    }

    #[test]
    fn test_magnetic_torque_basic() {
        // m 沿 +x, B 沿 +y → τ 沿 +z，大小 |m||B| sin90 = 2×3 = 6
        let tau = m_along_x(2.0).magnetic_torque(&b_along_y(3.0));
        assert_relative_eq!(tau.x.as_nm(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(tau.y.as_nm(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(tau.z.as_nm(), 6.0, epsilon = 1e-12);

        // 反向：m 沿 +y, B 沿 +x → τ 沿 -z
        let m = Vector3::new(
            MagneticMoment::from_am2(0.0),
            MagneticMoment::from_am2(2.0),
            MagneticMoment::from_am2(0.0),
        );
        let b = Vector3::new(
            MagneticInduction::from_tesla(3.0),
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
        );
        let tau = m.magnetic_torque(&b);
        assert_relative_eq!(tau.z.as_nm(), -6.0, epsilon = 1e-12);
    }

    #[test]
    fn test_magnetic_torque_parallel_is_zero() {
        // 平行时 θ = 0，sinθ = 0 → 力矩为零，这是稳态取向
        let b_along_x = Vector3::new(
            MagneticInduction::from_tesla(5.0),
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
        );
        let tau = m_along_x(2.0).magnetic_torque(&b_along_x);
        assert_relative_eq!(tau.x.as_nm(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(tau.y.as_nm(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(tau.z.as_nm(), 0.0, epsilon = 1e-12);

        // 反平行时 θ = 180°，sinθ = 0 → 力矩也为零，但这是不稳定平衡
        let b_anti = Vector3::new(
            MagneticInduction::from_tesla(-5.0),
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
        );
        let tau = m_along_x(2.0).magnetic_torque(&b_anti);
        assert_relative_eq!(tau.x.as_nm(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(tau.y.as_nm(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(tau.z.as_nm(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn test_magnetic_torque_45_degrees() {
        // m 与 B 夹角 45°，|τ| = |m||B| sin45 = 2 × 4 × 0.7071 = 5.6569
        let m = m_along_x(2.0);
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let b = Vector3::new(
            MagneticInduction::from_tesla(4.0 * s),
            MagneticInduction::from_tesla(4.0 * s),
            MagneticInduction::from_tesla(0.0),
        );
        let tau = m.magnetic_torque(&b);

        // 方向沿 +z（x × (x+y) 的 y 分量贡献）
        assert_relative_eq!(tau.z.as_nm(), 2.0 * 4.0 * s, epsilon = 1e-12);
        let mag = (tau.x.as_nm().powi(2) + tau.y.as_nm().powi(2) + tau.z.as_nm().powi(2)).sqrt();
        assert_relative_eq!(mag, 2.0 * 4.0 * s, epsilon = 1e-12);
    }

    #[test]
    fn test_magnetic_torque_perpendicular_to_both() {
        // 叉乘结果必须同时垂直于 m 与 B
        let m = Vector3::new(
            MagneticMoment::from_am2(1.0),
            MagneticMoment::from_am2(2.0),
            MagneticMoment::from_am2(3.0),
        );
        let b = Vector3::new(
            MagneticInduction::from_tesla(4.0),
            MagneticInduction::from_tesla(5.0),
            MagneticInduction::from_tesla(6.0),
        );
        let tau = m.magnetic_torque(&b);

        let dot_m = 1.0 * tau.x.as_nm() + 2.0 * tau.y.as_nm() + 3.0 * tau.z.as_nm();
        let dot_b = 4.0 * tau.x.as_nm() + 5.0 * tau.y.as_nm() + 6.0 * tau.z.as_nm();
        assert_relative_eq!(dot_m, 0.0, epsilon = 1e-9);
        assert_relative_eq!(dot_b, 0.0, epsilon = 1e-9);

        // 手算核对：(2*6-3*5, 3*4-1*6, 1*5-2*4) = (-3, 6, -3)
        assert_relative_eq!(tau.x.as_nm(), -3.0, epsilon = 1e-12);
        assert_relative_eq!(tau.y.as_nm(), 6.0, epsilon = 1e-12);
        assert_relative_eq!(tau.z.as_nm(), -3.0, epsilon = 1e-12);
    }

    #[test]
    fn test_magnetic_torque_multilinear() {
        // 磁矩翻倍 → 力矩翻倍
        let t1 = m_along_x(2.0).magnetic_torque(&b_along_y(3.0));
        let t2 = m_along_x(4.0).magnetic_torque(&b_along_y(3.0));
        assert_relative_eq!(t2.z.as_nm(), 2.0 * t1.z.as_nm(), epsilon = 1e-12);

        // 外场翻倍 → 力矩翻倍
        let t3 = m_along_x(2.0).magnetic_torque(&b_along_y(6.0));
        assert_relative_eq!(t3.z.as_nm(), 2.0 * t1.z.as_nm(), epsilon = 1e-12);

        // 整体取负 → 力矩取负
        let m_neg = Vector3::new(
            MagneticMoment::from_am2(-2.0),
            MagneticMoment::from_am2(0.0),
            MagneticMoment::from_am2(0.0),
        );
        assert_relative_eq!(
            m_neg.magnetic_torque(&b_along_y(3.0)).z.as_nm(),
            -t1.z.as_nm(),
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_potential_energy_vector() {
        let m = m_along_x(2.0);
        let b = Vector3::new(
            MagneticInduction::from_tesla(3.0),
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
        );

        // 平行：U = -m·B = -6 J（势能最低点）
        assert_relative_eq!(m.potential_energy(&b).as_joule(), -6.0, epsilon = 1e-12);

        // 垂直：U = 0
        assert_relative_eq!(
            m.potential_energy(&b_along_y(3.0)).as_joule(),
            0.0,
            epsilon = 1e-12
        );

        // 反平行：U = +6 J（势能最高点，不稳定平衡）
        let b_anti = Vector3::new(
            MagneticInduction::from_tesla(-3.0),
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
        );
        assert_relative_eq!(m.potential_energy(&b_anti).as_joule(), 6.0, epsilon = 1e-12);

        // 与标量路径数值一致
        let scalar = MagneticMoment::from_am2(2.0).potential_energy(MagneticInduction::from_tesla(3.0));
        assert_relative_eq!(
            m.potential_energy(&b).as_joule(),
            scalar.as_joule(),
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_potential_energy_and_torque_relation() {
        // |τ| = -dU/dθ。U(θ) = -mB cosθ，故 |τ| = mB sinθ
        // 取 θ = 45°，mB = 2×4 = 8
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let m = m_along_x(2.0);
        let b = Vector3::new(
            MagneticInduction::from_tesla(4.0 * s),
            MagneticInduction::from_tesla(4.0 * s),
            MagneticInduction::from_tesla(0.0),
        );
        // U = -mB cos45 = -8 × 0.7071 = -5.6569
        let u = m.potential_energy(&b).as_joule();
        // |τ| = mB sin45 = 8 × 0.7071 = 5.6569
        let torque = m.magnetic_torque(&b).z.as_nm();

        assert_relative_eq!(u, -8.0 * s, epsilon = 1e-12);
        assert_relative_eq!(torque, 8.0 * s, epsilon = 1e-12);
        // 关系：U = -|τ| 当 θ=45°（因 cos45 = sin45）
        assert_relative_eq!(u, -torque, epsilon = 1e-12);
    }
}
