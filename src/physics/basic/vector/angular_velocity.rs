use std::ops::Mul;
use std::time::Duration;
use crate::physics::basic::{Angular, AngularVelocity, Vector3};
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
}