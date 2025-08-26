use crate::physics::basic::{AngularMomentum, AngularMomentumType, Coef, Vector3};

impl Vector3<AngularMomentum> {
    pub fn to_vector3_coef(&self, angular_momentum_type: AngularMomentumType) -> Vector3<Coef> {
        match angular_momentum_type {
            AngularMomentumType::KgM2perSecond => {
                let (x, y, z) = (self.x.as_kg_m2_per_second(), self.y.as_kg_m2_per_second(), self.z.as_kg_m2_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularMomentumType::KgKm2perSecond => {
                let (x, y, z) = (self.x.as_kg_km2_per_second(), self.y.as_kg_km2_per_second(), self.z.as_kg_km2_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularMomentumType::Nms => {
                let (x, y, z) = (self.x.as_nms(), self.y.as_nms(), self.z.as_nms());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularMomentumType::MillNms => {
                let (x, y, z) = (self.x.as_mill_nms(), self.y.as_mill_nms(), self.z.as_mill_nms());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularMomentumType::MicroNms => {
                let (x, y, z) = (self.x.as_micro_nms(), self.y.as_micro_nms(), self.z.as_micro_nms());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularMomentumType::NanoNms => {
                let (x, y, z) = (self.x.as_nano_nms(), self.y.as_nano_nms(), self.z.as_nano_nms());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, angular_momentum_type: AngularMomentumType) -> Vector3<AngularMomentum> {
        let x = AngularMomentum {
            v: coef.x.v,
            default_type: angular_momentum_type,
        };
        let y = AngularMomentum {
            v: coef.y.v,
            default_type: angular_momentum_type,
        };
        let z = AngularMomentum {
            v: coef.z.v,
            default_type: angular_momentum_type,
        };
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_to_vector3_coef_kg_m2_s() {
        let angular_momentum_vec = Vector3::new(
            AngularMomentum::from_kg_m2_per_second(5.0),
            AngularMomentum::from_kg_m2_per_second(10.0),
            AngularMomentum::from_kg_m2_per_second(15.0),
        );
        
        let coef_vec = angular_momentum_vec.to_vector3_coef(AngularMomentumType::KgM2perSecond);
        
        assert_relative_eq!(coef_vec.x, 5.0);
        assert_relative_eq!(coef_vec.y, 10.0);
        assert_relative_eq!(coef_vec.z, 15.0);
    }

    #[test]
    fn test_to_vector3_coef_kg_km2_s() {
        let angular_momentum_vec = Vector3::new(
            AngularMomentum::from_kg_m2_per_second(1e6),   // 1e6 kg⋅m²/s = 1 kg⋅km²/s
            AngularMomentum::from_kg_m2_per_second(2e6),   // 2e6 kg⋅m²/s = 2 kg⋅km²/s
            AngularMomentum::from_kg_m2_per_second(3e6),   // 3e6 kg⋅m²/s = 3 kg⋅km²/s
        );
        
        let coef_vec = angular_momentum_vec.to_vector3_coef(AngularMomentumType::KgKm2perSecond);
        
        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_from_vector_coef_kg_m2_s() {
        let coef_vec = Vector3::new(
            Coef::new(5.0),
            Coef::new(10.0),
            Coef::new(15.0),
        );

        let angular_momentum_vec = Vector3::<AngularMomentum>::from_vector_coef(coef_vec, AngularMomentumType::KgM2perSecond);

        assert_relative_eq!(angular_momentum_vec.x.as_kg_m2_per_second(), 5.0);
        assert_relative_eq!(angular_momentum_vec.y.as_kg_m2_per_second(), 10.0);
        assert_relative_eq!(angular_momentum_vec.z.as_kg_m2_per_second(), 15.0);
    }

    #[test]
    fn test_from_vector_coef_kg_km2_s() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let angular_momentum_vec = Vector3::<AngularMomentum>::from_vector_coef(coef_vec, AngularMomentumType::KgKm2perSecond);

        assert_relative_eq!(angular_momentum_vec.x.as_kg_km2_per_second(), 1.0);
        assert_relative_eq!(angular_momentum_vec.y.as_kg_km2_per_second(), 2.0);
        assert_relative_eq!(angular_momentum_vec.z.as_kg_km2_per_second(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_angular_momentum_vec = Vector3::new(
            AngularMomentum::from_kg_m2_per_second(5.0),
            AngularMomentum::from_kg_m2_per_second(10.0),
            AngularMomentum::from_kg_m2_per_second(15.0),
        );

        let coef_vec = original_angular_momentum_vec.to_vector3_coef(AngularMomentumType::KgM2perSecond);
        let reconstructed_angular_momentum_vec = Vector3::<AngularMomentum>::from_vector_coef(coef_vec, AngularMomentumType::KgM2perSecond);

        assert_relative_eq!(original_angular_momentum_vec.x.as_kg_m2_per_second(), reconstructed_angular_momentum_vec.x.as_kg_m2_per_second());
        assert_relative_eq!(original_angular_momentum_vec.y.as_kg_m2_per_second(), reconstructed_angular_momentum_vec.y.as_kg_m2_per_second());
        assert_relative_eq!(original_angular_momentum_vec.z.as_kg_m2_per_second(), reconstructed_angular_momentum_vec.z.as_kg_m2_per_second());
    }

    #[test]
    fn test_angular_momentum_comprehensive_as_methods() {
        // 测试所有单位类型之间的转换
        let angular_momentum_vec = Vector3::new(
            AngularMomentum::from_kg_m2_per_second(1.0),
            AngularMomentum::from_kg_m2_per_second(2.0),
            AngularMomentum::from_kg_m2_per_second(3.0),
        );

        // 从 KgM2perSecond 转换到所有单位类型
        let coef_kg_m2_s = angular_momentum_vec.to_vector3_coef(AngularMomentumType::KgM2perSecond);
        let coef_kg_km2_s = angular_momentum_vec.to_vector3_coef(AngularMomentumType::KgKm2perSecond);
        let coef_nms = angular_momentum_vec.to_vector3_coef(AngularMomentumType::Nms);
        let coef_mill_nms = angular_momentum_vec.to_vector3_coef(AngularMomentumType::MillNms);
        let coef_micro_nms = angular_momentum_vec.to_vector3_coef(AngularMomentumType::MicroNms);
        let coef_nano_nms = angular_momentum_vec.to_vector3_coef(AngularMomentumType::NanoNms);

        // 验证转换结果
        assert_relative_eq!(coef_kg_m2_s.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_kg_km2_s.x.get_value(), 0.000001, epsilon = 1e-10);
        assert_relative_eq!(coef_nms.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_mill_nms.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_micro_nms.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_nano_nms.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 KgKm2perSecond 转换到所有单位类型
        let kg_km2_s_vec = Vector3::new(
            AngularMomentum::from_kg_km2_per_second(1.0),
            AngularMomentum::from_kg_km2_per_second(2.0),
            AngularMomentum::from_kg_km2_per_second(3.0),
        );

        let coef_from_kg_km2_s = kg_km2_s_vec.to_vector3_coef(AngularMomentumType::KgM2perSecond);
        let coef_from_kg_km2_s_km2 = kg_km2_s_vec.to_vector3_coef(AngularMomentumType::KgKm2perSecond);
        let coef_from_kg_km2_s_nms = kg_km2_s_vec.to_vector3_coef(AngularMomentumType::Nms);
        let coef_from_kg_km2_s_mill_nms = kg_km2_s_vec.to_vector3_coef(AngularMomentumType::MillNms);
        let coef_from_kg_km2_s_micro_nms = kg_km2_s_vec.to_vector3_coef(AngularMomentumType::MicroNms);
        let coef_from_kg_km2_s_nano_nms = kg_km2_s_vec.to_vector3_coef(AngularMomentumType::NanoNms);

        assert_relative_eq!(coef_from_kg_km2_s.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kg_km2_s_km2.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kg_km2_s_nms.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kg_km2_s_mill_nms.x.get_value(), 1000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kg_km2_s_micro_nms.x.get_value(), 1000000000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_kg_km2_s_nano_nms.x.get_value(), 1000000000000000.0, epsilon = 1e-10);

        // 从 Nms 转换到所有单位类型
        let nms_vec = Vector3::new(
            AngularMomentum::from_nms(1.0),
            AngularMomentum::from_nms(2.0),
            AngularMomentum::from_nms(3.0),
        );

        let coef_from_nms = nms_vec.to_vector3_coef(AngularMomentumType::KgM2perSecond);
        let coef_from_nms_km2 = nms_vec.to_vector3_coef(AngularMomentumType::KgKm2perSecond);
        let coef_from_nms_nms = nms_vec.to_vector3_coef(AngularMomentumType::Nms);
        let coef_from_nms_mill_nms = nms_vec.to_vector3_coef(AngularMomentumType::MillNms);
        let coef_from_nms_micro_nms = nms_vec.to_vector3_coef(AngularMomentumType::MicroNms);
        let coef_from_nms_nano_nms = nms_vec.to_vector3_coef(AngularMomentumType::NanoNms);

        assert_relative_eq!(coef_from_nms.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nms_km2.x.get_value(), 0.000001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nms_nms.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nms_mill_nms.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nms_micro_nms.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nms_nano_nms.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 MillNms 转换到所有单位类型
        let mill_nms_vec = Vector3::new(
            AngularMomentum::from_mill_nms(1000.0),
            AngularMomentum::from_mill_nms(2000.0),
            AngularMomentum::from_mill_nms(3000.0),
        );

        let coef_from_mill_nms = mill_nms_vec.to_vector3_coef(AngularMomentumType::KgM2perSecond);
        let coef_from_mill_nms_km2 = mill_nms_vec.to_vector3_coef(AngularMomentumType::KgKm2perSecond);
        let coef_from_mill_nms_nms = mill_nms_vec.to_vector3_coef(AngularMomentumType::Nms);
        let coef_from_mill_nms_mill_nms = mill_nms_vec.to_vector3_coef(AngularMomentumType::MillNms);
        let coef_from_mill_nms_micro_nms = mill_nms_vec.to_vector3_coef(AngularMomentumType::MicroNms);
        let coef_from_mill_nms_nano_nms = mill_nms_vec.to_vector3_coef(AngularMomentumType::NanoNms);

        assert_relative_eq!(coef_from_mill_nms.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nms_km2.x.get_value(), 0.000001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nms_nms.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nms_mill_nms.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nms_micro_nms.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_mill_nms_nano_nms.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 MicroNms 转换到所有单位类型
        let micro_nms_vec = Vector3::new(
            AngularMomentum::from_micro_nms(1000000.0),
            AngularMomentum::from_micro_nms(2000000.0),
            AngularMomentum::from_micro_nms(3000000.0),
        );

        let coef_from_micro_nms = micro_nms_vec.to_vector3_coef(AngularMomentumType::KgM2perSecond);
        let coef_from_micro_nms_km2 = micro_nms_vec.to_vector3_coef(AngularMomentumType::KgKm2perSecond);
        let coef_from_micro_nms_nms = micro_nms_vec.to_vector3_coef(AngularMomentumType::Nms);
        let coef_from_micro_nms_mill_nms = micro_nms_vec.to_vector3_coef(AngularMomentumType::MillNms);
        let coef_from_micro_nms_micro_nms = micro_nms_vec.to_vector3_coef(AngularMomentumType::MicroNms);
        let coef_from_micro_nms_nano_nms = micro_nms_vec.to_vector3_coef(AngularMomentumType::NanoNms);

        assert_relative_eq!(coef_from_micro_nms.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nms_km2.x.get_value(), 0.000001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nms_nms.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nms_mill_nms.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nms_micro_nms.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_micro_nms_nano_nms.x.get_value(), 1000000000.0, epsilon = 1e-10);

        // 从 NanoNms 转换到所有单位类型
        let nano_nms_vec = Vector3::new(
            AngularMomentum::from_nano_nms(1000000000.0),
            AngularMomentum::from_nano_nms(2000000000.0),
            AngularMomentum::from_nano_nms(3000000000.0),
        );

        let coef_from_nano_nms = nano_nms_vec.to_vector3_coef(AngularMomentumType::KgM2perSecond);
        let coef_from_nano_nms_km2 = nano_nms_vec.to_vector3_coef(AngularMomentumType::KgKm2perSecond);
        let coef_from_nano_nms_nms = nano_nms_vec.to_vector3_coef(AngularMomentumType::Nms);
        let coef_from_nano_nms_mill_nms = nano_nms_vec.to_vector3_coef(AngularMomentumType::MillNms);
        let coef_from_nano_nms_micro_nms = nano_nms_vec.to_vector3_coef(AngularMomentumType::MicroNms);
        let coef_from_nano_nms_nano_nms = nano_nms_vec.to_vector3_coef(AngularMomentumType::NanoNms);

        assert_relative_eq!(coef_from_nano_nms.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nms_km2.x.get_value(), 0.000001, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nms_nms.x.get_value(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nms_mill_nms.x.get_value(), 1000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nms_micro_nms.x.get_value(), 1000000.0, epsilon = 1e-10);
        assert_relative_eq!(coef_from_nano_nms_nano_nms.x.get_value(), 1000000000.0, epsilon = 1e-10);
    }
}
