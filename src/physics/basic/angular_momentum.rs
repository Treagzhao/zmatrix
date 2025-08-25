use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{ AngularMomentum, AngularMomentumType, Coef, PhysicalQuantity, Torque, AngularVelocity};

impl Default for AngularMomentum {
    fn default() -> Self {
        Self::from_nms(0.0)
    }
}


impl PhysicalQuantity for AngularMomentum {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_nms()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl AngularMomentum {
    pub fn from_kg_m2_per_second(kg: f64) -> Self {
        Self {
            default_type: AngularMomentumType::KgM2perSecond,
            v: kg,
        }
    }

    pub fn from_kg_km2_per_second(kg: f64) -> Self {
        Self {
            default_type: AngularMomentumType::KgKm2perSecond,
            v: kg,
        }
    }

    pub fn from_nms(nms: f64) -> Self {
        Self {
            default_type: AngularMomentumType::Nms,
            v: nms,
        }
    }

    pub fn from_mill_nms(mill_nms: f64) -> Self {
        Self {
            default_type: AngularMomentumType::MillNms,
            v: mill_nms,
        }
    }

    pub fn from_micro_nms(micro_nms: f64) -> Self {
        Self {
            default_type: AngularMomentumType::MicroNms,
            v: micro_nms,
        }
    }

    pub fn from_nano_nms(nano_nms: f64) -> Self {
        Self {
            default_type: AngularMomentumType::NanoNms,
            v: nano_nms,
        }
    }

    pub fn as_kg_m2_per_second(&self) -> f64 {
        match self.default_type {
            AngularMomentumType::KgM2perSecond => self.v,
            AngularMomentumType::KgKm2perSecond => self.v * 1e6,
            AngularMomentumType::Nms => self.v, // 1 N·m·s = 1 kg·m²/s
            AngularMomentumType::MillNms => self.v * 1e-3,
            AngularMomentumType::MicroNms => self.v * 1e-6,
            AngularMomentumType::NanoNms => self.v * 1e-9,
        }
    }

    pub fn as_kg_km2_per_second(&self) -> f64 {
        match self.default_type {
            AngularMomentumType::KgM2perSecond => self.v * 1e-6,
            AngularMomentumType::KgKm2perSecond => self.v,
            AngularMomentumType::Nms => self.v * 1e-6,
            AngularMomentumType::MillNms => self.v * 1e-9,
            AngularMomentumType::MicroNms => self.v * 1e-12,
            AngularMomentumType::NanoNms => self.v * 1e-15,
        }
    }

    pub fn as_nms(&self) -> f64 {
        match self.default_type {
            AngularMomentumType::KgM2perSecond => self.v,
            AngularMomentumType::KgKm2perSecond => self.v * 1e6,
            AngularMomentumType::Nms => self.v,
            AngularMomentumType::MillNms => self.v * 1e-3,
            AngularMomentumType::MicroNms => self.v * 1e-6,
            AngularMomentumType::NanoNms => self.v * 1e-9,
        }
    }

    pub fn as_mill_nms(&self) -> f64 {
        match self.default_type {
            AngularMomentumType::KgM2perSecond => self.v * 1e3,
            AngularMomentumType::KgKm2perSecond => self.v * 1e9,
            AngularMomentumType::Nms => self.v * 1e3,
            AngularMomentumType::MillNms => self.v,
            AngularMomentumType::MicroNms => self.v * 1e-3,
            AngularMomentumType::NanoNms => self.v * 1e-6,
        }
    }

    pub fn as_micro_nms(&self) -> f64 {
        match self.default_type {
            AngularMomentumType::KgM2perSecond => self.v * 1e6,
            AngularMomentumType::KgKm2perSecond => self.v * 1e12,
            AngularMomentumType::Nms => self.v * 1e6,
            AngularMomentumType::MillNms => self.v * 1e3,
            AngularMomentumType::MicroNms => self.v,
            AngularMomentumType::NanoNms => self.v * 1e-3,
        }
    }

    pub fn as_nano_nms(&self) -> f64 {
        match self.default_type {
            AngularMomentumType::KgM2perSecond => self.v * 1e9,
            AngularMomentumType::KgKm2perSecond => self.v * 1e15,
            AngularMomentumType::Nms => self.v * 1e9,
            AngularMomentumType::MillNms => self.v * 1e6,
            AngularMomentumType::MicroNms => self.v * 1e3,
            AngularMomentumType::NanoNms => self.v,
        }
    }
}


impl Add for AngularMomentum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_nms() + rhs.as_nms();
        Self::from_nms(v)
    }
}

impl Add<f64> for AngularMomentum {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        AngularMomentum {
            v,
            default_type: self.default_type,
        }
    }
}

impl Neg for AngularMomentum {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_nms();
        Self::from_nms(v)
    }
}

impl Sub for AngularMomentum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_nms() - rhs.as_nms();
        Self::from_nms(v)
    }
}

impl Sub<f64> for AngularMomentum {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        AngularMomentum {
            v,
            default_type: self.default_type,
        }
    }
}

impl Mul<f64> for AngularMomentum {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_nms() * rhs;
        Self::from_nms(v)
    }
}

impl Div<f64> for AngularMomentum {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_nms() / rhs;
        Self::from_nms(v)
    }
}

impl Mul<AngularMomentum> for f64 {
    type Output = AngularMomentum;
    fn mul(self, rhs: AngularMomentum) -> Self::Output {
        rhs * self
    }
}

impl Div<AngularMomentum> for f64 {
    type Output = AngularMomentum;
    fn div(self, rhs: AngularMomentum) -> Self::Output {
        let v = self / rhs.v;
        AngularMomentum {
            default_type: rhs.default_type,
            v: v,
        }
    }
}

impl Mul<Coef> for AngularMomentum {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_nms() * rhs.get_value();
        Self::from_nms(v)
    }
}

impl Div<Coef> for AngularMomentum {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_nms() / rhs.get_value();
        Self::from_nms(v)
    }
}

// 角动量 × 角速度 = 力矩
impl Mul<AngularVelocity> for AngularMomentum {
    type Output = Torque;
    fn mul(self, rhs: AngularVelocity) -> Self::Output {
        let torque_value = self.as_nms() * rhs.as_rad_per_second();
        Torque::from_nm(torque_value)
    }
}

// 角动量 ÷ 角速度 = 转动惯量（系数）
impl Div<AngularVelocity> for AngularMomentum {
    type Output = Coef;
    fn div(self, rhs: AngularVelocity) -> Self::Output {
        let moment_of_inertia_value = self.as_nms() / rhs.as_rad_per_second();
        Coef::new(moment_of_inertia_value)
    }
}


#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_angular_momentum() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        assert_eq!(m1.v, 1000.0);
        assert_eq!(m1.default_type, AngularMomentumType::KgM2perSecond);

        let m2 = AngularMomentum::default();
        assert_eq!(m2.v, 0.0);
        assert_eq!(m2.default_type, AngularMomentumType::Nms);

        let m3 = AngularMomentum::from_kg_km2_per_second(1000.0);
        assert_eq!(m3.v, 1000.0);
        assert_eq!(m3.default_type, AngularMomentumType::KgKm2perSecond);

        let m4 = AngularMomentum::from_nms(1000.0);
        assert_eq!(m4.v, 1000.0);
        assert_eq!(m4.default_type, AngularMomentumType::Nms);

        let m5 = AngularMomentum::from_mill_nms(1000.0);
        assert_eq!(m5.v, 1000.0);
        assert_eq!(m5.default_type, AngularMomentumType::MillNms);

        let m6 = AngularMomentum::from_micro_nms(1000.0);
        assert_eq!(m6.v, 1000.0);
        assert_eq!(m6.default_type, AngularMomentumType::MicroNms);

        let m7 = AngularMomentum::from_nano_nms(1000.0);
        assert_eq!(m7.v, 1000.0);
        assert_eq!(m7.default_type, AngularMomentumType::NanoNms);
    }

    #[test]
    fn test_angular_momentum_as() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        assert_eq!(m1.as_kg_m2_per_second(), 1000.0);
        assert_relative_eq!(m1.as_kg_km2_per_second(), 1e-3);
        assert_relative_eq!(m1.as_nms(), 1000.0);
        assert_relative_eq!(m1.as_mill_nms(), 1e6);
        assert_relative_eq!(m1.as_micro_nms(), 1e9);
        assert_relative_eq!(m1.as_nano_nms(), 1e12);

        let m2 = AngularMomentum::from_kg_km2_per_second(1000.0);
        assert_eq!(m2.as_kg_m2_per_second(), 1e9);
        assert_relative_eq!(m2.as_kg_km2_per_second(), 1000.0);
        assert_relative_eq!(m2.as_nms(), 1e9);
        assert_relative_eq!(m2.as_mill_nms(), 1e12);
        assert_relative_eq!(m2.as_micro_nms(), 1e15);
        assert_relative_eq!(m2.as_nano_nms(), 1e18);

        let m3 = AngularMomentum::from_nms(1000.0);
        assert_relative_eq!(m3.as_kg_m2_per_second(), 1000.0);
        assert_relative_eq!(m3.as_kg_km2_per_second(), 1e-3);
        assert_relative_eq!(m3.as_nms(), 1000.0);
        assert_relative_eq!(m3.as_mill_nms(), 1e6);
        assert_relative_eq!(m3.as_micro_nms(), 1e9);
        assert_relative_eq!(m3.as_nano_nms(), 1e12);

        let m4 = AngularMomentum::from_mill_nms(1000.0);
        assert_relative_eq!(m4.as_kg_m2_per_second(), 1.0);
        assert_relative_eq!(m4.as_kg_km2_per_second(), 1e-6);
        assert_relative_eq!(m4.as_nms(), 1.0);
        assert_relative_eq!(m4.as_mill_nms(), 1000.0);
        assert_relative_eq!(m4.as_micro_nms(), 1e6);
        assert_relative_eq!(m4.as_nano_nms(), 1e9);

        let m5 = AngularMomentum::from_micro_nms(1000.0);
        assert_relative_eq!(m5.as_kg_m2_per_second(), 1e-3);
        assert_relative_eq!(m5.as_kg_km2_per_second(), 1e-9);
        assert_relative_eq!(m5.as_nms(), 1e-3);
        assert_relative_eq!(m5.as_mill_nms(), 1.0);
        assert_relative_eq!(m5.as_micro_nms(), 1000.0);
        assert_relative_eq!(m5.as_nano_nms(), 1e6);

        let m6 = AngularMomentum::from_nano_nms(1000.0);
        assert_relative_eq!(m6.as_kg_m2_per_second(), 1e-6);
        assert_relative_eq!(m6.as_kg_km2_per_second(), 1e-12);
        assert_relative_eq!(m6.as_nms(), 1e-6);
        assert_relative_eq!(m6.as_mill_nms(), 1e-3);
        assert_relative_eq!(m6.as_micro_nms(), 1.0);
        assert_relative_eq!(m6.as_nano_nms(), 1000.0);
    }

    #[test]
    fn test_angular_momentum_add() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_nms(), 2000.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 + m2;
        assert_eq!(m3.as_nms(), 2.0 * 1000.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = m1 + 100.0;
        assert_eq!(m2.as_nms(), 1100.0);

        // 测试不同单位之间的加法
        let m1 = AngularMomentum::from_nms(1000.0);
        let m2 = AngularMomentum::from_mill_nms(1000.0);
        let m3 = m1 + m2;
        assert_relative_eq!(m3.as_nms(), 1001.0);

        let m1 = AngularMomentum::from_micro_nms(1000.0);
        let m2 = AngularMomentum::from_nano_nms(1000.0);
        let m3 = m1 + m2;
        assert_relative_eq!(m3.as_nms(), 1e-3 + 1e-6);
    }

    #[test]
    fn test_angular_momentum_sub() {
        let m1 = AngularMomentum::from_kg_m2_per_second(2000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_nms(), 1000.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(3000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_nms(), 2.0 * 1000.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(1100.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m3 = m1 - m2;
        assert_eq!(m3.as_nms(), 100.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = m1 - 100.0;
        assert_eq!(m2.as_nms(), 900.0);

        // 测试不同单位之间的减法
        let m1 = AngularMomentum::from_nms(1000.0);
        let m2 = AngularMomentum::from_mill_nms(1000.0);
        let m3 = m1 - m2;
        assert_relative_eq!(m3.as_nms(), 999.0);
    }

    #[test]
    fn test_angular_momentum_mul() {
        let m1 = AngularMomentum::from_kg_m2_per_second(2000.0);
        let m2 = m1 * 2.0;
        assert_eq!(m2.as_nms(), 4000.0);
        let m1 = AngularMomentum::from_kg_m2_per_second(3000.0);
        let m2 = m1 * 2.0;
        assert_eq!(m2.as_nms(), 6000.0);

        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = m1 * Coef::new(2.0);
        assert_eq!(m2.as_nms(), 2000.0);

        // 测试不同单位的乘法
        let m1 = AngularMomentum::from_mill_nms(1000.0);
        let m2 = m1 * 2.0;
        assert_relative_eq!(m2.as_nms(), 2.0);

        let m1 = AngularMomentum::from_micro_nms(1000.0);
        let m2 = m1 * 3.0;
        assert_relative_eq!(m2.as_nms(), 3e-3);
    }

    #[test]
    fn test_angular_momentum_div() {
        let m1 = AngularMomentum::from_kg_m2_per_second(2000.0);
        let m2 = m1 / 2.0;
        assert_eq!(m2.as_nms(), 1000.0);
        let m1 = AngularMomentum::from_kg_m2_per_second(3000.0);
        let m2 = m1 / 2.0;
        assert_eq!(m2.as_nms(), 1500.0);
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = m1 / Coef::new(2.0);
        assert_eq!(m2.as_nms(), 500.0);

        // 测试不同单位的除法
        let m1 = AngularMomentum::from_mill_nms(2000.0);
        let m2 = m1 / 2.0;
        assert_relative_eq!(m2.as_nms(), 1.0);

        let m1 = AngularMomentum::from_micro_nms(3000.0);
        let m2 = m1 / 3.0;
        assert_relative_eq!(m2.as_nms(), 1e-3);
    }

    #[test]
    fn test_is_zero() {
        let m1 = AngularMomentum::from_kg_m2_per_second(0.0);
        assert!(m1.is_zero());
        let m1 = AngularMomentum::from_kg_m2_per_second(1.0);
        assert!(!m1.is_zero());
    }

    #[test]
    fn test_default_unit_value() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1.0);
        assert_eq!(m1.default_unit_value(), 1.0);
    }

    #[test]
    fn test_set_value() {
        let mut m1 = AngularMomentum::from_kg_m2_per_second(1.0);
        m1.set_value(1000.0);
        assert_eq!(m1.as_kg_m2_per_second(), 1000.0);
    }

    #[test]
    fn test_negative() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = -m1;
        assert_eq!(m2.as_kg_m2_per_second(), -1000.0);
    }
    #[test]
    fn test_div() {
        let m1 = AngularMomentum::from_kg_m2_per_second(1000.0);
        let m2 = AngularMomentum::from_kg_m2_per_second(500.0);
        let m3 = m1 / m2;
        assert_eq!(m3.get_value(), 2.0);
    }

    #[test]
    fn test_f64_mul_angular_momentum() {
        // f64 * AngularMomentum 测试
        let m = AngularMomentum::from_kg_m2_per_second(2.0);
        let result = 3.0 * m;
        assert_relative_eq!(result.as_kg_m2_per_second(), 6.0);

        let m = AngularMomentum::from_kg_km2_per_second(1.0);
        let result = 2.0 * m;
        assert_relative_eq!(result.as_kg_km2_per_second(), 2.0);
    }

    #[test]
    fn test_f64_div_angular_momentum() {
        // f64 / AngularMomentum 测试
        let m = AngularMomentum::from_kg_m2_per_second(2.0);
        let result = 6.0 / m;
        assert_relative_eq!(result.as_kg_m2_per_second(), 3.0);

        let m = AngularMomentum::from_kg_km2_per_second(1.0);
        let result = 2.0 / m;
        assert_relative_eq!(result.as_kg_km2_per_second(), 2.0);
    }

    #[test]
    fn test_angular_momentum_mul_angular_velocity() {
        // 角动量 × 角速度 = 力矩
        let l = AngularMomentum::from_kg_m2_per_second(10.0);
        let omega = AngularVelocity::from_rad_per_second(5.0);
        let torque = l * omega;
        assert_relative_eq!(torque.as_nm(), 50.0);

        // 测试不同单位
        let l = AngularMomentum::from_kg_km2_per_second(1.0);
        let omega = AngularVelocity::from_deg_per_second(180.0);
        let torque = l * omega;
        assert_relative_eq!(torque.as_nm(), 1e6 * std::f64::consts::PI);
    }

    #[test]
    fn test_angular_momentum_div_angular_velocity() {
        // 角动量 ÷ 角速度 = 转动惯量（系数）
        let l = AngularMomentum::from_kg_m2_per_second(20.0);
        let omega = AngularVelocity::from_rad_per_second(4.0);
        let moment_of_inertia = l / omega;
        assert_relative_eq!(moment_of_inertia.get_value(), 5.0);

        // 测试不同单位
        let l = AngularMomentum::from_kg_km2_per_second(1.0);
        let omega = AngularVelocity::from_deg_per_second(360.0);
        let moment_of_inertia = l / omega;
        assert_relative_eq!(moment_of_inertia.get_value(), 1e6 / (2.0 * std::f64::consts::PI));
    }
}