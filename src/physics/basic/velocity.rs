use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;
use super::*;
impl Velocity {
    pub fn from_m_per_sec(v: f64) -> Self {
        Velocity {
            default_type: VelocityType::MPerSecond,
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
        };
    }

    pub fn as_km_per_h(&self) -> f64 {
        match self.default_type {
            VelocityType::KmPerHour => self.v,
            VelocityType::MPerSecond => self.v * 3600.0 / 1000.0,
            VelocityType::KmPerSecond => self.v * 3600.0,
        }
    }

    pub fn as_km_per_sec(&self) -> f64 {
        return match self.default_type {
            VelocityType::KmPerSecond => self.v,
            VelocityType::MPerSecond => self.v / 1000.0,
            VelocityType::KmPerHour => self.v / 3600.0,
        };
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
        };
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

impl Div<f64> for Velocity {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_m_per_sec() / rhs;
        Self::from_m_per_sec(v)
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


#[cfg(test)]
mod tests {
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
    fn test_as() {
        let v = Velocity::from_m_per_sec(1.0);
        assert_eq!(v.as_m_per_sec(), 1.0);
        assert_eq!(v.as_km_per_h(), 3.6);
        assert_relative_eq!(v.as_km_per_sec(),0.001);

        let v = Velocity::from_km_per_h(100.0);
        assert_eq!(v.as_km_per_h(), 100.0);
        assert_eq!(v.as_m_per_sec(), 27.77777777777778);
        assert_eq!(v.as_km_per_sec(), 100.0 / 3600.0);

        let v = Velocity::from_km_per_sec(100.0);
        assert_eq!(v.as_m_per_sec(), 100000.0);
        assert_relative_eq!(v.as_km_per_h(),100.0*3600.0);
        assert_relative_eq!(v.as_km_per_sec(),100.0);
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
        }
        {
            let v1 = Velocity::from_m_per_sec(15.0);
            let v2 = v1 - 10.0;
            assert_relative_eq!(v2.as_m_per_sec(), 5.0);

            let v1 = Velocity::from_km_per_h(15.0);
            let v2 = v1 - 10.0;
            assert_relative_eq!(v2.as_km_per_h(), 5.0,epsilon = 1e-8);
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
}