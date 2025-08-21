use super::*;
use crate::constant::FLT64_ZERO;
use crate::spatial_geometry::quaternion::Quaternion;

impl Vector3<Angular> {
    pub fn to_f32_array(&self) -> [f32; 3] {
        let result: [f32; 3] = [
            self.x.as_rad() as f32,
            self.y.as_rad() as f32,
            self.z.as_rad() as f32,
        ];
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

    pub fn mod_to_round(&self) -> Vector3<Angular> {
        let mut result: Vector3<Angular> = Vector3::new(
            self.x.mod_to_round(),
            self.y.mod_to_round(),
            self.z.mod_to_round(),
        );
        return result;
    }

    pub fn mod_to_round_half(&self) -> Vector3<Angular> {
        let mut result: Vector3<Angular> = Vector3::new(
            self.x.mod_to_round_half(),
            self.y.mod_to_round_half(),
            self.z.mod_to_round_half(),
        );
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;
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
        assert_relative_eq!(q.q0, 0.9999678, epsilon = 1e-7);
        assert_relative_eq!(q.q1, -0.0011784874, epsilon = 1e-7);
        assert_relative_eq!(q.q2, 0.007221023, epsilon = 1e-7);
        assert_relative_eq!(q.q3, 0.0032866648, epsilon = 1e-7);

        let vecotr: Vector3<Angular> = Vector3 {
            x: Angular::from_rad(0.0),
            y: Angular::from_rad(0.0),
            z: Angular::from_rad(0.0),
        };
        let q = vecotr.to_quaternion();
        assert_relative_eq!(q.q0, 1.0, epsilon = 1e-7);
        assert_relative_eq!(q.q1, 0.0, epsilon = 1e-7);
        assert_relative_eq!(q.q2, 0.0, epsilon = 1e-7);
        assert_relative_eq!(q.q3, 0.0, epsilon = 1e-7);
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

    // Test data generators
    fn generate_test_vectors() -> Vec<Vector3<Angular>> {
        vec![
            // Basic cases
            Vector3::new(
                Angular::from_rad(0.0),
                Angular::from_rad(0.0),
                Angular::from_rad(0.0)
            ),
            Vector3::new(
                Angular::from_rad(PI),
                Angular::from_rad(PI),
                Angular::from_rad(PI)
            ),
            // Edge cases
            Vector3::new(
                Angular::from_rad(2.0 * PI),
                Angular::from_rad(2.0 * PI),
                Angular::from_rad(2.0 * PI)
            ),
            Vector3::new(
                Angular::from_rad(PI / 2.0),
                Angular::from_rad(3.0 * PI / 2.0),
                Angular::from_rad(PI / 4.0)
            ),
            // Extreme values
            Vector3::new(
                Angular::from_rad(1000.0 * PI),
                Angular::from_rad(-500.0 * PI),
                Angular::from_rad(123.456 * PI)
            ),
        ]
    }

    #[test]
    fn test_mod_to_round() {
        // Test normal rounding behavior
        let vectors = generate_test_vectors();
        for vector in vectors {
            let result = vector.mod_to_round();

            // Check each component is properly rounded
            assert_relative_eq!(result.x.as_rad(), vector.x.mod_to_round().as_rad(), epsilon = 1e-7);
            assert_relative_eq!(result.y.as_rad(), vector.y.mod_to_round().as_rad(), epsilon = 1e-7);
            assert_relative_eq!(result.z.as_rad(), vector.z.mod_to_round().as_rad(), epsilon = 1e-7);

            // Verify results are within expected range [0, 2π)
            assert!(result.x.as_rad() >= 0.0 && result.x.as_rad() < 2.0 * PI);
            assert!(result.y.as_rad() >= 0.0 && result.y.as_rad() < 2.0 * PI);
            assert!(result.z.as_rad() >= 0.0 && result.z.as_rad() < 2.0 * PI);
        }
    }

    #[test]
    fn test_mod_to_round_half() {
        // Test rounding to nearest half interval behavior
        let vectors = generate_test_vectors();
        for vector in vectors {
            let result = vector.mod_to_round_half();

            // Check each component is properly rounded to half intervals
            assert_relative_eq!(result.x.as_rad(), vector.x.mod_to_round_half().as_rad(), epsilon = 1e-7);
            assert_relative_eq!(result.y.as_rad(), vector.y.mod_to_round_half().as_rad(), epsilon = 1e-7);
            assert_relative_eq!(result.z.as_rad(), vector.z.mod_to_round_half().as_rad(), epsilon = 1e-7);

            // Verify results are within expected range [0, 2π)
            assert!(result.x.as_rad() >= 0.0 && result.x.as_rad() < 2.0 * PI);
            assert!(result.y.as_rad() >= 0.0 && result.y.as_rad() < 2.0 * PI);
            assert!(result.z.as_rad() >= 0.0 && result.z.as_rad() < 2.0 * PI);
        }
    }

    #[test]
    fn test_zero_vector() {
        // Special test for zero vector
        let zero = Vector3::new(
            Angular::from_rad(0.0),
            Angular::from_rad(0.0),
            Angular::from_rad(0.0)
        );

        let rounded = zero.mod_to_round();
        assert_eq!(rounded.x.as_rad(), 0.0);
        assert_eq!(rounded.y.as_rad(), 0.0);
        assert_eq!(rounded.z.as_rad(), 0.0);

        let rounded_half = zero.mod_to_round_half();
        assert_eq!(rounded_half.x.as_rad(), 0.0);
        assert_eq!(rounded_half.y.as_rad(), 0.0);
        assert_eq!(rounded_half.z.as_rad(), 0.0);
    }

    #[test]
    fn test_full_rotation() {
        // Test vectors representing full rotations
        let full_rotation = Vector3::new(
            Angular::from_rad(2.0 * PI),
            Angular::from_rad(4.0 * PI),
            Angular::from_rad(6.0 * PI)
        );

        let rounded = full_rotation.mod_to_round();
        assert_relative_eq!(rounded.x.as_rad(), 0.0, epsilon = 1e-7);
        assert_relative_eq!(rounded.y.as_rad(), 0.0, epsilon = 1e-7);
        assert_relative_eq!(rounded.z.as_rad(), 0.0, epsilon = 1e-7);

        let rounded_half = full_rotation.mod_to_round_half();
        assert_relative_eq!(rounded_half.x.as_rad(), 0.0, epsilon = 1e-7);
        assert_relative_eq!(rounded_half.y.as_rad(), 0.0, epsilon = 1e-7);
        assert_relative_eq!(rounded_half.z.as_rad(), 0.0, epsilon = 1e-7);
    }
}
