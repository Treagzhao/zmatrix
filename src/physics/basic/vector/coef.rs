use crate::physics::basic::{Angular, Coef, Vector3};

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
    #[test]
    fn test_from() {
        let input = Vector3::new(Angular::from_rad(1.0), Angular::from_rad(2.0), Angular::from_rad(3.0));
        let result = Vector3::<Coef>::from(input);
        assert_eq!(result.x.get_value(),1.0);
        assert_eq!(result.y.get_value(),2.0);
        assert_eq!(result.z.get_value(),3.0);
    }
}