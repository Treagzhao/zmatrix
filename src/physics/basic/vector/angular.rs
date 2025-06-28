use crate::constant::FLT64_ZERO;
use crate::spatial_geometry::quaternion::Quaternion;
use super::*;

impl Vector3<Angular> {
    pub fn to_f32_array(&self) -> [f32; 3] {
        let result: [f32; 3] = [self.x.as_rad() as f32, self.y.as_rad() as f32, self.z.as_rad() as f32];
        result
    }

    pub fn to_quaternion(&self) -> Quaternion {
        let norm = self.norm();
        if norm.as_rad() < FLT64_ZERO {
            return Quaternion::default();
        } else {
            let tmp = (norm * 0.5).sin() / norm.as_rad();
            let mut q = Quaternion::default();
            q.q0 = (norm * 0.5).cos();
            q.q1 = (self.x * tmp).as_rad();
            q.q2 = (self.y * tmp).as_rad();
            q.q3 = (self.z * tmp).as_rad();
            return q;
        }
    }

    pub fn sin(&self) -> Vector3<Coef> {
        let x = self.x.sin();
        let y = self.y.sin();
        let z = self.z.sin();
        Vector3 {
            x: Coef::new(x),
            y: Coef::new(y),
            z: Coef::new(z),
        }
    }

    pub fn cos(&self) -> Vector3<Coef> {
        let x = self.x.cos();
        let y = self.y.cos();
        let z = self.z.cos();
        Vector3 {
            x: Coef::new(x),
            y: Coef::new(y),
            z: Coef::new(z),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_to_f32_array() {
        let vector = Vector3 {
            x: Angular::from_rad(1.0),
            y: Angular::from_rad(2.0),
            z: Angular::from_rad(3.0),
        };
        let result = vector.to_f32_array();
        assert_eq!(result, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_to_quaternion() {
        let vector: Vector3<Angular> = Vector3 {
            x: Angular::from_rad(-0.002357),
            y: Angular::from_rad(0.0144422),
            z: Angular::from_rad(0.0065734),
        };
        let q = vector.to_quaternion();
        assert_relative_eq!(q.q0,0.9999678,epsilon = 1e-7);
        assert_relative_eq!(q.q1,-0.0011784874,epsilon = 1e-7);
        assert_relative_eq!(q.q2, 0.007221023,epsilon = 1e-7);
        assert_relative_eq!(q.q3, 0.0032866648,epsilon = 1e-7);

        let vecotr: Vector3<Angular> = Vector3 {
            x: Angular::from_rad(0.0),
            y: Angular::from_rad(0.0),
            z: Angular::from_rad(0.0),
        };
        let q = vecotr.to_quaternion();
        assert_relative_eq!(q.q0,1.0,epsilon = 1e-7);
        assert_relative_eq!(q.q1,0.0,epsilon = 1e-7);
        assert_relative_eq!(q.q2, 0.0,epsilon = 1e-7);
        assert_relative_eq!(q.q3, 0.0,epsilon = 1e-7);
    }

    #[test]
    fn test_sin() {
        let vector: Vector3<Angular> = Vector3 {
            x: Angular::from_rad(PI / 6.0),
            y: Angular::from_rad(PI / 6.0),
            z: Angular::from_rad(PI / 6.0),
        };
        let result = vector.sin();
        assert_relative_eq!(result.x.get_value(), 0.5, epsilon = 1e-7);
        assert_relative_eq!(result.y.get_value(), 0.5, epsilon = 1e-7);
        assert_relative_eq!(result.z.get_value(), 0.5, epsilon = 1e-7);
    }

    #[test]
    fn test_cos() {
        let vector: Vector3<Angular> = Vector3 {
            x: Angular::from_rad(PI / 3.0),
            y: Angular::from_rad(PI / 3.0),
            z: Angular::from_rad(PI / 3.0),
        };
        let result = vector.cos();
        assert_relative_eq!(result.x.get_value(), 0.5, epsilon = 1e-7);
        assert_relative_eq!(result.y.get_value(), 0.5, epsilon = 1e-7);
        assert_relative_eq!(result.z.get_value(), 0.5, epsilon = 1e-7);
    }

}