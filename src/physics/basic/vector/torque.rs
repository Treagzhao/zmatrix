use crate::physics::basic::{
    Torque, TorqueType, Vector3, Coef,
};

impl Vector3<Torque> {
    pub fn to_vector3_coef(&self, torque_type: TorqueType) -> Vector3<Coef> {
        match torque_type {
            TorqueType::NM => {
                let (x, y, z) = (self.x.as_nm(), self.y.as_nm(), self.z.as_nm());
                Vector3::<Coef>::from_array([x, y, z])
            }
            TorqueType::MillNM => {
                let (x, y, z) = (self.x.as_mill_nm(), self.y.as_mill_nm(), self.z.as_mill_nm());
                Vector3::<Coef>::from_array([x, y, z])
            }
            TorqueType::MicroNM => {
                let (x, y, z) = (self.x.as_micro_nm(), self.y.as_micro_nm(), self.z.as_micro_nm());
                Vector3::<Coef>::from_array([x, y, z])
            }
            TorqueType::NanoNM => {
                let (x, y, z) = (self.x.as_nano_nm(), self.y.as_nano_nm(), self.z.as_nano_nm());
                Vector3::<Coef>::from_array([x, y, z])
            }
            TorqueType::KNM => {
                let (x, y, z) = (self.x.as_knm(), self.y.as_knm(), self.z.as_knm());
                Vector3::<Coef>::from_array([x, y, z])
            }
            TorqueType::MNM => {
                let (x, y, z) = (self.x.as_mnm(), self.y.as_mnm(), self.z.as_mnm());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, torque_type: TorqueType) -> Vector3<Torque> {
        let [x, y, z] = coef.to_array();
        match torque_type {
            TorqueType::NM => {
                Vector3::new(
                    Torque::from_nm(x),
                    Torque::from_nm(y),
                    Torque::from_nm(z),
                )
            }
            TorqueType::MillNM => {
                Vector3::new(
                    Torque::from_mill_nm(x),
                    Torque::from_mill_nm(y),
                    Torque::from_mill_nm(z),
                )
            }
            TorqueType::MicroNM => {
                Vector3::new(
                    Torque::from_micro_nm(x),
                    Torque::from_micro_nm(y),
                    Torque::from_micro_nm(z),
                )
            }
            TorqueType::NanoNM => {
                Vector3::new(
                    Torque::from_nano_nm(x),
                    Torque::from_nano_nm(y),
                    Torque::from_nano_nm(z),
                )
            }
            TorqueType::KNM => {
                Vector3::new(
                    Torque::from_knm(x),
                    Torque::from_knm(y),
                    Torque::from_knm(z),
                )
            }
            TorqueType::MNM => {
                Vector3::new(
                    Torque::from_mnm(x),
                    Torque::from_mnm(y),
                    Torque::from_mnm(z),
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
    fn test_to_vector3_coef_nm() {
        let torque_vec = Vector3::new(
            Torque::from_nm(1.0),
            Torque::from_nm(2.0),
            Torque::from_nm(3.0),
        );
        let coef_vec = torque_vec.to_vector3_coef(TorqueType::NM);
        assert_relative_eq!(coef_vec.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z.get_value(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_nm() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let torque_vec = Vector3::<Torque>::from_vector_coef(coef_vec, TorqueType::NM);
        assert_relative_eq!(torque_vec.x.as_nm(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.y.as_nm(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.z.as_nm(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_mill_nm() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let torque_vec = Vector3::<Torque>::from_vector_coef(coef_vec, TorqueType::MillNM);
        assert_relative_eq!(torque_vec.x.as_mill_nm(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.y.as_mill_nm(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.z.as_mill_nm(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_micro_nm() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let torque_vec = Vector3::<Torque>::from_vector_coef(coef_vec, TorqueType::MicroNM);
        assert_relative_eq!(torque_vec.x.as_micro_nm(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.y.as_micro_nm(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.z.as_micro_nm(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_nano_nm() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let torque_vec = Vector3::<Torque>::from_vector_coef(coef_vec, TorqueType::NanoNM);
        assert_relative_eq!(torque_vec.x.as_nano_nm(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.y.as_nano_nm(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.z.as_nano_nm(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_knm() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let torque_vec = Vector3::<Torque>::from_vector_coef(coef_vec, TorqueType::KNM);
        assert_relative_eq!(torque_vec.x.as_knm(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.y.as_knm(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.z.as_knm(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_mnm() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));
        let torque_vec = Vector3::<Torque>::from_vector_coef(coef_vec, TorqueType::MNM);
        assert_relative_eq!(torque_vec.x.as_mnm(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.y.as_mnm(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(torque_vec.z.as_mnm(), 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_torque_comprehensive_as_methods() {
        // 测试所有单位类型之间的转换
        let torque_vec = Vector3::new(
            Torque::from_nm(1.0),
            Torque::from_nm(2.0),
            Torque::from_nm(3.0),
        );

        // 从 NM 转换到所有单位类型
        let coef_nm = torque_vec.to_vector3_coef(TorqueType::NM);
        let coef_mill_nm = torque_vec.to_vector3_coef(TorqueType::MillNM);
        let coef_micro_nm = torque_vec.to_vector3_coef(TorqueType::MicroNM);
        let coef_nano_nm = torque_vec.to_vector3_coef(TorqueType::NanoNM);
        let coef_knm = torque_vec.to_vector3_coef(TorqueType::KNM);
        let coef_mnm = torque_vec.to_vector3_coef(TorqueType::MNM);

        // 验证转换结果
        assert_relative_eq!(coef_nm.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_mill_nm.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_micro_nm.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_nano_nm.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_knm.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_mnm.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 MillNM 转换到所有单位类型
        let mill_nm_vec = Vector3::new(
            Torque::from_mill_nm(1000.0),
            Torque::from_mill_nm(2000.0),
            Torque::from_mill_nm(3000.0),
        );

        let coef_from_mill_nm = mill_nm_vec.to_vector3_coef(TorqueType::NM);
        let coef_from_mill_nm_mill = mill_nm_vec.to_vector3_coef(TorqueType::MillNM);
        let coef_from_mill_nm_micro = mill_nm_vec.to_vector3_coef(TorqueType::MicroNM);
        let coef_from_mill_nm_nano = mill_nm_vec.to_vector3_coef(TorqueType::NanoNM);
        let coef_from_mill_nm_knm = mill_nm_vec.to_vector3_coef(TorqueType::KNM);
        let coef_from_mill_nm_mnm = mill_nm_vec.to_vector3_coef(TorqueType::MNM);

        assert_relative_eq!(coef_from_mill_nm.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nm_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nm_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nm_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nm_knm.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nm_mnm.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 MicroNM 转换到所有单位类型
        let micro_nm_vec = Vector3::new(
            Torque::from_micro_nm(1000000.0),
            Torque::from_micro_nm(2000000.0),
            Torque::from_micro_nm(3000000.0),
        );

        let coef_from_micro_nm = micro_nm_vec.to_vector3_coef(TorqueType::NM);
        let coef_from_micro_nm_mill = micro_nm_vec.to_vector3_coef(TorqueType::MillNM);
        let coef_from_micro_nm_micro = micro_nm_vec.to_vector3_coef(TorqueType::MicroNM);
        let coef_from_micro_nm_nano = micro_nm_vec.to_vector3_coef(TorqueType::NanoNM);
        let coef_from_micro_nm_knm = micro_nm_vec.to_vector3_coef(TorqueType::KNM);
        let coef_from_micro_nm_mnm = micro_nm_vec.to_vector3_coef(TorqueType::MNM);

        assert_relative_eq!(coef_from_micro_nm.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nm_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nm_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nm_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nm_knm.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nm_mnm.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 NanoNM 转换到所有单位类型
        let nano_nm_vec = Vector3::new(
            Torque::from_nano_nm(1000000000.0),
            Torque::from_nano_nm(2000000000.0),
            Torque::from_nano_nm(3000000000.0),
        );

        let coef_from_nano_nm = nano_nm_vec.to_vector3_coef(TorqueType::NM);
        let coef_from_nano_nm_mill = nano_nm_vec.to_vector3_coef(TorqueType::MillNM);
        let coef_from_nano_nm_micro = nano_nm_vec.to_vector3_coef(TorqueType::MicroNM);
        let coef_from_nano_nm_nano = nano_nm_vec.to_vector3_coef(TorqueType::NanoNM);
        let coef_from_nano_nm_knm = nano_nm_vec.to_vector3_coef(TorqueType::KNM);
        let coef_from_nano_nm_mnm = nano_nm_vec.to_vector3_coef(TorqueType::MNM);

        assert_relative_eq!(coef_from_nano_nm.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nm_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nm_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nm_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nm_knm.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nm_mnm.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 KNM 转换到所有单位类型
        let knm_vec = Vector3::new(
            Torque::from_knm(0.001),
            Torque::from_knm(0.002),
            Torque::from_knm(0.003),
        );

        let coef_from_knm = knm_vec.to_vector3_coef(TorqueType::NM);
        let coef_from_knm_mill = knm_vec.to_vector3_coef(TorqueType::MillNM);
        let coef_from_knm_micro = knm_vec.to_vector3_coef(TorqueType::MicroNM);
        let coef_from_knm_nano = knm_vec.to_vector3_coef(TorqueType::NanoNM);
        let coef_from_knm_knm = knm_vec.to_vector3_coef(TorqueType::KNM);
        let coef_from_knm_mnm = knm_vec.to_vector3_coef(TorqueType::MNM);

        assert_relative_eq!(coef_from_knm.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_knm_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_knm_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_knm_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_knm_knm.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_knm_mnm.x.get_value(), 0.000001, epsilon = 1e-10);

        // 从 MNM 转换到所有单位类型
        let mnm_vec = Vector3::new(
            Torque::from_mnm(0.000001),
            Torque::from_mnm(0.000002),
            Torque::from_mnm(0.000003),
        );

        let coef_from_mnm = mnm_vec.to_vector3_coef(TorqueType::NM);
        let coef_from_mnm_mill = mnm_vec.to_vector3_coef(TorqueType::MillNM);
        let coef_from_mnm_micro = mnm_vec.to_vector3_coef(TorqueType::MicroNM);
        let coef_from_mnm_nano = mnm_vec.to_vector3_coef(TorqueType::NanoNM);
        let coef_from_mnm_knm = mnm_vec.to_vector3_coef(TorqueType::KNM);
        let coef_from_mnm_mnm = mnm_vec.to_vector3_coef(TorqueType::MNM);

        assert_relative_eq!(coef_from_mnm.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mnm_mill.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mnm_micro.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mnm_nano.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mnm_knm.x.get_value(), 0.001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mnm_mnm.x.get_value(), 0.000001, epsilon = 1e-10);
    }
}
