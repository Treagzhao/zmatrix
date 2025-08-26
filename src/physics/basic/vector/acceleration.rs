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

    #[test]
    fn test_to_vector3_coef_km_per_h2() {
        let acceleration_vec = Vector3::new(
            Acceleration::from_m_per_s2(1.0),      // 1 m/s² = 12960 km/h²
            Acceleration::from_m_per_s2(2.0),      // 2 m/s² = 25920 km/h²
            Acceleration::from_m_per_s2(0.5),      // 0.5 m/s² = 6480 km/h²
        );
        
        let coef_vec = acceleration_vec.to_vector3_coef(AccelerationType::KmPerHour2);
        
        assert_relative_eq!(coef_vec.x.get_value(), 12960.0);
        assert_relative_eq!(coef_vec.y.get_value(), 25920.0);
        assert_relative_eq!(coef_vec.z.get_value(), 6480.0);
    }

    #[test]
    fn test_from_vector_coef_km_per_h2() {
        let coef_vec = Vector3::new(
            Coef::new(12960.0),
            Coef::new(25920.0),
            Coef::new(6480.0),
        );

        let acceleration_vec = Vector3::<Acceleration>::from_vector_coef(coef_vec, AccelerationType::KmPerHour2);

        assert_relative_eq!(acceleration_vec.x.as_km_per_h_2(), 12960.0);
        assert_relative_eq!(acceleration_vec.y.as_km_per_h_2(), 25920.0);
        assert_relative_eq!(acceleration_vec.z.as_km_per_h_2(), 6480.0);
    }

    #[test]
    fn test_roundtrip_all_types() {
        // 测试所有类型的往返转换
        let types = vec![
            AccelerationType::MPerSecond2,
            AccelerationType::KmPerHour2,
            AccelerationType::G,
        ];

        let original_acceleration_vec = Vector3::new(
            Acceleration::from_m_per_s2(9.8),
            Acceleration::from_m_per_s2(19.6),
            Acceleration::from_m_per_s2(29.4),
        );

        for acceleration_type in types {
            let coef_vec = original_acceleration_vec.to_vector3_coef(acceleration_type);
            let reconstructed_acceleration_vec = Vector3::<Acceleration>::from_vector_coef(coef_vec, acceleration_type);

            // 验证转换后的值在对应单位下是正确的
            match acceleration_type {
                AccelerationType::MPerSecond2 => {
                    assert_relative_eq!(reconstructed_acceleration_vec.x.as_m_per_s2(), 9.8);
                    assert_relative_eq!(reconstructed_acceleration_vec.y.as_m_per_s2(), 19.6);
                    assert_relative_eq!(reconstructed_acceleration_vec.z.as_m_per_s2(), 29.4);
                }
                AccelerationType::KmPerHour2 => {
                    assert_relative_eq!(reconstructed_acceleration_vec.x.as_km_per_h_2(), 127008.0);
                    assert_relative_eq!(reconstructed_acceleration_vec.y.as_km_per_h_2(), 254016.0);
                    assert_relative_eq!(reconstructed_acceleration_vec.z.as_km_per_h_2(), 381024.0);
                }
                AccelerationType::G => {
                    // 9.8 m/s² ≈ 0.999 g, 19.6 m/s² ≈ 1.998 g, 29.4 m/s² ≈ 2.997 g
                    assert_relative_eq!(reconstructed_acceleration_vec.x.as_g(), 9.8 / 9.80665);
                    assert_relative_eq!(reconstructed_acceleration_vec.y.as_g(), 19.6 / 9.80665);
                    assert_relative_eq!(reconstructed_acceleration_vec.z.as_g(), 29.4 / 9.80665);
                }
            }
        }
    }

    #[test]
    fn test_edge_cases() {
        // 测试零值
        let zero_acceleration_vec = Vector3::new(
            Acceleration::from_m_per_s2(0.0),
            Acceleration::from_m_per_s2(0.0),
            Acceleration::from_m_per_s2(0.0),
        );
        
        let coef_vec = zero_acceleration_vec.to_vector3_coef(AccelerationType::MPerSecond2);
        assert_relative_eq!(coef_vec.x.get_value(), 0.0);
        assert_relative_eq!(coef_vec.y.get_value(), 0.0);
        assert_relative_eq!(coef_vec.z.get_value(), 0.0);

        // 测试负值
        let negative_acceleration_vec = Vector3::new(
            Acceleration::from_m_per_s2(-9.8),
            Acceleration::from_m_per_s2(-19.6),
            Acceleration::from_m_per_s2(-29.4),
        );
        
        let coef_vec = negative_acceleration_vec.to_vector3_coef(AccelerationType::MPerSecond2);
        assert_relative_eq!(coef_vec.x.get_value(), -9.8);
        assert_relative_eq!(coef_vec.y.get_value(), -19.6);
        assert_relative_eq!(coef_vec.z.get_value(), -29.4);

        // 测试很小的值
        let small_acceleration_vec = Vector3::new(
            Acceleration::from_m_per_s2(1e-10),
            Acceleration::from_m_per_s2(1e-15),
            Acceleration::from_m_per_s2(1e-20),
        );
        
        let coef_vec = small_acceleration_vec.to_vector3_coef(AccelerationType::MPerSecond2);
        assert_relative_eq!(coef_vec.x.get_value(), 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 1e-15);
        assert_relative_eq!(coef_vec.z.get_value(), 1e-20);
    }
}
