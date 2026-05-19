use crate::physics::basic::{Angular, Coef, Vector3};
use crate::utils::float;

impl Vector3<Coef> {
    pub fn limit_float(&self, threshold: f64) -> Self {
        let x = Coef::new(float::limit_float(self.x.v, threshold));
        let y = Coef::new(float::limit_float(self.y.v, threshold));
        let z = Coef::new(float::limit_float(self.z.v, threshold));
        Self { x, y, z }
    }
}

impl From<Vector3<Angular>> for Vector3<Coef> {
    fn from(input: Vector3<Angular>) -> Self {
        Self {
            x: Coef::from(input.x),
            y: Coef::from(input.y),
            z: Coef::from(input.z),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_from() {
        let input = Vector3::new(Angular::from_rad(1.0), Angular::from_rad(2.0), Angular::from_rad(3.0));
        let result = Vector3::<Coef>::from(input);
        assert_eq!(result.x.get_value(),1.0);
        assert_eq!(result.y.get_value(),2.0);
        assert_eq!(result.z.get_value(),3.0);
    }

    #[test]
    fn test_limit_float() {
        let v = Vector3::new(Coef::new(1.0), Coef::new(-5.0), Coef::new(3.0));
        let result = v.limit_float(3.0);
        assert_relative_eq!(result.x.get_value(), 1.0);
        assert_relative_eq!(result.y.get_value(), -3.0);
        assert_relative_eq!(result.z.get_value(), 3.0);
    }
}