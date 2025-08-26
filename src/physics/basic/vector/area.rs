use crate::physics::basic::{Area, AreaType, Coef, Vector3};

impl Vector3<Area> {
    pub fn to_vector3_coef(&self, area_type: AreaType) -> Vector3<Coef> {
        match area_type {
            AreaType::M2 => {
                let (x, y, z) = (self.x.as_m2(), self.y.as_m2(), self.z.as_m2());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AreaType::KM2 => {
                let (x, y, z) = (self.x.as_km2(), self.y.as_km2(), self.z.as_km2());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, area_type: AreaType) -> Vector3<Area> {
        let x = Area {
            v: coef.x.v,
            default_type: area_type,
        };
        let y = Area {
            v: coef.y.v,
            default_type: area_type,
        };
        let z = Area {
            v: coef.z.v,
            default_type: area_type,
        };
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_to_vector3_coef_m2() {
        let area_vec = Vector3::new(
            Area::from_m2(100.0),
            Area::from_m2(200.0),
            Area::from_m2(300.0),
        );
        
        let coef_vec = area_vec.to_vector3_coef(AreaType::M2);
        
        assert_relative_eq!(coef_vec.x, 100.0);
        assert_relative_eq!(coef_vec.y, 200.0);
        assert_relative_eq!(coef_vec.z, 300.0);
    }

    #[test]
    fn test_to_vector3_coef_km2() {
        let area_vec = Vector3::new(
            Area::from_m2(1e6),    // 1e6 m² = 1 km²
            Area::from_m2(2e6),    // 2e6 m² = 2 km²
            Area::from_m2(3e6),    // 3e6 m² = 3 km²
        );
        
        let coef_vec = area_vec.to_vector3_coef(AreaType::KM2);
        
        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_from_vector_coef_m2() {
        let coef_vec = Vector3::new(
            Coef::new(100.0),
            Coef::new(200.0),
            Coef::new(300.0),
        );

        let area_vec = Vector3::<Area>::from_vector_coef(coef_vec, AreaType::M2);

        assert_relative_eq!(area_vec.x.as_m2(), 100.0);
        assert_relative_eq!(area_vec.y.as_m2(), 200.0);
        assert_relative_eq!(area_vec.z.as_m2(), 300.0);
    }

    #[test]
    fn test_from_vector_coef_km2() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let area_vec = Vector3::<Area>::from_vector_coef(coef_vec, AreaType::KM2);

        assert_relative_eq!(area_vec.x.as_km2(), 1.0);
        assert_relative_eq!(area_vec.y.as_km2(), 2.0);
        assert_relative_eq!(area_vec.z.as_km2(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_area_vec = Vector3::new(
            Area::from_m2(100.0),
            Area::from_m2(200.0),
            Area::from_m2(300.0),
        );

        let coef_vec = original_area_vec.to_vector3_coef(AreaType::M2);
        let reconstructed_area_vec = Vector3::<Area>::from_vector_coef(coef_vec, AreaType::M2);

        assert_relative_eq!(original_area_vec.x.as_m2(), reconstructed_area_vec.x.as_m2());
        assert_relative_eq!(original_area_vec.y.as_m2(), reconstructed_area_vec.y.as_m2());
        assert_relative_eq!(original_area_vec.z.as_m2(), reconstructed_area_vec.z.as_m2());
    }
}
