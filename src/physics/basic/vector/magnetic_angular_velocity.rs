use super::*;
use crate::physics::basic::{
    AngularMomentum, MagneticAngularVelocity, MagneticInduction, MagneticMoment, Vector3,
};
const FLOAT_F64_E_6: f64 = 1e-6;
// m = k * (B * w) / |B|^2 映射为磁矩 Vector3<MagneticMoment> 的方法。
// 入参：
// - ang_momentum: AngularMomentum→ 用其模长作为系数 k
// - magnetic_induction: Vector3<MagneticInduction> → 参与 (B * w) 与 |B| 的计算
impl Vector3<MagneticAngularVelocity> {
    // 以标量角动量 k 作为输入：m = k * (B*w) / |B|^2
    pub fn to_magnetic_moment(
        &self,
        k: &AngularMomentum,
        magnetic_induction: &Vector3<MagneticInduction>,
    ) -> Vector3<MagneticMoment> {
        let k_val = k.as_nms();
        let magnetic_induction: Vector3<MagneticInduction> = Vector3::new(
            MagneticInduction::from_nano_tesla(magnetic_induction.x.as_nano_tesla()),
            MagneticInduction::from_nano_tesla(magnetic_induction.y.as_nano_tesla()),
            MagneticInduction::from_nano_tesla(magnetic_induction.z.as_nano_tesla()),
        );
        let mut denom = magnetic_induction.norm_square();

        if denom == 0.0 {
            denom = FLOAT_F64_E_6;
        }

        let wx = self.x.as_nano_tesla_rad_per_second();
        let wy = self.y.as_nano_tesla_rad_per_second();
        let wz = self.z.as_nano_tesla_rad_per_second();

        let mx = MagneticMoment::from_nano_am2(k_val * wx / denom);
        let my = MagneticMoment::from_nano_am2(k_val * wy / denom);
        let mz = MagneticMoment::from_nano_am2(k_val * wz / denom);

        Vector3::new(mx, my, mz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_to_magnetic_moment_with_scalar_k() {
        let w = Vector3::new(
            MagneticAngularVelocity::from_tesla_rad_per_second(2.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(3.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(4.0),
        );
        let b = Vector3::new(
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_tesla(2.0),
            MagneticInduction::from_tesla(2.0),
        );
        let k = AngularMomentum::from_kg_m2_per_second(5.0);

        // |B|^2 = 9, m = 5 * [2,3,4] / 9
        let m = w.to_magnetic_moment(&k, &b);
        assert_relative_eq!(m.x.as_am2(), 10.0 / 9.0, epsilon = 1e-12);
        assert_relative_eq!(m.y.as_am2(), 15.0 / 9.0, epsilon = 1e-12);
        assert_relative_eq!(m.z.as_am2(), 20.0 / 9.0, epsilon = 1e-12);
    }

    #[test]
    fn test_to_magnetic_moment_zero_magnetic_induction() {
        // 测试磁感应强度为零的情况
        let w = Vector3::new(
            MagneticAngularVelocity::from_tesla_rad_per_second(1.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(2.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(3.0),
        );
        let b = Vector3::new(
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
        );
        let k = AngularMomentum::from_kg_m2_per_second(1.0);

        let m = w.to_magnetic_moment(&k, &b);
        
        // 当 |B|^2 = 0 时，应该使用 FLOAT_F64_E_6 作为分母
        let expected_denom = FLOAT_F64_E_6;
        assert_relative_eq!(m.x.as_am2(), 1.0 / expected_denom, epsilon = 1e-12);
        assert_relative_eq!(m.y.as_am2(), 2.0 / expected_denom, epsilon = 1e-12);
        assert_relative_eq!(m.z.as_am2(), 3.0 / expected_denom, epsilon = 1e-12);
    }

    #[test]
    fn test_to_magnetic_moment_different_units() {
        // 测试不同单位的磁角速度和磁感应强度
        let w = Vector3::new(
            MagneticAngularVelocity::from_mill_tesla_rad_per_second(1000.0), // 1 T·rad/s
            MagneticAngularVelocity::from_gauss_rad_per_second(10000.0),    // 1 T·rad/s
            MagneticAngularVelocity::from_micro_tesla_rad_per_second(1000000.0), // 1 T·rad/s
        );
        let b = Vector3::new(
            MagneticInduction::from_gauss(10000.0),     // 1 T
            MagneticInduction::from_mill_tesla(1000.0), // 1 T
            MagneticInduction::from_micro_tesla(1000000.0), // 1 T
        );
        let k = AngularMomentum::from_mill_nms(1000.0); // 1 N·m·s

        let m = w.to_magnetic_moment(&k, &b);
        
        // |B|^2 = 3, k = 1, 所以 m = [1,1,1] / 3
        assert_relative_eq!(m.x.as_am2(), 1.0 / 3.0, epsilon = 1e-12);
        assert_relative_eq!(m.y.as_am2(), 1.0 / 3.0, epsilon = 1e-12);
        assert_relative_eq!(m.z.as_am2(), 1.0 / 3.0, epsilon = 1e-12);
    }

    #[test]
    fn test_to_magnetic_moment_negative_values() {
        // 测试负值
        let w = Vector3::new(
            MagneticAngularVelocity::from_tesla_rad_per_second(-2.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(3.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(-1.5),
        );
        let b = Vector3::new(
            MagneticInduction::from_tesla(-1.0),
            MagneticInduction::from_tesla(2.0),
            MagneticInduction::from_tesla(-1.0),
        );
        let k = AngularMomentum::from_kg_m2_per_second(-2.0);

        let m = w.to_magnetic_moment(&k, &b);
        
        // |B|^2 = 6, k = -2, 所以 m = -2 * [-2,3,-1.5] / 6 = [4/6, -6/6, 3/6]
        assert_relative_eq!(m.x.as_am2(), 4.0 / 6.0, epsilon = 1e-12);
        assert_relative_eq!(m.y.as_am2(), -6.0 / 6.0, epsilon = 1e-12);
        assert_relative_eq!(m.z.as_am2(), 3.0 / 6.0, epsilon = 1e-12);
    }

    #[test]
    fn test_to_magnetic_moment_small_values() {
        // 测试很小的值
        let w = Vector3::new(
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(1e-9),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(2e-9),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(3e-9),
        );
        let b = Vector3::new(
            MagneticInduction::from_nano_tesla(1e-9),
            MagneticInduction::from_nano_tesla(2e-9),
            MagneticInduction::from_nano_tesla(3e-9),
        );
        let k = AngularMomentum::from_nano_nms(1e-9);

        let m = w.to_magnetic_moment(&k, &b);
        
        // |B|^2 = 14e-18, k = 1e-9, 所以 m = 1e-9 * [1e-9,2e-9,3e-9] / 14e-18
        let expected_denom = 14e-18;
        assert_relative_eq!(m.x.as_am2(), 1e-18 / expected_denom, epsilon = 1e-12);
        assert_relative_eq!(m.y.as_am2(), 2e-18 / expected_denom, epsilon = 1e-12);
        assert_relative_eq!(m.z.as_am2(), 3e-18 / expected_denom, epsilon = 1e-12);
    }

    #[test]
    fn test_to_magnetic_moment_large_values() {
        // 测试很大的值
        let w = Vector3::new(
            MagneticAngularVelocity::from_kilo_gauss_rad_per_second(1000.0), // 100 T·rad/s
            MagneticAngularVelocity::from_kilo_gauss_rad_per_second(2000.0), // 200 T·rad/s
            MagneticAngularVelocity::from_kilo_gauss_rad_per_second(3000.0), // 300 T·rad/s
        );
        let b = Vector3::new(
            MagneticInduction::from_kilo_gauss(10.0), // 1 T
            MagneticInduction::from_kilo_gauss(20.0), // 2 T
            MagneticInduction::from_kilo_gauss(30.0), // 3 T
        );
        let k = AngularMomentum::from_nms(1e6); // 1e6 N·m·s

        let m = w.to_magnetic_moment(&k, &b);
        
        // 重新计算：
        // w = [100, 200, 300] T·rad/s (转换为纳特斯拉·弧度/秒)
        // b = [1, 2, 3] T (转换为纳特斯拉)
        // |B|^2 = 1^2 + 2^2 + 3^2 = 14
        // k = 1e6 N·m·s
        // m = k * w / |B|^2 = 1e6 * [100, 200, 300] / 14
        assert_relative_eq!(m.x.as_am2(), 1e8 / 14.0, epsilon = 1e-12);
        assert_relative_eq!(m.y.as_am2(), 2e8 / 14.0, epsilon = 1e-12);
        assert_relative_eq!(m.z.as_am2(), 3e8 / 14.0, epsilon = 1e-12);
    }

    #[test]
    fn test_to_magnetic_moment_mixed_angular_momentum_units() {
        // 测试不同单位的角动量
        let w = Vector3::new(
            MagneticAngularVelocity::from_tesla_rad_per_second(1.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(1.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(1.0),
        );
        let b = Vector3::new(
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_tesla(1.0),
        );
        
        // 测试不同单位的角动量
        let k1 = AngularMomentum::from_kg_m2_per_second(1.0);
        let k2 = AngularMomentum::from_mill_nms(1000.0); // 1 N·m·s
        let k3 = AngularMomentum::from_micro_nms(1000000.0); // 1 N·m·s

        let m1 = w.to_magnetic_moment(&k1, &b);
        let m2 = w.to_magnetic_moment(&k2, &b);
        let m3 = w.to_magnetic_moment(&k3, &b);

        // |B|^2 = 3, 所有k都等于1 N·m·s，所以结果应该相同
        let expected = 1.0 / 3.0;
        assert_relative_eq!(m1.x.as_am2(), expected, epsilon = 1e-12);
        assert_relative_eq!(m2.x.as_am2(), expected, epsilon = 1e-12);
        assert_relative_eq!(m3.x.as_am2(), expected, epsilon = 1e-12);
    }
}
