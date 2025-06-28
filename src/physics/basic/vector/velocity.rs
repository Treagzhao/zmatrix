use std::ops::Mul;
use crate::physics::basic::{Mass, Momentum, Vector3, Velocity};

impl Mul<Mass> for Vector3<Velocity> {
    type Output = Vector3<Momentum>;

    fn mul(self, rhs: Mass) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_to_momentum() {
        let vec = Vector3::new(Velocity::from_m_per_sec(1.0), Velocity::from_m_per_sec(2.0), Velocity::from_m_per_sec(3.0));
        let mass = Mass::from_kg(4.0);
        let momentum = vec * mass;
        assert_relative_eq!(4.0, momentum.x.as_kg_m_s());
        assert_relative_eq!(8.0, momentum.y.as_kg_m_s());
        assert_relative_eq!(12.0, momentum.z.as_kg_m_s());
    }
}