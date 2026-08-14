use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{Coef, ElectricCurrent, ElectricCurrentType, PhysicalQuantity, ElectricConductance, ElectricPotential};
use approx::assert_relative_eq;

impl Default for ElectricCurrent {
    fn default() -> Self {
        Self::from_a(0.0)
    }
}

impl PhysicalQuantity for ElectricCurrent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_a()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
    fn unit_display_name(&self) -> String {
        "ElectricCurrent (A)".to_string()
    }
}

impl ElectricCurrent {
    pub fn from_a(v: f64) -> Self {
        Self {
            default_type: ElectricCurrentType::A,
            v,
        }
    }

    pub fn from_milli_a(v: f64) -> Self {
        Self {
            default_type: ElectricCurrentType::MilliA,
            v,
        }
    }

    pub fn from_micro_a(v: f64) -> Self {
        Self {
            default_type: ElectricCurrentType::MicroA,
            v,
        }
    }

    pub fn from_kilo_a(v: f64) -> Self {
        Self {
            default_type: ElectricCurrentType::KiloA,
            v,
        }
    }

    pub fn as_a(&self) -> f64 {
        match self.default_type {
            ElectricCurrentType::A => self.v,
            ElectricCurrentType::MilliA => self.v * 1e-3,
            ElectricCurrentType::MicroA => self.v * 1e-6,
            ElectricCurrentType::KiloA => self.v * 1e3,
        }
    }

    pub fn as_milli_a(&self) -> f64 {
        match self.default_type {
            ElectricCurrentType::A => self.v * 1e3,
            ElectricCurrentType::MilliA => self.v,
            ElectricCurrentType::MicroA => self.v * 1e-3,
            ElectricCurrentType::KiloA => self.v * 1e6,
        }
    }

    pub fn as_micro_a(&self) -> f64 {
        match self.default_type {
            ElectricCurrentType::A => self.v * 1e6,
            ElectricCurrentType::MilliA => self.v * 1000.0000000000001,
            ElectricCurrentType::MicroA => self.v,
            ElectricCurrentType::KiloA => self.v * 1e9,
        }
    }

    pub fn as_kilo_a(&self) -> f64 {
        match self.default_type {
            ElectricCurrentType::A => self.v * 1e-3,
            ElectricCurrentType::MilliA => self.v * 1e-6,
            ElectricCurrentType::MicroA => self.v * 9.999999999999999e-10,
            ElectricCurrentType::KiloA => self.v,
        }
    }

}

impl Add for ElectricCurrent {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_a() + rhs.as_a();
        Self::from_a(v)
    }
}

impl<'a, 'b> Add<&'b ElectricCurrent> for &'a ElectricCurrent {
    type Output = ElectricCurrent;
    fn add(self, rhs: &'b ElectricCurrent) -> Self::Output { ElectricCurrent::from_a(self.as_a() + rhs.as_a()) }
}
impl<'a> Add<&'a ElectricCurrent> for ElectricCurrent {
    type Output = ElectricCurrent;
    fn add(self, rhs: &'a ElectricCurrent) -> Self::Output { ElectricCurrent::from_a(self.as_a() + rhs.as_a()) }
}
impl<'a> Add<ElectricCurrent> for &'a ElectricCurrent {
    type Output = ElectricCurrent;
    fn add(self, rhs: ElectricCurrent) -> Self::Output { ElectricCurrent::from_a(self.as_a() + rhs.as_a()) }
}

impl Add<f64> for ElectricCurrent {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        ElectricCurrent { v, default_type: self.default_type }
    }
}

impl Sub for ElectricCurrent {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_a() - rhs.as_a();
        Self::from_a(v)
    }
}

impl<'a, 'b> Sub<&'b ElectricCurrent> for &'a ElectricCurrent {
    type Output = ElectricCurrent;
    fn sub(self, rhs: &'b ElectricCurrent) -> Self::Output { ElectricCurrent::from_a(self.as_a() - rhs.as_a()) }
}
impl<'a> Sub<&'a ElectricCurrent> for ElectricCurrent {
    type Output = ElectricCurrent;
    fn sub(self, rhs: &'a ElectricCurrent) -> Self::Output { ElectricCurrent::from_a(self.as_a() - rhs.as_a()) }
}
impl<'a> Sub<ElectricCurrent> for &'a ElectricCurrent {
    type Output = ElectricCurrent;
    fn sub(self, rhs: ElectricCurrent) -> Self::Output { ElectricCurrent::from_a(self.as_a() - rhs.as_a()) }
}

impl Sub<f64> for ElectricCurrent {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        ElectricCurrent { v, default_type: self.default_type }
    }
}

impl Mul<f64> for ElectricCurrent {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_a() * rhs;
        Self::from_a(v)
    }
}

impl Div<f64> for ElectricCurrent {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_a() / rhs;
        Self::from_a(v)
    }
}

impl Mul<ElectricCurrent> for f64 {
    type Output = ElectricCurrent;
    fn mul(self, rhs: ElectricCurrent) -> Self::Output {
        let v = self * rhs.as_a();
        ElectricCurrent::from_a(v)
    }
}

impl Div<ElectricCurrent> for f64 {
    type Output = ElectricCurrent;
    fn div(self, rhs: ElectricCurrent) -> Self::Output {
        let v = self / rhs.as_a();
        ElectricCurrent::from_a(v)
    }
}

impl Mul<Coef> for ElectricCurrent {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_a() * rhs.get_value();
        Self::from_a(v)
    }
}

impl Div<Coef> for ElectricCurrent {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_a() / rhs.get_value();
        Self::from_a(v)
    }
}

impl Neg for ElectricCurrent {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_a();
        Self::from_a(v)
    }
}


// ElectricPotential × ElectricConductance → ElectricCurrent
impl Mul<ElectricConductance> for ElectricPotential {
    type Output = ElectricCurrent;
    fn mul(self, rhs: ElectricConductance) -> Self::Output {
        ElectricCurrent::from_a(self.as_v() * rhs.as_siemens())
    }
}

impl<'a, 'b> Mul<&'b ElectricConductance> for &'a ElectricPotential {
    type Output = ElectricCurrent;
    fn mul(self, rhs: &'b ElectricConductance) -> Self::Output { ElectricCurrent::from_a(self.as_v() * rhs.as_siemens()) }
}
impl<'a> Mul<&'a ElectricConductance> for ElectricPotential {
    type Output = ElectricCurrent;
    fn mul(self, rhs: &'a ElectricConductance) -> Self::Output { ElectricCurrent::from_a(self.as_v() * rhs.as_siemens()) }
}
impl<'a> Mul<ElectricConductance> for &'a ElectricPotential {
    type Output = ElectricCurrent;
    fn mul(self, rhs: ElectricConductance) -> Self::Output { ElectricCurrent::from_a(self.as_v() * rhs.as_siemens()) }
}

// ElectricConductance × ElectricPotential → ElectricCurrent
impl Mul<ElectricPotential> for ElectricConductance {
    type Output = ElectricCurrent;
    fn mul(self, rhs: ElectricPotential) -> Self::Output {
        ElectricCurrent::from_a(self.as_siemens() * rhs.as_v())
    }
}

impl<'a, 'b> Mul<&'b ElectricPotential> for &'a ElectricConductance {
    type Output = ElectricCurrent;
    fn mul(self, rhs: &'b ElectricPotential) -> Self::Output { ElectricCurrent::from_a(self.as_siemens() * rhs.as_v()) }
}
impl<'a> Mul<&'a ElectricPotential> for ElectricConductance {
    type Output = ElectricCurrent;
    fn mul(self, rhs: &'a ElectricPotential) -> Self::Output { ElectricCurrent::from_a(self.as_siemens() * rhs.as_v()) }
}
impl<'a> Mul<ElectricPotential> for &'a ElectricConductance {
    type Output = ElectricCurrent;
    fn mul(self, rhs: ElectricPotential) -> Self::Output { ElectricCurrent::from_a(self.as_siemens() * rhs.as_v()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electric_current_create_and_convert() {
        let v1 = ElectricCurrent::from_a(1.0);
        assert_relative_eq!(v1.as_a(), 1.0);
        let v2 = ElectricCurrent::from_milli_a(1e3);
        assert_relative_eq!(v2.as_a(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_milli_a(), 1e3, epsilon = 1e-8);
        let v2 = ElectricCurrent::from_micro_a(1e6);
        assert_relative_eq!(v2.as_a(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_micro_a(), 1e6, epsilon = 1e-8);
        let v2 = ElectricCurrent::from_kilo_a(1e-3);
        assert_relative_eq!(v2.as_a(), 1.0, epsilon = 1e-8);
        assert_relative_eq!(v1.as_kilo_a(), 1e-3, epsilon = 1e-8);
    }

    #[test]
    fn test_electric_current_add_sub() {
        let a = ElectricCurrent::from_a(5.0);
        let b = ElectricCurrent::from_a(3.0);
        let s = a + b;
        assert_relative_eq!(s.as_a(), 8.0);
        let d = a - b;
        assert_relative_eq!(d.as_a(), 2.0);
    }

    #[test]
    fn test_electric_current_mul_div_f64() {
        let a = ElectricCurrent::from_a(6.0);
        let m = a * 2.0;
        assert_relative_eq!(m.as_a(), 12.0);
        let d = a / 3.0;
        assert_relative_eq!(d.as_a(), 2.0);
        let r = 3.0 * a;
        assert_relative_eq!(r.as_a(), 18.0);
    }

    #[test]
    fn test_electric_current_coef_ops() {
        let a = ElectricCurrent::from_a(10.0);
        let c = Coef::new(2.0);
        assert_relative_eq!((a * c).as_a(), 20.0);
        assert_relative_eq!((a / c).as_a(), 5.0);
    }

    #[test]
    fn test_electric_current_neg() {
        let a = ElectricCurrent::from_a(5.0);
        let n = -a;
        assert_relative_eq!(n.as_a(), -5.0);
    }

    #[test]
    fn test_electric_current_default_is_zero() {
        let a = ElectricCurrent::default();
        assert!(a.is_zero());
        assert_relative_eq!(a.default_unit_value(), 0.0);
    }

    #[test]
    fn test_electric_current_ref_ops() {
        let a = ElectricCurrent::from_a(2.0);
        let b = ElectricCurrent::from_a(3.0);
        assert_relative_eq!((&a + &b).as_a(), 5.0);
        assert_relative_eq!((&a - &b).as_a(), -1.0);
        assert_relative_eq!((a + &b).as_a(), 5.0);
        assert_relative_eq!((&a + b).as_a(), 5.0);
    }

    #[test]
    fn test_electric_current_as_any() {
        let g: &dyn PhysicalQuantity = &ElectricCurrent::from_a(1.23);
        let d = g.as_any().downcast_ref::<ElectricCurrent>().unwrap();
        assert_relative_eq!(d.as_a(), 1.23);
    }

    #[test]
    fn test_electric_current_set_value() {
        let mut a = ElectricCurrent::from_a(1.0);
        a.set_value(42.0);
        assert_relative_eq!(a.as_a(), 42.0);
    }
    #[test]
    fn test_electricpotential_mul_electricconductance() {
        let l = ElectricPotential::from_v(2.0);
        let r = ElectricConductance::from_siemens(3.0);
        let result: ElectricCurrent = l * r;
        assert_relative_eq!(result.as_a(), 6.0);

        let l2 = ElectricPotential::from_v(2.0);
        let r2 = ElectricConductance::from_siemens(3.0);
        let result2: ElectricCurrent = r2 * l2;
        assert_relative_eq!(result2.as_a(), 6.0);
    }

    #[test]
    fn test_electric_current_all_as_branches() {
        let ma = ElectricCurrent::from_milli_a(1.0);
        let ua = ElectricCurrent::from_micro_a(1.0);
        let ka = ElectricCurrent::from_kilo_a(1.0);

        // as_milli_a 非默认分支
        assert_relative_eq!(ma.as_milli_a(), 1.0);
        assert_relative_eq!(ua.as_milli_a(), 1e-3);
        assert_relative_eq!(ka.as_milli_a(), 1e6);

        // as_micro_a 非默认分支
        assert_relative_eq!(ma.as_micro_a(), 1000.0000000000001);
        assert_relative_eq!(ua.as_micro_a(), 1.0);
        assert_relative_eq!(ka.as_micro_a(), 1e9);

        // as_kilo_a 非默认分支
        assert_relative_eq!(ma.as_kilo_a(), 1e-6);
        assert_relative_eq!(ua.as_kilo_a(), 9.999999999999999e-10);
        assert_relative_eq!(ka.as_kilo_a(), 1.0);
    }

    #[test]
    fn test_electric_current_f64_and_ref_sub() {
        let a = ElectricCurrent::from_a(10.0);
        let b = ElectricCurrent::from_a(3.0);
        // Add<f64>
        assert_relative_eq!((a + 5.0).as_a(), 15.0);
        // Sub<f64>
        assert_relative_eq!((a - 5.0).as_a(), 5.0);
        // Sub 引用形式
        assert_relative_eq!((a - &b).as_a(), 7.0);
        assert_relative_eq!((&a - b).as_a(), 7.0);
        // f64 / 量
        assert_relative_eq!((100.0 / a).as_a(), 10.0);
    }

    #[test]
    fn test_voltage_times_conductance_ref_forms() {
        // V × G 引用形式
        let v = ElectricPotential::from_v(2.0);
        let g = ElectricConductance::from_siemens(3.0);
        assert_relative_eq!((&v * &g).as_a(), 6.0);
        let v2 = ElectricPotential::from_v(2.0);
        let g2 = ElectricConductance::from_siemens(3.0);
        assert_relative_eq!((v2 * &g2).as_a(), 6.0);
        let v3 = ElectricPotential::from_v(2.0);
        let g3 = ElectricConductance::from_siemens(3.0);
        assert_relative_eq!((&v3 * g3).as_a(), 6.0);

        // G × V 引用形式
        let g4 = ElectricConductance::from_siemens(2.0);
        let v4 = ElectricPotential::from_v(3.0);
        assert_relative_eq!((&g4 * &v4).as_a(), 6.0);
        let g5 = ElectricConductance::from_siemens(2.0);
        let v5 = ElectricPotential::from_v(3.0);
        assert_relative_eq!((g5 * &v5).as_a(), 6.0);
        let g6 = ElectricConductance::from_siemens(2.0);
        let v6 = ElectricPotential::from_v(3.0);
        assert_relative_eq!((&g6 * v6).as_a(), 6.0);
    }
}
