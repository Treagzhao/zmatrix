use crate::physics::basic::{Acceleration, AccelerationType, Coef, Vector3};

impl Vector3<Acceleration> {
    pub fn to_vector3_coef(&self, acceleration_type: AccelerationType) -> Vector3<Coef> {
        match acceleration_type {
            AccelerationType::MPerSecond2 => {
                let (x, y, z) = (self.x.as_m_per_s2(), self.y.as_m_per_s2(), self.z.as_m_per_s2());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AccelerationType::KmPerHour2 => {
                let (x, y, z) = (self.x.as_km_per_h_2(), self.y.as_km_per_h_2(), self.z.as_km_per_h_2());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AccelerationType::G => {
                let (x, y, z) = (self.x.as_g(), self.y.as_g(), self.z.as_g());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, acceleration_type: AccelerationType) -> Vector3<Acceleration> {
        let x = Acceleration {
            v: coef.x.v,
            default_type: acceleration_type,
        };
        let y = Acceleration {
            v: coef.y.v,
            default_type: acceleration_type,
        };
        let z = Acceleration {
            v: coef.z.v,
            default_type: acceleration_type,
        };
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_to_vector3_coef_m_per_s2() {
        let acceleration_vec = Vector3::new(
            Acceleration::from_m_per_s2(9.8),
            Acceleration::from_m_per_s2(19.6),
            Acceleration::from_m_per_s2(29.4),
        );
        
        let coef_vec = acceleration_vec.to_vector3_coef(AccelerationType::MPerSecond2);
        
        assert_relative_eq!(coef_vec.x, 9.8);
        assert_relative_eq!(coef_vec.y, 19.6);
        assert_relative_eq!(coef_vec.z, 29.4);
    }

    #[test]
    fn test_to_vector3_coef_g() {
        let acceleration_vec = Vector3::new(
            Acceleration::from_g(1.0),
            Acceleration::from_g(2.0),
            Acceleration::from_g(3.0),
        );
        
        let coef_vec = acceleration_vec.to_vector3_coef(AccelerationType::G);
        
        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_from_vector_coef_m_per_s2() {
        let coef_vec = Vector3::new(
            Coef::new(9.8),
            Coef::new(19.6),
            Coef::new(29.4),
        );

        let acceleration_vec = Vector3::<Acceleration>::from_vector_coef(coef_vec, AccelerationType::MPerSecond2);

        assert_relative_eq!(acceleration_vec.x.as_m_per_s2(), 9.8);
        assert_relative_eq!(acceleration_vec.y.as_m_per_s2(), 19.6);
        assert_relative_eq!(acceleration_vec.z.as_m_per_s2(), 29.4);
    }

    #[test]
    fn test_from_vector_coef_g() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let acceleration_vec = Vector3::<Acceleration>::from_vector_coef(coef_vec, AccelerationType::G);

        assert_relative_eq!(acceleration_vec.x.as_g(), 1.0);
        assert_relative_eq!(acceleration_vec.y.as_g(), 2.0);
        assert_relative_eq!(acceleration_vec.z.as_g(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_acceleration_vec = Vector3::new(
            Acceleration::from_m_per_s2(9.8),
            Acceleration::from_m_per_s2(19.6),
            Acceleration::from_m_per_s2(29.4),
        );

        let coef_vec = original_acceleration_vec.to_vector3_coef(AccelerationType::MPerSecond2);
        let reconstructed_acceleration_vec = Vector3::<Acceleration>::from_vector_coef(coef_vec, AccelerationType::MPerSecond2);

        assert_relative_eq!(original_acceleration_vec.x.as_m_per_s2(), reconstructed_acceleration_vec.x.as_m_per_s2());
        assert_relative_eq!(original_acceleration_vec.y.as_m_per_s2(), reconstructed_acceleration_vec.y.as_m_per_s2());
        assert_relative_eq!(original_acceleration_vec.z.as_m_per_s2(), reconstructed_acceleration_vec.z.as_m_per_s2());
    }
}
