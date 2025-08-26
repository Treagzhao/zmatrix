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

    #[test]
    fn test_to_vector3_coef_rad_per_hour() {
        let angular_velocity_vec = Vector3::new(
            AngularVelocity::from_rad_per_second(1.0),     // 1 rad/s = 3600 rad/h
            AngularVelocity::from_rad_per_second(0.5),     // 0.5 rad/s = 1800 rad/h
            AngularVelocity::from_rad_per_second(0.1),     // 0.1 rad/s = 360 rad/h
        );
        
        let coef_vec = angular_velocity_vec.to_vector3_coef(AngularVelocityType::RadperHour);
        
        assert_relative_eq!(coef_vec.x.get_value(), 3600.0);
        assert_relative_eq!(coef_vec.y.get_value(), 1800.0);
        assert_relative_eq!(coef_vec.z.get_value(), 360.0);
    }

    #[test]
    fn test_to_vector3_coef_deg_per_hour() {
        let angular_velocity_vec = Vector3::new(
            AngularVelocity::from_rad_per_second(1.0),     // 1 rad/s = 206264.8 deg/h
            AngularVelocity::from_rad_per_second(0.5),     // 0.5 rad/s = 103132.4 deg/h
            AngularVelocity::from_rad_per_second(0.1),     // 0.1 rad/s = 20626.48 deg/h
        );
        
        let coef_vec = angular_velocity_vec.to_vector3_coef(AngularVelocityType::DegperHour);
        
        assert_relative_eq!(coef_vec.x.get_value(), 206264.8, epsilon = 1e-1);
        assert_relative_eq!(coef_vec.y.get_value(), 103132.4, epsilon = 1e-1);
        assert_relative_eq!(coef_vec.z.get_value(), 20626.48, epsilon = 1e-2);
    }

    #[test]
    fn test_from_vector_coef_rad_per_hour() {
        let coef_vec = Vector3::new(
            Coef::new(3600.0),
            Coef::new(1800.0),
            Coef::new(360.0),
        );

        let angular_velocity_vec = Vector3::<AngularVelocity>::from_vector_coef(coef_vec, AngularVelocityType::RadperHour);

        assert_relative_eq!(angular_velocity_vec.x.as_rad_per_hour(), 3600.0);
        assert_relative_eq!(angular_velocity_vec.y.as_rad_per_hour(), 1800.0);
        assert_relative_eq!(angular_velocity_vec.z.as_rad_per_hour(), 360.0);
    }

    #[test]
    fn test_from_vector_coef_deg_per_hour() {
        let coef_vec = Vector3::new(
            Coef::new(206264.8),
            Coef::new(103132.4),
            Coef::new(20626.48),
        );

        let angular_velocity_vec = Vector3::<AngularVelocity>::from_vector_coef(coef_vec, AngularVelocityType::DegperHour);

        assert_relative_eq!(angular_velocity_vec.x.as_deg_per_hour(), 206264.8);
        assert_relative_eq!(angular_velocity_vec.y.as_deg_per_hour(), 103132.4);
        assert_relative_eq!(angular_velocity_vec.z.as_deg_per_hour(), 20626.48);
    }

    #[test]
    fn test_roundtrip_all_types() {
        // 测试所有类型的往返转换
        let types = vec![
            AngularVelocityType::RadperSecond,
            AngularVelocityType::DegPerSecond,
            AngularVelocityType::RadperHour,
            AngularVelocityType::DegperHour,
        ];

        let original_angular_velocity_vec = Vector3::new(
            AngularVelocity::from_rad_per_second(1.0),
            AngularVelocity::from_rad_per_second(2.0),
            AngularVelocity::from_rad_per_second(3.0),
        );

        for angular_velocity_type in types {
            let coef_vec = original_angular_velocity_vec.to_vector3_coef(angular_velocity_type);
            let reconstructed_angular_velocity_vec = Vector3::<AngularVelocity>::from_vector_coef(coef_vec, angular_velocity_type);

            // 验证转换后的值在对应单位下是正确的
            match angular_velocity_type {
                AngularVelocityType::RadperSecond => {
                    assert_relative_eq!(reconstructed_angular_velocity_vec.x.as_rad_per_second(), 1.0);
                    assert_relative_eq!(reconstructed_angular_velocity_vec.y.as_rad_per_second(), 2.0);
                    assert_relative_eq!(reconstructed_angular_velocity_vec.z.as_rad_per_second(), 3.0);
                }
                AngularVelocityType::DegPerSecond => {
                    assert_relative_eq!(reconstructed_angular_velocity_vec.x.as_deg_per_second(), 57.2958, epsilon = 1e-4);
                    assert_relative_eq!(reconstructed_angular_velocity_vec.y.as_deg_per_second(), 114.5916, epsilon = 1e-4);
                    assert_relative_eq!(reconstructed_angular_velocity_vec.z.as_deg_per_second(), 171.8873, epsilon = 1e-4);
                }
                AngularVelocityType::RadperHour => {
                    assert_relative_eq!(reconstructed_angular_velocity_vec.x.as_rad_per_hour(), 3600.0);
                    assert_relative_eq!(reconstructed_angular_velocity_vec.y.as_rad_per_hour(), 7200.0);
                    assert_relative_eq!(reconstructed_angular_velocity_vec.z.as_rad_per_hour(), 10800.0);
                }
                AngularVelocityType::DegperHour => {
                    assert_relative_eq!(reconstructed_angular_velocity_vec.x.as_deg_per_hour(), 206264.8, epsilon = 1e-1);
                    assert_relative_eq!(reconstructed_angular_velocity_vec.y.as_deg_per_hour(), 412529.6, epsilon = 1e-1);
                    assert_relative_eq!(reconstructed_angular_velocity_vec.z.as_deg_per_hour(), 618794.4, epsilon = 1e-1);
                }
            }
        }
    }

    #[test]
    fn test_edge_cases() {
        // 测试零值
        let zero_angular_velocity_vec = Vector3::new(
            AngularVelocity::from_rad_per_second(0.0),
            AngularVelocity::from_rad_per_second(0.0),
            AngularVelocity::from_rad_per_second(0.0),
        );
        
        let coef_vec = zero_angular_velocity_vec.to_vector3_coef(AngularVelocityType::RadperSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 0.0);
        assert_relative_eq!(coef_vec.y.get_value(), 0.0);
        assert_relative_eq!(coef_vec.z.get_value(), 0.0);

        // 测试负值
        let negative_angular_velocity_vec = Vector3::new(
            AngularVelocity::from_rad_per_second(-1.0),
            AngularVelocity::from_rad_per_second(-2.0),
            AngularVelocity::from_rad_per_second(-3.0),
        );
        
        let coef_vec = negative_angular_velocity_vec.to_vector3_coef(AngularVelocityType::RadperSecond);
        assert_relative_eq!(coef_vec.x.get_value(), -1.0);
        assert_relative_eq!(coef_vec.y.get_value(), -2.0);
        assert_relative_eq!(coef_vec.z.get_value(), -3.0);

        // 测试很小的值
        let small_angular_velocity_vec = Vector3::new(
            AngularVelocity::from_rad_per_second(1e-10),
            AngularVelocity::from_rad_per_second(1e-15),
            AngularVelocity::from_rad_per_second(1e-20),
        );
        
        let coef_vec = small_angular_velocity_vec.to_vector3_coef(AngularVelocityType::RadperSecond);
        assert_relative_eq!(coef_vec.x.get_value(), 1e-10);
        assert_relative_eq!(coef_vec.y.get_value(), 1e-15);
        assert_relative_eq!(coef_vec.z.get_value(), 1e-20);
    }

    #[test]
    fn test_filter_edge_cases() {
        // 测试滤波的边界情况
        let vec = Vector3::new(
            AngularVelocity::from_rad_per_second(0.0),
            AngularVelocity::from_rad_per_second(-5.0),
            AngularVelocity::from_rad_per_second(5.0),
        );
        
        let filtered_vec = vec.filter(3.0);
        
        assert_relative_eq!(filtered_vec.x.as_rad_per_second(), 0.0);
        assert_relative_eq!(filtered_vec.y.as_rad_per_second(), -3.0);
        assert_relative_eq!(filtered_vec.z.as_rad_per_second(), 3.0);
    }

    #[test]
    fn test_to_f32_array_edge_cases() {
        // 测试 f32 数组转换的边界情况
        let vector = Vector3::new(
            AngularVelocity::from_rad_per_second(0.0),
            AngularVelocity::from_rad_per_second(-1.0),
            AngularVelocity::from_rad_per_second(1e-10),
        );
        
        let array = vector.to_f32_array();
        assert_eq!(array[0], 0.0);
        assert_eq!(array[1], -1.0);
        assert_eq!(array[2], 1e-10);
    }
}