use std::ops::Mul;
use crate::physics::basic::{Coef, Mass, Momentum, Vector3, Velocity, VelocityType};

impl Mul<Mass> for Vector3<Velocity> {
    type Output = Vector3<Momentum>;

    fn mul(self, rhs: Mass) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Vector3<Velocity> {
    pub fn to_vector3_coef(&self, velocity_type: VelocityType) -> Vector3<Coef> {
        match velocity_type {
            VelocityType::MPerSecond => {
                let (x, y, z) = (self.x.as_m_per_sec(), self.y.as_m_per_sec(), self.z.as_m_per_sec());
                Vector3::<Coef>::from_array([x, y, z])
            }
            VelocityType::KmPerHour => {
                let (x, y, z) = (self.x.as_km_per_h(), self.y.as_km_per_h(), self.z.as_km_per_h());
                Vector3::<Coef>::from_array([x, y, z])
            }
            VelocityType::KmPerSecond => {
                let (x, y, z) = (self.x.as_km_per_sec(), self.y.as_km_per_sec(), self.z.as_km_per_sec());
                Vector3::<Coef>::from_array([x, y, z])
            }
            VelocityType::LightSpeed => {
                let (x, y, z) = (self.x.as_light_speed(), self.y.as_light_speed(), self.z.as_light_speed());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, velocity_type: VelocityType) -> Vector3<Velocity> {
        let x = Velocity {
            v: coef.x.v,
            default_type: velocity_type,
        };
        let y = Velocity {
            v: coef.y.v,
            default_type: velocity_type,
        };
        let z = Velocity {
            v: coef.z.v,
            default_type: velocity_type,
        };
        Vector3::new(x, y, z)
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

    #[test]
    fn test_to_vector3_coef_m_per_sec() {
        let velocity_vec = Vector3::new(
            Velocity::from_m_per_sec(10.0),
            Velocity::from_m_per_sec(20.0),
            Velocity::from_m_per_sec(30.0),
        );
        
        let coef_vec = velocity_vec.to_vector3_coef(VelocityType::MPerSecond);
        
        assert_relative_eq!(coef_vec.x, 10.0);
        assert_relative_eq!(coef_vec.y, 20.0);
        assert_relative_eq!(coef_vec.z, 30.0);
    }

    #[test]
    fn test_to_vector3_coef_km_per_h() {
        let velocity_vec = Vector3::new(
            Velocity::from_m_per_sec(10.0),  // 10 m/s = 36 km/h
            Velocity::from_m_per_sec(20.0),  // 20 m/s = 72 km/h
            Velocity::from_m_per_sec(30.0),  // 30 m/s = 108 km/h
        );
        
        let coef_vec = velocity_vec.to_vector3_coef(VelocityType::KmPerHour);
        
        assert_relative_eq!(coef_vec.x, 36.0);
        assert_relative_eq!(coef_vec.y, 72.0);
        assert_relative_eq!(coef_vec.z, 108.0);
    }

    #[test]
    fn test_from_vector_coef_m_per_sec() {
        let coef_vec = Vector3::new(
            Coef::new(10.0),
            Coef::new(20.0),
            Coef::new(30.0),
        );

        let velocity_vec = Vector3::<Velocity>::from_vector_coef(coef_vec, VelocityType::MPerSecond);

        assert_relative_eq!(velocity_vec.x.as_m_per_sec(), 10.0);
        assert_relative_eq!(velocity_vec.y.as_m_per_sec(), 20.0);
        assert_relative_eq!(velocity_vec.z.as_m_per_sec(), 30.0);
    }

    #[test]
    fn test_from_vector_coef_km_per_h() {
        let coef_vec = Vector3::new(
            Coef::new(36.0),
            Coef::new(72.0),
            Coef::new(108.0),
        );

        let velocity_vec = Vector3::<Velocity>::from_vector_coef(coef_vec, VelocityType::KmPerHour);

        assert_relative_eq!(velocity_vec.x.as_km_per_h(), 36.0);
        assert_relative_eq!(velocity_vec.y.as_km_per_h(), 72.0);
        assert_relative_eq!(velocity_vec.z.as_km_per_h(), 108.0);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_velocity_vec = Vector3::new(
            Velocity::from_m_per_sec(15.0),
            Velocity::from_m_per_sec(25.0),
            Velocity::from_m_per_sec(35.0),
        );

        let coef_vec = original_velocity_vec.to_vector3_coef(VelocityType::MPerSecond);
        let reconstructed_velocity_vec = Vector3::<Velocity>::from_vector_coef(coef_vec, VelocityType::MPerSecond);

        assert_relative_eq!(original_velocity_vec.x.as_m_per_sec(), reconstructed_velocity_vec.x.as_m_per_sec());
        assert_relative_eq!(original_velocity_vec.y.as_m_per_sec(), reconstructed_velocity_vec.y.as_m_per_sec());
        assert_relative_eq!(original_velocity_vec.z.as_m_per_sec(), reconstructed_velocity_vec.z.as_m_per_sec());
    }
}