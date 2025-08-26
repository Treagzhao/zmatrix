use crate::physics::basic::{
    Force, ForceType, Vector3, Coef,
};

impl Vector3<Force> {
    pub fn to_vector3_coef(&self, force_type: ForceType) -> Vector3<Coef> {
        match force_type {
            ForceType::Newton => {
                let (x, y, z) = (self.x.as_newton(), self.y.as_newton(), self.z.as_newton());
                Vector3::<Coef>::from_array([x, y, z])
            }
            ForceType::MillNewton => {
                let (x, y, z) = (self.x.as_mill_newton(), self.y.as_mill_newton(), self.z.as_mill_newton());
                Vector3::<Coef>::from_array([x, y, z])
            }
            ForceType::MicroNewton => {
                let (x, y, z) = (self.x.as_micro_newton(), self.y.as_micro_newton(), self.z.as_micro_newton());
                Vector3::<Coef>::from_array([x, y, z])
            }
            ForceType::NanoNewton => {
                let (x, y, z) = (self.x.as_nano_newton(), self.y.as_nano_newton(), self.z.as_nano_newton());
                Vector3::<Coef>::from_array([x, y, z])
            }
            ForceType::KiloNewton => {
                let (x, y, z) = (self.x.as_kilo_newton(), self.y.as_kilo_newton(), self.z.as_kilo_newton());
                Vector3::<Coef>::from_array([x, y, z])
            }
            ForceType::MegaNewton => {
                let (x, y, z) = (self.x.as_mega_newton(), self.y.as_mega_newton(), self.z.as_mega_newton());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, force_type: ForceType) -> Vector3<Force> {
        let [x, y, z] = coef.to_array();
        match force_type {
            ForceType::Newton => {
                Vector3::new(
                    Force::from_newton(x),
                    Force::from_newton(y),
                    Force::from_newton(z),
                )
            }
            ForceType::MillNewton => {
                Vector3::new(
                    Force::from_mill_newton(x),
                    Force::from_mill_newton(y),
                    Force::from_mill_newton(z),
                )
            }
            ForceType::MicroNewton => {
                Vector3::new(
                    Force::from_micro_newton(x),
                    Force::from_micro_newton(y),
                    Force::from_micro_newton(z),
                )
            }
            ForceType::NanoNewton => {
                Vector3::new(
                    Force::from_nano_newton(x),
                    Force::from_nano_newton(y),
                    Force::from_nano_newton(z),
                )
            }
            ForceType::KiloNewton => {
                Vector3::new(
                    Force::from_kilo_newton(x),
                    Force::from_kilo_newton(y),
                    Force::from_kilo_newton(z),
                )
            }
            ForceType::MegaNewton => {
                Vector3::new(
                    Force::from_mega_newton(x),
                    Force::from_mega_newton(y),
                    Force::from_mega_newton(z),
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
    fn test_to_vector3_coef_newton() {
        let force_vec = Vector3::new(
            Force::from_newton(1.0),
            Force::from_newton(2.0),
            Force::from_newton(3.0),
        );
        let coef_vec = force_vec.to_vector3_coef(ForceType::Newton);
        assert_relative_eq!(coef_vec.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_newton() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let force_vec = Vector3::<Force>::from_vector_coef(coef_vec, ForceType::Newton);
        assert_relative_eq!(force_vec.x.as_newton(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.y.as_newton(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.z.as_newton(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_mill_newton() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let force_vec = Vector3::<Force>::from_vector_coef(coef_vec, ForceType::MillNewton);
        assert_relative_eq!(force_vec.x.as_mill_newton(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.y.as_mill_newton(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.z.as_mill_newton(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_micro_newton() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let force_vec = Vector3::<Force>::from_vector_coef(coef_vec, ForceType::MicroNewton);
        assert_relative_eq!(force_vec.x.as_micro_newton(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.y.as_micro_newton(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.z.as_micro_newton(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_nano_newton() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let force_vec = Vector3::<Force>::from_vector_coef(coef_vec, ForceType::NanoNewton);
        assert_relative_eq!(force_vec.x.as_nano_newton(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.y.as_nano_newton(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.z.as_nano_newton(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_kilo_newton() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let force_vec = Vector3::<Force>::from_vector_coef(coef_vec, ForceType::KiloNewton);
        assert_relative_eq!(force_vec.x.as_kilo_newton(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.y.as_kilo_newton(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.z.as_kilo_newton(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_mega_newton() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let force_vec = Vector3::<Force>::from_vector_coef(coef_vec, ForceType::MegaNewton);
        assert_relative_eq!(force_vec.x.as_mega_newton(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.y.as_mega_newton(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(force_vec.z.as_mega_newton(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_force_comprehensive_as_methods() {
        // 测试所有单位类型之间的转换
        let force_vec = Vector3::new(
            Force::from_newton(1.0),
            Force::from_newton(2.0),
            Force::from_newton(3.0),
        );

        // 从 Newton 转换到所有单位类型
        let coef_newton = force_vec.to_vector3_coef(ForceType::Newton);
        let coef_mill_newton = force_vec.to_vector3_coef(ForceType::MillNewton);
        let coef_micro_newton = force_vec.to_vector3_coef(ForceType::MicroNewton);
        let coef_nano_newton = force_vec.to_vector3_coef(ForceType::NanoNewton);
        let coef_kilo_newton = force_vec.to_vector3_coef(ForceType::KiloNewton);
        let coef_mega_newton = force_vec.to_vector3_coef(ForceType::MegaNewton);

        // 验证转换结果
        assert_relative_eq!(coef_newton.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_mill_newton.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_micro_newton.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_nano_newton.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_kilo_newton.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_mega_newton.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 MillNewton 转换到所有单位类型
        let mill_newton_vec = Vector3::new(
            Force::from_mill_newton(1000.0),
            Force::from_mill_newton(2000.0),
            Force::from_mill_newton(3000.0),
        );

        let coef_from_mill_newton = mill_newton_vec.to_vector3_coef(ForceType::Newton);
        let coef_from_mill_newton_mill = mill_newton_vec.to_vector3_coef(ForceType::MillNewton);
        let coef_from_mill_newton_micro = mill_newton_vec.to_vector3_coef(ForceType::MicroNewton);
        let coef_from_mill_newton_nano = mill_newton_vec.to_vector3_coef(ForceType::NanoNewton);
        let coef_from_mill_newton_kilo = mill_newton_vec.to_vector3_coef(ForceType::KiloNewton);
        let coef_from_mill_newton_mega = mill_newton_vec.to_vector3_coef(ForceType::MegaNewton);

        assert_relative_eq!(coef_from_mill_newton.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_newton_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_newton_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_newton_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_newton_kilo.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_newton_mega.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 MicroNewton 转换到所有单位类型
        let micro_newton_vec = Vector3::new(
            Force::from_micro_newton(1000000.0),
            Force::from_micro_newton(2000000.0),
            Force::from_micro_newton(3000000.0),
        );

        let coef_from_micro_newton = micro_newton_vec.to_vector3_coef(ForceType::Newton);
        let coef_from_micro_newton_mill = micro_newton_vec.to_vector3_coef(ForceType::MillNewton);
        let coef_from_micro_newton_micro = micro_newton_vec.to_vector3_coef(ForceType::MicroNewton);
        let coef_from_micro_newton_nano = micro_newton_vec.to_vector3_coef(ForceType::NanoNewton);
        let coef_from_micro_newton_kilo = micro_newton_vec.to_vector3_coef(ForceType::KiloNewton);
        let coef_from_micro_newton_mega = micro_newton_vec.to_vector3_coef(ForceType::MegaNewton);

        assert_relative_eq!(coef_from_micro_newton.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_newton_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_newton_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_newton_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_newton_kilo.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_newton_mega.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 NanoNewton 转换到所有单位类型
        let nano_newton_vec = Vector3::new(
            Force::from_nano_newton(1000000000.0),
            Force::from_nano_newton(2000000000.0),
            Force::from_nano_newton(3000000000.0),
        );

        let coef_from_nano_newton = nano_newton_vec.to_vector3_coef(ForceType::Newton);
        let coef_from_nano_newton_mill = nano_newton_vec.to_vector3_coef(ForceType::MillNewton);
        let coef_from_nano_newton_micro = nano_newton_vec.to_vector3_coef(ForceType::MicroNewton);
        let coef_from_nano_newton_nano = nano_newton_vec.to_vector3_coef(ForceType::NanoNewton);
        let coef_from_nano_newton_kilo = nano_newton_vec.to_vector3_coef(ForceType::KiloNewton);
        let coef_from_nano_newton_mega = nano_newton_vec.to_vector3_coef(ForceType::MegaNewton);

        assert_relative_eq!(coef_from_nano_newton.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_newton_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_newton_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_newton_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_newton_kilo.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_newton_mega.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 KiloNewton 转换到所有单位类型
        let kilo_newton_vec = Vector3::new(
            Force::from_kilo_newton(0.001),
            Force::from_kilo_newton(0.002),
            Force::from_kilo_newton(0.003),
        );

        let coef_from_kilo_newton = kilo_newton_vec.to_vector3_coef(ForceType::Newton);
        let coef_from_kilo_newton_mill = kilo_newton_vec.to_vector3_coef(ForceType::MillNewton);
        let coef_from_kilo_newton_micro = kilo_newton_vec.to_vector3_coef(ForceType::MicroNewton);
        let coef_from_kilo_newton_nano = kilo_newton_vec.to_vector3_coef(ForceType::NanoNewton);
        let coef_from_kilo_newton_kilo = kilo_newton_vec.to_vector3_coef(ForceType::KiloNewton);
        let coef_from_kilo_newton_mega = kilo_newton_vec.to_vector3_coef(ForceType::MegaNewton);

        assert_relative_eq!(coef_from_kilo_newton.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_newton_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_newton_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_newton_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_newton_kilo.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kilo_newton_mega.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 MegaNewton 转换到所有单位类型
        let mega_newton_vec = Vector3::new(
            Force::from_mega_newton(0.000001),
            Force::from_mega_newton(0.000002),
            Force::from_mega_newton(0.000003),
        );

        let coef_from_mega_newton = mega_newton_vec.to_vector3_coef(ForceType::Newton);
        let coef_from_mega_newton_mill = mega_newton_vec.to_vector3_coef(ForceType::MillNewton);
        let coef_from_mega_newton_micro = mega_newton_vec.to_vector3_coef(ForceType::MicroNewton);
        let coef_from_mega_newton_nano = mega_newton_vec.to_vector3_coef(ForceType::NanoNewton);
        let coef_from_mega_newton_kilo = mega_newton_vec.to_vector3_coef(ForceType::KiloNewton);
        let coef_from_mega_newton_mega = mega_newton_vec.to_vector3_coef(ForceType::MegaNewton);

        assert_relative_eq!(coef_from_mega_newton.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mega_newton_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mega_newton_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mega_newton_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mega_newton_kilo.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mega_newton_mega.x.get_value(), 0.000001, epsilon = 1e-10);
    }
}
