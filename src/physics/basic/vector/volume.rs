use crate::physics::basic::{Coef, Vector3, Volume, VolumeType};

impl Vector3<Volume> {
    pub fn to_vector3_coef(&self, volume_type: VolumeType) -> Vector3<Coef> {
        match volume_type {
            VolumeType::M3 => {
                let (x, y, z) = (self.x.as_m3(), self.y.as_m3(), self.z.as_m3());
                Vector3::<Coef>::from_array([x, y, z])
            }
            VolumeType::KM3 => {
                let (x, y, z) = (self.x.as_km3(), self.y.as_km3(), self.z.as_km3());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, volume_type: VolumeType) -> Vector3<Volume> {
        let x = Volume {
            v: coef.x.v,
            default_type: volume_type,
        };
        let y = Volume {
            v: coef.y.v,
            default_type: volume_type,
        };
        let z = Volume {
            v: coef.z.v,
            default_type: volume_type,
        };
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_to_vector3_coef_m3() {
        let volume_vec = Vector3::new(
            Volume::from_m3(1000.0),
            Volume::from_m3(2000.0),
            Volume::from_m3(3000.0),
        );
        
        let coef_vec = volume_vec.to_vector3_coef(VolumeType::M3);
        
        assert_relative_eq!(coef_vec.x, 1000.0);
        assert_relative_eq!(coef_vec.y, 2000.0);
        assert_relative_eq!(coef_vec.z, 3000.0);
    }

    #[test]
    fn test_to_vector3_coef_km3() {
        let volume_vec = Vector3::new(
            Volume::from_m3(1e9),    // 1e9 m³ = 1 km³
            Volume::from_m3(2e9),    // 2e9 m³ = 2 km³
            Volume::from_m3(3e9),    // 3e9 m³ = 3 km³
        );
        
        let coef_vec = volume_vec.to_vector3_coef(VolumeType::KM3);
        
        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_from_vector_coef_m3() {
        let coef_vec = Vector3::new(
            Coef::new(1000.0),
            Coef::new(2000.0),
            Coef::new(3000.0),
        );

        let volume_vec = Vector3::<Volume>::from_vector_coef(coef_vec, VolumeType::M3);

        assert_relative_eq!(volume_vec.x.as_m3(), 1000.0);
        assert_relative_eq!(volume_vec.y.as_m3(), 2000.0);
        assert_relative_eq!(volume_vec.z.as_m3(), 3000.0);
    }

    #[test]
    fn test_from_vector_coef_km3() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let volume_vec = Vector3::<Volume>::from_vector_coef(coef_vec, VolumeType::KM3);

        assert_relative_eq!(volume_vec.x.as_km3(), 1.0);
        assert_relative_eq!(volume_vec.y.as_km3(), 2.0);
        assert_relative_eq!(volume_vec.z.as_km3(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_volume_vec = Vector3::new(
            Volume::from_m3(1000.0),
            Volume::from_m3(2000.0),
            Volume::from_m3(3000.0),
        );

        let coef_vec = original_volume_vec.to_vector3_coef(VolumeType::M3);
        let reconstructed_volume_vec = Vector3::<Volume>::from_vector_coef(coef_vec, VolumeType::M3);

        assert_relative_eq!(original_volume_vec.x.as_m3(), reconstructed_volume_vec.x.as_m3());
        assert_relative_eq!(original_volume_vec.y.as_m3(), reconstructed_volume_vec.y.as_m3());
        assert_relative_eq!(original_volume_vec.z.as_m3(), reconstructed_volume_vec.z.as_m3());
    }
}
