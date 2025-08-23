use std::ops::Mul;
use std::time::Duration;
use crate::physics::basic::{Angular, AngularVelocity, AngularVelocityType, Coef, Vector3};
use crate::utils::float;

impl Vector3<AngularVelocity> {
    //滤波
    pub fn filter(&self, limit: f64) -> Self {
        let x = float::limit_float(self.x.as_rad_per_second(), limit); // 这个limit是从lagacy里粘贴出来的，我也不知道含义.
        let y = float::limit_float(self.y.as_rad_per_second(), limit);
        let z = float::limit_float(self.z.as_rad_per_second(), limit);
        Self {
            x: AngularVelocity::from_rad_per_second(x),
            y: AngularVelocity::from_rad_per_second(y),
            z: AngularVelocity::from_rad_per_second(z),
        }
    }

    pub fn to_f32_array(&self) -> [f32; 3] {
        let result: [f32; 3] = [self.x.as_rad_per_second() as f32, self.y.as_rad_per_second() as f32, self.z.as_rad_per_second() as f32];
        result
    }

    pub fn to_vector3_coef(&self, angular_velocity_type: AngularVelocityType) -> Vector3<Coef> {
        match angular_velocity_type {
            AngularVelocityType::RadperSecond => {
                let (x, y, z) = (self.x.as_rad_per_second(), self.y.as_rad_per_second(), self.z.as_rad_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularVelocityType::DegPerSecond => {
                let (x, y, z) = (self.x.as_deg_per_second(), self.y.as_deg_per_second(), self.z.as_deg_per_second());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularVelocityType::RadperHour => {
                let (x, y, z) = (self.x.as_rad_per_hour(), self.y.as_rad_per_hour(), self.z.as_rad_per_hour());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularVelocityType::DegperHour => {
                let (x, y, z) = (self.x.as_deg_per_hour(), self.y.as_deg_per_hour(), self.z.as_deg_per_hour());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, angular_velocity_type: AngularVelocityType) -> Vector3<AngularVelocity> {
        let x = AngularVelocity {
            v: coef.x.v,
            default_type: angular_velocity_type,
        };
        let y = AngularVelocity {
            v: coef.y.v,
            default_type: angular_velocity_type,
        };
        let z = AngularVelocity {
            v: coef.z.v,
            default_type: angular_velocity_type,
        };
        Vector3::new(x, y, z)
    }
}

impl Mul<Duration> for Vector3<AngularVelocity> {
    type Output = Vector3<Angular>;

    fn mul(self, rhs: Duration) -> Self::Output {
        let x= self.x * rhs;
        let y= self.y * rhs;
        let z = self.z * rhs;
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    #[test]
    fn test_filter() {
        let vec = Vector3::new(AngularVelocity::from_rad_per_second(1.0), AngularVelocity::from_rad_per_second(2.0), AngularVelocity::from_rad_per_second(3.0));
        let vec2 = vec.filter(2.0);
        assert_eq!(vec2.x, AngularVelocity::from_rad_per_second(1.0));
        assert_eq!(vec2.y, AngularVelocity::from_rad_per_second(2.0));
        assert_eq!(vec2.z, AngularVelocity::from_rad_per_second(2.0));

        let vec = Vector3::new(AngularVelocity::from_rad_per_second(3.0), AngularVelocity::from_rad_per_second(2.0), AngularVelocity::from_rad_per_second(1.0));
        let vec2 = vec.filter(2.0);
        assert_eq!(vec2.x, AngularVelocity::from_rad_per_second(2.0));
        assert_eq!(vec2.y, AngularVelocity::from_rad_per_second(2.0));
        assert_eq!(vec2.z, AngularVelocity::from_rad_per_second(1.0));

        let vec = Vector3::new(AngularVelocity::from_rad_per_second(1.0), AngularVelocity::from_rad_per_second(3.0), AngularVelocity::from_rad_per_second(1.0));
        let vec2 = vec.filter(2.0);
        assert_eq!(vec2.x, AngularVelocity::from_rad_per_second(1.0));
        assert_eq!(vec2.y, AngularVelocity::from_rad_per_second(2.0));
        assert_eq!(vec2.z, AngularVelocity::from_rad_per_second(1.0));
    }

    #[test]
    fn test_to_f32_array() {
        let vector = Vector3 {
            x: AngularVelocity::from_rad_per_second(1.0),
            y: AngularVelocity::from_rad_per_second(2.0),
            z: AngularVelocity::from_rad_per_second(3.0),
        };
        let array = vector.to_f32_array();
        assert_eq!(array[0], 1.0);
        assert_eq!(array[1], 2.0);
        assert_eq!(array[2], 3.0);
    }

    #[test]
    fn test_convert(){
        let v:Vector3<AngularVelocity> = Vector3::new(AngularVelocity::from_rad_per_second(1.0),AngularVelocity::from_rad_per_second(2.0),AngularVelocity::from_rad_per_second(3.0));
        let duration = Duration::from_secs(2);
        let v2 = v * duration;
        assert_relative_eq!(v2.x.as_rad(),2.0);
        assert_relative_eq!(v2.y.as_rad(),4.0);
        assert_relative_eq!(v2.z.as_rad(),6.0);
    }

    #[test]
    fn test_to_vector3_coef_rad_per_second() {
        let angular_velocity_vec = Vector3::new(
            AngularVelocity::from_rad_per_second(1.0),
            AngularVelocity::from_rad_per_second(2.0),
            AngularVelocity::from_rad_per_second(3.0),
        );
        
        let coef_vec = angular_velocity_vec.to_vector3_coef(AngularVelocityType::RadperSecond);
        
        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_to_vector3_coef_deg_per_second() {
        let angular_velocity_vec = Vector3::new(
            AngularVelocity::from_rad_per_second(1.0),     // 1 rad/s = 57.2958 deg/s
            AngularVelocity::from_rad_per_second(2.0),     // 2 rad/s = 114.5916 deg/s
            AngularVelocity::from_rad_per_second(3.0),     // 3 rad/s = 171.8873 deg/s
        );
        
        let coef_vec = angular_velocity_vec.to_vector3_coef(AngularVelocityType::DegPerSecond);
        
        assert_relative_eq!(coef_vec.x, 57.2958, epsilon = 1e-4);
        assert_relative_eq!(coef_vec.y, 114.5916, epsilon = 1e-4);
        assert_relative_eq!(coef_vec.z, 171.8873, epsilon = 1e-4);
    }

    #[test]
    fn test_from_vector_coef_rad_per_second() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let angular_velocity_vec = Vector3::<AngularVelocity>::from_vector_coef(coef_vec, AngularVelocityType::RadperSecond);

        assert_relative_eq!(angular_velocity_vec.x.as_rad_per_second(), 1.0);
        assert_relative_eq!(angular_velocity_vec.y.as_rad_per_second(), 2.0);
        assert_relative_eq!(angular_velocity_vec.z.as_rad_per_second(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_deg_per_second() {
        let coef_vec = Vector3::new(
            Coef::new(57.2958),
            Coef::new(114.5916),
            Coef::new(171.8873),
        );

        let angular_velocity_vec = Vector3::<AngularVelocity>::from_vector_coef(coef_vec, AngularVelocityType::DegPerSecond);

        assert_relative_eq!(angular_velocity_vec.x.as_deg_per_second(), 57.2958);
        assert_relative_eq!(angular_velocity_vec.y.as_deg_per_second(), 114.5916);
        assert_relative_eq!(angular_velocity_vec.z.as_deg_per_second(), 171.8873);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_angular_velocity_vec = Vector3::new(
            AngularVelocity::from_rad_per_second(1.0),
            AngularVelocity::from_rad_per_second(2.0),
            AngularVelocity::from_rad_per_second(3.0),
        );

        let coef_vec = original_angular_velocity_vec.to_vector3_coef(AngularVelocityType::RadperSecond);
        let reconstructed_angular_velocity_vec = Vector3::<AngularVelocity>::from_vector_coef(coef_vec, AngularVelocityType::RadperSecond);

        assert_relative_eq!(original_angular_velocity_vec.x.as_rad_per_second(), reconstructed_angular_velocity_vec.x.as_rad_per_second());
        assert_relative_eq!(original_angular_velocity_vec.y.as_rad_per_second(), reconstructed_angular_velocity_vec.y.as_rad_per_second());
        assert_relative_eq!(original_angular_velocity_vec.z.as_rad_per_second(), reconstructed_angular_velocity_vec.z.as_rad_per_second());
    }
}