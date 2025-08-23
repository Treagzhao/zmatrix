use crate::physics::basic::{Coef, Momentum, MomentumType, Vector3};

impl Vector3<Momentum> {
    pub fn to_vector3_coef(&self, momentum_type: MomentumType) -> Vector3<Coef> {
        match momentum_type {
            MomentumType::KgMperSecond => {
                let (x, y, z) = (self.x.as_kg_m_s(), self.y.as_kg_m_s(), self.z.as_kg_m_s());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MomentumType::KgKmperSecond => {
                let (x, y, z) = (self.x.as_kg_km_s(), self.y.as_kg_km_s(), self.z.as_kg_km_s());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, momentum_type: MomentumType) -> Vector3<Momentum> {
        let x = Momentum {
            v: coef.x.v,
            default_type: momentum_type,
        };
        let y = Momentum {
            v: coef.y.v,
            default_type: momentum_type,
        };
        let z = Momentum {
            v: coef.z.v,
            default_type: momentum_type,
        };
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_to_vector3_coef_kg_m_s() {
        let momentum_vec = Vector3::new(
            Momentum::from_kg_m_s(10.0),
            Momentum::from_kg_m_s(20.0),
            Momentum::from_kg_m_s(30.0),
        );
        
        let coef_vec = momentum_vec.to_vector3_coef(MomentumType::KgMperSecond);
        
        assert_relative_eq!(coef_vec.x, 10.0);
        assert_relative_eq!(coef_vec.y, 20.0);
        assert_relative_eq!(coef_vec.z, 30.0);
    }

    #[test]
    fn test_to_vector3_coef_kg_km_s() {
        let momentum_vec = Vector3::new(
            Momentum::from_kg_m_s(1000.0),  // 1000 kg⋅m/s = 1 kg⋅km/s
            Momentum::from_kg_m_s(2000.0),  // 2000 kg⋅m/s = 2 kg⋅km/s
            Momentum::from_kg_m_s(3000.0),  // 3000 kg⋅m/s = 3 kg⋅km/s
        );
        
        let coef_vec = momentum_vec.to_vector3_coef(MomentumType::KgKmperSecond);
        
        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_from_vector_coef_kg_m_s() {
        let coef_vec = Vector3::new(
            Coef::new(10.0),
            Coef::new(20.0),
            Coef::new(30.0),
        );

        let momentum_vec = Vector3::<Momentum>::from_vector_coef(coef_vec, MomentumType::KgMperSecond);

        assert_relative_eq!(momentum_vec.x.as_kg_m_s(), 10.0);
        assert_relative_eq!(momentum_vec.y.as_kg_m_s(), 20.0);
        assert_relative_eq!(momentum_vec.z.as_kg_m_s(), 30.0);
    }

    #[test]
    fn test_from_vector_coef_kg_km_s() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let momentum_vec = Vector3::<Momentum>::from_vector_coef(coef_vec, MomentumType::KgKmperSecond);

        assert_relative_eq!(momentum_vec.x.as_kg_km_s(), 1.0);
        assert_relative_eq!(momentum_vec.y.as_kg_km_s(), 2.0);
        assert_relative_eq!(momentum_vec.z.as_kg_km_s(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_momentum_vec = Vector3::new(
            Momentum::from_kg_m_s(10.0),
            Momentum::from_kg_m_s(20.0),
            Momentum::from_kg_m_s(30.0),
        );

        let coef_vec = original_momentum_vec.to_vector3_coef(MomentumType::KgMperSecond);
        let reconstructed_momentum_vec = Vector3::<Momentum>::from_vector_coef(coef_vec, MomentumType::KgMperSecond);

        assert_relative_eq!(original_momentum_vec.x.as_kg_m_s(), reconstructed_momentum_vec.x.as_kg_m_s());
        assert_relative_eq!(original_momentum_vec.y.as_kg_m_s(), reconstructed_momentum_vec.y.as_kg_m_s());
        assert_relative_eq!(original_momentum_vec.z.as_kg_m_s(), reconstructed_momentum_vec.z.as_kg_m_s());
    }
}
