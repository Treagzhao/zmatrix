use crate::physics::basic::{AngularAcceleration, AngularAccelerationType, Coef, Vector3};

impl Vector3<AngularAcceleration> {
    pub fn to_f32_array(&self) -> [f32; 3] {
        let result: [f32; 3] = [self.x.as_rad_per_second2() as f32, self.y.as_rad_per_second2() as f32, self.z.as_rad_per_second2() as f32];
        result
    }

    pub fn to_vector3_coef(&self, angular_acceleration_type: AngularAccelerationType) -> Vector3<Coef> {
        match angular_acceleration_type {
            AngularAccelerationType::RadperSecond2 => {
                let (x, y, z) = (self.x.as_rad_per_second2(), self.y.as_rad_per_second2(), self.z.as_rad_per_second2());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularAccelerationType::DegPerSecond2 => {
                let (x, y, z) = (self.x.as_deg_per_second2(), self.y.as_deg_per_second2(), self.z.as_deg_per_second2());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, angular_acceleration_type: AngularAccelerationType) -> Vector3<AngularAcceleration> {
        let x = AngularAcceleration {
            v: coef.x.v,
            default_type: angular_acceleration_type,
        };
        let y = AngularAcceleration {
            v: coef.y.v,
            default_type: angular_acceleration_type,
        };
        let z = AngularAcceleration {
            v: coef.z.v,
            default_type: angular_acceleration_type,
        };
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use super::*;

    #[test]
    fn test_to_f32_array() {
        let a:Vector3<AngularAcceleration> = Vector3::new(
            AngularAcceleration::from_rad_per_second2(1.0),
            AngularAcceleration::from_rad_per_second2(2.0),
            AngularAcceleration::from_rad_per_second2(3.0),
        );
        let result  = a.to_f32_array();
        assert_eq!(3,result.len());
        assert_relative_eq!(1.0, result[0]);
        assert_relative_eq!(2.0, result[1]);
        assert_relative_eq!(3.0, result[2]);
    }

    #[test]
    fn test_to_vector3_coef_rad_per_second2() {
        let angular_acceleration_vec = Vector3::new(
            AngularAcceleration::from_rad_per_second2(1.0),
            AngularAcceleration::from_rad_per_second2(2.0),
            AngularAcceleration::from_rad_per_second2(3.0),
        );
        
        let coef_vec = angular_acceleration_vec.to_vector3_coef(AngularAccelerationType::RadperSecond2);
        
        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_to_vector3_coef_deg_per_second2() {
        let angular_acceleration_vec = Vector3::new(
            AngularAcceleration::from_rad_per_second2(1.0),     // 1 rad/s² = 57.2958 deg/s²
            AngularAcceleration::from_rad_per_second2(2.0),     // 2 rad/s² = 114.5916 deg/s²
            AngularAcceleration::from_rad_per_second2(3.0),     // 3 rad/s² = 171.8873 deg/s²
        );
        
        let coef_vec = angular_acceleration_vec.to_vector3_coef(AngularAccelerationType::DegPerSecond2);
        
        assert_relative_eq!(coef_vec.x, 57.2958, epsilon = 1e-4);
        assert_relative_eq!(coef_vec.y, 114.5916, epsilon = 1e-4);
        assert_relative_eq!(coef_vec.z, 171.8873, epsilon = 1e-4);
    }

    #[test]
    fn test_from_vector_coef_rad_per_second2() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let angular_acceleration_vec = Vector3::<AngularAcceleration>::from_vector_coef(coef_vec, AngularAccelerationType::RadperSecond2);

        assert_relative_eq!(angular_acceleration_vec.x.as_rad_per_second2(), 1.0);
        assert_relative_eq!(angular_acceleration_vec.y.as_rad_per_second2(), 2.0);
        assert_relative_eq!(angular_acceleration_vec.z.as_rad_per_second2(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_deg_per_second2() {
        let coef_vec = Vector3::new(
            Coef::new(57.2958),
            Coef::new(114.5916),
            Coef::new(171.8873),
        );

        let angular_acceleration_vec = Vector3::<AngularAcceleration>::from_vector_coef(coef_vec, AngularAccelerationType::DegPerSecond2);

        assert_relative_eq!(angular_acceleration_vec.x.as_deg_per_second2(), 57.2958);
        assert_relative_eq!(angular_acceleration_vec.y.as_deg_per_second2(), 114.5916);
        assert_relative_eq!(angular_acceleration_vec.z.as_deg_per_second2(), 171.8873);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_angular_acceleration_vec = Vector3::new(
            AngularAcceleration::from_rad_per_second2(1.0),
            AngularAcceleration::from_rad_per_second2(2.0),
            AngularAcceleration::from_rad_per_second2(3.0),
        );

        let coef_vec = original_angular_acceleration_vec.to_vector3_coef(AngularAccelerationType::RadperSecond2);
        let reconstructed_angular_acceleration_vec = Vector3::<AngularAcceleration>::from_vector_coef(coef_vec, AngularAccelerationType::RadperSecond2);

        assert_relative_eq!(original_angular_acceleration_vec.x.as_rad_per_second2(), reconstructed_angular_acceleration_vec.x.as_rad_per_second2());
        assert_relative_eq!(original_angular_acceleration_vec.y.as_rad_per_second2(), reconstructed_angular_acceleration_vec.y.as_rad_per_second2());
        assert_relative_eq!(original_angular_acceleration_vec.z.as_rad_per_second2(), reconstructed_angular_acceleration_vec.z.as_rad_per_second2());
    }
}