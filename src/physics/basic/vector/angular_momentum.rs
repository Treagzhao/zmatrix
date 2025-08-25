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
}
