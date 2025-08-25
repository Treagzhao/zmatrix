use std::any::Any;
use std::ops::{Add, Div, Mul, Sub};
use crate::physics::basic::{Coef, Torque, TorqueType, PhysicalQuantity, Distance, Energy, AngularMomentum, AngularVelocity, Force, Power};
use approx::assert_relative_eq;

impl Default for Torque {
    fn default() -> Self {
        Self::from_nm(0.0)
    }
}

impl PhysicalQuantity for Torque {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_nm()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Torque {
    pub fn from_nm(nm: f64) -> Self {
        Self {
            default_type: TorqueType::NM,
            v: nm,
        }
    }

    pub fn from_mill_nm(mill_nm: f64) -> Self {
        Self {
            default_type: TorqueType::MillNM,
            v: mill_nm,
        }
    }

    pub fn from_micro_nm(micro_nm: f64) -> Self {
        Self {
            default_type: TorqueType::MicroNM,
            v: micro_nm,
        }
    }

    pub fn from_nano_nm(nano_nm: f64) -> Self {
        Self {
            default_type: TorqueType::NanoNM,
            v: nano_nm,
        }
    }

    pub fn from_knm(knm: f64) -> Self {
        Self {
            default_type: TorqueType::KNM,
            v: knm,
        }
    }

    pub fn from_mnm(mnm: f64) -> Self {
        Self {
            default_type: TorqueType::MNM,
            v: mnm,
        }
    }

    pub fn as_nm(&self) -> f64 {
        match self.default_type {
            TorqueType::NM => self.v,
            TorqueType::MillNM => self.v * 1e-3,
            TorqueType::MicroNM => self.v * 1e-6,
            TorqueType::NanoNM => self.v * 1e-9,
            TorqueType::KNM => self.v * 1e3,
            TorqueType::MNM => self.v * 1e6,
        }
    }

    pub fn as_mill_nm(&self) -> f64 {
        match self.default_type {
            TorqueType::NM => self.v * 1e3,
            TorqueType::MillNM => self.v,
            TorqueType::MicroNM => self.v * 1e-3,
            TorqueType::NanoNM => self.v * 1e-6,
            TorqueType::KNM => self.v * 1e6,
            TorqueType::MNM => self.v * 1e9,
        }
    }

    pub fn as_micro_nm(&self) -> f64 {
        match self.default_type {
            TorqueType::NM => self.v * 1e6,
            TorqueType::MillNM => self.v * 1e3,
            TorqueType::MicroNM => self.v,
            TorqueType::NanoNM => self.v * 1e-3,
            TorqueType::KNM => self.v * 1e9,
            TorqueType::MNM => self.v * 1e12,
        }
    }

    pub fn as_nano_nm(&self) -> f64 {
        match self.default_type {
            TorqueType::NM => self.v * 1e9,
            TorqueType::MillNM => self.v * 1e6,
            TorqueType::MicroNM => self.v * 1e3,
            TorqueType::NanoNM => self.v,
            TorqueType::KNM => self.v * 1e12,
            TorqueType::MNM => self.v * 1e15,
        }
    }

    pub fn as_knm(&self) -> f64 {
        match self.default_type {
            TorqueType::NM => self.v * 1e-3,
            TorqueType::MillNM => self.v * 1e-6,
            TorqueType::MicroNM => self.v * 1e-9,
            TorqueType::NanoNM => self.v * 1e-12,
            TorqueType::KNM => self.v,
            TorqueType::MNM => self.v * 1e3,
        }
    }

    pub fn as_mnm(&self) -> f64 {
        match self.default_type {
            TorqueType::NM => self.v * 1e-6,
            TorqueType::MillNM => self.v * 1e-9,
            TorqueType::MicroNM => self.v * 1e-12,
            TorqueType::NanoNM => self.v * 1e-15,
            TorqueType::KNM => self.v * 1e-3,
            TorqueType::MNM => self.v,
        }
    }
}

// 力矩与力矩的运算
impl Add for Torque {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_nm() + rhs.as_nm();
        Self::from_nm(v)
    }
}

impl Sub for Torque {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_nm() - rhs.as_nm();
        Self::from_nm(v)
    }
}

// 力矩与标量的运算
impl Add<f64> for Torque {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        match self.default_type {
            TorqueType::NM => {
                let v = self.as_nm() + rhs;
                Self::from_nm(v)
            }
            TorqueType::MillNM => {
                let v = self.as_mill_nm() + rhs;
                Self::from_mill_nm(v)
            }
            TorqueType::MicroNM => {
                let v = self.as_micro_nm() + rhs;
                Self::from_micro_nm(v)
            }
            TorqueType::NanoNM => {
                let v = self.as_nano_nm() + rhs;
                Self::from_nano_nm(v)
            }
            TorqueType::KNM => {
                let v = self.as_knm() + rhs;
                Self::from_knm(v)
            }
            TorqueType::MNM => {
                let v = self.as_mnm() + rhs;
                Self::from_mnm(v)
            }
        }
    }
}

impl Sub<f64> for Torque {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        Torque {
            v,
            default_type: self.default_type,
        }
    }
}

impl Mul<f64> for Torque {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_nm() * rhs;
        Self::from_nm(v)
    }
}

impl Div<f64> for Torque {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_nm() / rhs;
        Self::from_nm(v)
    }
}

// 标量与力矩的运算
impl Mul<Torque> for f64 {
    type Output = Torque;
    fn mul(self, rhs: Torque) -> Self::Output {
        let v = self * rhs.as_nm();
        Torque::from_nm(v)
    }
}

impl Div<Torque> for f64 {
    type Output = Torque;
    fn div(self, rhs: Torque) -> Self::Output {
        let v = self / rhs.as_nm();
        Torque::from_nm(v)
    }
}

// 力矩与系数的运算
impl Mul<Coef> for Torque {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_nm() * rhs.get_value();
        Self::from_nm(v)
    }
}

impl Div<Coef> for Torque {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_nm() / rhs.get_value();
        Self::from_nm(v)
    }
}

// 力矩 ÷ 角速度 = 角动量
impl Div<AngularVelocity> for Torque {
    type Output = AngularMomentum;
    fn div(self, rhs: AngularVelocity) -> Self::Output {
        let angular_momentum_value = self.as_nm() / rhs.as_rad_per_second();
        AngularMomentum::from_kg_m2_per_second(angular_momentum_value)
    }
}

// 力矩 ÷ 距离 = 力
impl Div<Distance> for Torque {
    type Output = Force;
    fn div(self, rhs: Distance) -> Self::Output {
        let force_value = self.as_nm() / rhs.as_m();
        Force::from_newton(force_value)
    }
}

// 力矩 × 角速度 = 功率
impl Mul<AngularVelocity> for Torque {
    type Output = Power;
    fn mul(self, rhs: AngularVelocity) -> Self::Output {
        let power_value = self.as_nm() * rhs.as_rad_per_second();
        Power::from_watt(power_value)
    }
}

// 力矩与距离的乘积得到功（能量）
impl Mul<Distance> for Torque {
    type Output = Energy; // 功，单位：焦耳
    fn mul(self, rhs: Distance) -> Self::Output {
        let energy_value = self.as_nm() * rhs.as_m();
        Energy::from_joule(energy_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torque() {
        let t1 = Torque::from_nm(1000.0);
        assert_eq!(t1.v, 1000.0);
        assert_eq!(t1.default_type, TorqueType::NM);

        let t2 = Torque::from_mill_nm(1000.0);
        assert_eq!(t2.v, 1000.0);
        assert_eq!(t2.default_type, TorqueType::MillNM);

        let t3 = Torque::from_micro_nm(1000.0);
        assert_eq!(t3.v, 1000.0);
        assert_eq!(t3.default_type, TorqueType::MicroNM);

        let t4 = Torque::from_nano_nm(1000.0);
        assert_eq!(t4.v, 1000.0);
        assert_eq!(t4.default_type, TorqueType::NanoNM);

        let t5 = Torque::from_knm(1.0);
        assert_eq!(t5.v, 1.0);
        assert_eq!(t5.default_type, TorqueType::KNM);

        let t6 = Torque::from_mnm(1.0);
        assert_eq!(t6.v, 1.0);
        assert_eq!(t6.default_type, TorqueType::MNM);

        let t7 = Torque::default();
        assert_eq!(t7.v, 0.0);
        assert_eq!(t7.default_type, TorqueType::NM);
    }

    #[test]
    fn test_torque_as() {
        let t1 = Torque::from_nm(1.0);
        assert_relative_eq!(t1.as_nm(), 1.0);
        assert_relative_eq!(t1.as_mill_nm(), 1e3);
        assert_relative_eq!(t1.as_micro_nm(), 1e6);
        assert_relative_eq!(t1.as_nano_nm(), 1e9);
        assert_relative_eq!(t1.as_knm(), 1e-3);
        assert_relative_eq!(t1.as_mnm(), 1e-6);

        let t2 = Torque::from_mill_nm(1e3);
        assert_relative_eq!(t2.as_nm(), 1.0);
        assert_relative_eq!(t2.as_mill_nm(), 1e3);
        assert_relative_eq!(t2.as_micro_nm(), 1e6);

        let t3 = Torque::from_micro_nm(1e6);
        assert_relative_eq!(t3.as_nm(), 1.0);
        assert_relative_eq!(t3.as_micro_nm(), 1e6);

        let t4 = Torque::from_nano_nm(1e9);
        assert_relative_eq!(t4.as_nm(), 1.0);
        assert_relative_eq!(t4.as_nano_nm(), 1e9);

        let t5 = Torque::from_knm(1e-3);
        assert_relative_eq!(t5.as_nm(), 1.0);
        assert_relative_eq!(t5.as_knm(), 1e-3);

        let t6 = Torque::from_mnm(1e-6);
        assert_relative_eq!(t6.as_nm(), 1.0);
        assert_relative_eq!(t6.as_mnm(), 1e-6);
    }

    #[test]
    fn test_torque_add() {
        let t1 = Torque::from_nm(1000.0);
        let t2 = Torque::from_nm(1000.0);
        let t3 = t1 + t2;
        assert_relative_eq!(t3.as_nm(), 2000.0);

        let t1 = Torque::from_mill_nm(1e6);
        let t2 = Torque::from_nm(1000.0);
        let t3 = t1 + t2;
        assert_relative_eq!(t3.as_nm(), 2000.0);

        let t1 = Torque::from_nm(1000.0);
        let t2 = t1 + 100.0;
        assert_relative_eq!(t2.as_nm(), 1100.0);

        let t1 = Torque::from_mill_nm(1000.0);
        let t2 = t1 + 100.0;
        assert_relative_eq!(t2.as_mill_nm(), 1100.0);
    }

    #[test]
    fn test_torque_sub() {
        let t1 = Torque::from_nm(2000.0);
        let t2 = Torque::from_nm(1000.0);
        let t3 = t1 - t2;
        assert_relative_eq!(t3.as_nm(), 1000.0);

        let t1 = Torque::from_mill_nm(2e6); // 2000 mN·m = 2 N·m
        let t2 = Torque::from_nm(1000.0);
        let t3 = t1 - t2;
        assert_relative_eq!(t3.as_nm(), 1000.0);

        let t1 = Torque::from_nm(2000.0);
        let t2 = t1 - 100.0;
        assert_relative_eq!(t2.as_nm(), 1900.0);
    }

    #[test]
    fn test_torque_mul() {
        let t1 = Torque::from_nm(2000.0);
        let t2 = t1 * 2.0;
        assert_relative_eq!(t2.as_nm(), 4000.0);

        let t1 = Torque::from_mill_nm(2000.0);
        let t2 = t1 * 2.0;
        assert_relative_eq!(t2.as_mill_nm(), 4000.0);

        let t1 = Torque::from_nm(1000.0);
        let t2 = t1 * Coef::new(2.0);
        assert_relative_eq!(t2.as_nm(), 2000.0);
    }

    #[test]
    fn test_torque_div() {
        let t1 = Torque::from_nm(2000.0);
        let t2 = t1 / 2.0;
        assert_relative_eq!(t2.as_nm(), 1000.0);

        let t1 = Torque::from_mill_nm(2000.0);
        let t2 = t1 / 2.0;
        assert_relative_eq!(t2.as_mill_nm(), 1000.0);

        let t1 = Torque::from_nm(2000.0);
        let t2 = t1 / Coef::new(2.0);
        assert_relative_eq!(t2.as_nm(), 1000.0);
    }

    #[test]
    fn test_default_unit_value() {
        let t1 = Torque::from_nm(1.0);
        assert_relative_eq!(t1.default_unit_value(), 1.0);
        let t1 = Torque::from_mill_nm(1.0);
        assert_relative_eq!(t1.default_unit_value(), 1e-3);
    }

    #[test]
    fn test_f64_mul_torque() {
        let t = Torque::from_nm(2.0);
        let result = 3.0 * t;
        assert_relative_eq!(result.as_nm(), 6.0);

        let t = Torque::from_mill_nm(2.0);
        let result = 3.0 * t;
        assert_relative_eq!(result.as_mill_nm(), 6.0);
    }

    #[test]
    fn test_f64_div_torque() {
        let t = Torque::from_nm(2.0);
        let result = 6.0 / t;
        assert_relative_eq!(result.as_nm(), 3.0);

        let t = Torque::from_mill_nm(2000.0); // 2000 mN·m = 2 N·m
        let result = 6.0 / t;
        assert_relative_eq!(result.as_nm(), 3.0); // 6 N·m / 2 N·m = 3 N·m
    }

    #[test]
    fn test_torque_mul_distance() {
        let t = Torque::from_nm(1.0);
        let d = Distance::from_m(2.0);
        let work = t * d;
        assert_relative_eq!(work.as_joule(), 2.0); // 1 N·m × 2 m = 2 J

        let t = Torque::from_mill_nm(1000.0);
        let d = Distance::from_m(1.0);
        let work = t * d;
        assert_relative_eq!(work.as_joule(), 1.0); // 1 N·m × 1 m = 1 J
    }

    #[test]
    fn test_set_value() {
        let mut t = Torque::from_nm(1.0);
        t.set_value(2.0);
        assert_relative_eq!(t.as_nm(), 2.0);
    }

    #[test]
    fn test_torque_div_angular_velocity() {
        // 力矩 ÷ 角速度 = 角动量
        let torque = Torque::from_nm(20.0);
        let omega = AngularVelocity::from_rad_per_second(4.0);
        let angular_momentum = torque / omega;
        assert_relative_eq!(angular_momentum.as_kg_m2_per_second(), 5.0);

        // 测试不同单位
        let torque = Torque::from_mill_nm(2000.0); // 2 N·m
        let omega = AngularVelocity::from_deg_per_second(360.0); // 2π rad/s
        let angular_momentum = torque / omega;
        assert_relative_eq!(angular_momentum.as_kg_m2_per_second(), 1.0 / std::f64::consts::PI);
    }

    #[test]
    fn test_torque_div_distance() {
        let torque = Torque::from_nm(20.0); // 20 N·m
        let distance = Distance::from_m(4.0); // 4 m
        let force: Force = torque / distance; // 5 N
        
        assert_relative_eq!(force.as_newton(), 5.0);
    }

    #[test]
    fn test_torque_mul_angular_velocity() {
        let torque = Torque::from_nm(100.0); // 100 N·m
        let angular_velocity = AngularVelocity::from_rad_per_second(50.0); // 50 rad/s
        let power: Power = torque * angular_velocity; // 5000 W
        
        assert_relative_eq!(power.as_watt(), 5000.0);
    }

    #[test]
    fn test_torque_comprehensive_as_methods() {
        // 测试从每个单位类型创建，然后调用所有 as_xxx 方法
        
        // 从 NM 创建
        let t_nm = Torque::from_nm(1.0);
        assert_relative_eq!(t_nm.as_nm(), 1.0);
        assert_relative_eq!(t_nm.as_mill_nm(), 1000.0);
        assert_relative_eq!(t_nm.as_micro_nm(), 1000000.0);
        assert_relative_eq!(t_nm.as_nano_nm(), 1000000000.0);
        assert_relative_eq!(t_nm.as_knm(), 0.001);
        assert_relative_eq!(t_nm.as_mnm(), 0.000001);

        // 从 MillNM 创建
        let t_mill_nm = Torque::from_mill_nm(1000.0);
        assert_relative_eq!(t_mill_nm.as_nm(), 1.0);
        assert_relative_eq!(t_mill_nm.as_mill_nm(), 1000.0);
        assert_relative_eq!(t_mill_nm.as_micro_nm(), 1000000.0);
        assert_relative_eq!(t_mill_nm.as_nano_nm(), 1000000000.0);
        assert_relative_eq!(t_mill_nm.as_knm(), 0.001);
        assert_relative_eq!(t_mill_nm.as_mnm(), 0.000001);

        // 从 MicroNM 创建
        let t_micro_nm = Torque::from_micro_nm(1000000.0);
        assert_relative_eq!(t_micro_nm.as_nm(), 1.0);
        assert_relative_eq!(t_micro_nm.as_mill_nm(), 1000.0);
        assert_relative_eq!(t_micro_nm.as_micro_nm(), 1000000.0);
        assert_relative_eq!(t_micro_nm.as_nano_nm(), 1000000000.0);
        assert_relative_eq!(t_micro_nm.as_knm(), 0.001);
        assert_relative_eq!(t_micro_nm.as_mnm(), 0.000001);

        // 从 NanoNM 创建
        let t_nano_nm = Torque::from_nano_nm(1000000000.0);
        assert_relative_eq!(t_nano_nm.as_nm(), 1.0);
        assert_relative_eq!(t_nano_nm.as_mill_nm(), 1000.0);
        assert_relative_eq!(t_nano_nm.as_micro_nm(), 1000000.0);
        assert_relative_eq!(t_nano_nm.as_nano_nm(), 1000000000.0);
        assert_relative_eq!(t_nano_nm.as_knm(), 0.001);
        assert_relative_eq!(t_nano_nm.as_mnm(), 0.000001);

        // 从 KNM 创建
        let t_knm = Torque::from_knm(0.001);
        assert_relative_eq!(t_knm.as_nm(), 1.0);
        assert_relative_eq!(t_knm.as_mill_nm(), 1000.0);
        assert_relative_eq!(t_knm.as_micro_nm(), 1000000.0);
        assert_relative_eq!(t_knm.as_nano_nm(), 1000000000.0);
        assert_relative_eq!(t_knm.as_knm(), 0.001);
        assert_relative_eq!(t_knm.as_mnm(), 0.000001);

        // 从 MNM 创建
        let t_mnm = Torque::from_mnm(0.000001);
        assert_relative_eq!(t_mnm.as_nm(), 1.0);
        assert_relative_eq!(t_mnm.as_mill_nm(), 1000.0);
        assert_relative_eq!(t_mnm.as_micro_nm(), 1000000.0);
        assert_relative_eq!(t_mnm.as_nano_nm(), 1000000000.0);
        assert_relative_eq!(t_mnm.as_knm(), 0.001);
        assert_relative_eq!(t_mnm.as_mnm(), 0.000001);
    }

    #[test]
    fn test_torque_comprehensive_arithmetic_operations() {
        // 测试所有类型的 Torque 与 f64 的运算
        
        // 测试从不同单位创建的 Torque 与 f64 的加法
        let t_nm = Torque::from_nm(10.0);
        let result = t_nm + 5.0;
        assert_relative_eq!(result.as_nm(), 15.0);
        
        let t_mill_nm = Torque::from_mill_nm(10000.0);
        let result = t_mill_nm + 5000.0;
        assert_relative_eq!(result.as_mill_nm(), 15000.0);
        
        let t_micro_nm = Torque::from_micro_nm(10000000.0);
        let result = t_micro_nm + 5000000.0;
        assert_relative_eq!(result.as_micro_nm(), 15000000.0);
        
        let t_nano_nm = Torque::from_nano_nm(10000000000.0);
        let result = t_nano_nm + 5000000000.0;
        assert_relative_eq!(result.as_nano_nm(), 15000000000.0);
        
        let t_knm = Torque::from_knm(0.01);
        let result = t_knm + 0.005;
        assert_relative_eq!(result.as_knm(), 0.015);
        
        let t_mnm = Torque::from_mnm(0.00001);
        let result = t_mnm + 0.000005;
        assert_relative_eq!(result.as_mnm(), 0.000015);
        
        // 测试从不同单位创建的 Torque 与 f64 的减法
        let t_nm = Torque::from_nm(20.0);
        let result = t_nm - 5.0;
        assert_relative_eq!(result.as_nm(), 15.0);
        
        let t_mill_nm = Torque::from_mill_nm(20000.0);
        let result = t_mill_nm - 5000.0;
        assert_relative_eq!(result.as_mill_nm(), 15000.0);
        
        let t_micro_nm = Torque::from_micro_nm(20000000.0);
        let result = t_micro_nm - 5000000.0;
        assert_relative_eq!(result.as_micro_nm(), 15000000.0);
        
        let t_nano_nm = Torque::from_nano_nm(20000000000.0);
        let result = t_nano_nm - 5000000000.0;
        assert_relative_eq!(result.as_nano_nm(), 15000000000.0);
        
        let t_knm = Torque::from_knm(0.02);
        let result = t_knm - 0.005;
        assert_relative_eq!(result.as_knm(), 0.015);
        
        let t_mnm = Torque::from_mnm(0.00002);
        let result = t_mnm - 0.000005;
        assert_relative_eq!(result.as_mnm(), 0.000015);
    }

    #[test]
    fn test_torque_multiplication_operations() {
        // 测试所有类型的 Torque 与 f64 的乘法
        
        let t_nm = Torque::from_nm(10.0);
        let result = t_nm * 2.0;
        assert_relative_eq!(result.as_nm(), 20.0);
        
        let t_mill_nm = Torque::from_mill_nm(1000.0);
        let result = t_mill_nm * 3.0;
        assert_relative_eq!(result.as_nm(), 3.0);
        
        let t_micro_nm = Torque::from_micro_nm(1000000.0);
        let result = t_micro_nm * 2.5;
        assert_relative_eq!(result.as_nm(), 2.5);
        
        let t_nano_nm = Torque::from_nano_nm(1000000000.0);
        let result = t_nano_nm * 1.5;
        assert_relative_eq!(result.as_nm(), 1.5);
        
        let t_knm = Torque::from_knm(0.001);
        let result = t_knm * 2.0;
        assert_relative_eq!(result.as_nm(), 2.0);
        
        let t_mnm = Torque::from_mnm(0.000001);
        let result = t_mnm * 3.0;
        assert_relative_eq!(result.as_nm(), 3.0);
    }

    #[test]
    fn test_torque_division_operations() {
        // 测试所有类型的 Torque 与 f64 的除法
        
        let t_nm = Torque::from_nm(20.0);
        let result = t_nm / 2.0;
        assert_relative_eq!(result.as_nm(), 10.0);
        
        let t_mill_nm = Torque::from_mill_nm(3000.0);
        let result = t_mill_nm / 3.0;
        assert_relative_eq!(result.as_nm(), 1.0);
        
        let t_micro_nm = Torque::from_micro_nm(2500000.0);
        let result = t_micro_nm / 2.5;
        assert_relative_eq!(result.as_nm(), 1.0);
        
        let t_nano_nm = Torque::from_nano_nm(1500000000.0);
        let result = t_nano_nm / 1.5;
        assert_relative_eq!(result.as_nm(), 1.0);
        
        let t_knm = Torque::from_knm(0.002);
        let result = t_knm / 2.0;
        assert_relative_eq!(result.as_nm(), 1.0);
        
        let t_mnm = Torque::from_mnm(0.000003);
        let result = t_mnm / 3.0;
        assert_relative_eq!(result.as_nm(), 1.0);
    }

    #[test]
    fn test_torque_coef_operations() {
        // 测试与 Coef 的运算
        
        let t = Torque::from_nm(10.0);
        let coef = Coef::new(2.0);
        
        // 测试与系数的乘法
        let result = t * coef;
        assert_relative_eq!(result.as_nm(), 20.0);
        
        // 测试与系数的除法
        let result = t / coef;
        assert_relative_eq!(result.as_nm(), 5.0);
        
        // 测试不同单位的系数运算
        let t_knm = Torque::from_knm(0.01);
        let coef = Coef::new(3.0);
        let result = t_knm * coef;
        assert_relative_eq!(result.as_nm(), 30.0);
    }

    #[test]
    fn test_torque_edge_cases_arithmetic() {
        // 测试边界情况的算术运算
        
        // 测试零值
        let zero_torque = Torque::from_nm(0.0);
        let result = zero_torque + 10.0;
        assert_relative_eq!(result.as_nm(), 10.0);
        
        let result = zero_torque - 5.0;
        assert_relative_eq!(result.as_nm(), -5.0);
        
        let result = zero_torque * 100.0;
        assert_relative_eq!(result.as_nm(), 0.0);
        
        // 测试负值
        let negative_torque = Torque::from_nm(-10.0);
        let result = negative_torque + 15.0;
        assert_relative_eq!(result.as_nm(), 5.0);
        
        let result = negative_torque - 5.0;
        assert_relative_eq!(result.as_nm(), -15.0);
        
        let result = negative_torque * 2.0;
        assert_relative_eq!(result.as_nm(), -20.0);
        
        // 测试大数值
        let large_torque = Torque::from_mnm(1000.0);
        let result = large_torque * 2.0;
        assert_relative_eq!(result.as_nm(), 2e9);
        
        // 测试小数值
        let small_torque = Torque::from_nano_nm(1.0);
        let result = small_torque * 1000.0;
        assert_relative_eq!(result.as_nm(), 1e-6);
    }
}
