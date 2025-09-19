use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::time::Duration;

use crate::physics::basic::{AngularVelocity, MagneticInduction, MagneticAngularVelocity};
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


impl PhysicalQuantity for AngularVelocity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_rad_per_second()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
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
        let v = self.as_rad_per_second() + rhs.as_rad_per_second();
        AngularVelocity::from_rad_per_second(v)
    }
}

// 引用-引用 与 混合引用：AngularVelocity 加法
impl<'a, 'b> Add<&'b AngularVelocity> for &'a AngularVelocity {
    type Output = AngularVelocity;
    fn add(self, rhs: &'b AngularVelocity) -> Self::Output { AngularVelocity::from_rad_per_second(self.as_rad_per_second() + rhs.as_rad_per_second()) }
}
impl<'a> Add<&'a AngularVelocity> for AngularVelocity {
    type Output = AngularVelocity;
    fn add(self, rhs: &'a AngularVelocity) -> Self::Output { AngularVelocity::from_rad_per_second(self.as_rad_per_second() + rhs.as_rad_per_second()) }
}
impl<'a> Add<AngularVelocity> for &'a AngularVelocity {
    type Output = AngularVelocity;
    fn add(self, rhs: AngularVelocity) -> Self::Output { AngularVelocity::from_rad_per_second(self.as_rad_per_second() + rhs.as_rad_per_second()) }
}

impl Add<f64> for AngularVelocity {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        AngularVelocity {
            v,
            default_type: self.default_type,
        }
    }
}

impl Sub for AngularVelocity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_rad_per_second() - rhs.as_rad_per_second();
        AngularVelocity::from_rad_per_second(v)
    }
}

// 引用-引用 与 混合引用：AngularVelocity 减法
impl<'a, 'b> Sub<&'b AngularVelocity> for &'a AngularVelocity {
    type Output = AngularVelocity;
    fn sub(self, rhs: &'b AngularVelocity) -> Self::Output { AngularVelocity::from_rad_per_second(self.as_rad_per_second() - rhs.as_rad_per_second()) }
}
impl<'a> Sub<&'a AngularVelocity> for AngularVelocity {
    type Output = AngularVelocity;
    fn sub(self, rhs: &'a AngularVelocity) -> Self::Output { AngularVelocity::from_rad_per_second(self.as_rad_per_second() - rhs.as_rad_per_second()) }
}
impl<'a> Sub<AngularVelocity> for &'a AngularVelocity {
    type Output = AngularVelocity;
    fn sub(self, rhs: AngularVelocity) -> Self::Output { AngularVelocity::from_rad_per_second(self.as_rad_per_second() - rhs.as_rad_per_second()) }
}

impl Sub<f64> for AngularVelocity {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        AngularVelocity {
            v,
            default_type: self.default_type,
        }
    }
}

impl Mul<Distance> for AngularVelocity {
    type Output = Velocity;
    fn mul(self, rhs: Distance) -> Self::Output {
        let v = self.as_rad_per_second() * rhs.as_m();
        Velocity::from_m_per_sec(v)
    }
}

// 引用版本：AngularVelocity * Distance -> Velocity
impl<'a, 'b> Mul<&'b Distance> for &'a AngularVelocity {
    type Output = Velocity;
    fn mul(self, rhs: &'b Distance) -> Self::Output { Velocity::from_m_per_sec(self.as_rad_per_second() * rhs.as_m()) }
}
impl<'a> Mul<&'a Distance> for AngularVelocity {
    type Output = Velocity;
    fn mul(self, rhs: &'a Distance) -> Self::Output { Velocity::from_m_per_sec(self.as_rad_per_second() * rhs.as_m()) }
}
impl<'a> Mul<Distance> for &'a AngularVelocity {
    type Output = Velocity;
    fn mul(self, rhs: Distance) -> Self::Output { Velocity::from_m_per_sec(self.as_rad_per_second() * rhs.as_m()) }
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

impl Mul<AngularVelocity> for f64 {
    type Output = AngularVelocity;
    fn mul(self, rhs: AngularVelocity) -> Self::Output {
        rhs * self
    }
}

impl Div<AngularVelocity> for f64 {
    type Output = AngularVelocity;
    fn div(self, rhs: AngularVelocity) -> Self::Output {
        let v = self / rhs.v;
        AngularVelocity {
            default_type: rhs.default_type,
            v: v,
        }
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

// 角速度 × 角动量 = 力矩（满足交换律）
impl Mul<AngularMomentum> for AngularVelocity {
    type Output = Torque;
    fn mul(self, rhs: AngularMomentum) -> Self::Output {
        let torque_value = self.as_rad_per_second() * rhs.as_kg_m2_per_second();
        Torque::from_nm(torque_value)
    }
}

// 角速度 × 磁感应强度 = 磁角速度（满足交换律）
impl Mul<MagneticInduction> for AngularVelocity {
    type Output = MagneticAngularVelocity;
    fn mul(self, rhs: MagneticInduction) -> Self::Output {
        let rad_per_second_value = self.as_rad_per_second();
        let tesla_value = rhs.as_tesla();
        let result_value = rad_per_second_value * tesla_value;
        MagneticAngularVelocity::from_tesla_rad_per_second(result_value)
    }
}

impl Div for AngularVelocity {
    type Output = Coef;
    fn div(self, rhs: Self) -> Self::Output {
        let v = self.as_rad_per_second() / rhs.as_rad_per_second();
        Coef::new(v)
    }
}

// 引用版本：AngularVelocity / AngularVelocity -> Coef
impl<'a, 'b> Div<&'b AngularVelocity> for &'a AngularVelocity {
    type Output = Coef;
    fn div(self, rhs: &'b AngularVelocity) -> Self::Output { Coef::new(self.as_rad_per_second() / rhs.as_rad_per_second()) }
}
impl<'a> Div<&'a AngularVelocity> for AngularVelocity {
    type Output = Coef;
    fn div(self, rhs: &'a AngularVelocity) -> Self::Output { Coef::new(self.as_rad_per_second() / rhs.as_rad_per_second()) }
}
impl<'a> Div<AngularVelocity> for &'a AngularVelocity {
    type Output = Coef;
    fn div(self, rhs: AngularVelocity) -> Self::Output { Coef::new(self.as_rad_per_second() / rhs.as_rad_per_second()) }
}

// 角速度 ÷ 角加速度 = 时间
impl Div<AngularAcceleration> for AngularVelocity {
    type Output = std::time::Duration;
    fn div(self, rhs: AngularAcceleration) -> Self::Output {
        let time_value = self.as_rad_per_second() / rhs.as_rad_per_second2();
        std::time::Duration::from_secs_f64(time_value)
    }
}

// 引用版本：AngularVelocity / AngularAcceleration -> Duration
impl<'a, 'b> Div<&'b AngularAcceleration> for &'a AngularVelocity {
    type Output = std::time::Duration;
    fn div(self, rhs: &'b AngularAcceleration) -> Self::Output { std::time::Duration::from_secs_f64(self.as_rad_per_second() / rhs.as_rad_per_second2()) }
}
impl<'a> Div<&'a AngularAcceleration> for AngularVelocity {
    type Output = std::time::Duration;
    fn div(self, rhs: &'a AngularAcceleration) -> Self::Output { std::time::Duration::from_secs_f64(self.as_rad_per_second() / rhs.as_rad_per_second2()) }
}
impl<'a> Div<AngularAcceleration> for &'a AngularVelocity {
    type Output = std::time::Duration;
    fn div(self, rhs: AngularAcceleration) -> Self::Output { std::time::Duration::from_secs_f64(self.as_rad_per_second() / rhs.as_rad_per_second2()) }
}

impl Neg for AngularVelocity {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = self.as_rad_per_second();
        Self::from_rad_per_second(-v)
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
        let d1 = AngularVelocity::from_rad_per_second(1000.0);
        let d2 = d1 * Coef::new(2.0);
        assert_eq!(d2.as_rad_per_second(), 2000.0);
    }

    #[test]
    fn test_to_div() {
        let d1 = AngularVelocity::from_rad_per_second(1000.0);
        let d2 = d1 / 2.0;
        assert_eq!(d2.as_rad_per_second(), 500.0);
        let d1 = AngularVelocity::from_rad_per_second(1000.0);
        let d2 = d1 / Coef::new(2.0);
        assert_eq!(d2.as_rad_per_second(), 500.0);

        let d1 = AngularVelocity::from_rad_per_second(1000.0);
        let d2 = AngularVelocity::from_rad_per_second(2000.0);
        let d3 = d1 / d2;
        assert_relative_eq!(d3.get_value(),0.5,epsilon = 1e-8);
    }

    #[test]
    fn test_convert_to_velocity() {
        let omg1 = AngularVelocity::from_rad_per_second(PI);
        let v1 = omg1 * Distance::from_m(2.0);
        assert_eq!(v1.default_type, VelocityType::MPerSecond);
        assert_relative_eq!(v1.as_m_per_sec(),6.283185307180,epsilon = 1e-8);
    }

    #[test]
    fn test_as_any() {
        let d1 = AngularVelocity::from_rad_per_second(1000.0);
        let d2 = d1.as_any();
        let d3 = d2.downcast_ref::<AngularVelocity>().unwrap();
    }

    #[test]
    fn test_is_zero() {
        let d1 = AngularVelocity::from_rad_per_second(0.0);
        assert!(d1.is_zero());
        let d1 = AngularVelocity::from_rad_per_second(1000.0);
        assert!(!d1.is_zero());
    }

    #[test]
    fn test_default_unit_value() {
        let d1 = AngularVelocity::from_deg_per_second(180.0);
        assert_eq!(d1.default_unit_value(), PI);

        let mut d1 = AngularVelocity::from_rad_per_second(1000.0);
        d1.set_value(100.0);
        assert_relative_eq!(d1.v,100.0);
    }

    #[test]
    fn test_negative() {
        let d1 = AngularVelocity::from_rad_per_second(-10.0);
        let d2 = -d1;
        assert_relative_eq!(d2.as_rad_per_second(),10.0)
    }

    #[test]
    fn test_f64_mul_angular_velocity() {
        // f64 * AngularVelocity 测试
        let a = AngularVelocity::from_rad_per_second(2.0);
        let result = 3.0 * a;
        assert_relative_eq!(result.as_rad_per_second(), 6.0);

        let a = AngularVelocity::from_deg_per_second(180.0);
        let result = 2.0 * a;
        assert_relative_eq!(result.as_deg_per_second(), 360.0);

        let a = AngularVelocity::from_rad_per_hour(PI);
        let result = 2.0 * a;
        assert_relative_eq!(result.as_rad_per_hour(), 2.0 * PI);
    }

    #[test]
    fn test_f64_div_angular_velocity() {
        // f64 / AngularVelocity 测试
        let a = AngularVelocity::from_rad_per_second(2.0);
        let result = 6.0 / a;
        assert_relative_eq!(result.as_rad_per_second(), 3.0);

        let a = AngularVelocity::from_deg_per_second(180.0);
        let result = 180.0 / a;
        assert_relative_eq!(result.as_deg_per_second(), 1.0);

        let a = AngularVelocity::from_rad_per_hour(PI);
        let result = 2.0 * PI / a;
        assert_relative_eq!(result.as_rad_per_hour(), 2.0);
    }

    #[test]
    fn test_angular_velocity_mul_angular_momentum() {
        // 角速度 × 角动量 = 力矩（满足交换律）
        let omega = AngularVelocity::from_rad_per_second(5.0);
        let l = AngularMomentum::from_kg_m2_per_second(10.0);
        let torque = omega * l;
        assert_relative_eq!(torque.as_nm(), 50.0);

        // 测试不同单位
        let omega = AngularVelocity::from_deg_per_second(180.0);
        let l = AngularMomentum::from_kg_km2_per_second(1.0);
        let torque = omega * l;
        assert_relative_eq!(torque.as_nm(), 1e6 * std::f64::consts::PI);
    }

    #[test]
    fn test_angular_velocity_div_angular_acceleration() {
        let omega = AngularVelocity::from_rad_per_second(10.0); // 10 rad/s
        let alpha = AngularAcceleration::from_rad_per_second2(2.0); // 2 rad/s²
        let time = omega / alpha; // 5 s
        
        assert_relative_eq!(time.as_secs_f64(), 5.0);
    }

    #[test]
    fn test_angular_velocity_mul_magnetic_induction() {
        // 角速度 × 磁感应强度 = 磁角速度
        let angular_velocity = AngularVelocity::from_rad_per_second(4.0);
        let magnetic_induction = MagneticInduction::from_tesla(2.5);
        let result = angular_velocity * magnetic_induction;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 10.0);

        // 测试不同单位的角速度
        let angular_velocity = AngularVelocity::from_deg_per_second(180.0); // π rad/s
        let magnetic_induction = MagneticInduction::from_tesla(1.0);
        let result = angular_velocity * magnetic_induction;
        assert_relative_eq!(result.as_tesla_rad_per_second(), std::f64::consts::PI);

        // 测试不同单位的磁感应强度
        let angular_velocity = AngularVelocity::from_rad_per_second(3.0);
        let magnetic_induction = MagneticInduction::from_gauss(10000.0); // 1 T
        let result = angular_velocity * magnetic_induction;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 3.0);

        // 测试交换律
        let angular_velocity = AngularVelocity::from_rad_per_second(2.0);
        let magnetic_induction = MagneticInduction::from_tesla(3.0);
        let result1 = angular_velocity * magnetic_induction;
        let result2 = magnetic_induction * angular_velocity;
        assert_relative_eq!(result1.as_tesla_rad_per_second(), result2.as_tesla_rad_per_second());
    }

    #[test]
    fn test_angular_velocity_ref_ops() {
        let w1 = AngularVelocity::from_rad_per_second(std::f64::consts::PI);
        let w2 = AngularVelocity::from_deg_per_second(180.0); // PI rad/s
        let s = &w1 + &w2;
        assert_relative_eq!(s.as_rad_per_second(), 2.0 * std::f64::consts::PI);

        let d = &w1 - &w2;
        assert_relative_eq!(d.as_rad_per_second(), 0.0);

        let v = &w1 * &Distance::from_m(2.0);
        assert_relative_eq!(v.as_m_per_sec(), 2.0 * std::f64::consts::PI);

        let coef = &w1 / &w2;
        assert_relative_eq!(coef.get_value(), 1.0);

        let t = &w1 / &AngularAcceleration::from_rad_per_second2(std::f64::consts::PI);
        assert_relative_eq!(t.as_secs_f64(), 1.0);

        // 混合引用：加/减
        let s2 = w1 + &w2;
        assert_relative_eq!(s2.as_rad_per_second(), 2.0 * std::f64::consts::PI);
        let s3 = &w1 + w2;
        assert_relative_eq!(s3.as_rad_per_second(), 2.0 * std::f64::consts::PI);
        let w2b = AngularVelocity::from_deg_per_second(180.0);
        let d2 = w1 - &w2b;
        assert_relative_eq!(d2.as_rad_per_second(), 0.0);
        let d3 = &w1 - w2b;
        assert_relative_eq!(d3.as_rad_per_second(), 0.0);

        // 混合引用：* Distance
        let v2 = w1 * &Distance::from_m(2.0);
        assert_relative_eq!(v2.as_m_per_sec(), 2.0 * std::f64::consts::PI);
        let v3 = &w1 * Distance::from_m(2.0);
        assert_relative_eq!(v3.as_m_per_sec(), 2.0 * std::f64::consts::PI);

        // 混合引用：/ AngularVelocity -> Coef
        let c2 = w1 / &w1;
        assert_relative_eq!(c2.get_value(), 1.0);
        let c3 = &w1 / w1;
        assert_relative_eq!(c3.get_value(), 1.0);

        // 混合引用：/ AngularAcceleration -> Duration
        let ta = w1 / &AngularAcceleration::from_rad_per_second2(std::f64::consts::PI);
        assert_relative_eq!(ta.as_secs_f64(), 1.0);
        let tb = &w1 / AngularAcceleration::from_rad_per_second2(std::f64::consts::PI);
        assert_relative_eq!(tb.as_secs_f64(), 1.0);
    }
}
