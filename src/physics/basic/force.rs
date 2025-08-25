use std::any::Any;
use std::ops::{Add, Div, Mul, Sub};
use crate::physics::basic::{Coef, Force, ForceType, PhysicalQuantity, Distance, Energy, Mass, Acceleration};
use approx::assert_relative_eq;

impl Default for Force {
    fn default() -> Self {
        Self::from_newton(0.0)
    }
}

impl PhysicalQuantity for Force {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_newton()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Force {
    pub fn from_newton(newton: f64) -> Self {
        Self {
            default_type: ForceType::Newton,
            v: newton,
        }
    }

    pub fn from_mill_newton(mill_newton: f64) -> Self {
        Self {
            default_type: ForceType::MillNewton,
            v: mill_newton,
        }
    }

    pub fn from_micro_newton(micro_newton: f64) -> Self {
        Self {
            default_type: ForceType::MicroNewton,
            v: micro_newton,
        }
    }

    pub fn from_nano_newton(nano_newton: f64) -> Self {
        Self {
            default_type: ForceType::NanoNewton,
            v: nano_newton,
        }
    }

    pub fn from_kilo_newton(kilo_newton: f64) -> Self {
        Self {
            default_type: ForceType::KiloNewton,
            v: kilo_newton,
        }
    }

    pub fn from_mega_newton(mega_newton: f64) -> Self {
        Self {
            default_type: ForceType::MegaNewton,
            v: mega_newton,
        }
    }

    pub fn as_newton(&self) -> f64 {
        match self.default_type {
            ForceType::Newton => self.v,
            ForceType::MillNewton => self.v * 1e-3,
            ForceType::MicroNewton => self.v * 1e-6,
            ForceType::NanoNewton => self.v * 1e-9,
            ForceType::KiloNewton => self.v * 1e3,
            ForceType::MegaNewton => self.v * 1e6,
        }
    }

    pub fn as_mill_newton(&self) -> f64 {
        match self.default_type {
            ForceType::Newton => self.v * 1e3,
            ForceType::MillNewton => self.v,
            ForceType::MicroNewton => self.v * 1e-3,
            ForceType::NanoNewton => self.v * 1e-6,
            ForceType::KiloNewton => self.v * 1e6,
            ForceType::MegaNewton => self.v * 1e9,
        }
    }

    pub fn as_micro_newton(&self) -> f64 {
        match self.default_type {
            ForceType::Newton => self.v * 1e6,
            ForceType::MillNewton => self.v * 1e3,
            ForceType::MicroNewton => self.v,
            ForceType::NanoNewton => self.v * 1e-3,
            ForceType::KiloNewton => self.v * 1e9,
            ForceType::MegaNewton => self.v * 1e12,
        }
    }

    pub fn as_nano_newton(&self) -> f64 {
        match self.default_type {
            ForceType::Newton => self.v * 1e9,
            ForceType::MillNewton => self.v * 1e6,
            ForceType::MicroNewton => self.v * 1e3,
            ForceType::NanoNewton => self.v,
            ForceType::KiloNewton => self.v * 1e12,
            ForceType::MegaNewton => self.v * 1e15,
        }
    }

    pub fn as_kilo_newton(&self) -> f64 {
        match self.default_type {
            ForceType::Newton => self.v * 1e-3,
            ForceType::MillNewton => self.v * 1e-6,
            ForceType::MicroNewton => self.v * 1e-9,
            ForceType::NanoNewton => self.v * 1e-12,
            ForceType::KiloNewton => self.v,
            ForceType::MegaNewton => self.v * 1e3,
        }
    }

    pub fn as_mega_newton(&self) -> f64 {
        match self.default_type {
            ForceType::Newton => self.v * 1e-6,
            ForceType::MillNewton => self.v * 1e-9,
            ForceType::MicroNewton => self.v * 1e-12,
            ForceType::NanoNewton => self.v * 1e-15,
            ForceType::KiloNewton => self.v * 1e-3,
            ForceType::MegaNewton => self.v,
        }
    }
}

impl Add for Force {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_newton() + rhs.as_newton();
        Self::from_newton(v)
    }
}

impl Add<f64> for Force {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        Force {
            v,
            default_type: self.default_type,
        }
    }
}

impl Sub for Force {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_newton() - rhs.as_newton();
        Self::from_newton(v)
    }
}

impl Sub<f64> for Force {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        Force {
            v,
            default_type: self.default_type,
        }
    }
}

impl Mul<f64> for Force {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_newton() * rhs;
        Self::from_newton(v)
    }
}

impl Div<f64> for Force {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_newton() / rhs;
        Self::from_newton(v)
    }
}

impl Mul<Force> for f64 {
    type Output = Force;
    fn mul(self, rhs: Force) -> Self::Output {
        let v = self * rhs.as_newton();
        Force::from_newton(v)
    }
}

impl Div<Force> for f64 {
    type Output = Force;
    fn div(self, rhs: Force) -> Self::Output {
        let v = self / rhs.as_newton();
        Force::from_newton(v)
    }
}

impl Mul<Coef> for Force {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_newton() * rhs.get_value();
        Self::from_newton(v)
    }
}

impl Div<Coef> for Force {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_newton() / rhs.get_value();
        Self::from_newton(v)
    }
}

// 力与距离的乘积（得到能量）
impl Mul<Distance> for Force {
    type Output = Energy; // 能量，单位：焦耳
    fn mul(self, rhs: Distance) -> Self::Output {
        let energy_value = self.as_newton() * rhs.as_m();
        Energy::from_joule(energy_value)
    }
}

// 距离与力的乘积（得到能量，满足交换律）
impl Mul<Force> for Distance {
    type Output = Energy; // 能量，单位：焦耳
    fn mul(self, rhs: Force) -> Self::Output {
        let energy_value = self.as_m() * rhs.as_newton();
        Energy::from_joule(energy_value)
    }
}

// 质量与加速度的乘积（得到力）
impl Mul<Acceleration> for Mass {
    type Output = Force; // 力，单位：牛顿
    fn mul(self, rhs: Acceleration) -> Self::Output {
        let force_value = self.as_kg() * rhs.as_m_per_s2();
        Force::from_newton(force_value)
    }
}

// 加速度与质量的乘积（得到力，满足交换律）
impl Mul<Mass> for Acceleration {
    type Output = Force; // 力，单位：牛顿
    fn mul(self, rhs: Mass) -> Self::Output {
        let force_value = self.as_m_per_s2() * rhs.as_kg();
        Force::from_newton(force_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::basic::{Distance, Energy, Mass, Acceleration};

    #[test]
    fn test_force() {
        let f1 = Force::from_newton(10.0);
        let f2 = Force::from_mill_newton(10000.0);
        let f3 = Force::from_micro_newton(10000000.0);
        let f4 = Force::from_nano_newton(10000000000.0);
        let f5 = Force::from_kilo_newton(0.01);
        let f6 = Force::from_mega_newton(0.00001);

        assert_relative_eq!(f1.as_newton(), 10.0);
        assert_relative_eq!(f2.as_newton(), 10.0);
        assert_relative_eq!(f3.as_newton(), 10.0);
        assert_relative_eq!(f4.as_newton(), 10.0);
        assert_relative_eq!(f5.as_newton(), 10.0);
        assert_relative_eq!(f6.as_newton(), 10.0);
    }

    #[test]
    fn test_force_as() {
        let f = Force::from_newton(1.0);
        
        assert_relative_eq!(f.as_mill_newton(), 1000.0);
        assert_relative_eq!(f.as_micro_newton(), 1000000.0);
        assert_relative_eq!(f.as_nano_newton(), 1000000000.0);
        assert_relative_eq!(f.as_kilo_newton(), 0.001);
        assert_relative_eq!(f.as_mega_newton(), 0.000001);
    }

    #[test]
    fn test_force_add() {
        let f1 = Force::from_newton(5.0);
        let f2 = Force::from_mill_newton(3000.0); // 3 N
        let result = f1 + f2;
        
        assert_relative_eq!(result.as_newton(), 8.0);
    }

    #[test]
    fn test_force_sub() {
        let f1 = Force::from_newton(10.0);
        let f2 = Force::from_mill_newton(2000.0); // 2 N
        let result = f1 - f2;
        
        assert_relative_eq!(result.as_newton(), 8.0);
    }

    #[test]
    fn test_force_mul() {
        let f = Force::from_newton(5.0);
        let result = f * 3.0;
        
        assert_relative_eq!(result.as_newton(), 15.0);
    }

    #[test]
    fn test_force_div() {
        let f = Force::from_newton(15.0);
        let result = f / 3.0;
        
        assert_relative_eq!(result.as_newton(), 5.0);
    }

    #[test]
    fn test_force_mul_distance() {
        let force = Force::from_newton(10.0); // 10 N
        let distance = Distance::from_m(5.0);  // 5 m
        let energy: Energy = force * distance; // 50 J
        
        assert_relative_eq!(energy.as_joule(), 50.0);
    }

    #[test]
    fn test_distance_mul_force() {
        let distance = Distance::from_m(5.0);  // 5 m
        let force = Force::from_newton(10.0); // 10 N
        let energy: Energy = distance * force; // 50 J
        
        assert_relative_eq!(energy.as_joule(), 50.0);
    }

    #[test]
    fn test_mass_mul_acceleration() {
        let mass = Mass::from_kg(2.0);                    // 2 kg
        let acceleration = Acceleration::from_m_per_s2(5.0); // 5 m/s²
        let force: Force = mass * acceleration;           // 10 N
        
        assert_relative_eq!(force.as_newton(), 10.0);
    }

    #[test]
    fn test_acceleration_mul_mass() {
        let acceleration = Acceleration::from_m_per_s2(5.0); // 5 m/s²
        let mass = Mass::from_kg(2.0);                    // 2 kg
        let force: Force = acceleration * mass;           // 10 N
        
        assert_relative_eq!(force.as_newton(), 10.0);
    }

    #[test]
    fn test_default_unit_value() {
        let f = Force::from_newton(5.0);
        assert_relative_eq!(f.default_unit_value(), 5.0);
    }

    #[test]
    fn test_is_zero() {
        let f1 = Force::from_newton(0.0);
        let f2 = Force::from_newton(1.0);
        
        assert!(f1.is_zero());
        assert!(!f2.is_zero());
    }

    #[test]
    fn test_set_value() {
        let mut f = Force::from_newton(5.0);
        f.set_value(10.0);
        assert_relative_eq!(f.as_newton(), 10.0);
    }

    #[test]
    fn test_force_operations_with_different_types() {
        // 测试不同力类型之间的运算
        let f1 = Force::from_newton(1000.0);
        let f2 = Force::from_kilo_newton(1.0);
        let f3 = Force::from_mega_newton(0.001);
        
        // 加法
        let sum = f1 + f2 + f3;
        assert_relative_eq!(sum.as_newton(), 3000.0);
        
        // 减法
        let diff = f1 - f2;
        assert_relative_eq!(diff.as_newton(), 0.0);
        
        // 乘法
        let product = f1 * 2.0;
        assert_relative_eq!(product.as_newton(), 2000.0);
        
        // 除法
        let quotient = f1 / 2.0;
        assert_relative_eq!(quotient.as_newton(), 500.0);
    }

    #[test]
    fn test_force_all_unit_conversions() {
        let f = Force::from_newton(1.0);
        
        // 测试所有单位的转换
        assert_relative_eq!(f.as_newton(), 1.0);
        assert_relative_eq!(f.as_mill_newton(), 1000.0);
        assert_relative_eq!(f.as_micro_newton(), 1000000.0);
        assert_relative_eq!(f.as_nano_newton(), 1000000000.0);
        assert_relative_eq!(f.as_kilo_newton(), 0.001);
        assert_relative_eq!(f.as_mega_newton(), 0.000001);
        
        // 从不同单位创建并测试转换
        let f_mill = Force::from_mill_newton(1000.0);
        assert_relative_eq!(f_mill.as_newton(), 1.0);
        
        let f_micro = Force::from_micro_newton(1000000.0);
        assert_relative_eq!(f_micro.as_newton(), 1.0);
        
        let f_nano = Force::from_nano_newton(1000000000.0);
        assert_relative_eq!(f_nano.as_newton(), 1.0);
        
        let f_kilo = Force::from_kilo_newton(0.001);
        assert_relative_eq!(f_kilo.as_newton(), 1.0);
        
        let f_mega = Force::from_mega_newton(0.000001);
        assert_relative_eq!(f_mega.as_newton(), 1.0);
    }

    #[test]
    fn test_force_edge_cases() {
        // 测试边界情况
        let zero_force = Force::from_newton(0.0);
        assert!(zero_force.is_zero());
        
        let negative_force = Force::from_newton(-100.0);
        assert!(!negative_force.is_zero());
        assert_relative_eq!(negative_force.as_newton(), -100.0);
        
        // 测试大数值
        let large_force = Force::from_mega_newton(1000.0);
        assert_relative_eq!(large_force.as_newton(), 1e9);
        
        // 测试小数值
        let small_force = Force::from_nano_newton(1.0);
        assert_relative_eq!(small_force.as_newton(), 1e-9);
    }

    #[test]
    fn test_force_physical_quantity_trait() {
        let mut f = Force::from_newton(10.0);
        
        // 测试 default_unit_value
        assert_relative_eq!(f.default_unit_value(), 10.0);
        
        // 测试 set_value
        f.set_value(20.0);
        assert_relative_eq!(f.as_newton(), 20.0);
        
        // 测试 is_zero
        assert!(!f.is_zero());
        f.set_value(0.0);
        assert!(f.is_zero());
        
        // 测试 as_any
        let any_ref = f.as_any();
        assert!(any_ref.is::<Force>());
    }

    #[test]
    fn test_force_coef_operations() {
        let f = Force::from_newton(10.0);
        let coef = Coef::new(2.0);
        
        // 测试与系数的乘法
        let result = f * coef;
        assert_relative_eq!(result.as_newton(), 20.0);
        
        // 测试与系数的除法
        let result = f / coef;
        assert_relative_eq!(result.as_newton(), 5.0);
    }

    #[test]
    fn test_force_f64_operations() {
        let f = Force::from_newton(10.0);
        
        // 测试 f64 与力的乘法
        let result = 3.0 * f;
        assert_relative_eq!(result.as_newton(), 30.0);
        
        // 测试 f64 与力的除法
        let result = 30.0 / f;
        assert_relative_eq!(result.as_newton(), 3.0);
    }

    #[test]
    fn test_force_arithmetic_with_f64() {
        let f = Force::from_newton(10.0);
        
        // 测试与 f64 的加法
        let result = f + 5.0;
        assert_relative_eq!(result.as_newton(), 15.0);
        
        // 测试与 f64 的减法
        let result = f - 3.0;
        assert_relative_eq!(result.as_newton(), 7.0);
    }
}
