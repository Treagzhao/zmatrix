use super::*;
use crate::physics::basic::{
    AngularMomentum, MagneticAngularVelocity, MagneticInduction, MagneticMoment, Vector3, Coef, MagneticAngularVelocityType,
};
const FLOAT_F64_E_6: f64 = 1e-6;

impl Vector3<MagneticAngularVelocity> {
    pub fn to_vector3_coef(&self, magnetic_angular_velocity_type: MagneticAngularVelocityType) -> Vector3<Coef> {
        match magnetic_angular_velocity_type {
            MagneticAngularVelocityType::TeslaRadPerSecond => {
                let (x, y, z) = (self.x.as_tesla_rad_per_second(), self.y.as_tesla_rad_per_second(), self.z.as_tesla_rad_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticAngularVelocityType::MillTeslaRadPerSecond => {
                let (x, y, z) = (self.x.as_mill_tesla_rad_per_second(), self.y.as_mill_tesla_rad_per_second(), self.z.as_mill_tesla_rad_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticAngularVelocityType::MicroTeslaRadPerSecond => {
                let (x, y, z) = (self.x.as_micro_tesla_rad_per_second(), self.y.as_micro_tesla_rad_per_second(), self.z.as_micro_tesla_rad_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticAngularVelocityType::NanoTeslaRadPerSecond => {
                let (x, y, z) = (self.x.as_nano_tesla_rad_per_second(), self.y.as_nano_tesla_rad_per_second(), self.z.as_nano_tesla_rad_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticAngularVelocityType::GaussRadPerSecond => {
                let (x, y, z) = (self.x.as_gauss_rad_per_second(), self.y.as_gauss_rad_per_second(), self.z.as_gauss_rad_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticAngularVelocityType::MillGaussRadPerSecond => {
                let (x, y, z) = (self.x.as_mill_gauss_rad_per_second(), self.y.as_mill_gauss_rad_per_second(), self.z.as_mill_gauss_rad_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticAngularVelocityType::KiloGaussRadPerSecond => {
                let (x, y, z) = (self.x.as_kilo_gauss_rad_per_second(), self.y.as_kilo_gauss_rad_per_second(), self.z.as_kilo_gauss_rad_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, magnetic_angular_velocity_type: MagneticAngularVelocityType) -> Vector3<MagneticAngularVelocity> {
        let [x, y, z] = coef.to_array();
        match magnetic_angular_velocity_type {
            MagneticAngularVelocityType::TeslaRadPerSecond => {
                Vector3::new(
                    MagneticAngularVelocity::from_tesla_rad_per_second(x),
                    MagneticAngularVelocity::from_tesla_rad_per_second(y),
                    MagneticAngularVelocity::from_tesla_rad_per_second(z),
                )
            }
            MagneticAngularVelocityType::MillTeslaRadPerSecond => {
                Vector3::new(
                    MagneticAngularVelocity::from_mill_tesla_rad_per_second(x),
                    MagneticAngularVelocity::from_mill_tesla_rad_per_second(y),
                    MagneticAngularVelocity::from_mill_tesla_rad_per_second(z),
                )
            }
            MagneticAngularVelocityType::MicroTeslaRadPerSecond => {
                Vector3::new(
                    MagneticAngularVelocity::from_micro_tesla_rad_per_second(x),
                    MagneticAngularVelocity::from_micro_tesla_rad_per_second(y),
                    MagneticAngularVelocity::from_micro_tesla_rad_per_second(z),
                )
            }
            MagneticAngularVelocityType::NanoTeslaRadPerSecond => {
                Vector3::new(
                    MagneticAngularVelocity::from_nano_tesla_rad_per_second(x),
                    MagneticAngularVelocity::from_nano_tesla_rad_per_second(y),
                    MagneticAngularVelocity::from_nano_tesla_rad_per_second(z),
                )
            }
            MagneticAngularVelocityType::GaussRadPerSecond => {
                Vector3::new(
                    MagneticAngularVelocity::from_gauss_rad_per_second(x),
                    MagneticAngularVelocity::from_gauss_rad_per_second(y),
                    MagneticAngularVelocity::from_gauss_rad_per_second(z),
                )
            }
            MagneticAngularVelocityType::MillGaussRadPerSecond => {
                Vector3::new(
                    MagneticAngularVelocity::from_mill_gauss_rad_per_second(x),
                    MagneticAngularVelocity::from_mill_gauss_rad_per_second(y),
                    MagneticAngularVelocity::from_mill_gauss_rad_per_second(z),
                )
            }
            MagneticAngularVelocityType::KiloGaussRadPerSecond => {
                Vector3::new(
                    MagneticAngularVelocity::from_kilo_gauss_rad_per_second(x),
                    MagneticAngularVelocity::from_kilo_gauss_rad_per_second(y),
                    MagneticAngularVelocity::from_kilo_gauss_rad_per_second(z),
                )
            }
        }
    }

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
    fn test_to_vector3_coef_tesla_rad_per_second() {
        let magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_tesla_rad_per_second(1.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(2.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(3.0),
        );
        let coef_vec = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_mill_tesla_rad_per_second() {
        let magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_mill_tesla_rad_per_second(1000.0),
            MagneticAngularVelocity::from_mill_tesla_rad_per_second(2000.0),
            MagneticAngularVelocity::from_mill_tesla_rad_per_second(3000.0),
        );
        let coef_vec = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::MillTeslaRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_micro_tesla_rad_per_second() {
        let magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_micro_tesla_rad_per_second(1000000.0),
            MagneticAngularVelocity::from_micro_tesla_rad_per_second(2000000.0),
            MagneticAngularVelocity::from_micro_tesla_rad_per_second(3000000.0),
        );
        let coef_vec = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::MicroTeslaRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_nano_tesla_rad_per_second() {
        let magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(1000000000.0),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(2000000000.0),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(3000000000.0),
        );
        let coef_vec = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3000000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_gauss_rad_per_second() {
        let magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_gauss_rad_per_second(10000.0),
            MagneticAngularVelocity::from_gauss_rad_per_second(20000.0),
            MagneticAngularVelocity::from_gauss_rad_per_second(30000.0),
        );
        let coef_vec = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::GaussRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 10000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 20000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 30000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_mill_gauss_rad_per_second() {
        let magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_mill_gauss_rad_per_second(10000000.0),
            MagneticAngularVelocity::from_mill_gauss_rad_per_second(20000000.0),
            MagneticAngularVelocity::from_mill_gauss_rad_per_second(30000000.0),
        );
        let coef_vec = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::MillGaussRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 10000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 20000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 30000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_kilo_gauss_rad_per_second() {
        let magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_kilo_gauss_rad_per_second(10.0),
            MagneticAngularVelocity::from_kilo_gauss_rad_per_second(20.0),
            MagneticAngularVelocity::from_kilo_gauss_rad_per_second(30.0),
        );
        let coef_vec = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::KiloGaussRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 10.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 20.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 30.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_tesla_rad_per_second() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let magnetic_angular_velocity_vec = Vector3::<MagneticAngularVelocity>::from_vector_coef(coef_vec, MagneticAngularVelocityType::TeslaRadPerSecond);
        assert_relative_eq!(magnetic_angular_velocity_vec.x.as_tesla_rad_per_second(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.y.as_tesla_rad_per_second(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.z.as_tesla_rad_per_second(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_mill_tesla_rad_per_second() {
        let coef_vec = Vector3::new(Coef::new(1000.0), Coef::new(2000.0), Coef::new(3000.0));
        let magnetic_angular_velocity_vec = Vector3::<MagneticAngularVelocity>::from_vector_coef(coef_vec, MagneticAngularVelocityType::MillTeslaRadPerSecond);
        assert_relative_eq!(magnetic_angular_velocity_vec.x.as_mill_tesla_rad_per_second(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.y.as_mill_tesla_rad_per_second(), 2000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.z.as_mill_tesla_rad_per_second(), 3000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_micro_tesla_rad_per_second() {
        let coef_vec = Vector3::new(Coef::new(1000000.0), Coef::new(2000000.0), Coef::new(3000000.0));
        let magnetic_angular_velocity_vec = Vector3::<MagneticAngularVelocity>::from_vector_coef(coef_vec, MagneticAngularVelocityType::MicroTeslaRadPerSecond);
        assert_relative_eq!(magnetic_angular_velocity_vec.x.as_micro_tesla_rad_per_second(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.y.as_micro_tesla_rad_per_second(), 2000000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.z.as_micro_tesla_rad_per_second(), 3000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_nano_tesla_rad_per_second() {
        let coef_vec = Vector3::new(Coef::new(1000000000.0), Coef::new(2000000000.0), Coef::new(3000000000.0));
        let magnetic_angular_velocity_vec = Vector3::<MagneticAngularVelocity>::from_vector_coef(coef_vec, MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        assert_relative_eq!(magnetic_angular_velocity_vec.x.as_nano_tesla_rad_per_second(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.y.as_nano_tesla_rad_per_second(), 2000000000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.z.as_nano_tesla_rad_per_second(), 3000000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_gauss_rad_per_second() {
        let coef_vec = Vector3::new(Coef::new(10000.0), Coef::new(20000.0), Coef::new(30000.0));
        let magnetic_angular_velocity_vec = Vector3::<MagneticAngularVelocity>::from_vector_coef(coef_vec, MagneticAngularVelocityType::GaussRadPerSecond);
        assert_relative_eq!(magnetic_angular_velocity_vec.x.as_gauss_rad_per_second(), 10000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.y.as_gauss_rad_per_second(), 20000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.z.as_gauss_rad_per_second(), 30000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_mill_gauss_rad_per_second() {
        let coef_vec = Vector3::new(Coef::new(10000000.0), Coef::new(20000000.0), Coef::new(30000000.0));
        let magnetic_angular_velocity_vec = Vector3::<MagneticAngularVelocity>::from_vector_coef(coef_vec, MagneticAngularVelocityType::MillGaussRadPerSecond);
        assert_relative_eq!(magnetic_angular_velocity_vec.x.as_mill_gauss_rad_per_second(), 10000000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.y.as_mill_gauss_rad_per_second(), 20000000.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.z.as_mill_gauss_rad_per_second(), 30000000.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_kilo_gauss_rad_per_second() {
        let coef_vec = Vector3::new(Coef::new(10.0), Coef::new(20.0), Coef::new(30.0));
        let magnetic_angular_velocity_vec = Vector3::<MagneticAngularVelocity>::from_vector_coef(coef_vec, MagneticAngularVelocityType::KiloGaussRadPerSecond);
        assert_relative_eq!(magnetic_angular_velocity_vec.x.as_kilo_gauss_rad_per_second(), 10.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.y.as_kilo_gauss_rad_per_second(), 20.0, epsilon = 1e-10);
        assert_relative_eq!(magnetic_angular_velocity_vec.z.as_kilo_gauss_rad_per_second(), 30.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_tesla_rad_per_second(1.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(2.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(3.0),
        );
        let coef_vec = original_magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        let reconstructed_magnetic_angular_velocity_vec = Vector3::<MagneticAngularVelocity>::from_vector_coef(coef_vec, MagneticAngularVelocityType::TeslaRadPerSecond);
        assert_relative_eq!(reconstructed_magnetic_angular_velocity_vec.x.as_tesla_rad_per_second(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(reconstructed_magnetic_angular_velocity_vec.y.as_tesla_rad_per_second(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(reconstructed_magnetic_angular_velocity_vec.z.as_tesla_rad_per_second(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_magnetic_angular_velocity_comprehensive_as_methods() {
        // 测试所有单位类型之间的转换
        let magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_tesla_rad_per_second(1.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(2.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(3.0),
        );

        // 从 TeslaRadPerSecond 转换到所有单位类型
        let coef_tesla = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        let coef_mill_tesla = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::MillTeslaRadPerSecond);
        let coef_micro_tesla = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::MicroTeslaRadPerSecond);
        let coef_nano_tesla = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        let coef_gauss = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::GaussRadPerSecond);
        let coef_mill_gauss = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::MillGaussRadPerSecond);
        let coef_kilo_gauss = magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::KiloGaussRadPerSecond);

        // 验证转换结果
        assert_relative_eq!(coef_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_mill_tesla.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_micro_tesla.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_nano_tesla.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_gauss.x.get_value(), 10000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_mill_gauss.x.get_value(), 10000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_kilo_gauss.x.get_value(), 10.0, epsilon = 1e-10);

        // 从 MillTeslaRadPerSecond 转换到所有单位类型
        let mill_tesla_vec = Vector3::new(
            MagneticAngularVelocity::from_mill_tesla_rad_per_second(1000.0),
            MagneticAngularVelocity::from_mill_tesla_rad_per_second(2000.0),
            MagneticAngularVelocity::from_mill_tesla_rad_per_second(3000.0),
        );

        let coef_from_mill_tesla = mill_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        let coef_from_mill_tesla_mill = mill_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::MillTeslaRadPerSecond);
        let coef_from_mill_tesla_micro = mill_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::MicroTeslaRadPerSecond);
        let coef_from_mill_tesla_nano = mill_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        let coef_from_mill_tesla_gauss = mill_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::GaussRadPerSecond);
        let coef_from_mill_tesla_mill_gauss = mill_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::MillGaussRadPerSecond);
        let coef_from_mill_tesla_kilo_gauss = mill_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::KiloGaussRadPerSecond);

        assert_relative_eq!(coef_from_mill_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_tesla_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_tesla_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_tesla_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_tesla_gauss.x.get_value(), 10000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_tesla_mill_gauss.x.get_value(), 10000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_tesla_kilo_gauss.x.get_value(), 10.0, epsilon = 1e-10);

        // 从 MicroTeslaRadPerSecond 转换到所有单位类型
        let micro_tesla_vec = Vector3::new(
            MagneticAngularVelocity::from_micro_tesla_rad_per_second(1000000.0),
            MagneticAngularVelocity::from_micro_tesla_rad_per_second(2000000.0),
            MagneticAngularVelocity::from_micro_tesla_rad_per_second(3000000.0),
        );

        let coef_from_micro_tesla = micro_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        let coef_from_micro_tesla_mill = micro_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::MillTeslaRadPerSecond);
        let coef_from_micro_tesla_micro = micro_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::MicroTeslaRadPerSecond);
        let coef_from_micro_tesla_nano = micro_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        let coef_from_micro_tesla_gauss = micro_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::GaussRadPerSecond);
        let coef_from_micro_tesla_mill_gauss = micro_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::MillGaussRadPerSecond);
        let coef_from_micro_tesla_kilo_gauss = micro_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::KiloGaussRadPerSecond);

        assert_relative_eq!(coef_from_micro_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_tesla_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_tesla_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_tesla_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_tesla_gauss.x.get_value(), 10000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_tesla_mill_gauss.x.get_value(), 10000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_tesla_kilo_gauss.x.get_value(), 10.0, epsilon = 1e-10);

        // 从 NanoTeslaRadPerSecond 转换到所有单位类型
        let nano_tesla_vec = Vector3::new(
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(1000000000.0),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(2000000000.0),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(3000000000.0),
        );

        let coef_from_nano_tesla = nano_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        let coef_from_nano_tesla_mill = nano_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::MillTeslaRadPerSecond);
        let coef_from_nano_tesla_micro = nano_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::MicroTeslaRadPerSecond);
        let coef_from_nano_tesla_nano = nano_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        let coef_from_nano_tesla_gauss = nano_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::GaussRadPerSecond);
        let coef_from_nano_tesla_mill_gauss = nano_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::MillGaussRadPerSecond);
        let coef_from_nano_tesla_kilo_gauss = nano_tesla_vec.to_vector3_coef(MagneticAngularVelocityType::KiloGaussRadPerSecond);

        assert_relative_eq!(coef_from_nano_tesla.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_tesla_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_tesla_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_tesla_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_tesla_gauss.x.get_value(), 10000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_tesla_mill_gauss.x.get_value(), 10000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_tesla_kilo_gauss.x.get_value(), 10.0, epsilon = 1e-10);

        // 从 GaussRadPerSecond 转换到所有单位类型
        let gauss_vec = Vector3::new(
            MagneticAngularVelocity::from_gauss_rad_per_second(10000.0),
            MagneticAngularVelocity::from_gauss_rad_per_second(20000.0),
            MagneticAngularVelocity::from_gauss_rad_per_second(30000.0),
        );

        let coef_from_gauss = gauss_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        let coef_from_gauss_mill = gauss_vec.to_vector3_coef(MagneticAngularVelocityType::MillTeslaRadPerSecond);
        let coef_from_gauss_micro = gauss_vec.to_vector3_coef(MagneticAngularVelocityType::MicroTeslaRadPerSecond);
        let coef_from_gauss_nano = gauss_vec.to_vector3_coef(MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        let coef_from_gauss_gauss = gauss_vec.to_vector3_coef(MagneticAngularVelocityType::GaussRadPerSecond);
        let coef_from_gauss_mill_gauss = gauss_vec.to_vector3_coef(MagneticAngularVelocityType::MillGaussRadPerSecond);
        let coef_from_gauss_kilo_gauss = gauss_vec.to_vector3_coef(MagneticAngularVelocityType::KiloGaussRadPerSecond);

        assert_relative_eq!(coef_from_gauss.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_gauss_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_gauss_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_gauss_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_gauss_gauss.x.get_value(), 10000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_gauss_mill_gauss.x.get_value(), 10000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_gauss_kilo_gauss.x.get_value(), 10.0, epsilon = 1e-10);

        // 从 MillGaussRadPerSecond 转换到所有单位类型
        let mill_gauss_vec = Vector3::new(
            MagneticAngularVelocity::from_mill_gauss_rad_per_second(10000000.0),
            MagneticAngularVelocity::from_mill_gauss_rad_per_second(20000000.0),
            MagneticAngularVelocity::from_mill_gauss_rad_per_second(30000000.0),
        );

        let coef_from_mill_gauss = mill_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        let coef_from_mill_gauss_mill = mill_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::MillTeslaRadPerSecond);
        let coef_from_mill_gauss_micro = mill_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::MicroTeslaRadPerSecond);
        let coef_from_mill_gauss_nano = mill_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        let coef_from_mill_gauss_gauss = mill_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::GaussRadPerSecond);
        let coef_from_mill_gauss_mill_gauss = mill_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::MillGaussRadPerSecond);
        let coef_from_mill_gauss_kilo_gauss = mill_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::KiloGaussRadPerSecond);

        assert_relative_eq!(coef_from_mill_gauss.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_gauss_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_gauss_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_gauss_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_gauss_gauss.x.get_value(), 10000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_gauss_mill_gauss.x.get_value(), 10000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_gauss_kilo_gauss.x.get_value(), 10.0, epsilon = 1e-10);

        // 从 KiloGaussRadPerSecond 转换到所有单位类型
        let kilo_gauss_vec = Vector3::new(
            MagneticAngularVelocity::from_kilo_gauss_rad_per_second(10.0),
            MagneticAngularVelocity::from_kilo_gauss_rad_per_second(20.0),
            MagneticAngularVelocity::from_kilo_gauss_rad_per_second(30.0),
        );

        let coef_from_kilo_gauss = kilo_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        let coef_from_kilo_gauss_mill = kilo_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::MillTeslaRadPerSecond);
        let coef_from_kilo_gauss_micro = kilo_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::MicroTeslaRadPerSecond);
        let coef_from_kilo_gauss_nano = kilo_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        let coef_from_kilo_gauss_gauss = kilo_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::GaussRadPerSecond);
        let coef_from_kilo_gauss_mill_gauss = kilo_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::MillGaussRadPerSecond);
        let coef_from_kilo_gauss_kilo_gauss = kilo_gauss_vec.to_vector3_coef(MagneticAngularVelocityType::KiloGaussRadPerSecond);

        assert_relative_eq!(coef_from_kilo_gauss.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_gauss_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_gauss_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_gauss_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_gauss_gauss.x.get_value(), 10000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_gauss_mill_gauss.x.get_value(), 10000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_gauss_kilo_gauss.x.get_value(), 10.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_zero_values() {
        let zero_magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_tesla_rad_per_second(0.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(0.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(0.0),
        );
        let coef_vec = zero_magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 0.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 0.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_negative_values() {
        let negative_magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_tesla_rad_per_second(-1.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(-2.0),
            MagneticAngularVelocity::from_tesla_rad_per_second(-3.0),
        );
        let coef_vec = negative_magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::TeslaRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), -1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), -2.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), -3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_to_vector3_coef_small_values() {
        let small_magnetic_angular_velocity_vec = Vector3::new(
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(1e-9),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(2e-9),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(3e-9),
        );
        let coef_vec = small_magnetic_angular_velocity_vec.to_vector3_coef(MagneticAngularVelocityType::NanoTeslaRadPerSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 1e-9, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2e-9, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3e-9, epsilon = 1e-10);
    }

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
        let k_nms = AngularMomentum::from_nms(1.0);
        let m_nms = w.to_magnetic_moment(&k_nms, &b);
        assert_relative_eq!(m_nms.x.as_am2(), 1.0 / 3.0, epsilon = 1e-12);
        assert_relative_eq!(m_nms.y.as_am2(), 1.0 / 3.0, epsilon = 1e-12);
        assert_relative_eq!(m_nms.z.as_am2(), 1.0 / 3.0, epsilon = 1e-12);
        
        let k_mill_nms = AngularMomentum::from_mill_nms(1000.0); // 1 N·m·s
        let m_mill_nms = w.to_magnetic_moment(&k_mill_nms, &b);
        assert_relative_eq!(m_mill_nms.x.as_am2(), 1.0 / 3.0, epsilon = 1e-12);
        assert_relative_eq!(m_mill_nms.y.as_am2(), 1.0 / 3.0, epsilon = 1e-12);
        assert_relative_eq!(m_mill_nms.z.as_am2(), 1.0 / 3.0, epsilon = 1e-12);
    }
}
