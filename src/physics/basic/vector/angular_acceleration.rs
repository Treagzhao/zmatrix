use crate::physics::basic::{ AngularAcceleration, Vector3};

impl Vector3<AngularAcceleration> {
    pub fn to_f32_array(&self) -> [f32; 3] {
        let result: [f32; 3] = [self.x.as_rad_per_second2() as f32, self.y.as_rad_per_second2() as f32, self.z.as_rad_per_second2() as f32];
        result
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
}