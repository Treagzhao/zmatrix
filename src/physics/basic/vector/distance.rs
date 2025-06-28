use std::ops::{Div, Mul};
use std::time::Duration;
use crate::physics::basic::{AngularMomentum, Distance, Mass, Momentum, Vector3, Velocity};

impl Mul<Vector3<Momentum>> for Vector3<Distance> {
    type Output = Vector3<AngularMomentum>;
    fn mul(self, rhs: Vector3<Momentum>) -> Self::Output {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        Vector3::new(x, y, z)
    }
}

impl Div<Duration> for Vector3<Distance> {
    type Output =Vector3<Velocity>;

    fn div(self, rhs: Duration) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Vector3::<Velocity>::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_distance() {
        let vec = Vector3::new(Distance::from_m(1.0), Distance::from_m(2.0), Distance::from_m(3.0));
        let m: Vector3<Momentum> = Vector3::new(Momentum::from_kg_m_s(3.0), Momentum::from_kg_m_s(5.0), Momentum::from_kg_m_s(7.0));
        let result = vec * m;
        assert_eq!(-1.0, result.x.as_kg_m2_per_second());
        assert_eq!(2.0, result.y.as_kg_m2_per_second());
        assert_eq!(-1.0, result.z.as_kg_m2_per_second());
    }

    #[test]
    fn test_distance_to_velocity(){
        let a:Vector3<Distance> = Vector3::new(
            Distance::from_m(10.0),
            Distance::from_m(20.0),
            Distance::from_m(30.0),
        );
        let v = a / Duration::from_secs(10);
        assert_relative_eq!(v.x.as_m_per_sec(),1.0);
        assert_relative_eq!(v.y.as_m_per_sec(),2.0);
        assert_relative_eq!(v.z.as_m_per_sec(),3.0);
    }
}
