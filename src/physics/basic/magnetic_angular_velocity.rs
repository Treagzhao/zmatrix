use std::any::Any;
use std::ops::{Add, Div, Mul, Sub};
use crate::physics::basic::{Coef, MagneticAngularVelocity, MagneticAngularVelocityType, PhysicalQuantity};
use approx::assert_relative_eq;

impl Default for MagneticAngularVelocity {
    fn default() -> Self {
        Self::from_tesla_rad_per_second(0.0)
    }
}

impl PhysicalQuantity for MagneticAngularVelocity {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_tesla_rad_per_second()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl MagneticAngularVelocity {
    // 特斯拉·弧度/秒构造函数
    pub fn from_tesla_rad_per_second(v: f64) -> Self {
        Self {
            default_type: MagneticAngularVelocityType::TeslaRadPerSecond,
            v,
        }
    }

    // 毫特斯拉·弧度/秒构造函数
    pub fn from_mill_tesla_rad_per_second(v: f64) -> Self {
        Self {
            default_type: MagneticAngularVelocityType::MillTeslaRadPerSecond,
            v,
        }
    }

    // 微特斯拉·弧度/秒构造函数
    pub fn from_micro_tesla_rad_per_second(v: f64) -> Self {
        Self {
            default_type: MagneticAngularVelocityType::MicroTeslaRadPerSecond,
            v,
        }
    }

    // 纳特斯拉·弧度/秒构造函数
    pub fn from_nano_tesla_rad_per_second(v: f64) -> Self {
        Self {
            default_type: MagneticAngularVelocityType::NanoTeslaRadPerSecond,
            v,
        }
    }

    // 高斯·弧度/秒构造函数
    pub fn from_gauss_rad_per_second(v: f64) -> Self {
        Self {
            default_type: MagneticAngularVelocityType::GaussRadPerSecond,
            v,
        }
    }

    // 毫高斯·弧度/秒构造函数
    pub fn from_mill_gauss_rad_per_second(v: f64) -> Self {
        Self {
            default_type: MagneticAngularVelocityType::MillGaussRadPerSecond,
            v,
        }
    }

    // 千高斯·弧度/秒构造函数
    pub fn from_kilo_gauss_rad_per_second(v: f64) -> Self {
        Self {
            default_type: MagneticAngularVelocityType::KiloGaussRadPerSecond,
            v,
        }
    }

    // 转换为特斯拉·弧度/秒
    pub fn as_tesla_rad_per_second(&self) -> f64 {
        match self.default_type {
            MagneticAngularVelocityType::TeslaRadPerSecond => self.v,
            MagneticAngularVelocityType::MillTeslaRadPerSecond => self.v * 1e-3,
            MagneticAngularVelocityType::MicroTeslaRadPerSecond => self.v * 1e-6,
            MagneticAngularVelocityType::NanoTeslaRadPerSecond => self.v * 1e-9,
            MagneticAngularVelocityType::GaussRadPerSecond => self.v * 1e-4,
            MagneticAngularVelocityType::MillGaussRadPerSecond => self.v * 1e-7,
            MagneticAngularVelocityType::KiloGaussRadPerSecond => self.v * 1e-1,
        }
    }

    // 转换为毫特斯拉·弧度/秒
    pub fn as_mill_tesla_rad_per_second(&self) -> f64 {
        match self.default_type {
            MagneticAngularVelocityType::TeslaRadPerSecond => self.v * 1e3,
            MagneticAngularVelocityType::MillTeslaRadPerSecond => self.v,
            MagneticAngularVelocityType::MicroTeslaRadPerSecond => self.v * 1e-3,
            MagneticAngularVelocityType::NanoTeslaRadPerSecond => self.v * 1e-6,
            MagneticAngularVelocityType::GaussRadPerSecond => self.v * 1e-1,
            MagneticAngularVelocityType::MillGaussRadPerSecond => self.v * 1e-4,
            MagneticAngularVelocityType::KiloGaussRadPerSecond => self.v * 1e2,
        }
    }

    // 转换为微特斯拉·弧度/秒
    pub fn as_micro_tesla_rad_per_second(&self) -> f64 {
        match self.default_type {
            MagneticAngularVelocityType::TeslaRadPerSecond => self.v * 1e6,
            MagneticAngularVelocityType::MillTeslaRadPerSecond => self.v * 1e3,
            MagneticAngularVelocityType::MicroTeslaRadPerSecond => self.v,
            MagneticAngularVelocityType::NanoTeslaRadPerSecond => self.v * 1e-3,
            MagneticAngularVelocityType::GaussRadPerSecond => self.v * 1e2,
            MagneticAngularVelocityType::MillGaussRadPerSecond => self.v * 1e-1,
            MagneticAngularVelocityType::KiloGaussRadPerSecond => self.v * 1e5,
        }
    }

    // 转换为纳特斯拉·弧度/秒
    pub fn as_nano_tesla_rad_per_second(&self) -> f64 {
        match self.default_type {
            MagneticAngularVelocityType::TeslaRadPerSecond => self.v * 1e9,
            MagneticAngularVelocityType::MillTeslaRadPerSecond => self.v * 1e6,
            MagneticAngularVelocityType::MicroTeslaRadPerSecond => self.v * 1e3,
            MagneticAngularVelocityType::NanoTeslaRadPerSecond => self.v,
            MagneticAngularVelocityType::GaussRadPerSecond => self.v * 1e5,
            MagneticAngularVelocityType::MillGaussRadPerSecond => self.v * 1e2,
            MagneticAngularVelocityType::KiloGaussRadPerSecond => self.v * 1e8,
        }
    }

    // 转换为高斯·弧度/秒
    pub fn as_gauss_rad_per_second(&self) -> f64 {
        match self.default_type {
            MagneticAngularVelocityType::TeslaRadPerSecond => self.v * 1e4,
            MagneticAngularVelocityType::MillTeslaRadPerSecond => self.v * 1e1,
            MagneticAngularVelocityType::MicroTeslaRadPerSecond => self.v * 1e-2,
            MagneticAngularVelocityType::NanoTeslaRadPerSecond => self.v * 1e-5,
            MagneticAngularVelocityType::GaussRadPerSecond => self.v,
            MagneticAngularVelocityType::MillGaussRadPerSecond => self.v * 1e-3,
            MagneticAngularVelocityType::KiloGaussRadPerSecond => self.v * 1e3,
        }
    }

    // 转换为毫高斯·弧度/秒
    pub fn as_mill_gauss_rad_per_second(&self) -> f64 {
        match self.default_type {
            MagneticAngularVelocityType::TeslaRadPerSecond => self.v * 1e7,
            MagneticAngularVelocityType::MillTeslaRadPerSecond => self.v * 1e4,
            MagneticAngularVelocityType::MicroTeslaRadPerSecond => self.v * 1e1,
            MagneticAngularVelocityType::NanoTeslaRadPerSecond => self.v * 1e-2,
            MagneticAngularVelocityType::GaussRadPerSecond => self.v * 1e3,
            MagneticAngularVelocityType::MillGaussRadPerSecond => self.v,
            MagneticAngularVelocityType::KiloGaussRadPerSecond => self.v * 1e6,
        }
    }

    // 转换为千高斯·弧度/秒
    pub fn as_kilo_gauss_rad_per_second(&self) -> f64 {
        match self.default_type {
            MagneticAngularVelocityType::TeslaRadPerSecond => self.v * 1e1,
            MagneticAngularVelocityType::MillTeslaRadPerSecond => self.v * 1e-2,
            MagneticAngularVelocityType::MicroTeslaRadPerSecond => self.v * 1e-5,
            MagneticAngularVelocityType::NanoTeslaRadPerSecond => self.v * 1e-8,
            MagneticAngularVelocityType::GaussRadPerSecond => self.v * 1e-3,
            MagneticAngularVelocityType::MillGaussRadPerSecond => self.v * 1e-6,
            MagneticAngularVelocityType::KiloGaussRadPerSecond => self.v,
        }
    }
}

// 加法运算
impl Add for MagneticAngularVelocity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_tesla_rad_per_second() + rhs.as_tesla_rad_per_second();
        Self::from_tesla_rad_per_second(v)
    }
}

impl Add<f64> for MagneticAngularVelocity {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.as_tesla_rad_per_second() + rhs;
        Self::from_tesla_rad_per_second(v)
    }
}

// 减法运算
impl Sub for MagneticAngularVelocity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_tesla_rad_per_second() - rhs.as_tesla_rad_per_second();
        Self::from_tesla_rad_per_second(v)
    }
}

impl Sub<f64> for MagneticAngularVelocity {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.as_tesla_rad_per_second() - rhs;
        Self::from_tesla_rad_per_second(v)
    }
}

// 乘法运算
impl Mul<f64> for MagneticAngularVelocity {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_tesla_rad_per_second() * rhs;
        Self::from_tesla_rad_per_second(v)
    }
}

impl Mul<Coef> for MagneticAngularVelocity {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_tesla_rad_per_second() * rhs.get_value();
        Self::from_tesla_rad_per_second(v)
    }
}

// 除法运算
impl Div<f64> for MagneticAngularVelocity {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_tesla_rad_per_second() / rhs;
        Self::from_tesla_rad_per_second(v)
    }
}

impl Div<Coef> for MagneticAngularVelocity {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_tesla_rad_per_second() / rhs.get_value();
        Self::from_tesla_rad_per_second(v)
    }
}

// f64 与 MagneticAngularVelocity 的乘法
impl Mul<MagneticAngularVelocity> for f64 {
    type Output = MagneticAngularVelocity;
    fn mul(self, rhs: MagneticAngularVelocity) -> Self::Output {
        let v = self * rhs.as_tesla_rad_per_second();
        MagneticAngularVelocity::from_tesla_rad_per_second(v)
    }
}

// f64 与 MagneticAngularVelocity 的除法
impl Div<MagneticAngularVelocity> for f64 {
    type Output = MagneticAngularVelocity;
    fn div(self, rhs: MagneticAngularVelocity) -> Self::Output {
        let v = self / rhs.as_tesla_rad_per_second();
        MagneticAngularVelocity::from_tesla_rad_per_second(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_magnetic_angular_velocity_from() {
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
        assert_eq!(mav.v, 1.0);
        assert_eq!(mav.default_type, MagneticAngularVelocityType::TeslaRadPerSecond);

        let mav = MagneticAngularVelocity::from_gauss_rad_per_second(1.0);
        assert_eq!(mav.v, 1.0);
        assert_eq!(mav.default_type, MagneticAngularVelocityType::GaussRadPerSecond);
    }

    #[test]
    fn test_magnetic_angular_velocity_convert() {
        // 测试特斯拉单位转换
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
        assert_eq!(mav.as_tesla_rad_per_second(), 1.0);
        assert_eq!(mav.as_mill_tesla_rad_per_second(), 1000.0);
        assert_eq!(mav.as_micro_tesla_rad_per_second(), 1e6);
        assert_eq!(mav.as_nano_tesla_rad_per_second(), 1e9);
        assert_eq!(mav.as_gauss_rad_per_second(), 10000.0);
        assert_eq!(mav.as_mill_gauss_rad_per_second(), 1e7);
        assert_eq!(mav.as_kilo_gauss_rad_per_second(), 10.0);

        // 测试高斯单位转换
        let mav = MagneticAngularVelocity::from_gauss_rad_per_second(10000.0);
        assert_relative_eq!(mav.as_tesla_rad_per_second(), 1.0);
        assert_eq!(mav.as_gauss_rad_per_second(), 10000.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_add() {
        let mav1 = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
        let mav2 = MagneticAngularVelocity::from_gauss_rad_per_second(10000.0); // 1 T·rad/s
        let result = mav1 + mav2;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 3.0);

        let mav1 = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
        let result = mav1 + 2.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 3.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_sub() {
        let mav1 = MagneticAngularVelocity::from_tesla_rad_per_second(3.0);
        let mav2 = MagneticAngularVelocity::from_gauss_rad_per_second(10000.0); // 1 T·rad/s
        let result = mav1 - mav2;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 2.0);

        let mav1 = MagneticAngularVelocity::from_tesla_rad_per_second(3.0);
        let result = mav1 - 1.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 2.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_mul() {
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
        let result = mav * 3.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 6.0);

        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
        let result = mav * Coef::new(3.0);
        assert_relative_eq!(result.as_tesla_rad_per_second(), 6.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_div() {
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(6.0);
        let result = mav / 2.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 3.0);

        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(6.0);
        let result = mav / Coef::new(2.0);
        assert_relative_eq!(result.as_tesla_rad_per_second(), 3.0);
    }

    #[test]
    fn test_f64_mul_magnetic_angular_velocity() {
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
        let result = 3.0 * mav;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 6.0);
    }

    #[test]
    fn test_f64_div_magnetic_angular_velocity() {
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
        let result = 6.0 / mav;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 3.0);
    }

    #[test]
    fn test_default() {
        let mav = MagneticAngularVelocity::default();
        assert_eq!(mav.v, 0.0);
        assert_eq!(mav.default_type, MagneticAngularVelocityType::TeslaRadPerSecond);
    }

    #[test]
    fn test_is_zero() {
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(0.0);
        assert!(mav.is_zero());

        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
        assert!(!mav.is_zero());
    }

    #[test]
    fn test_default_unit_value() {
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
        assert_relative_eq!(mav.default_unit_value(), 1.0);

        let mav = MagneticAngularVelocity::from_gauss_rad_per_second(10000.0);
        assert_relative_eq!(mav.default_unit_value(), 1.0);
    }

    #[test]
    fn test_set_value() {
        let mut mav = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
        mav.set_value(2.0);
        assert_relative_eq!(mav.as_tesla_rad_per_second(), 2.0);
    }

    #[test]
    fn test_as_any() {
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
        let any = mav.as_any();
        let _mav_ref = any.downcast_ref::<MagneticAngularVelocity>().unwrap();
    }
}
