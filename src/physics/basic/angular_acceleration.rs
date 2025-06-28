use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;
use super::*;

impl AngularAcceleration {
    pub fn from_rad_per_second2(v: f64) -> Self {
        AngularAcceleration {
            default_type: AngularAccelerationType::RadperSecond2,
            v,
        }
    }

    pub fn from_deg_per_second2(v: f64) -> Self {
        AngularAcceleration {
            default_type: AngularAccelerationType::DegPerSecond2,
            v,
        }
    }

    pub fn as_rad_per_second2(&self) -> f64 {
        return match self.default_type {
            AngularAccelerationType::RadperSecond2 => self.v,
            AngularAccelerationType::DegPerSecond2 => {
                self.v * PI / 180.0
            }
        };
    }

    pub fn as_deg_per_second2(&self) -> f64 {
        return match self.default_type {
            AngularAccelerationType::RadperSecond2 => self.v * 180.0 / PI,
            AngularAccelerationType::DegPerSecond2 => self.v,
        };
    }
}

impl Div<Duration> for AngularAcceleration {
    type Output = AngularVelocity;

    fn div(self, rhs: Duration) -> Self::Output {
        let v = self.as_rad_per_second2() / rhs.as_secs_f64();
        AngularVelocity::from_rad_per_second(v)
    }
}


impl Add for AngularAcceleration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_rad_per_second2() + rhs.as_rad_per_second2();
        AngularAcceleration::from_rad_per_second2(v)
    }
}

impl Add<f64> for AngularAcceleration {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AngularAccelerationType::RadperSecond2 => {
                let v = self.as_rad_per_second2() + rhs;
                Self::from_rad_per_second2(v)
            }
            AngularAccelerationType::DegPerSecond2 => {
                let v = self.as_deg_per_second2() + rhs;
                Self::from_deg_per_second2(v)
            }
        };
    }
}

impl Sub for AngularAcceleration {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_rad_per_second2() - rhs.as_rad_per_second2();
        AngularAcceleration::from_rad_per_second2(v)
    }
}

impl Sub<f64> for AngularAcceleration {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AngularAccelerationType::RadperSecond2 => {
                let v = self.as_rad_per_second2() - rhs;
                Self::from_rad_per_second2(v)
            }
            AngularAccelerationType::DegPerSecond2 => {
                let v = self.as_deg_per_second2() - rhs;
                Self::from_deg_per_second2(v)
            }
        };
    }
}
// 从角加速度变成角速度
impl Mul<Duration> for AngularAcceleration {
    type Output = AngularVelocity;

    fn mul(self, rhs: Duration) -> Self::Output {
        let v = self.as_rad_per_second2() * rhs.as_secs_f64();
        AngularVelocity::from_rad_per_second(v)
    }
}

// 从角加速度变成线加速度
impl Mul<Distance> for AngularAcceleration {
    type Output = Acceleration;
    fn mul(self, rhs: Distance) -> Self::Output {
        let v = self.as_rad_per_second2() * rhs.as_m();
        Acceleration::from_m_per_s2(v)
    }
}

impl Mul<f64> for AngularAcceleration {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_rad_per_second2() * rhs;
        Self::from_rad_per_second2(v)
    }
}

impl Div<f64> for AngularAcceleration {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_rad_per_second2() / rhs;
        Self::from_rad_per_second2(v)
    }
}

impl Mul<Coef> for AngularAcceleration {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_rad_per_second2() * rhs.get_value();
        Self::from_rad_per_second2(v)
    }
}

impl Div<Coef> for AngularAcceleration {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_rad_per_second2() / rhs.get_value();
        Self::from_rad_per_second2(v)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_angular_acceleration_from() {
        let alpha1 = AngularAcceleration::from_rad_per_second2(6.0 * PI);
        assert_eq!(alpha1.default_type, AngularAccelerationType::RadperSecond2);
        assert_eq!(alpha1.v, 6.0 * PI);

        let alpha1 = AngularAcceleration::from_deg_per_second2(360.0);
        assert_eq!(alpha1.default_type, AngularAccelerationType::DegPerSecond2);
        assert_eq!(alpha1.v, 360.0);
    }

    #[test]
    fn test_angular_acceleration_as() {
        let alpha1 = AngularAcceleration::from_rad_per_second2(PI);
        assert_relative_eq!(alpha1.as_rad_per_second2(), 1.0*PI);
        assert_relative_eq!(alpha1.as_deg_per_second2(), 180.0);

        let alpha1 = AngularAcceleration::from_deg_per_second2(180.0);
        assert_eq!(alpha1.as_deg_per_second2(), 180.0);
        assert_relative_eq!(alpha1.as_rad_per_second2(), PI);
    }

    #[test]
    fn test_angular_acceleration_change() {
        let alpha1 = AngularAcceleration::from_rad_per_second2(PI);
        let duration = Duration::from_secs_f64(10.0);
        let omg1 = alpha1 / duration;
        assert_eq!(omg1.default_type, AngularVelocityType::RadperSecond);
        assert_relative_eq!(omg1.as_rad_per_second(), 0.1 * PI);

        let alpha1 = AngularAcceleration::from_rad_per_second2(PI);
        let distance = Distance::from_m(10.0);
        let a1 = alpha1 * distance;
        assert_relative_eq!(a1.as_m_per_s2(),10.0 * PI);

        let alpha1 = AngularAcceleration::from_rad_per_second2(PI);
        let omg = alpha1 * duration;
        assert_relative_eq!(omg.as_rad_per_second(), 10.0* PI);
    }

    #[test]
    fn test_angular_acceleration_add() {
        {
            let alpha1 = AngularAcceleration::from_rad_per_second2(PI);
            let alpha2 = AngularAcceleration::from_rad_per_second2(PI + PI);
            let alpha3 = alpha1 + alpha2;
            assert_relative_eq!(alpha3.as_rad_per_second2(), 3.0 * PI);

            let alpha1 = AngularAcceleration::from_deg_per_second2(100.0);
            let alpha2 = AngularAcceleration::from_deg_per_second2(100.0);
            let alpha3 = alpha1 + alpha2;
            assert_relative_eq!(alpha3.as_deg_per_second2(), 200.0);

            let alpha1 = AngularAcceleration::from_deg_per_second2(180.0);
            let alpha2 = AngularAcceleration::from_rad_per_second2(PI);
            let alpha3 = alpha1 + alpha2;
            assert_relative_eq!(alpha3.as_deg_per_second2(), 360.0);

            let alpha1 = AngularAcceleration::from_rad_per_second2(PI);
            let alpha2 = AngularAcceleration::from_deg_per_second2(180.0);
            let alpha3 = alpha1 + alpha2;
            assert_relative_eq!(alpha3.as_deg_per_second2(), 360.0);
        }
        {
            let alpha1 = AngularAcceleration::from_rad_per_second2(PI);
            let alpha2 = alpha1 + PI;
            assert_relative_eq!(alpha2.as_rad_per_second2(), 2.0*PI);

            let alpha1 = AngularAcceleration::from_deg_per_second2(90.0);
            let alpha2 = alpha1 + 90.0;
            assert_relative_eq!(alpha2.as_deg_per_second2(), 180.0);
        }
    }

    #[test]
    fn test_angular_acceleration_sub() {
        {
            let alpha1 = AngularAcceleration::from_rad_per_second2(PI + PI);
            let alpha2 = AngularAcceleration::from_rad_per_second2(PI);
            let alpha3 = alpha1 - alpha2;
            assert_relative_eq!(alpha3.as_rad_per_second2(), PI);

            let alpha1 = AngularAcceleration::from_deg_per_second2(100.0);
            let alpha2 = AngularAcceleration::from_deg_per_second2(100.0);
            let alpha3 = alpha1 - alpha2;
            assert_relative_eq!(alpha3.as_deg_per_second2(), 0.0);

            let alpha1 = AngularAcceleration::from_deg_per_second2(180.0);
            let alpha2 = AngularAcceleration::from_rad_per_second2(PI);
            let alpha3 = alpha1 - alpha2;
            assert_relative_eq!(alpha3.as_deg_per_second2(), 0.0);

            let alpha1 = AngularAcceleration::from_rad_per_second2(PI);
            let alpha2 = AngularAcceleration::from_deg_per_second2(180.0);
            let alpha3 = alpha1 - alpha2;
            assert_relative_eq!(alpha3.as_deg_per_second2(), 0.0);
        }
        {
            let alpha1 = AngularAcceleration::from_rad_per_second2(PI);
            let alpha2 = alpha1 - PI;
            assert_relative_eq!(alpha2.as_rad_per_second2(), 0.0);

            let alpha1 = AngularAcceleration::from_deg_per_second2(90.0);
            let alpha2 = alpha1 - 90.0;
            assert_relative_eq!(alpha2.as_deg_per_second2(), 0.0);
        }
    }
    #[test]
    fn test_to_mul() {
        let d1 = AngularAcceleration::from_rad_per_second2(1000.0);
        let d2 = d1 * 2.0;
        assert_eq!(d2.as_rad_per_second2(), 2000.0);
        let d1 = AngularAcceleration::from_rad_per_second2(1000.0);
        let d2 = d1 * Coef::new(2.0);
        assert_eq!(d2.as_rad_per_second2(), 2000.0);
    }

    #[test]
    fn test_to_div() {
        let d1 = AngularAcceleration::from_rad_per_second2(1000.0);
        let d2 = d1 / 2.0;
        assert_eq!(d2.as_rad_per_second2(), 500.0);
        let d1 = AngularAcceleration::from_rad_per_second2(1000.0);
        let d2 = d1 / Coef::new(2.0);
        assert_eq!(d2.as_rad_per_second2(), 500.0);
    }
}