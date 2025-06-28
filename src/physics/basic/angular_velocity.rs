use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;

use crate::physics::basic::AngularVelocity;
use super::*;
impl AngularVelocity {
    pub fn from_rad_per_second(v: f64) -> Self {
        AngularVelocity {
            default_type: AngularVelocityType::RadperSecond,
            v,
        }
    }

    pub fn from_deg_per_second(v: f64) -> Self {
        AngularVelocity {
            default_type: AngularVelocityType::DegPerSecond,
            v,
        }
    }
    pub fn from_deg_per_hour(v: f64) -> Self {
        AngularVelocity {
            default_type: AngularVelocityType::DegperHour,
            v,
        }
    }

    pub fn from_rad_per_hour(v: f64) -> Self {
        AngularVelocity {
            default_type: AngularVelocityType::RadperHour,
            v,
        }
    }

    pub fn as_rad_per_second(&self) -> f64 {
        return match self.default_type {
            AngularVelocityType::RadperSecond => self.v,
            AngularVelocityType::DegPerSecond => { self.v * PI / 180.0 }
            AngularVelocityType::RadperHour => { self.v / 3600.0 }
            AngularVelocityType::DegperHour => { self.v * PI / 180.0 / 3600.0 }
        };
    }

    pub fn as_deg_per_second(&self) -> f64 {
        return match self.default_type {
            AngularVelocityType::RadperSecond => { self.v * 180.0 / PI }
            AngularVelocityType::DegPerSecond => { self.v }
            AngularVelocityType::RadperHour => { self.v * 180.0 / PI / 3600.0 }
            AngularVelocityType::DegperHour => { self.v / 3600.0 }
        };
    }

    pub fn as_rad_per_hour(&self) -> f64 {
        return match self.default_type {
            AngularVelocityType::RadperSecond => { self.v * 3600.0 }
            AngularVelocityType::DegPerSecond => { self.v * 3600.0 * PI / 180.0 }
            AngularVelocityType::RadperHour => { self.v }
            AngularVelocityType::DegperHour => { self.v * PI / 180.0 }
        };
    }

    pub fn as_deg_per_hour(&self) -> f64 {
        return match self.default_type {
            AngularVelocityType::RadperSecond => { self.v * 3600.0 * 180.0 / PI }
            AngularVelocityType::DegPerSecond => { self.v * 3600.0 }
            AngularVelocityType::RadperHour => { self.v * 180.0 / PI }
            AngularVelocityType::DegperHour => { self.v }
        };
    }
}

impl Mul<Duration> for AngularVelocity {
    type Output = Angular;
    fn mul(self, rhs: Duration) -> Self::Output {
        let v = self.as_rad_per_second() * rhs.as_secs_f64();
        Angular::from_rad(v)
    }
}

impl Div<Duration> for AngularVelocity {
    type Output = AngularAcceleration;

    fn div(self, rhs: Duration) -> Self::Output {
        let v = self.as_rad_per_second() / rhs.as_secs_f64();
        AngularAcceleration::from_rad_per_second2(v)
    }
}

impl Add for AngularVelocity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut v = self.as_rad_per_second() + rhs.as_rad_per_second();
        AngularVelocity::from_rad_per_second(v)
    }
}

impl Add<f64> for AngularVelocity {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AngularVelocityType::RadperSecond => {
                let v = self.as_rad_per_second() + rhs;
                Self::from_rad_per_second(v)
            }
            AngularVelocityType::DegPerSecond => {
                let v = self.as_deg_per_second() + rhs;
                Self::from_deg_per_second(v)
            }
            AngularVelocityType::RadperHour => {
                let v = self.as_rad_per_hour() + rhs;
                Self::from_rad_per_hour(v)
            }
            AngularVelocityType::DegperHour => {
                let v = self.as_deg_per_hour() + rhs;
                Self::from_deg_per_hour(v)
            }
        };
    }
}

impl Sub for AngularVelocity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut v = self.as_rad_per_second() - rhs.as_rad_per_second();
        AngularVelocity::from_rad_per_second(v)
    }
}

impl Sub<f64> for AngularVelocity {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AngularVelocityType::RadperSecond => {
                let v = self.as_rad_per_second() - rhs;
                Self::from_rad_per_second(v)
            }
            AngularVelocityType::DegPerSecond => {
                let v = self.as_deg_per_second() - rhs;
                Self::from_deg_per_second(v)
            }
            AngularVelocityType::RadperHour => {
                let v = self.as_rad_per_hour() - rhs;
                Self::from_rad_per_hour(v)
            }
            AngularVelocityType::DegperHour => {
                let v = self.as_deg_per_hour() - rhs;
                Self::from_deg_per_hour(v)
            }
        };
    }
}

impl Mul<Distance> for AngularVelocity {
    type Output = Velocity;
    fn mul(self, rhs: Distance) -> Self::Output {
        let v = self.as_rad_per_second() * rhs.as_m();
        Velocity::from_m_per_sec(v)
    }
}

impl Mul<f64> for AngularVelocity {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_rad_per_second() * rhs;
        Self::from_rad_per_second(v)
    }
}

impl Div<f64> for AngularVelocity {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_rad_per_second() / rhs;
        Self::from_rad_per_second(v)
    }
}

impl Mul<Coef> for AngularVelocity {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_rad_per_second() * rhs.get_value();
        Self::from_rad_per_second(v)
    }
}

impl Div<Coef> for AngularVelocity {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_rad_per_second() / rhs.get_value();
        Self::from_rad_per_second(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_angular_velocity_from() {
        let v = AngularVelocity::from_deg_per_second(1000.0);
        assert_eq!(v.default_type, AngularVelocityType::DegPerSecond);
        assert_eq!(v.v, 1000.0);
        let v = AngularVelocity::from_rad_per_second(1000.0);
        assert_eq!(v.default_type, AngularVelocityType::RadperSecond);
        assert_eq!(v.v, 1000.0);
        let v = AngularVelocity::from_rad_per_hour(1000.0);
        assert_eq!(v.default_type, AngularVelocityType::RadperHour);
        assert_eq!(v.v, 1000.0);
        let v = AngularVelocity::from_deg_per_hour(1000.0);
        assert_eq!(v.default_type, AngularVelocityType::DegperHour);
        assert_eq!(v.v, 1000.0);
    }


    #[test]
    fn test_angular_velocity_convert() {
        {
            // from_rad_per_second
            let omg = AngularVelocity::from_rad_per_second(PI * 0.5);
            assert_eq!(omg.as_rad_per_second(), PI * 0.5);
            assert_relative_eq!(omg.as_deg_per_second(),90.0);
            assert_relative_eq!(omg.as_rad_per_hour(), PI * 0.5*3600.0);
            assert_relative_eq!(omg.as_deg_per_hour(), 90.0*3600.0);
        }
        {
            let omg = AngularVelocity::from_deg_per_second(30.0);
            assert_relative_eq!(omg.as_deg_per_second(), 30.0);
            assert_relative_eq!(omg.as_rad_per_second(), PI / 6.0);
            assert_relative_eq!(omg.as_rad_per_hour(),3600.0 * PI / 6.0);
            assert_relative_eq!(omg.as_deg_per_hour(), 30.0*3600.0);
        }
        {
            // 36 PI 是18圈
            let omg = AngularVelocity::from_rad_per_hour(36.0 * PI);
            assert_relative_eq!(omg.as_rad_per_hour(), 36.0*PI);
            assert_relative_eq!(omg.as_deg_per_hour(), 18.0*360.0);
            assert_relative_eq!(omg.as_rad_per_second(),0.01 * PI);
            assert_relative_eq!(omg.as_deg_per_second(),1.8);
        }
        {
            // 7200 °是20圈
            let omg = AngularVelocity::from_deg_per_hour(7200.0);
            assert_relative_eq!(omg.as_deg_per_hour(),7200.0);
            assert_relative_eq!(omg.as_rad_per_hour(),20.0 * 2.0 * PI);
            assert_relative_eq!(omg.as_deg_per_second(),2.0);
            assert_relative_eq!(omg.as_rad_per_second(),0.034906585041,epsilon = 1e-8);
        }
    }

    #[test]
    fn test_angular_velocity_change() {
        let omg1 = AngularVelocity::from_rad_per_second(10.0);
        let duration = Duration::from_secs_f64(2.0);
        let theta1 = omg1 * duration;
        assert_eq!(theta1.default_type, AngularType::Rad);
        assert_eq!(theta1.as_rad(), 20.0);

        let omg1 = AngularVelocity::from_rad_per_second(10.0);
        let duration = Duration::from_secs_f64(2.0);
        let alpha = omg1 / duration;
        assert_eq!(alpha.default_type, AngularAccelerationType::RadperSecond2);
        assert_relative_eq!(alpha.as_rad_per_second2(),5.0);
    }

    #[test]
    fn test_angular_velocity_sub() {
        {
            let a1 = AngularVelocity::from_rad_per_second(1000.0);
            let a2 = AngularVelocity::from_rad_per_second(1000.0);
            let a3 = a1 - a2;
            assert_eq!(a3.as_rad_per_second(), 0.0);

            let a1 = AngularVelocity::from_rad_per_second(0.5 * PI);
            let a2 = AngularVelocity::from_deg_per_second(60.0);
            let a3 = a1 - a2;
            assert_relative_eq!(a3.as_deg_per_second(), 30.0,epsilon = 1e-8);
        }
        {
            let a1 = AngularVelocity::from_rad_per_second(1000.0);
            let a2 = a1 - 100.0;
            assert_eq!(a2.as_rad_per_second(), 900.0);

            let a1 = AngularVelocity::from_deg_per_second(1000.0);
            let a2 = a1 - 100.0;
            assert_eq!(a2.as_deg_per_second(), 900.0);

            let a1 = AngularVelocity::from_rad_per_hour(2.0);
            let a2 = a1 - 1.5;
            assert_relative_eq!(a2.as_rad_per_hour(), 0.5,epsilon = 1e-8);

            let a1 = AngularVelocity::from_deg_per_hour(2.0);
            let a2 = a1 - 1.5;
            assert_relative_eq!(a2.as_deg_per_hour(), 0.5,epsilon = 1e-8);
        }
    }

    #[test]
    fn test_angular_velocity_add() {
        {
            let a1 = AngularVelocity::from_rad_per_second(1000.0);
            let a2 = AngularVelocity::from_rad_per_second(1000.0);
            let a3 = a1 + a2;
            assert_eq!(a3.as_rad_per_second(), 2000.0);

            let a1 = AngularVelocity::from_deg_per_second(10.0);
            let a2 = AngularVelocity::from_deg_per_second(1000.0);
            let a3 = a1 + a2;
            assert_relative_eq!(a3.as_deg_per_second(),1010.0,epsilon = 1e-8);

            let a1 = AngularVelocity::from_rad_per_hour(1000.0);
            let a2 = AngularVelocity::from_rad_per_hour(10.0);
            let a3 = a1 + a2;
            assert_relative_eq!(a3.as_rad_per_hour(),1010.0,epsilon = 1e-8);

            let a1 = AngularVelocity::from_deg_per_hour(10.0);
            let a2 = AngularVelocity::from_deg_per_hour(1.0);
            let a3 = a1 + a2;
            assert_relative_eq!(a3.as_deg_per_hour(), 11.0,epsilon = 1e-8);

            let a1 = AngularVelocity::from_rad_per_second(0.5 * PI);
            let a2 = AngularVelocity::from_deg_per_second(30.0);
            let a3 = a1 + a2;
            assert_relative_eq!(a3.as_deg_per_second(), 120.0,epsilon = 1e-8);
        }
        {
            let a1 = AngularVelocity::from_rad_per_second(1000.0);
            let a2 = a1 + 1.0;
            assert_relative_eq!(a2.as_rad_per_second(), 1001.0);

            let a1 = AngularVelocity::from_deg_per_second(1000.0);
            let a2 = a1 + 1.0;
            assert_relative_eq!(a2.as_deg_per_second(),1001.0);

            let a1 = AngularVelocity::from_rad_per_hour(1.5);
            let a2 = a1 + 1.5;
            assert_relative_eq!(a2.as_rad_per_hour(), 3.0);

            let a1 = AngularVelocity::from_deg_per_hour(1.5);
            let a2 = a1 + 1.5;
            assert_relative_eq!(a2.as_deg_per_hour(), 3.0);
        }
    }

    #[test]
    fn test_to_mul() {
        let d1 = AngularVelocity::from_rad_per_second(1000.0);
        let d2 = d1 * 2.0;
        assert_eq!(d2.as_rad_per_second(), 2000.0);
        let d1 =  AngularVelocity::from_rad_per_second(1000.0);
        let d2 = d1 * Coef::new(2.0);
        assert_eq!(d2.as_rad_per_second(), 2000.0);
    }

    #[test]
    fn test_to_div() {
        let d1 = AngularVelocity::from_rad_per_second(1000.0);
        let d2 = d1 / 2.0;
        assert_eq!(d2.as_rad_per_second(), 500.0);
        let d1 =  AngularVelocity::from_rad_per_second(1000.0);
        let d2 = d1 / Coef::new(2.0);
        assert_eq!(d2.as_rad_per_second(), 500.0);
    }

    #[test]
    fn test_convert_to_velocity(){
        let omg1 = AngularVelocity::from_rad_per_second(PI);
        let v1 = omg1 * Distance::from_m(2.0);
        assert_eq!(v1.default_type, VelocityType::MPerSecond);
        assert_relative_eq!(v1.as_m_per_sec(),6.283185307180,epsilon = 1e-8);
    }
}
