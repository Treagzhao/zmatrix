use std::any::Any;
use std::ops::{Add, Div, Mul, Sub};
use crate::physics::basic::{Coef, MagneticMoment, MagneticMomentType, PhysicalQuantity, MagneticInduction, Energy};
use approx::assert_relative_eq;

impl Default for MagneticMoment {
    fn default() -> Self {
        Self::from_am2(0.0)
    }
}

impl PhysicalQuantity for MagneticMoment {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_am2()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl MagneticMoment {
    pub fn from_am2(am2: f64) -> Self {
        Self {
            default_type: MagneticMomentType::AM2,
            v: am2,
        }
    }

    pub fn from_mill_am2(mill_am2: f64) -> Self {
        Self {
            default_type: MagneticMomentType::MillAM2,
            v: mill_am2,
        }
    }

    pub fn from_micro_am2(micro_am2: f64) -> Self {
        Self {
            default_type: MagneticMomentType::MicroAM2,
            v: micro_am2,
        }
    }

    pub fn from_nano_am2(nano_am2: f64) -> Self {
        Self {
            default_type: MagneticMomentType::NanoAM2,
            v: nano_am2,
        }
    }

    pub fn from_j_per_tesla(j_per_tesla: f64) -> Self {
        Self {
            default_type: MagneticMomentType::JPerTesla,
            v: j_per_tesla,
        }
    }

    pub fn from_mill_j_per_tesla(mill_j_per_tesla: f64) -> Self {
        Self {
            default_type: MagneticMomentType::MillJPerTesla,
            v: mill_j_per_tesla,
        }
    }

    pub fn from_micro_j_per_tesla(micro_j_per_tesla: f64) -> Self {
        Self {
            default_type: MagneticMomentType::MicroJPerTesla,
            v: micro_j_per_tesla,
        }
    }

    pub fn from_nano_j_per_tesla(nano_j_per_tesla: f64) -> Self {
        Self {
            default_type: MagneticMomentType::NanoJPerTesla,
            v: nano_j_per_tesla,
        }
    }

    pub fn as_am2(&self) -> f64 {
        match self.default_type {
            MagneticMomentType::AM2 => self.v,
            MagneticMomentType::MillAM2 => self.v * 1e-3,
            MagneticMomentType::MicroAM2 => self.v * 1e-6,
            MagneticMomentType::NanoAM2 => self.v * 1e-9,
            MagneticMomentType::JPerTesla => self.v, // 1 J/T = 1 A·m²
            MagneticMomentType::MillJPerTesla => self.v * 1e-3,
            MagneticMomentType::MicroJPerTesla => self.v * 1e-6,
            MagneticMomentType::NanoJPerTesla => self.v * 1e-9,
        }
    }

    pub fn as_mill_am2(&self) -> f64 {
        match self.default_type {
            MagneticMomentType::AM2 => self.v * 1e3,
            MagneticMomentType::MillAM2 => self.v,
            MagneticMomentType::MicroAM2 => self.v * 1e-3,
            MagneticMomentType::NanoAM2 => self.v * 1e-6,
            MagneticMomentType::JPerTesla => self.v * 1e3,
            MagneticMomentType::MillJPerTesla => self.v,
            MagneticMomentType::MicroJPerTesla => self.v * 1e-3,
            MagneticMomentType::NanoJPerTesla => self.v * 1e-6,
        }
    }

    pub fn as_micro_am2(&self) -> f64 {
        match self.default_type {
            MagneticMomentType::AM2 => self.v * 1e6,
            MagneticMomentType::MillAM2 => self.v * 1e3,
            MagneticMomentType::MicroAM2 => self.v,
            MagneticMomentType::NanoAM2 => self.v * 1e-3,
            MagneticMomentType::JPerTesla => self.v * 1e6,
            MagneticMomentType::MillJPerTesla => self.v * 1e3,
            MagneticMomentType::MicroJPerTesla => self.v,
            MagneticMomentType::NanoJPerTesla => self.v * 1e-3,
        }
    }

    pub fn as_nano_am2(&self) -> f64 {
        match self.default_type {
            MagneticMomentType::AM2 => self.v * 1e9,
            MagneticMomentType::MillAM2 => self.v * 1e6,
            MagneticMomentType::MicroAM2 => self.v * 1e3,
            MagneticMomentType::NanoAM2 => self.v,
            MagneticMomentType::JPerTesla => self.v * 1e9,
            MagneticMomentType::MillJPerTesla => self.v * 1e6,
            MagneticMomentType::MicroJPerTesla => self.v * 1e3,
            MagneticMomentType::NanoJPerTesla => self.v,
        }
    }

    pub fn as_j_per_tesla(&self) -> f64 {
        match self.default_type {
            MagneticMomentType::AM2 => self.v, // 1 A·m² = 1 J/T
            MagneticMomentType::MillAM2 => self.v * 1e-3,
            MagneticMomentType::MicroAM2 => self.v * 1e-6,
            MagneticMomentType::NanoAM2 => self.v * 1e-9,
            MagneticMomentType::JPerTesla => self.v,
            MagneticMomentType::MillJPerTesla => self.v * 1e-3,
            MagneticMomentType::MicroJPerTesla => self.v * 1e-6,
            MagneticMomentType::NanoJPerTesla => self.v * 1e-9,
        }
    }

    pub fn as_mill_j_per_tesla(&self) -> f64 {
        match self.default_type {
            MagneticMomentType::AM2 => self.v * 1e3,
            MagneticMomentType::MillAM2 => self.v,
            MagneticMomentType::MicroAM2 => self.v * 1e-3,
            MagneticMomentType::NanoAM2 => self.v * 1e-6,
            MagneticMomentType::JPerTesla => self.v * 1e3,
            MagneticMomentType::MillJPerTesla => self.v,
            MagneticMomentType::MicroJPerTesla => self.v * 1e-3,
            MagneticMomentType::NanoJPerTesla => self.v * 1e-6,
        }
    }

    pub fn as_micro_j_per_tesla(&self) -> f64 {
        match self.default_type {
            MagneticMomentType::AM2 => self.v * 1e6,
            MagneticMomentType::MillAM2 => self.v * 1e3,
            MagneticMomentType::MicroAM2 => self.v,
            MagneticMomentType::NanoAM2 => self.v * 1e-3,
            MagneticMomentType::JPerTesla => self.v * 1e6,
            MagneticMomentType::MillJPerTesla => self.v * 1e3,
            MagneticMomentType::MicroJPerTesla => self.v,
            MagneticMomentType::NanoJPerTesla => self.v * 1e-3,
        }
    }

    pub fn as_nano_j_per_tesla(&self) -> f64 {
        match self.default_type {
            MagneticMomentType::AM2 => self.v * 1e9,
            MagneticMomentType::MillAM2 => self.v * 1e6,
            MagneticMomentType::MicroAM2 => self.v * 1e3,
            MagneticMomentType::NanoAM2 => self.v,
            MagneticMomentType::JPerTesla => self.v * 1e9,
            MagneticMomentType::MillJPerTesla => self.v * 1e6,
            MagneticMomentType::MicroJPerTesla => self.v * 1e3,
            MagneticMomentType::NanoJPerTesla => self.v,
        }
    }
}

impl Add for MagneticMoment {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_am2() + rhs.as_am2();
        Self::from_am2(v)
    }
}

impl Add<f64> for MagneticMoment {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        MagneticMoment {
            v,
            default_type: self.default_type,
        }
    }
}

impl Sub for MagneticMoment {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_am2() - rhs.as_am2();
        Self::from_am2(v)
    }
}

impl Sub<f64> for MagneticMoment {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        MagneticMoment {
            v,
            default_type: self.default_type,
        }
    }
}

impl Mul<f64> for MagneticMoment {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_am2() * rhs;
        Self::from_am2(v)
    }
}

impl Div<f64> for MagneticMoment {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_am2() / rhs;
        Self::from_am2(v)
    }
}

impl Mul<MagneticMoment> for f64 {
    type Output = MagneticMoment;
    fn mul(self, rhs: MagneticMoment) -> Self::Output {
        rhs * self
    }
}

impl Div<MagneticMoment> for f64 {
    type Output = MagneticMoment;
    fn div(self, rhs: MagneticMoment) -> Self::Output {
        let v = self / rhs.as_am2();
        MagneticMoment {
            default_type: rhs.default_type,
            v: v,
        }
    }
}

impl Mul<Coef> for MagneticMoment {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_am2() * rhs.get_value();
        Self::from_am2(v)
    }
}

impl Div<Coef> for MagneticMoment {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_am2() / rhs.get_value();
        Self::from_am2(v)
    }
}

// 磁矩与磁感应强度的乘积（得到能量）
impl Mul<MagneticInduction> for MagneticMoment {
    type Output = Energy; // 能量，单位：焦耳
    fn mul(self, rhs: MagneticInduction) -> Self::Output {
        let energy_value = self.as_j_per_tesla() * rhs.as_tesla();
        Energy::from_joule(energy_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::basic::{MagneticInduction, Area};

    #[test]
    fn test_magnetic_moment() {
        let mm1 = MagneticMoment::from_am2(1000.0);
        assert_eq!(mm1.v, 1000.0);
        assert_eq!(mm1.default_type, MagneticMomentType::AM2);

        let mm2 = MagneticMoment::from_j_per_tesla(1000.0);
        assert_eq!(mm2.v, 1000.0);
        assert_eq!(mm2.default_type, MagneticMomentType::JPerTesla);

        let mm3 = MagneticMoment::from_mill_am2(1000.0);
        assert_eq!(mm3.v, 1000.0);
        assert_eq!(mm3.default_type, MagneticMomentType::MillAM2);

        let mm4 = MagneticMoment::from_micro_am2(1000.0);
        assert_eq!(mm4.v, 1000.0);
        assert_eq!(mm4.default_type, MagneticMomentType::MicroAM2);

        let mm5 = MagneticMoment::from_nano_am2(1000.0);
        assert_eq!(mm5.v, 1000.0);
        assert_eq!(mm5.default_type, MagneticMomentType::NanoAM2);

        let mm6 = MagneticMoment::default();
        assert_eq!(mm6.v, 0.0);
        assert_eq!(mm6.default_type, MagneticMomentType::AM2);
    }

    #[test]
    fn test_magnetic_moment_as() {
        let mm1 = MagneticMoment::from_am2(1.0);
        assert_relative_eq!(mm1.as_am2(), 1.0);
        assert_relative_eq!(mm1.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm1.as_mill_am2(), 1e3);
        assert_relative_eq!(mm1.as_micro_am2(), 1e6);
        assert_relative_eq!(mm1.as_nano_am2(), 1e9);

        let mm2 = MagneticMoment::from_j_per_tesla(1.0);
        assert_relative_eq!(mm2.as_am2(), 1.0);
        assert_relative_eq!(mm2.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm2.as_mill_j_per_tesla(), 1e3);
        assert_relative_eq!(mm2.as_micro_j_per_tesla(), 1e6);
        assert_relative_eq!(mm2.as_nano_j_per_tesla(), 1e9);

        let mm3 = MagneticMoment::from_mill_am2(1e3);
        assert_relative_eq!(mm3.as_am2(), 1.0);
        assert_relative_eq!(mm3.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm3.as_mill_am2(), 1e3);

        let mm4 = MagneticMoment::from_micro_am2(1e6);
        assert_relative_eq!(mm4.as_am2(), 1.0);
        assert_relative_eq!(mm4.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm4.as_micro_am2(), 1e6);

        let mm5 = MagneticMoment::from_nano_am2(1e9);
        assert_relative_eq!(mm5.as_am2(), 1.0);
        assert_relative_eq!(mm5.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm5.as_nano_am2(), 1e9);
    }

    #[test]
    fn test_magnetic_moment_add() {
        let mm1 = MagneticMoment::from_am2(1000.0);
        let mm2 = MagneticMoment::from_am2(1000.0);
        let mm3 = mm1 + mm2;
        assert_relative_eq!(mm3.as_am2(), 2000.0);

        let mm1 = MagneticMoment::from_j_per_tesla(1000.0);
        let mm2 = MagneticMoment::from_am2(1000.0);
        let mm3 = mm1 + mm2;
        assert_relative_eq!(mm3.as_am2(), 2000.0);

        let mm1 = MagneticMoment::from_mill_am2(1e6);
        let mm2 = MagneticMoment::from_am2(1000.0);
        let mm3 = mm1 + mm2;
        assert_relative_eq!(mm3.as_am2(), 2000.0);

        let mm1 = MagneticMoment::from_am2(1000.0);
        let mm2 = mm1 + 100.0;
        assert_relative_eq!(mm2.as_am2(), 1100.0);

        let mm1 = MagneticMoment::from_j_per_tesla(1000.0);
        let mm2 = mm1 + 100.0;
        assert_relative_eq!(mm2.as_j_per_tesla(), 1100.0);
    }

    #[test]
    fn test_magnetic_moment_sub() {
        let mm1 = MagneticMoment::from_am2(2000.0);
        let mm2 = MagneticMoment::from_am2(1000.0);
        let mm3 = mm1 - mm2;
        assert_relative_eq!(mm3.as_am2(), 1000.0);

        let mm1 = MagneticMoment::from_j_per_tesla(2000.0);
        let mm2 = MagneticMoment::from_am2(1000.0);
        let mm3 = mm1 - mm2;
        assert_relative_eq!(mm3.as_am2(), 1000.0);

        let mm1 = MagneticMoment::from_am2(2000.0);
        let mm2 = mm1 - 100.0;
        assert_relative_eq!(mm2.as_am2(), 1900.0);
    }

    #[test]
    fn test_magnetic_moment_mul() {
        let mm1 = MagneticMoment::from_am2(2000.0);
        let mm2 = mm1 * 2.0;
        assert_relative_eq!(mm2.as_am2(), 4000.0);

        let mm1 = MagneticMoment::from_j_per_tesla(2000.0);
        let mm2 = mm1 * 2.0;
        assert_relative_eq!(mm2.as_j_per_tesla(), 4000.0);

        let mm1 = MagneticMoment::from_am2(1000.0);
        let mm2 = mm1 * Coef::new(2.0);
        assert_relative_eq!(mm2.as_am2(), 2000.0);
    }

    #[test]
    fn test_magnetic_moment_div() {
        let mm1 = MagneticMoment::from_am2(2000.0);
        let mm2 = mm1 / 2.0;
        assert_relative_eq!(mm2.as_am2(), 1000.0);

        let mm1 = MagneticMoment::from_j_per_tesla(2000.0);
        let mm2 = mm1 / 2.0;
        assert_relative_eq!(mm2.as_j_per_tesla(), 1000.0);

        let mm1 = MagneticMoment::from_am2(2000.0);
        let mm2 = mm1 / Coef::new(2.0);
        assert_relative_eq!(mm2.as_am2(), 1000.0);
    }

    #[test]
    fn test_is_zero() {
        let mm1 = MagneticMoment::from_am2(0.0);
        assert!(mm1.is_zero());
        let mm1 = MagneticMoment::from_am2(1.0);
        assert!(!mm1.is_zero());
    }

    #[test]
    fn test_default_unit_value() {
        let mm1 = MagneticMoment::from_am2(1.0);
        assert_relative_eq!(mm1.default_unit_value(), 1.0);
        let mm1 = MagneticMoment::from_j_per_tesla(1.0);
        assert_relative_eq!(mm1.default_unit_value(), 1.0);
    }

    #[test]
    fn test_set_value() {
        let mut mm1 = MagneticMoment::from_am2(1.0);
        mm1.set_value(1000.0);
        assert_relative_eq!(mm1.as_am2(), 1000.0);
    }

    #[test]
    fn test_magnetic_moment_mul_magnetic_induction() {
        let mm = MagneticMoment::from_am2(1.0);
        let b = MagneticInduction::from_tesla(2.0);
        let energy = mm * b;
        assert_relative_eq!(energy.as_joule(), 2.0); // 1 A·m² × 2 T = 2 J

        let mm = MagneticMoment::from_j_per_tesla(1.0);
        let b = MagneticInduction::from_gauss(10000.0);
        let energy = mm * b;
        assert_relative_eq!(energy.as_joule(), 1.0); // 1 J/T × 1 T = 1 J
    }

    #[test]
    fn test_f64_mul_magnetic_moment() {
        let mm = MagneticMoment::from_am2(2.0);
        let result = 3.0 * mm;
        assert_relative_eq!(result.as_am2(), 6.0);

        let mm = MagneticMoment::from_j_per_tesla(2.0);
        let result = 3.0 * mm;
        assert_relative_eq!(result.as_j_per_tesla(), 6.0);
    }

    #[test]
    fn test_f64_div_magnetic_moment() {
        let mm = MagneticMoment::from_am2(2.0);
        let result = 6.0 / mm;
        assert_relative_eq!(result.as_am2(), 3.0);

        let mm = MagneticMoment::from_j_per_tesla(2.0);
        let result = 6.0 / mm;
        assert_relative_eq!(result.as_j_per_tesla(), 3.0);
    }

    #[test]
    fn test_magnetic_moment_operations_with_different_types() {
        // 测试不同磁矩类型之间的运算
        let mm1 = MagneticMoment::from_am2(1000.0);
        let mm2 = MagneticMoment::from_j_per_tesla(1000.0);
        let mm3 = MagneticMoment::from_mill_am2(1e6);
        
        // 加法
        let sum = mm1 + mm2 + mm3;
        assert_relative_eq!(sum.as_am2(), 3000.0);
        
        // 减法
        let diff = mm1 - mm2;
        assert_relative_eq!(diff.as_am2(), 0.0);
        
        // 乘法
        let product = mm1 * 2.0;
        assert_relative_eq!(product.as_am2(), 2000.0);
        
        // 除法
        let quotient = mm1 / 2.0;
        assert_relative_eq!(quotient.as_am2(), 500.0);
    }

    #[test]
    fn test_magnetic_moment_all_unit_conversions() {
        let mm = MagneticMoment::from_am2(1.0);
        
        // 测试所有 A·m² 单位的转换
        assert_relative_eq!(mm.as_am2(), 1.0);
        assert_relative_eq!(mm.as_mill_am2(), 1000.0);
        assert_relative_eq!(mm.as_micro_am2(), 1000000.0);
        assert_relative_eq!(mm.as_nano_am2(), 1000000000.0);
        
        // 测试所有 J/T 单位的转换
        assert_relative_eq!(mm.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(mm.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(mm.as_nano_j_per_tesla(), 1000000000.0);
        
        // 从不同单位创建并测试转换
        let mm_mill = MagneticMoment::from_mill_am2(1000.0);
        assert_relative_eq!(mm_mill.as_am2(), 1.0);
        
        let mm_micro = MagneticMoment::from_micro_am2(1000000.0);
        assert_relative_eq!(mm_micro.as_am2(), 1.0);
        
        let mm_nano = MagneticMoment::from_nano_am2(1000000000.0);
        assert_relative_eq!(mm_nano.as_am2(), 1.0);
        
        let mm_j_per_t = MagneticMoment::from_j_per_tesla(1.0);
        assert_relative_eq!(mm_j_per_t.as_am2(), 1.0);
        
        let mm_mill_j_per_t = MagneticMoment::from_mill_j_per_tesla(1000.0);
        assert_relative_eq!(mm_mill_j_per_t.as_j_per_tesla(), 1.0);
        
        let mm_micro_j_per_t = MagneticMoment::from_micro_j_per_tesla(1000000.0);
        assert_relative_eq!(mm_micro_j_per_t.as_j_per_tesla(), 1.0);
        
        let mm_nano_j_per_t = MagneticMoment::from_nano_j_per_tesla(1000000000.0);
        assert_relative_eq!(mm_nano_j_per_t.as_j_per_tesla(), 1.0);
    }

    #[test]
    fn test_magnetic_moment_edge_cases() {
        // 测试边界情况
        let zero_mm = MagneticMoment::from_am2(0.0);
        assert!(zero_mm.is_zero());
        
        let negative_mm = MagneticMoment::from_am2(-100.0);
        assert!(!negative_mm.is_zero());
        assert_relative_eq!(negative_mm.as_am2(), -100.0);
        
        // 测试大数值
        let large_mm = MagneticMoment::from_micro_am2(1000.0);
        assert_relative_eq!(large_mm.as_am2(), 0.001);
        
        // 测试小数值
        let small_mm = MagneticMoment::from_nano_am2(1.0);
        assert_relative_eq!(small_mm.as_am2(), 1e-9);
    }

    #[test]
    fn test_magnetic_moment_physical_quantity_trait() {
        let mut mm = MagneticMoment::from_am2(10.0);
        
        // 测试 default_unit_value
        assert_relative_eq!(mm.default_unit_value(), 10.0);
        
        // 测试 set_value
        mm.set_value(20.0);
        assert_relative_eq!(mm.as_am2(), 20.0);
        
        // 测试 is_zero
        assert!(!mm.is_zero());
        mm.set_value(0.0);
        assert!(mm.is_zero());
        
        // 测试 as_any
        let any_ref = mm.as_any();
        assert!(any_ref.is::<MagneticMoment>());
    }

    #[test]
    fn test_magnetic_moment_coef_operations() {
        let mm = MagneticMoment::from_am2(10.0);
        let coef = Coef::new(2.0);
        
        // 测试与系数的乘法
        let result = mm * coef;
        assert_relative_eq!(result.as_am2(), 20.0);
        
        // 测试与系数的除法
        let result = mm / coef;
        assert_relative_eq!(result.as_am2(), 5.0);
    }

    #[test]
    fn test_magnetic_moment_f64_operations() {
        let mm = MagneticMoment::from_am2(10.0);
        
        // 测试 f64 与磁矩的乘法
        let result = 3.0 * mm;
        assert_relative_eq!(result.as_am2(), 30.0);
        
        // 测试 f64 与磁矩的除法
        let result = 30.0 / mm;
        assert_relative_eq!(result.as_am2(), 3.0);
    }

    #[test]
    fn test_magnetic_moment_arithmetic_with_f64() {
        let mm = MagneticMoment::from_am2(10.0);
        
        // 测试与 f64 的加法
        let result = mm + 5.0;
        assert_relative_eq!(result.as_am2(), 15.0);
        
        // 测试与 f64 的减法
        let result = mm - 3.0;
        assert_relative_eq!(result.as_am2(), 7.0);
        
        // 测试不同单位的加法
        let mm_j_per_t = MagneticMoment::from_j_per_tesla(10.0);
        let result = mm_j_per_t + 5.0;
        assert_relative_eq!(result.as_j_per_tesla(), 15.0);
        
        // 测试不同单位的减法
        let result = mm_j_per_t - 3.0;
        assert_relative_eq!(result.as_j_per_tesla(), 7.0);
    }

    #[test]
    fn test_magnetic_moment_multiplication_with_magnetic_induction() {
        // 测试磁矩与磁感应强度的乘法（得到能量）
        let mm = MagneticMoment::from_am2(2.0);
        let b = MagneticInduction::from_tesla(3.0);
        let energy = mm * b;
        assert_relative_eq!(energy.as_joule(), 6.0); // 2 A·m² × 3 T = 6 J
        
        // 测试不同单位
        let mm_j_per_t = MagneticMoment::from_j_per_tesla(2.0);
        let b_gauss = MagneticInduction::from_gauss(30000.0); // 3 T
        let energy = mm_j_per_t * b_gauss;
        assert_relative_eq!(energy.as_joule(), 6.0); // 2 J/T × 3 T = 6 J
    }

    #[test]
    fn test_magnetic_moment_comprehensive_as_methods() {
        // 测试所有单位到所有单位的转换
        // 从 AM2 开始
        let mm_am2 = MagneticMoment::from_am2(1.0);
        assert_relative_eq!(mm_am2.as_am2(), 1.0);
        assert_relative_eq!(mm_am2.as_mill_am2(), 1000.0);
        assert_relative_eq!(mm_am2.as_micro_am2(), 1000000.0);
        assert_relative_eq!(mm_am2.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(mm_am2.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm_am2.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(mm_am2.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(mm_am2.as_nano_j_per_tesla(), 1000000000.0);

        // 从 MillAM2 开始
        let mm_mill_am2 = MagneticMoment::from_mill_am2(1000.0);
        assert_relative_eq!(mm_mill_am2.as_am2(), 1.0);
        assert_relative_eq!(mm_mill_am2.as_mill_am2(), 1000.0);
        assert_relative_eq!(mm_mill_am2.as_micro_am2(), 1000000.0);
        assert_relative_eq!(mm_mill_am2.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(mm_mill_am2.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm_mill_am2.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(mm_mill_am2.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(mm_mill_am2.as_nano_j_per_tesla(), 1000000000.0);

        // 从 MicroAM2 开始
        let mm_micro_am2 = MagneticMoment::from_micro_am2(1000000.0);
        assert_relative_eq!(mm_micro_am2.as_am2(), 1.0);
        assert_relative_eq!(mm_micro_am2.as_mill_am2(), 1000.0);
        assert_relative_eq!(mm_micro_am2.as_micro_am2(), 1000000.0);
        assert_relative_eq!(mm_micro_am2.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(mm_micro_am2.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm_micro_am2.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(mm_micro_am2.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(mm_micro_am2.as_nano_j_per_tesla(), 1000000000.0);

        // 从 NanoAM2 开始
        let mm_nano_am2 = MagneticMoment::from_nano_am2(1000000000.0);
        assert_relative_eq!(mm_nano_am2.as_am2(), 1.0);
        assert_relative_eq!(mm_nano_am2.as_mill_am2(), 1000.0);
        assert_relative_eq!(mm_nano_am2.as_micro_am2(), 1000000.0);
        assert_relative_eq!(mm_nano_am2.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(mm_nano_am2.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm_nano_am2.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(mm_nano_am2.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(mm_nano_am2.as_nano_j_per_tesla(), 1000000000.0);

        // 从 JPerTesla 开始
        let mm_j_per_t = MagneticMoment::from_j_per_tesla(1.0);
        assert_relative_eq!(mm_j_per_t.as_am2(), 1.0);
        assert_relative_eq!(mm_j_per_t.as_mill_am2(), 1000.0);
        assert_relative_eq!(mm_j_per_t.as_micro_am2(), 1000000.0);
        assert_relative_eq!(mm_j_per_t.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(mm_j_per_t.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm_j_per_t.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(mm_j_per_t.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(mm_j_per_t.as_nano_j_per_tesla(), 1000000000.0);

        // 从 MillJPerTesla 开始
        let mm_mill_j_per_t = MagneticMoment::from_mill_j_per_tesla(1000.0);
        assert_relative_eq!(mm_mill_j_per_t.as_am2(), 1.0);
        assert_relative_eq!(mm_mill_j_per_t.as_mill_am2(), 1000.0);
        assert_relative_eq!(mm_mill_j_per_t.as_micro_am2(), 1000000.0);
        assert_relative_eq!(mm_mill_j_per_t.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(mm_mill_j_per_t.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm_mill_j_per_t.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(mm_mill_j_per_t.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(mm_mill_j_per_t.as_nano_j_per_tesla(), 1000000000.0);

        // 从 MicroJPerTesla 开始
        let mm_micro_j_per_t = MagneticMoment::from_micro_j_per_tesla(1000000.0);
        assert_relative_eq!(mm_micro_j_per_t.as_am2(), 1.0);
        assert_relative_eq!(mm_micro_j_per_t.as_mill_am2(), 1000.0);
        assert_relative_eq!(mm_micro_j_per_t.as_micro_am2(), 1000000.0);
        assert_relative_eq!(mm_micro_j_per_t.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(mm_micro_j_per_t.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm_micro_j_per_t.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(mm_micro_j_per_t.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(mm_micro_j_per_t.as_nano_j_per_tesla(), 1000000000.0);

        // 从 NanoJPerTesla 开始
        let mm_nano_j_per_t = MagneticMoment::from_nano_j_per_tesla(1000000000.0);
        assert_relative_eq!(mm_nano_j_per_t.as_am2(), 1.0);
        assert_relative_eq!(mm_nano_j_per_t.as_mill_am2(), 1000.0);
        assert_relative_eq!(mm_nano_j_per_t.as_micro_am2(), 1000000.0);
        assert_relative_eq!(mm_nano_j_per_t.as_nano_am2(), 1000000000.0);
        assert_relative_eq!(mm_nano_j_per_t.as_j_per_tesla(), 1.0);
        assert_relative_eq!(mm_nano_j_per_t.as_mill_j_per_tesla(), 1000.0);
        assert_relative_eq!(mm_nano_j_per_t.as_micro_j_per_tesla(), 1000000.0);
        assert_relative_eq!(mm_nano_j_per_t.as_nano_j_per_tesla(), 1000000000.0);

        // 测试负值
        let mm_negative = MagneticMoment::from_am2(-2.0);
        assert_relative_eq!(mm_negative.as_am2(), -2.0);
        assert_relative_eq!(mm_negative.as_mill_am2(), -2000.0);
        assert_relative_eq!(mm_negative.as_micro_am2(), -2000000.0);
        assert_relative_eq!(mm_negative.as_nano_am2(), -2000000000.0);
        assert_relative_eq!(mm_negative.as_j_per_tesla(), -2.0);
        assert_relative_eq!(mm_negative.as_mill_j_per_tesla(), -2000.0);
        assert_relative_eq!(mm_negative.as_micro_j_per_tesla(), -2000000.0);
        assert_relative_eq!(mm_negative.as_nano_j_per_tesla(), -2000000000.0);

        // 测试零值
        let mm_zero = MagneticMoment::from_am2(0.0);
        assert_relative_eq!(mm_zero.as_am2(), 0.0);
        assert_relative_eq!(mm_zero.as_mill_am2(), 0.0);
        assert_relative_eq!(mm_zero.as_micro_am2(), 0.0);
        assert_relative_eq!(mm_zero.as_nano_am2(), 0.0);
        assert_relative_eq!(mm_zero.as_j_per_tesla(), 0.0);
        assert_relative_eq!(mm_zero.as_mill_j_per_tesla(), 0.0);
        assert_relative_eq!(mm_zero.as_micro_j_per_tesla(), 0.0);
        assert_relative_eq!(mm_zero.as_nano_j_per_tesla(), 0.0);
    }
}
