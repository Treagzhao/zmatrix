use crate::physics::basic::{
    MagneticMoment, MagneticMomentType, Vector3, Coef,
};

impl Vector3<MagneticMoment> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

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
}
