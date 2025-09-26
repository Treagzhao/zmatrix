use std::any::Any;
use std::ops::{Add, Div, Mul, Sub, Neg};
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

// 引用-引用 与 混合引用：MagneticAngularVelocity 加法
impl<'a, 'b> Add<&'b MagneticAngularVelocity> for &'a MagneticAngularVelocity {
    type Output = MagneticAngularVelocity;
    fn add(self, rhs: &'b MagneticAngularVelocity) -> Self::Output { MagneticAngularVelocity::from_tesla_rad_per_second(self.as_tesla_rad_per_second() + rhs.as_tesla_rad_per_second()) }
}
impl<'a> Add<&'a MagneticAngularVelocity> for MagneticAngularVelocity {
    type Output = MagneticAngularVelocity;
    fn add(self, rhs: &'a MagneticAngularVelocity) -> Self::Output { MagneticAngularVelocity::from_tesla_rad_per_second(self.as_tesla_rad_per_second() + rhs.as_tesla_rad_per_second()) }
}
impl<'a> Add<MagneticAngularVelocity> for &'a MagneticAngularVelocity {
    type Output = MagneticAngularVelocity;
    fn add(self, rhs: MagneticAngularVelocity) -> Self::Output { MagneticAngularVelocity::from_tesla_rad_per_second(self.as_tesla_rad_per_second() + rhs.as_tesla_rad_per_second()) }
}

impl Add<f64> for MagneticAngularVelocity {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        MagneticAngularVelocity {
            v,
            default_type: self.default_type,
        }
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

// 引用-引用 与 混合引用：MagneticAngularVelocity 减法
impl<'a, 'b> Sub<&'b MagneticAngularVelocity> for &'a MagneticAngularVelocity {
    type Output = MagneticAngularVelocity;
    fn sub(self, rhs: &'b MagneticAngularVelocity) -> Self::Output { MagneticAngularVelocity::from_tesla_rad_per_second(self.as_tesla_rad_per_second() - rhs.as_tesla_rad_per_second()) }
}
impl<'a> Sub<&'a MagneticAngularVelocity> for MagneticAngularVelocity {
    type Output = MagneticAngularVelocity;
    fn sub(self, rhs: &'a MagneticAngularVelocity) -> Self::Output { MagneticAngularVelocity::from_tesla_rad_per_second(self.as_tesla_rad_per_second() - rhs.as_tesla_rad_per_second()) }
}
impl<'a> Sub<MagneticAngularVelocity> for &'a MagneticAngularVelocity {
    type Output = MagneticAngularVelocity;
    fn sub(self, rhs: MagneticAngularVelocity) -> Self::Output { MagneticAngularVelocity::from_tesla_rad_per_second(self.as_tesla_rad_per_second() - rhs.as_tesla_rad_per_second()) }
}

impl Sub<f64> for MagneticAngularVelocity {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        MagneticAngularVelocity {
            v,
            default_type: self.default_type,
        }
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

impl Neg for MagneticAngularVelocity {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_tesla_rad_per_second();
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

    #[test]
    fn test_magnetic_angular_velocity_comprehensive_as_methods() {
        // 测试从每个单位类型创建，然后调用所有 as_xxx 方法
        
        // 从 TeslaRadPerSecond 创建
        let mav_tesla = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
        assert_relative_eq!(mav_tesla.as_tesla_rad_per_second(), 1.0);
        assert_relative_eq!(mav_tesla.as_mill_tesla_rad_per_second(), 1000.0);
        assert_relative_eq!(mav_tesla.as_micro_tesla_rad_per_second(), 1000000.0);
        assert_relative_eq!(mav_tesla.as_nano_tesla_rad_per_second(), 1000000000.0);
        assert_relative_eq!(mav_tesla.as_gauss_rad_per_second(), 10000.0);
        assert_relative_eq!(mav_tesla.as_mill_gauss_rad_per_second(), 10000000.0);
        assert_relative_eq!(mav_tesla.as_kilo_gauss_rad_per_second(), 10.0);

        // 从 MillTeslaRadPerSecond 创建
        let mav_mill_tesla = MagneticAngularVelocity::from_mill_tesla_rad_per_second(1000.0);
        assert_relative_eq!(mav_mill_tesla.as_tesla_rad_per_second(), 1.0);
        assert_relative_eq!(mav_mill_tesla.as_mill_tesla_rad_per_second(), 1000.0);
        assert_relative_eq!(mav_mill_tesla.as_micro_tesla_rad_per_second(), 1000000.0);
        assert_relative_eq!(mav_mill_tesla.as_nano_tesla_rad_per_second(), 1000000000.0);
        assert_relative_eq!(mav_mill_tesla.as_gauss_rad_per_second(), 10000.0);
        assert_relative_eq!(mav_mill_tesla.as_mill_gauss_rad_per_second(), 10000000.0);
        assert_relative_eq!(mav_mill_tesla.as_kilo_gauss_rad_per_second(), 10.0);

        // 从 MicroTeslaRadPerSecond 创建
        let mav_micro_tesla = MagneticAngularVelocity::from_micro_tesla_rad_per_second(1000000.0);
        assert_relative_eq!(mav_micro_tesla.as_tesla_rad_per_second(), 1.0);
        assert_relative_eq!(mav_micro_tesla.as_mill_tesla_rad_per_second(), 1000.0);
        assert_relative_eq!(mav_micro_tesla.as_micro_tesla_rad_per_second(), 1000000.0);
        assert_relative_eq!(mav_micro_tesla.as_nano_tesla_rad_per_second(), 1000000000.0);
        assert_relative_eq!(mav_micro_tesla.as_gauss_rad_per_second(), 10000.0);
        assert_relative_eq!(mav_micro_tesla.as_mill_gauss_rad_per_second(), 10000000.0);
        assert_relative_eq!(mav_micro_tesla.as_kilo_gauss_rad_per_second(), 10.0);

        // 从 NanoTeslaRadPerSecond 创建
        let mav_nano_tesla = MagneticAngularVelocity::from_nano_tesla_rad_per_second(1000000000.0);
        assert_relative_eq!(mav_nano_tesla.as_tesla_rad_per_second(), 1.0);
        assert_relative_eq!(mav_nano_tesla.as_mill_tesla_rad_per_second(), 1000.0);
        assert_relative_eq!(mav_nano_tesla.as_micro_tesla_rad_per_second(), 1000000.0);
        assert_relative_eq!(mav_nano_tesla.as_nano_tesla_rad_per_second(), 1000000000.0);
        assert_relative_eq!(mav_nano_tesla.as_gauss_rad_per_second(), 10000.0);
        assert_relative_eq!(mav_nano_tesla.as_mill_gauss_rad_per_second(), 10000000.0);
        assert_relative_eq!(mav_nano_tesla.as_kilo_gauss_rad_per_second(), 10.0);

        // 从 GaussRadPerSecond 创建
        let mav_gauss = MagneticAngularVelocity::from_gauss_rad_per_second(10000.0);
        assert_relative_eq!(mav_gauss.as_tesla_rad_per_second(), 1.0);
        assert_relative_eq!(mav_gauss.as_mill_tesla_rad_per_second(), 1000.0);
        assert_relative_eq!(mav_gauss.as_micro_tesla_rad_per_second(), 1000000.0);
        assert_relative_eq!(mav_gauss.as_nano_tesla_rad_per_second(), 1000000000.0);
        assert_relative_eq!(mav_gauss.as_gauss_rad_per_second(), 10000.0);
        assert_relative_eq!(mav_gauss.as_mill_gauss_rad_per_second(), 10000000.0);
        assert_relative_eq!(mav_gauss.as_kilo_gauss_rad_per_second(), 10.0);

        // 从 MillGaussRadPerSecond 创建
        let mav_mill_gauss = MagneticAngularVelocity::from_mill_gauss_rad_per_second(10000000.0);
        assert_relative_eq!(mav_mill_gauss.as_tesla_rad_per_second(), 1.0);
        assert_relative_eq!(mav_mill_gauss.as_mill_tesla_rad_per_second(), 1000.0);
        assert_relative_eq!(mav_mill_gauss.as_micro_tesla_rad_per_second(), 1000000.0);
        assert_relative_eq!(mav_mill_gauss.as_nano_tesla_rad_per_second(), 1000000000.0);
        assert_relative_eq!(mav_mill_gauss.as_gauss_rad_per_second(), 10000.0);
        assert_relative_eq!(mav_mill_gauss.as_mill_gauss_rad_per_second(), 10000000.0);
        assert_relative_eq!(mav_mill_gauss.as_kilo_gauss_rad_per_second(), 10.0);

        // 从 KiloGaussRadPerSecond 创建
        let mav_kilo_gauss = MagneticAngularVelocity::from_kilo_gauss_rad_per_second(10.0);
        assert_relative_eq!(mav_kilo_gauss.as_tesla_rad_per_second(), 1.0);
        assert_relative_eq!(mav_kilo_gauss.as_mill_tesla_rad_per_second(), 1000.0);
        assert_relative_eq!(mav_kilo_gauss.as_micro_tesla_rad_per_second(), 1000000.0);
        assert_relative_eq!(mav_kilo_gauss.as_nano_tesla_rad_per_second(), 1000000000.0);
        assert_relative_eq!(mav_kilo_gauss.as_gauss_rad_per_second(), 10000.0);
        assert_relative_eq!(mav_kilo_gauss.as_mill_gauss_rad_per_second(), 10000000.0);
        assert_relative_eq!(mav_kilo_gauss.as_kilo_gauss_rad_per_second(), 10.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_comprehensive_arithmetic_operations() {
        // 测试所有类型的 MagneticAngularVelocity 与 f64 的运算
        
        // 测试从不同单位创建的 MagneticAngularVelocity 与 f64 的加法
        let mav_tesla = MagneticAngularVelocity::from_tesla_rad_per_second(10.0);
        let result = mav_tesla + 5.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 15.0);
        
        let mav_mill_tesla = MagneticAngularVelocity::from_mill_tesla_rad_per_second(10000.0);
        let result = mav_mill_tesla + 5000.0;
        assert_relative_eq!(result.as_mill_tesla_rad_per_second(), 15000.0);
        
        let mav_micro_tesla = MagneticAngularVelocity::from_micro_tesla_rad_per_second(10000000.0);
        let result = mav_micro_tesla + 5000000.0;
        assert_relative_eq!(result.as_micro_tesla_rad_per_second(), 15000000.0);
        
        let mav_nano_tesla = MagneticAngularVelocity::from_nano_tesla_rad_per_second(10000000000.0);
        let result = mav_nano_tesla + 5000000000.0;
        assert_relative_eq!(result.as_nano_tesla_rad_per_second(), 15000000000.0);
        
        let mav_gauss = MagneticAngularVelocity::from_gauss_rad_per_second(100000.0);
        let result = mav_gauss + 50000.0;
        assert_relative_eq!(result.as_gauss_rad_per_second(), 150000.0);
        
        let mav_mill_gauss = MagneticAngularVelocity::from_mill_gauss_rad_per_second(100000000.0);
        let result = mav_mill_gauss + 50000000.0;
        assert_relative_eq!(result.as_mill_gauss_rad_per_second(), 150000000.0);
        
        let mav_kilo_gauss = MagneticAngularVelocity::from_kilo_gauss_rad_per_second(100.0);
        let result = mav_kilo_gauss + 50.0;
        assert_relative_eq!(result.as_kilo_gauss_rad_per_second(), 150.0);
        
        // 测试从不同单位创建的 MagneticAngularVelocity 与 f64 的减法
        let mav_tesla = MagneticAngularVelocity::from_tesla_rad_per_second(20.0);
        let result = mav_tesla - 5.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 15.0);
        
        let mav_mill_tesla = MagneticAngularVelocity::from_mill_tesla_rad_per_second(20000.0);
        let result = mav_mill_tesla - 5000.0;
        assert_relative_eq!(result.as_mill_tesla_rad_per_second(), 15000.0);
        
        let mav_micro_tesla = MagneticAngularVelocity::from_micro_tesla_rad_per_second(20000000.0);
        let result = mav_micro_tesla - 5000000.0;
        assert_relative_eq!(result.as_micro_tesla_rad_per_second(), 15000000.0);
        
        let mav_nano_tesla = MagneticAngularVelocity::from_nano_tesla_rad_per_second(20000000000.0);
        let result = mav_nano_tesla - 5000000000.0;
        assert_relative_eq!(result.as_nano_tesla_rad_per_second(), 15000000000.0);
        
        let mav_gauss = MagneticAngularVelocity::from_gauss_rad_per_second(200000.0);
        let result = mav_gauss - 50000.0;
        assert_relative_eq!(result.as_gauss_rad_per_second(), 150000.0);
        
        let mav_mill_gauss = MagneticAngularVelocity::from_mill_gauss_rad_per_second(200000000.0);
        let result = mav_mill_gauss - 50000000.0;
        assert_relative_eq!(result.as_mill_gauss_rad_per_second(), 150000000.0);
        
        let mav_kilo_gauss = MagneticAngularVelocity::from_kilo_gauss_rad_per_second(200.0);
        let result = mav_kilo_gauss - 50.0;
        assert_relative_eq!(result.as_kilo_gauss_rad_per_second(), 150.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_multiplication_operations() {
        // 测试所有类型的 MagneticAngularVelocity 与 f64 的乘法
        
        let mav_tesla = MagneticAngularVelocity::from_tesla_rad_per_second(10.0);
        let result = mav_tesla * 2.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 20.0);
        
        let mav_mill_tesla = MagneticAngularVelocity::from_mill_tesla_rad_per_second(1000.0);
        let result = mav_mill_tesla * 3.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 3.0);
        
        let mav_micro_tesla = MagneticAngularVelocity::from_micro_tesla_rad_per_second(1000000.0);
        let result = mav_micro_tesla * 2.5;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 2.5);
        
        let mav_nano_tesla = MagneticAngularVelocity::from_nano_tesla_rad_per_second(1000000000.0);
        let result = mav_nano_tesla * 1.5;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 1.5);
        
        let mav_gauss = MagneticAngularVelocity::from_gauss_rad_per_second(10000.0);
        let result = mav_gauss * 2.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 2.0);
        
        let mav_mill_gauss = MagneticAngularVelocity::from_mill_gauss_rad_per_second(10000000.0);
        let result = mav_mill_gauss * 3.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 3.0);
        
        let mav_kilo_gauss = MagneticAngularVelocity::from_kilo_gauss_rad_per_second(10.0);
        let result = mav_kilo_gauss * 4.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 4.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_division_operations() {
        // 测试所有类型的 MagneticAngularVelocity 与 f64 的除法
        
        let mav_tesla = MagneticAngularVelocity::from_tesla_rad_per_second(20.0);
        let result = mav_tesla / 2.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 10.0);
        
        let mav_mill_tesla = MagneticAngularVelocity::from_mill_tesla_rad_per_second(3000.0);
        let result = mav_mill_tesla / 3.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 1.0);
        
        let mav_micro_tesla = MagneticAngularVelocity::from_micro_tesla_rad_per_second(2500000.0);
        let result = mav_micro_tesla / 2.5;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 1.0);
        
        let mav_nano_tesla = MagneticAngularVelocity::from_nano_tesla_rad_per_second(1500000000.0);
        let result = mav_nano_tesla / 1.5;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 1.0);
        
        let mav_gauss = MagneticAngularVelocity::from_gauss_rad_per_second(20000.0);
        let result = mav_gauss / 2.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 1.0);
        
        let mav_mill_gauss = MagneticAngularVelocity::from_mill_gauss_rad_per_second(30000000.0);
        let result = mav_mill_gauss / 3.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 1.0);
        
        let mav_kilo_gauss = MagneticAngularVelocity::from_kilo_gauss_rad_per_second(40.0);
        let result = mav_kilo_gauss / 4.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 1.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_coef_operations() {
        // 测试与 Coef 的运算
        
        let mav = MagneticAngularVelocity::from_tesla_rad_per_second(10.0);
        let coef = Coef::new(2.0);
        
        // 测试与系数的乘法
        let result = mav * coef;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 20.0);
        
        // 测试与系数的除法
        let result = mav / coef;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 5.0);
        
        // 测试不同单位的系数运算
        let mav_gauss = MagneticAngularVelocity::from_gauss_rad_per_second(100000.0);
        let coef = Coef::new(3.0);
        let result = mav_gauss * coef;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 30.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_edge_cases_arithmetic() {
        // 测试边界情况的算术运算
        
        // 测试零值
        let zero_mav = MagneticAngularVelocity::from_tesla_rad_per_second(0.0);
        let result = zero_mav + 10.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 10.0);
        
        let result = zero_mav - 5.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), -5.0);
        
        let result = zero_mav * 100.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 0.0);
        
        // 测试负值
        let negative_mav = MagneticAngularVelocity::from_tesla_rad_per_second(-10.0);
        let result = negative_mav + 15.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 5.0);
        
        let result = negative_mav - 5.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), -15.0);
        
        let result = negative_mav * 2.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), -20.0);
        
        // 测试大数值
        let large_mav = MagneticAngularVelocity::from_kilo_gauss_rad_per_second(1000.0);
        let result = large_mav * 2.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 200.0);
        
        // 测试小数值
        let small_mav = MagneticAngularVelocity::from_nano_tesla_rad_per_second(1.0);
        let result = small_mav * 1000.0;
        assert_relative_eq!(result.as_tesla_rad_per_second(), 1e-6);
    }

    #[test]
    fn test_magnetic_angular_velocity_ref_ops() {
        let a = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
        let b = MagneticAngularVelocity::from_gauss_rad_per_second(10_000.0); // 1 T·rad/s
        let s = &a + &b;
        assert_relative_eq!(s.as_tesla_rad_per_second(), 3.0);

        let d = &a - &b;
        assert_relative_eq!(d.as_tesla_rad_per_second(), 1.0);

        // 混合引用：加/减
        let s2 = a + &b;
        assert_relative_eq!(s2.as_tesla_rad_per_second(), 3.0);
        let a1 = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
        let s3 = &a1 + b;
        assert_relative_eq!(s3.as_tesla_rad_per_second(), 3.0);

        let a2 = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
        let b2 = MagneticAngularVelocity::from_gauss_rad_per_second(10_000.0);
        let d2 = a2 - &b2;
        assert_relative_eq!(d2.as_tesla_rad_per_second(), 1.0);
        let a3 = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
        let b3 = MagneticAngularVelocity::from_gauss_rad_per_second(10_000.0);
        let d3 = &a3 - b3;
        assert_relative_eq!(d3.as_tesla_rad_per_second(), 1.0);
    }

    #[test]
    fn test_magnetic_angular_velocity_neg() {
        // 测试正值的负号
        let mav1 = MagneticAngularVelocity::from_tesla_rad_per_second(5.0);
        let neg_mav1 = -mav1;
        assert_relative_eq!(neg_mav1.as_tesla_rad_per_second(), -5.0);

        // 测试负值的负号
        let mav2 = MagneticAngularVelocity::from_gauss_rad_per_second(-10000.0);
        let neg_mav2 = -mav2;
        assert_relative_eq!(neg_mav2.as_gauss_rad_per_second(), 10000.0);

        // 测试零值
        let mav3 = MagneticAngularVelocity::from_tesla_rad_per_second(0.0);
        let neg_mav3 = -mav3;
        assert_relative_eq!(neg_mav3.as_tesla_rad_per_second(), 0.0);

        // 测试不同单位的负号操作
        let mav4 = MagneticAngularVelocity::from_mill_tesla_rad_per_second(1000.0);
        let neg_mav4 = -mav4;
        assert_relative_eq!(neg_mav4.as_tesla_rad_per_second(), -1.0);

        // 测试大数值
        let mav5 = MagneticAngularVelocity::from_kilo_gauss_rad_per_second(10.0);
        let neg_mav5 = -mav5;
        assert_relative_eq!(neg_mav5.as_tesla_rad_per_second(), -1.0);
    }
}
