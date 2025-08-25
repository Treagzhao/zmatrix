use std::ops::{Div, Mul};
use crate::physics::basic::{AngularVelocity, MagneticAngularVelocity, MagneticInduction, Vector3};

impl Mul<AngularVelocity> for Vector3<MagneticInduction>{
    type Output = Vector3<MagneticAngularVelocity>;

    fn mul(self, rhs: AngularVelocity) -> Self::Output {
        let x = self.x.as_nano_tesla() * rhs.as_rad_per_second();
        let y = self.y.as_nano_tesla() * rhs.as_rad_per_second();
        let z = self.z.as_nano_tesla() * rhs.as_rad_per_second();
        Vector3::new(
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(x),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(y),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(z),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_vector3_magnetic_induction_mul_angular_velocity() {
        // 创建磁感应强度向量
        let b = Vector3::new(
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_gauss(100.0),
            MagneticInduction::from_mill_tesla(500.0),
        );

        // 创建角速度
        let omega = AngularVelocity::from_rad_per_second(2.0);

        // 计算 B * ω
        let result = b * omega;

        // 验证结果
        // 1 T * 2 rad/s = 2 T·rad/s
        assert_relative_eq!(result.x.as_tesla_rad_per_second(), 2.0, epsilon = 1e-12);
        // 100 G * 2 rad/s = 100 * 1e-4 T * 2 rad/s = 0.02 T·rad/s
        assert_relative_eq!(result.y.as_tesla_rad_per_second(), 0.02, epsilon = 1e-12);
        // 500 mT * 2 rad/s = 0.5 T * 2 rad/s = 1.0 T·rad/s
        assert_relative_eq!(result.z.as_tesla_rad_per_second(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn test_vector3_magnetic_induction_mul_angular_velocity_different_units() {
        // 测试不同单位的磁感应强度
        let b = Vector3::new(
            MagneticInduction::from_micro_tesla(1000000.0), // 1 T
            MagneticInduction::from_kilo_gauss(1.0),        // 0.1 T
            MagneticInduction::from_nano_tesla(1000000000.0), // 1 T
        );

        let omega = AngularVelocity::from_deg_per_second(180.0); // π rad/s

        let result = b * omega;

        // 验证结果
        // 1 T * π rad/s = π T·rad/s
        assert_relative_eq!(result.x.as_tesla_rad_per_second(), std::f64::consts::PI, epsilon = 1e-12);
        // 0.1 T * π rad/s = 0.1π T·rad/s
        assert_relative_eq!(result.y.as_tesla_rad_per_second(), 0.1 * std::f64::consts::PI, epsilon = 1e-12);
        // 1 T * π rad/s = π T·rad/s
        assert_relative_eq!(result.z.as_tesla_rad_per_second(), std::f64::consts::PI, epsilon = 1e-12);
    }

    #[test]
    fn test_vector3_magnetic_induction_mul_angular_velocity_zero() {
        // 测试零值
        let b = Vector3::new(
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
        );

        let omega = AngularVelocity::from_rad_per_second(5.0);

        let result = b * omega;

        // 验证结果为零
        assert_relative_eq!(result.x.as_tesla_rad_per_second(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(result.y.as_tesla_rad_per_second(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(result.z.as_tesla_rad_per_second(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn test_vector3_magnetic_induction_mul_angular_velocity_negative() {
        // 测试负值
        let b = Vector3::new(
            MagneticInduction::from_tesla(-2.0),
            MagneticInduction::from_tesla(3.0),
            MagneticInduction::from_tesla(-1.5),
        );

        let omega = AngularVelocity::from_rad_per_second(-1.5);

        let result = b * omega;

        // 验证结果
        // -2 T * (-1.5) rad/s = 3 T·rad/s
        assert_relative_eq!(result.x.as_tesla_rad_per_second(), 3.0, epsilon = 1e-12);
        // 3 T * (-1.5) rad/s = -4.5 T·rad/s
        assert_relative_eq!(result.y.as_tesla_rad_per_second(), -4.5, epsilon = 1e-12);
        // -1.5 T * (-1.5) rad/s = 2.25 T·rad/s
        assert_relative_eq!(result.z.as_tesla_rad_per_second(), 2.25, epsilon = 1e-12);
    }

    #[test]
    fn test_vector3_magnetic_induction_mul_angular_velocity_mixed_units() {
        // 测试混合单位
        let b = Vector3::new(
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_mill_gauss(1000.0), // 1 G = 1e-3 mG
            MagneticInduction::from_micro_tesla(1000000.0), // 1 T
        );

        let omega = AngularVelocity::from_rad_per_hour(3600.0); // 1 rad/s

        let result = b * omega;

        // 验证结果
        // 1 T * 1 rad/s = 1 T·rad/s
        assert_relative_eq!(result.x.as_tesla_rad_per_second(), 1.0, epsilon = 1e-12);
        // 1000 mG * 1 rad/s = 1 G * 1 rad/s = 1e-4 T * 1 rad/s = 1e-4 T·rad/s
        assert_relative_eq!(result.y.as_tesla_rad_per_second(), 1e-4, epsilon = 1e-12);
        // 1 T * 1 rad/s = 1 T·rad/s
        assert_relative_eq!(result.z.as_tesla_rad_per_second(), 1.0, epsilon = 1e-12);
    }
}