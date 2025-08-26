use crate::physics::basic::{
    AngularMomentum, Coef, Distance, DistanceType, Mass, Momentum, Vector3, Velocity,
};
use std::ops::{Div, Mul};
use std::time::Duration;

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
    type Output = Vector3<Velocity>;

    fn div(self, rhs: Duration) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Vector3::<Velocity>::new(x, y, z)
    }
}

impl Vector3<Distance> {
    pub fn to_vector3_coef(&self, distance_type: DistanceType) -> Vector3<Coef> {
        match distance_type {
            DistanceType::M => {
                let (x, y, z) = (self.x.as_m(), self.y.as_m(), self.z.as_m());
                Vector3::<Coef>::from_array([x, y, z])
            }
            DistanceType::KM => {
                let (x, y, z) = (self.x.as_km(), self.y.as_km(), self.z.as_km());
                Vector3::<Coef>::from_array([x, y, z])
            }
            DistanceType::LightYear => {
                let (x, y, z) = (
                    self.x.as_light_year(),
                    self.y.as_light_year(),
                    self.z.as_light_year(),
                );
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, distance_type: DistanceType) -> Vector3<Distance> {
        let x = Distance {
            v: coef.x.v,
            default_type: distance_type,
        };
        let y = Distance {
            v: coef.y.v,
            default_type: distance_type,
        };
        let z = Distance {
            v: coef.z.v,
            default_type: distance_type,
        };
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_distance() {
        let vec = Vector3::new(
            Distance::from_m(1.0),
            Distance::from_m(2.0),
            Distance::from_m(3.0),
        );
        let m: Vector3<Momentum> = Vector3::new(
            Momentum::from_kg_m_s(3.0),
            Momentum::from_kg_m_s(5.0),
            Momentum::from_kg_m_s(7.0),
        );
        let result = vec * m;
        assert_eq!(-1.0, result.x.as_kg_m2_per_second());
        assert_eq!(2.0, result.y.as_kg_m2_per_second());
        assert_eq!(-1.0, result.z.as_kg_m2_per_second());
    }

    #[test]
    fn test_distance_to_velocity() {
        let a: Vector3<Distance> = Vector3::new(
            Distance::from_m(10.0),
            Distance::from_m(20.0),
            Distance::from_m(30.0),
        );
        let v = a / Duration::from_secs(10);
        assert_relative_eq!(v.x.as_m_per_sec(), 1.0);
        assert_relative_eq!(v.y.as_m_per_sec(), 2.0);
        assert_relative_eq!(v.z.as_m_per_sec(), 3.0);
    }

    #[test]
    fn test_to_vector3_coef_meters() {
        let distance_vec = Vector3::new(
            Distance::from_m(1000.0),
            Distance::from_m(2000.0),
            Distance::from_m(3000.0),
        );

        let coef_vec = distance_vec.to_vector3_coef(DistanceType::M);

        assert_relative_eq!(coef_vec.x, 1000.0);
        assert_relative_eq!(coef_vec.y, 2000.0);
        assert_relative_eq!(coef_vec.z, 3000.0);
    }

    #[test]
    fn test_to_vector3_coef_kilometers() {
        let distance_vec = Vector3::new(
            Distance::from_m(1000.0),
            Distance::from_m(2000.0),
            Distance::from_m(3000.0),
        );

        let coef_vec = distance_vec.to_vector3_coef(DistanceType::KM);

        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_to_vector3_coef_light_years() {
        let distance_vec = Vector3::new(
            Distance::from_m(9.461e15), // 1光年
            Distance::from_m(1.892e16), // 2光年
            Distance::from_m(2.838e16), // 3光年
        );

        let coef_vec = distance_vec.to_vector3_coef(DistanceType::LightYear);

        assert_relative_eq!(coef_vec.x, 1.0, epsilon = 1e-3);
        assert_relative_eq!(coef_vec.y, 2.0, epsilon = 1e-3);
        assert_relative_eq!(coef_vec.z, 3.0, epsilon = 1e-3);
    }

    #[test]
    fn test_to_vector3_coef_zero_distance() {
        let distance_vec = Vector3::new(
            Distance::from_m(0.0),
            Distance::from_m(0.0),
            Distance::from_m(0.0),
        );

        let coef_vec = distance_vec.to_vector3_coef(DistanceType::M);

        assert_relative_eq!(coef_vec.x, 0.0);
        assert_relative_eq!(coef_vec.y, 0.0);
        assert_relative_eq!(coef_vec.z, 0.0);
    }

    #[test]
    fn test_to_vector3_coef_negative_distance() {
        let distance_vec = Vector3::new(
            Distance::from_m(-100.0),
            Distance::from_m(-200.0),
            Distance::from_m(-300.0),
        );

        let coef_vec = distance_vec.to_vector3_coef(DistanceType::M);

        assert_relative_eq!(coef_vec.x, -100.0);
        assert_relative_eq!(coef_vec.y, -200.0);
        assert_relative_eq!(coef_vec.z, -300.0);
    }

    #[test]
    fn test_to_vector3_coef_mixed_units() {
        // 测试从千米创建的距离向量转换为米
        let distance_vec = Vector3::new(
            Distance::from_km(1.5),
            Distance::from_km(2.5),
            Distance::from_km(3.5),
        );

        let coef_vec = distance_vec.to_vector3_coef(DistanceType::M);

        assert_relative_eq!(coef_vec.x, 1500.0);
        assert_relative_eq!(coef_vec.y, 2500.0);
        assert_relative_eq!(coef_vec.z, 3500.0);
    }

    #[test]
    fn test_to_vector3_coef_all_distance_types() {
        let distance_vec = Vector3::new(
            Distance::from_m(1000.0),
            Distance::from_m(2000.0),
            Distance::from_m(3000.0),
        );

        // 测试所有距离类型
        let coef_m = distance_vec.to_vector3_coef(DistanceType::M);
        let coef_km = distance_vec.to_vector3_coef(DistanceType::KM);
        let coef_ly = distance_vec.to_vector3_coef(DistanceType::LightYear);

        // 验证米
        assert_relative_eq!(coef_m.x, 1000.0);
        assert_relative_eq!(coef_m.y, 2000.0);
        assert_relative_eq!(coef_m.z, 3000.0);

        // 验证千米
        assert_relative_eq!(coef_km.x, 1.0);
        assert_relative_eq!(coef_km.y, 2.0);
        assert_relative_eq!(coef_km.z, 3.0);

        // 验证光年（应该是一个很小的值）
        assert!(coef_ly.x < Coef::new(1e-12));
        assert!(coef_ly.y < Coef::new(1e-12));
        assert!(coef_ly.z < Coef::new(1e-12));
    }

    #[test]
    fn test_from_vector_coef_meters() {
        let coef_vec = Vector3::new(
            Coef::new(1000.0),
            Coef::new(2000.0),
            Coef::new(3000.0),
        );

        let distance_vec = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::M);

        assert_relative_eq!(distance_vec.x.as_m(), 1000.0);
        assert_relative_eq!(distance_vec.y.as_m(), 2000.0);
        assert_relative_eq!(distance_vec.z.as_m(), 3000.0);
    }

    #[test]
    fn test_from_vector_coef_kilometers() {
        let coef_vec = Vector3::new(
            Coef::new(1.5),
            Coef::new(2.5),
            Coef::new(3.5),
        );

        let distance_vec = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::KM);

        assert_relative_eq!(distance_vec.x.as_km(), 1.5);
        assert_relative_eq!(distance_vec.y.as_km(), 2.5);
        assert_relative_eq!(distance_vec.z.as_km(), 3.5);
    }

    #[test]
    fn test_from_vector_coef_light_years() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let distance_vec = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::LightYear);

        assert_relative_eq!(distance_vec.x.as_light_year(), 1.0);
        assert_relative_eq!(distance_vec.y.as_light_year(), 2.0);
        assert_relative_eq!(distance_vec.z.as_light_year(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_zero_values() {
        let coef_vec = Vector3::new(
            Coef::new(0.0),
            Coef::new(0.0),
            Coef::new(0.0),
        );

        let distance_vec = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::M);

        assert_relative_eq!(distance_vec.x.as_m(), 0.0);
        assert_relative_eq!(distance_vec.y.as_m(), 0.0);
        assert_relative_eq!(distance_vec.z.as_m(), 0.0);
    }

    #[test]
    fn test_from_vector_coef_negative_values() {
        let coef_vec = Vector3::new(
            Coef::new(-100.0),
            Coef::new(-200.0),
            Coef::new(-300.0),
        );

        let distance_vec = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::M);

        assert_relative_eq!(distance_vec.x.as_m(), -100.0);
        assert_relative_eq!(distance_vec.y.as_m(), -200.0);
        assert_relative_eq!(distance_vec.z.as_m(), -300.0);
    }

    #[test]
    fn test_from_vector_coef_decimal_values() {
        let coef_vec = Vector3::new(
            Coef::new(3.14159),
            Coef::new(2.71828),
            Coef::new(1.41421),
        );

        let distance_vec = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::M);

        assert_relative_eq!(distance_vec.x.as_m(), 3.14159);
        assert_relative_eq!(distance_vec.y.as_m(), 2.71828);
        assert_relative_eq!(distance_vec.z.as_m(), 1.41421);
    }

    #[test]
    fn test_from_vector_coef_large_values() {
        let coef_vec = Vector3::new(
            Coef::new(1e9),
            Coef::new(2e9),
            Coef::new(3e9),
        );

        let distance_vec = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::M);

        assert_relative_eq!(distance_vec.x.as_m(), 1e9);
        assert_relative_eq!(distance_vec.y.as_m(), 2e9);
        assert_relative_eq!(distance_vec.z.as_m(), 3e9);
    }

    #[test]
    fn test_from_vector_coef_small_values() {
        let coef_vec = Vector3::new(
            Coef::new(1e-9),
            Coef::new(2e-9),
            Coef::new(3e-9),
        );

        let distance_vec = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::M);

        assert_relative_eq!(distance_vec.x.as_m(), 1e-9);
        assert_relative_eq!(distance_vec.y.as_m(), 2e-9);
        assert_relative_eq!(distance_vec.z.as_m(), 3e-9);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        // 测试 from_vector_coef 和 to_vector3_coef 的往返转换
        let original_distance_vec = Vector3::new(
            Distance::from_m(1234.567),
            Distance::from_m(2345.678),
            Distance::from_m(3456.789),
        );

        // 先转换为系数向量
        let coef_vec = original_distance_vec.to_vector3_coef(DistanceType::M);
        
        // 再从系数向量创建距离向量
        let reconstructed_distance_vec = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::M);

        // 验证往返转换的一致性
        assert_relative_eq!(original_distance_vec.x.as_m(), reconstructed_distance_vec.x.as_m());
        assert_relative_eq!(original_distance_vec.y.as_m(), reconstructed_distance_vec.y.as_m());
        assert_relative_eq!(original_distance_vec.z.as_m(), reconstructed_distance_vec.z.as_m());
    }

    #[test]
    fn test_from_vector_coef_all_distance_types() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        // 测试所有距离类型
        let distance_m = Vector3::<Distance>::from_vector_coef(coef_vec.clone(), DistanceType::M);
        let distance_km = Vector3::<Distance>::from_vector_coef(coef_vec.clone(), DistanceType::KM);
        let distance_ly = Vector3::<Distance>::from_vector_coef(coef_vec, DistanceType::LightYear);

        // 验证米
        assert_relative_eq!(distance_m.x.as_m(), 1.0);
        assert_relative_eq!(distance_m.y.as_m(), 2.0);
        assert_relative_eq!(distance_m.z.as_m(), 3.0);

        // 验证千米
        assert_relative_eq!(distance_km.x.as_km(), 1.0);
        assert_relative_eq!(distance_km.y.as_km(), 2.0);
        assert_relative_eq!(distance_km.z.as_km(), 3.0);

        // 验证光年
        assert_relative_eq!(distance_ly.x.as_light_year(), 1.0);
        assert_relative_eq!(distance_ly.y.as_light_year(), 2.0);
        assert_relative_eq!(distance_ly.z.as_light_year(), 3.0);
    }
}
