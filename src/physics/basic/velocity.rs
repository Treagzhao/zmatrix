use std::ops::{Add, Div, Mul, Neg, Sub};
use std::time::Duration;
use crate::physics::basic::VelocityType::LightSpeed;
use super::*;

const VELOCITY_OF_LIGHT: f64 = 299_792.458;  // km/s

impl Velocity {
    pub fn from_m_per_sec(v: f64) -> Self {
        Velocity {
            default_type: VelocityType::MPerSecond,
            v,
        }
    }

    pub fn from_light_speed(v: f64) -> Self {
        Velocity {
            default_type: VelocityType::LightSpeed,
            v,
        }
    }

    pub fn from_km_per_h(v: f64) -> Self {
        Velocity {
            default_type: VelocityType::KmPerHour,
            v,
        }
    }

    pub fn from_km_per_sec(v: f64) -> Self {
        Velocity {
            default_type: VelocityType::KmPerSecond,
            v,
        }
    }

    pub fn as_m_per_sec(&self) -> f64 {
        return match self.default_type {
            VelocityType::MPerSecond => self.v,
            VelocityType::KmPerHour => self.v * 1000.0 / 3600.0,
            VelocityType::KmPerSecond => self.v * 1000.0,
            VelocityType::LightSpeed => self.v * VELOCITY_OF_LIGHT * 1E3,
        };
    }

    pub fn as_km_per_h(&self) -> f64 {
        match self.default_type {
            VelocityType::KmPerHour => self.v,
            VelocityType::MPerSecond => self.v * 3600.0 / 1000.0,
            VelocityType::KmPerSecond => self.v * 3600.0,
            VelocityType::LightSpeed => VELOCITY_OF_LIGHT * 3600.0,
        }
    }

    pub fn as_km_per_sec(&self) -> f64 {
        return match self.default_type {
            VelocityType::KmPerSecond => self.v,
            VelocityType::MPerSecond => self.v / 1000.0,
            VelocityType::KmPerHour => self.v / 3600.0,
            VelocityType::LightSpeed => VELOCITY_OF_LIGHT,
        };
    }

    pub fn as_light_speed(&self) -> f64 {
        match self.default_type {
            VelocityType::LightSpeed => self.v,
            VelocityType::MPerSecond => self.v / (VELOCITY_OF_LIGHT * 1e3),
            VelocityType::KmPerHour => self.v  / VELOCITY_OF_LIGHT / 3600.0,
            VelocityType::KmPerSecond => self.v / VELOCITY_OF_LIGHT,
        }
    }
}
impl Default for Velocity {
    fn default() -> Self {
        Velocity::from_m_per_sec(0.0)
    }
}

impl Neg for Velocity {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_m_per_sec(-self.v)
    }
}
impl Mul<Duration> for Velocity {
    type Output = Distance;

    fn mul(self, duration: Duration) -> Self::Output {
        let v = self.as_m_per_sec() * duration.as_secs_f64();
        Distance::from_m(v)
    }
}

impl Div<Duration> for Velocity {
    type Output = Acceleration;
    fn div(self, duration: Duration) -> Self::Output {
        let v = self.as_m_per_sec() / duration.as_secs_f64();
        Acceleration::from_m_per_s2(v)
    }
}

impl Sub for Velocity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_m_per_sec() - rhs.as_m_per_sec();
        Self::from_m_per_sec(v)
    }
}

impl Sub<f64> for Velocity {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            VelocityType::MPerSecond => {
                let v = self.as_m_per_sec() - rhs;
                Self::from_m_per_sec(v)
            }
            VelocityType::KmPerHour => {
                let v = self.as_km_per_h() - rhs;
                Self::from_km_per_h(v)
            }
            VelocityType::KmPerSecond => {
                let v = self.as_km_per_sec() - rhs;
                Self::from_km_per_sec(v)
            }
            VelocityType::LightSpeed => {
                let v = self.as_light_speed() - rhs;
                Self::from_light_speed(v)
            }
        };
    }
}

impl PhysicalQuantity for Velocity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn is_zero(&self) -> bool {
        self.v == 0.0
    }
    fn default_unit_value(&self) -> f64 {
        self.as_m_per_sec()
    }
    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Add for Velocity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_m_per_sec() + rhs.as_m_per_sec();
        Self::from_m_per_sec(v)
    }
}

impl Add<f64> for Velocity {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            VelocityType::MPerSecond => {
                let v = self.as_m_per_sec() + rhs;
                Self::from_m_per_sec(v)
            }
            VelocityType::KmPerHour => {
                let v = self.as_km_per_h() + rhs;
                Self::from_km_per_h(v)
            }
            VelocityType::KmPerSecond => {
                let v = self.as_km_per_sec() + rhs;
                Self::from_km_per_sec(v)
            }
            VelocityType::LightSpeed => {
                let v = self.as_light_speed() + rhs;
                Self::from_light_speed(v)
            }
        };
    }
}


impl Mul<f64> for Velocity {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_m_per_sec() * rhs;
        Self::from_m_per_sec(v)
    }
}

impl Mul<Mass> for Velocity {
    type Output = Momentum;

    fn mul(self, rhs: Mass) -> Self::Output {
        let v = self.as_m_per_sec() * rhs.as_kg();
        Momentum::from_kg_m_s(v)
    }
}

impl Div<f64> for Velocity {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_m_per_sec() / rhs;
        Self::from_m_per_sec(v)
    }
}

impl Mul<Velocity> for f64 {
    type Output = Velocity;
    fn mul(self, rhs: Velocity) -> Self::Output {
        rhs * self
    }
}

impl Div<Velocity> for f64 {
    type Output = Velocity;
    fn div(self, rhs: Velocity) -> Self::Output {
        let v = self / rhs.v;
        Velocity {
            default_type: rhs.default_type,
            v: v,
        }
    }
}

impl Mul<Coef> for Velocity {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_m_per_sec() * rhs.get_value();
        Self::from_m_per_sec(v)
    }
}

impl Div<Coef> for Velocity {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_m_per_sec() / rhs.get_value();
        Self::from_m_per_sec(v)
    }
}

impl Div<Distance> for Velocity {
    type Output = AngularVelocity;
    fn div(self, rhs: Distance) -> Self::Output {
        let v = self.as_m_per_sec() / rhs.as_m();
        AngularVelocity::from_rad_per_second(v)
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_from() {
        let v = Velocity::from_m_per_sec(1.0);
        assert_eq!(v.v, 1.0);
        assert_eq!(v.default_type, VelocityType::MPerSecond);

        let v = Velocity::from_km_per_h(1.0);
        assert_eq!(v.v, 1.0);
        assert_eq!(v.default_type, VelocityType::KmPerHour);

        let v = Velocity::from_km_per_sec(100.0);
        assert_eq!(v.v, 100.0);
        assert_eq!(v.default_type, VelocityType::KmPerSecond);
    }
    #[test]
    fn test_default() {
        let v = Velocity::default();
        assert_eq!(v.v, 0.0);
        assert_eq!(v.default_type, VelocityType::MPerSecond);
    }
    #[test]
    fn test_as() {
        let v = Velocity::from_m_per_sec(1.0);
        assert_eq!(v.as_m_per_sec(), 1.0);
        assert_eq!(v.as_km_per_h(), 3.6);
        assert_relative_eq!(v.as_km_per_sec(),0.001);
        assert_relative_eq!(v.as_light_speed(),3.33564 * 1e-9,epsilon=1e-6);

        let v = Velocity::from_km_per_h(100.0);
        assert_eq!(v.as_km_per_h(), 100.0);
        assert_eq!(v.as_m_per_sec(), 27.77777777777778);
        assert_eq!(v.as_km_per_sec(), 100.0 / 3600.0);
        assert_relative_eq!(v.as_light_speed(),9.266 * 1e-8,epsilon=1e-6);

        let v = Velocity::from_km_per_sec(100.0);
        assert_eq!(v.as_m_per_sec(), 100000.0);
        assert_relative_eq!(v.as_km_per_h(),100.0*3600.0);
        assert_relative_eq!(v.as_km_per_sec(),100.0);
        assert_relative_eq!(v.as_light_speed(),3.33564*1e-4,epsilon = 1e-6);

        let v = Velocity::from_light_speed(1.0);
        assert_relative_eq!(v.as_m_per_sec(), 299792458.0);
        assert_relative_eq!(v.as_km_per_h(),1.0792528*1e9,epsilon=1e2);
        assert_relative_eq!(v.as_km_per_sec(),299792.4580);
    }

    #[test]
    fn test_mul() {
        let v = Velocity::from_m_per_sec(3.0);
        let duration = Duration::from_secs_f64(15.0);
        let d = v * duration;
        assert_eq!(d.as_m(), 45.0);
        assert_eq!(d.as_km(), 0.045);

        let v = Velocity::from_m_per_sec(3.0);
        let duration = Duration::from_millis(500);
        let d = v * duration;
        assert_eq!(d.as_m(), 1.5);
        assert_eq!(d.as_km(), 0.0015);

        let v = Velocity::from_km_per_h(36.0);
        let duration = Duration::from_millis(500);
        let d = v * duration;
        assert_eq!(d.as_m(), 5.0);
        assert_eq!(d.as_km(), 0.005);

        let v = Velocity::from_km_per_h(36.0);
        let duration = Duration::from_secs_f64(500.0);
        let d = v * duration;
        assert_eq!(d.as_m(), 5000.0);
        assert_eq!(d.as_km(), 5.0);
    }

    #[test]
    fn test_div() {
        let v = Velocity::from_m_per_sec(1.0);
        let duration = Duration::from_secs_f64(15.0);
        let d = v / duration;
        assert_eq!(d.as_m_per_s2(), 0.06666666666666667);
    }

    #[test]
    fn test_sub() {
        {
            let v1 = Velocity::from_m_per_sec(15.0);
            let v2 = Velocity::from_m_per_sec(10.0);
            let v3 = v1 - v2;
            assert_relative_eq!(v3.as_m_per_sec(), 5.0);

            let v1 = Velocity::from_km_per_h(15.0);
            let v2 = Velocity::from_km_per_h(10.0);
            let v3 = v1 - v2;
            assert_relative_eq!(v3.as_km_per_h(), 5.0,epsilon = 1e-8);

            let v1 = Velocity::from_m_per_sec(5.0);
            let v2 = Velocity::from_km_per_h(10.0);
            let v3 = v1 - v2;
            assert_relative_eq!(v3.as_m_per_sec(), 2.2222222222222, epsilon = 1e-8);

            let v1 = Velocity::from_km_per_h(5.0);
            let v2 = Velocity::from_km_per_h(3.0);
            let r = v1 -v2 ;
            assert_relative_eq!(r.as_km_per_h(), 2.0,epsilon = 1e-8);

            let v1 = Velocity::from_light_speed(1.0);
            let v2 = Velocity::from_light_speed(0.5);
            let r = v1 - v2;
            assert_relative_eq!(r.as_light_speed(), 0.5,epsilon = 1e-8);
        }
        {
            let v1 = Velocity::from_m_per_sec(15.0);
            let v2 = v1 - 10.0;
            assert_relative_eq!(v2.as_m_per_sec(), 5.0);

            let v1 = Velocity::from_km_per_h(15.0);
            let v2 = v1 - 10.0;
            assert_relative_eq!(v2.as_km_per_h(), 5.0,epsilon = 1e-8);

            let v1 = Velocity::from_km_per_h(15.0);
            let r = v1 - 10.0;
            assert_relative_eq!(r.as_km_per_h(), 5.0,epsilon = 1e-8);

            let v1 = Velocity::from_light_speed(1.0);
            let r = v1 - 0.4;
            assert_relative_eq!(r.as_light_speed(), 0.6,epsilon = 1e-8);
        }
    }

    #[test]
    fn test_add() {
        {
            let v1 = Velocity::from_m_per_sec(1.0);
            let v2 = Velocity::from_m_per_sec(2.0);
            let v3 = v1 + v2;
            assert_relative_eq!(v3.as_m_per_sec(), 3.0);

            let v1 = Velocity::from_m_per_sec(1.0);
            let v2 = Velocity::from_km_per_h(2.0);
            let v3 = v1 + v2;
            assert_relative_eq!(v3.as_m_per_sec(), 1.5555555555555556,epsilon = 1e-8);
        }
        {
            let v1 = Velocity::from_m_per_sec(1.0);
            let v2 = v1 + 10.0;
            assert_relative_eq!(v2.as_m_per_sec(), 11.0);

            let v1 = Velocity::from_km_per_h(1.0);
            let v2 = v1 + 10.0;
            assert_relative_eq!(v2.as_km_per_h(), 11.0);
        }
    }


    #[test]
    fn test_to_mul() {
        let d1 = Velocity::from_m_per_sec(1000.0);
        let d2 = d1 * 2.0;
        assert_eq!(d2.as_m_per_sec(), 2000.0);
        let d1 = Velocity::from_m_per_sec(1000.0);
        let d2 = d1 * Coef::new(2.0);
        assert_eq!(d2.as_m_per_sec(), 2000.0);
    }

    #[test]
    fn test_to_div() {
        let d1 = Velocity::from_m_per_sec(1000.0);
        let d2 = d1 / 2.0;
        assert_eq!(d2.as_m_per_sec(), 500.0);
        let d1 = Velocity::from_m_per_sec(1000.0);
        let d2 = d1 / Coef::new(2.0);
        assert_eq!(d2.as_m_per_sec(), 500.0);
    }

    #[test]
    fn test_as_any() {
        let v = Velocity::from_m_per_sec(1.0);
        let v_any = v.as_any();
        let a = v_any.downcast_ref::<Velocity>().unwrap();
    }

    #[test]
    fn test_is_zero() {
        let v = Velocity::from_m_per_sec(0.0);
        assert!(v.is_zero());
        let v = Velocity::from_m_per_sec(1.0);
        assert!(!v.is_zero());
    }

    #[test]
    fn test_negative() {
        let v = Velocity::from_m_per_sec(-1.0);
        assert_eq!(v.as_m_per_sec(), -1.0);
        let v1 = -v;
        assert_eq!(v1.as_m_per_sec(), 1.0);
    }

    #[test]
    fn test_convert() {
        let m1 = Mass::from_kg(2.0);
        let v1 = Velocity::from_m_per_sec(2.0);
        let m2 = v1 * m1;
        assert_eq!(m2.default_type, MomentumType::KgMperSecond);
        assert_relative_eq!(m2.as_kg_m_s(),4.0);

        let v1 = Velocity::from_m_per_sec(5.0);
        let d1 = Distance::from_m(2.0);
        let omg1 = v1 / d1;
        assert_relative_eq!(omg1.as_rad_per_second(),2.5);
    }

    #[test]
    fn test_f64_mul_velocity() {
        // f64 * Velocity 测试
        let v = Velocity::from_m_per_sec(2.0);
        let result = 3.0 * v;
        assert_relative_eq!(result.as_m_per_sec(), 6.0);

        let v = Velocity::from_km_per_h(100.0);
        let result = 2.0 * v;
        assert_relative_eq!(result.as_km_per_h(), 200.0);

        let v = Velocity::from_light_speed(0.5);
        let result = 2.0 * v;
        assert_relative_eq!(result.as_light_speed(), 1.0);
    }

    #[test]
    fn test_f64_div_velocity() {
        // f64 / Velocity 测试
        let v = Velocity::from_m_per_sec(2.0);
        let result = 6.0 / v;
        assert_relative_eq!(result.as_m_per_sec(), 3.0);

        let v = Velocity::from_km_per_h(100.0);
        let result = 200.0 / v;
        assert_relative_eq!(result.as_km_per_h(), 2.0);

        let v = Velocity::from_light_speed(0.5);
        let result = 1.0 / v;
        assert_relative_eq!(result.as_light_speed(), 2.0);
    }
}