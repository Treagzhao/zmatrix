use crate::physics::basic::{
    Acceleration, Coef, Distance, Energy, EnergyType, Force, Mass, PhysicalQuantity, Velocity,
};
use approx::assert_relative_eq;
use std::any::Any;
use std::ops::{Add, Div, Mul, Sub};

impl Default for Energy {
    fn default() -> Self {
        Self::from_joule(0.0)
    }
}

impl PhysicalQuantity for Energy {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_joule()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Energy {
    pub fn from_joule(joule: f64) -> Self {
        Self {
            default_type: EnergyType::Joule,
            v: joule,
        }
    }

    pub fn from_mill_joule(mill_joule: f64) -> Self {
        Self {
            default_type: EnergyType::MillJoule,
            v: mill_joule,
        }
    }

    pub fn from_micro_joule(micro_joule: f64) -> Self {
        Self {
            default_type: EnergyType::MicroJoule,
            v: micro_joule,
        }
    }

    pub fn from_nano_joule(nano_joule: f64) -> Self {
        Self {
            default_type: EnergyType::NanoJoule,
            v: nano_joule,
        }
    }

    pub fn from_kilo_joule(kilo_joule: f64) -> Self {
        Self {
            default_type: EnergyType::KiloJoule,
            v: kilo_joule,
        }
    }

    pub fn from_mega_joule(mega_joule: f64) -> Self {
        Self {
            default_type: EnergyType::MegaJoule,
            v: mega_joule,
        }
    }

    pub fn from_electron_volt(electron_volt: f64) -> Self {
        Self {
            default_type: EnergyType::ElectronVolt,
            v: electron_volt,
        }
    }

    pub fn from_kilo_electron_volt(kilo_electron_volt: f64) -> Self {
        Self {
            default_type: EnergyType::KiloElectronVolt,
            v: kilo_electron_volt,
        }
    }

    pub fn from_mega_electron_volt(mega_electron_volt: f64) -> Self {
        Self {
            default_type: EnergyType::MegaElectronVolt,
            v: mega_electron_volt,
        }
    }

    pub fn as_joule(&self) -> f64 {
        match self.default_type {
            EnergyType::Joule => self.v,
            EnergyType::MillJoule => self.v * 1e-3,
            EnergyType::MicroJoule => self.v * 1e-6,
            EnergyType::NanoJoule => self.v * 1e-9,
            EnergyType::KiloJoule => self.v * 1e3,
            EnergyType::MegaJoule => self.v * 1e6,
            EnergyType::ElectronVolt => self.v * 1.602176634e-19,
            EnergyType::KiloElectronVolt => self.v * 1.602176634e-16,
            EnergyType::MegaElectronVolt => self.v * 1.602176634e-13,
        }
    }

    pub fn as_mill_joule(&self) -> f64 {
        match self.default_type {
            EnergyType::Joule => self.v * 1e3,
            EnergyType::MillJoule => self.v,
            EnergyType::MicroJoule => self.v * 1e-3,
            EnergyType::NanoJoule => self.v * 1e-6,
            EnergyType::KiloJoule => self.v * 1e6,
            EnergyType::MegaJoule => self.v * 1e9,
            EnergyType::ElectronVolt => self.v * 1.602176634e-16,
            EnergyType::KiloElectronVolt => self.v * 1.602176634e-13,
            EnergyType::MegaElectronVolt => self.v * 1.602176634e-10,
        }
    }

    pub fn as_micro_joule(&self) -> f64 {
        match self.default_type {
            EnergyType::Joule => self.v * 1e6,
            EnergyType::MillJoule => self.v * 1e3,
            EnergyType::MicroJoule => self.v,
            EnergyType::NanoJoule => self.v * 1e-3,
            EnergyType::KiloJoule => self.v * 1e9,
            EnergyType::MegaJoule => self.v * 1e12,
            EnergyType::ElectronVolt => self.v * 1.602176634e-13,
            EnergyType::KiloElectronVolt => self.v * 1.602176634e-10,
            EnergyType::MegaElectronVolt => self.v * 1.602176634e-7,
        }
    }

    pub fn as_nano_joule(&self) -> f64 {
        match self.default_type {
            EnergyType::Joule => self.v * 1e9,
            EnergyType::MillJoule => self.v * 1e6,
            EnergyType::MicroJoule => self.v * 1e3,
            EnergyType::NanoJoule => self.v,
            EnergyType::KiloJoule => self.v * 1e12,
            EnergyType::MegaJoule => self.v * 1e15,
            EnergyType::ElectronVolt => self.v * 1.602176634e-10,
            EnergyType::KiloElectronVolt => self.v * 1.602176634e-7,
            EnergyType::MegaElectronVolt => self.v * 1.602176634e-4,
        }
    }

    pub fn as_kilo_joule(&self) -> f64 {
        match self.default_type {
            EnergyType::Joule => self.v * 1e-3,
            EnergyType::MillJoule => self.v * 1e-6,
            EnergyType::MicroJoule => self.v * 1e-9,
            EnergyType::NanoJoule => self.v * 1e-12,
            EnergyType::KiloJoule => self.v,
            EnergyType::MegaJoule => self.v * 1e3,
            EnergyType::ElectronVolt => self.v * 1.602176634e-22,
            EnergyType::KiloElectronVolt => self.v * 1.602176634e-19,
            EnergyType::MegaElectronVolt => self.v * 1.602176634e-16,
        }
    }

    pub fn as_mega_joule(&self) -> f64 {
        match self.default_type {
            EnergyType::Joule => self.v * 1e-6,
            EnergyType::MillJoule => self.v * 1e-9,
            EnergyType::MicroJoule => self.v * 1e-12,
            EnergyType::NanoJoule => self.v * 1e-15,
            EnergyType::KiloJoule => self.v * 1e-3,
            EnergyType::MegaJoule => self.v,
            EnergyType::ElectronVolt => self.v * 1.602176634e-25,
            EnergyType::KiloElectronVolt => self.v * 1.602176634e-22,
            EnergyType::MegaElectronVolt => self.v * 1.602176634e-19,
        }
    }

    pub fn as_electron_volt(&self) -> f64 {
        match self.default_type {
            EnergyType::Joule => self.v / 1.602176634e-19,
            EnergyType::MillJoule => self.v / 1.602176634e-16,
            EnergyType::MicroJoule => self.v / 1.602176634e-13,
            EnergyType::NanoJoule => self.v / 1.602176634e-10,
            EnergyType::KiloJoule => self.v / 1.602176634e-22,
            EnergyType::MegaJoule => self.v / 1.602176634e-25,
            EnergyType::ElectronVolt => self.v,
            EnergyType::KiloElectronVolt => self.v * 1e3,
            EnergyType::MegaElectronVolt => self.v * 1e6,
        }
    }

    pub fn as_kilo_electron_volt(&self) -> f64 {
        match self.default_type {
            EnergyType::Joule => self.v / 1.602176634e-16,
            EnergyType::MillJoule => self.v / 1.602176634e-13,
            EnergyType::MicroJoule => self.v / 1.602176634e-10,
            EnergyType::NanoJoule => self.v / 1.602176634e-7,
            EnergyType::KiloJoule => self.v / 1.602176634e-19,
            EnergyType::MegaJoule => self.v / 1.602176634e-22,
            EnergyType::ElectronVolt => self.v * 1e-3,
            EnergyType::KiloElectronVolt => self.v,
            EnergyType::MegaElectronVolt => self.v * 1e3,
        }
    }

    pub fn as_mega_electron_volt(&self) -> f64 {
        match self.default_type {
            EnergyType::Joule => self.v / 1.602176634e-13,
            EnergyType::MillJoule => self.v / 1.602176634e-10,
            EnergyType::MicroJoule => self.v / 1.602176634e-7,
            EnergyType::NanoJoule => self.v / 1.602176634e-4,
            EnergyType::KiloJoule => self.v / 1.602176634e-16,
            EnergyType::MegaJoule => self.v / 1.602176634e-19,
            EnergyType::ElectronVolt => self.v * 1e-6,
            EnergyType::KiloElectronVolt => self.v * 1e-3,
            EnergyType::MegaElectronVolt => self.v,
        }
    }
}

// 能量与能量的运算
impl Add for Energy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_joule() + rhs.as_joule();
        Self::from_joule(v)
    }
}

impl Sub for Energy {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_joule() - rhs.as_joule();
        Self::from_joule(v)
    }
}

// 能量与标量的运算
impl Add<f64> for Energy {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        Energy {
            v,
            default_type: self.default_type,
        }
    }
}

impl Sub<f64> for Energy {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        Energy {
            v,
            default_type: self.default_type,
        }
    }
}

impl Mul<f64> for Energy {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_joule() * rhs;
        Self::from_joule(v)
    }
}

impl Div<f64> for Energy {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_joule() / rhs;
        Self::from_joule(v)
    }
}

// 标量与能量的运算
impl Mul<Energy> for f64 {
    type Output = Energy;
    fn mul(self, rhs: Energy) -> Self::Output {
        let v = self * rhs.as_joule();
        Energy::from_joule(v)
    }
}

impl Div<Energy> for f64 {
    type Output = Energy;
    fn div(self, rhs: Energy) -> Self::Output {
        let v = self / rhs.as_joule();
        Energy::from_joule(v)
    }
}

// 能量与系数的运算
impl Mul<Coef> for Energy {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_joule() * rhs.get_value();
        Self::from_joule(v)
    }
}

impl Div<Coef> for Energy {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_joule() / rhs.get_value();
        Self::from_joule(v)
    }
}

// 能量 ÷ 时间 = 功率
impl Div<std::time::Duration> for Energy {
    type Output = crate::physics::basic::Power;
    fn div(self, rhs: std::time::Duration) -> Self::Output {
        let power_value = self.as_joule() / rhs.as_secs_f64();
        crate::physics::basic::Power::from_watt(power_value)
    }
}

// 能量 ÷ 距离 = 力
impl Div<Distance> for Energy {
    type Output = Force;
    fn div(self, rhs: Distance) -> Self::Output {
        let force_value = self.as_joule() / rhs.as_m();
        Force::from_newton(force_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy() {
        let e1 = Energy::from_joule(1000.0);
        assert_eq!(e1.v, 1000.0);
        assert_eq!(e1.default_type, EnergyType::Joule);

        let e2 = Energy::from_mill_joule(1000.0);
        assert_eq!(e2.v, 1000.0);
        assert_eq!(e2.default_type, EnergyType::MillJoule);

        let e3 = Energy::from_micro_joule(1000.0);
        assert_eq!(e3.v, 1000.0);
        assert_eq!(e3.default_type, EnergyType::MicroJoule);

        let e4 = Energy::from_nano_joule(1000.0);
        assert_eq!(e4.v, 1000.0);
        assert_eq!(e4.default_type, EnergyType::NanoJoule);

        let e5 = Energy::from_kilo_joule(1.0);
        assert_eq!(e5.v, 1.0);
        assert_eq!(e5.default_type, EnergyType::KiloJoule);

        let e6 = Energy::from_mega_joule(1.0);
        assert_eq!(e6.v, 1.0);
        assert_eq!(e6.default_type, EnergyType::MegaJoule);

        let e7 = Energy::from_electron_volt(1.0);
        assert_eq!(e7.v, 1.0);
        assert_eq!(e7.default_type, EnergyType::ElectronVolt);

        let e8 = Energy::from_kilo_electron_volt(1.0);
        assert_eq!(e8.v, 1.0);
        assert_eq!(e8.default_type, EnergyType::KiloElectronVolt);

        let e9 = Energy::from_mega_electron_volt(1.0);
        assert_eq!(e9.v, 1.0);
        assert_eq!(e9.default_type, EnergyType::MegaElectronVolt);

        let e10 = Energy::default();
        assert_eq!(e10.v, 0.0);
        assert_eq!(e10.default_type, EnergyType::Joule);
    }

    #[test]
    fn test_energy_as() {
        let e1 = Energy::from_joule(1.0);
        assert_relative_eq!(e1.as_joule(), 1.0);
        assert_relative_eq!(e1.as_mill_joule(), 1e3);
        assert_relative_eq!(e1.as_micro_joule(), 1e6);
        assert_relative_eq!(e1.as_nano_joule(), 1e9);
        assert_relative_eq!(e1.as_kilo_joule(), 1e-3);
        assert_relative_eq!(e1.as_mega_joule(), 1e-6);

        let e2 = Energy::from_mill_joule(1e3);
        assert_relative_eq!(e2.as_joule(), 1.0);
        assert_relative_eq!(e2.as_mill_joule(), 1e3);
        assert_relative_eq!(e2.as_micro_joule(), 1e6);

        let e3 = Energy::from_micro_joule(1e6);
        assert_relative_eq!(e3.as_joule(), 1.0);
        assert_relative_eq!(e3.as_micro_joule(), 1e6);

        let e4 = Energy::from_nano_joule(1e9);
        assert_relative_eq!(e4.as_joule(), 1.0);
        assert_relative_eq!(e4.as_nano_joule(), 1e9);

        let e5 = Energy::from_kilo_joule(1e-3);
        assert_relative_eq!(e5.as_joule(), 1.0);
        assert_relative_eq!(e5.as_kilo_joule(), 1e-3);

        let e6 = Energy::from_mega_joule(1e-6);
        assert_relative_eq!(e6.as_joule(), 1.0);
        assert_relative_eq!(e6.as_mega_joule(), 1e-6);

        // 测试电子伏特转换
        let e7 = Energy::from_electron_volt(1.0);
        assert_relative_eq!(e7.as_joule(), 1.602176634e-19, epsilon = 1e-25);
        assert_relative_eq!(e7.as_electron_volt(), 1.0);

        let e8 = Energy::from_kilo_electron_volt(1.0);
        assert_relative_eq!(e8.as_joule(), 1.602176634e-16, epsilon = 1e-22);
        assert_relative_eq!(e8.as_kilo_electron_volt(), 1.0);

        let e9 = Energy::from_mega_electron_volt(1.0);
        assert_relative_eq!(e9.as_joule(), 1.602176634e-13, epsilon = 1e-19);
        assert_relative_eq!(e9.as_mega_electron_volt(), 1.0);
    }

    #[test]
    fn test_energy_add() {
        let e1 = Energy::from_joule(1000.0);
        let e2 = Energy::from_joule(1000.0);
        let e3 = e1 + e2;
        assert_relative_eq!(e3.as_joule(), 2000.0);

        let e1 = Energy::from_mill_joule(1e6);
        let e2 = Energy::from_joule(1000.0);
        let e3 = e1 + e2;
        assert_relative_eq!(e3.as_joule(), 2000.0);

        let e1 = Energy::from_joule(1000.0);
        let e2 = e1 + 100.0;
        assert_relative_eq!(e2.as_joule(), 1100.0);

        let e1 = Energy::from_mill_joule(1000.0);
        let e2 = e1 + 100.0;
        assert_relative_eq!(e2.as_mill_joule(), 1100.0);
    }

    #[test]
    fn test_energy_sub() {
        let e1 = Energy::from_joule(2000.0);
        let e2 = Energy::from_joule(1000.0);
        let e3 = e1 - e2;
        assert_relative_eq!(e3.as_joule(), 1000.0);

        let e1 = Energy::from_mill_joule(2e6);
        let e2 = Energy::from_joule(1000.0);
        let e3 = e1 - e2;
        assert_relative_eq!(e3.as_joule(), 1000.0);

        let e1 = Energy::from_joule(2000.0);
        let e2 = e1 - 100.0;
        assert_relative_eq!(e2.as_joule(), 1900.0);
    }

    #[test]
    fn test_energy_mul() {
        let e1 = Energy::from_joule(2000.0);
        let e2 = e1 * 2.0;
        assert_relative_eq!(e2.as_joule(), 4000.0);

        let e1 = Energy::from_mill_joule(2000.0);
        let e2 = e1 * 2.0;
        assert_relative_eq!(e2.as_mill_joule(), 4000.0);

        let e1 = Energy::from_joule(1000.0);
        let e2 = e1 * Coef::new(2.0);
        assert_relative_eq!(e2.as_joule(), 2000.0);
    }

    #[test]
    fn test_energy_div() {
        let e1 = Energy::from_joule(2000.0);
        let e2 = e1 / 2.0;
        assert_relative_eq!(e2.as_joule(), 1000.0);

        let e1 = Energy::from_mill_joule(2000.0);
        let e2 = e1 / 2.0;
        assert_relative_eq!(e2.as_mill_joule(), 1000.0);

        let e1 = Energy::from_joule(2000.0);
        let e2 = e1 / Coef::new(2.0);
        assert_relative_eq!(e2.as_joule(), 1000.0);
    }

    #[test]
    fn test_default_unit_value() {
        let e1 = Energy::from_joule(1.0);
        assert_relative_eq!(e1.default_unit_value(), 1.0);
        let e1 = Energy::from_mill_joule(1.0);
        assert_relative_eq!(e1.default_unit_value(), 1e-3);
    }

    #[test]
    fn test_f64_mul_energy() {
        let e = Energy::from_joule(2.0);
        let result = 3.0 * e;
        assert_relative_eq!(result.as_joule(), 6.0);

        let e = Energy::from_mill_joule(2.0);
        let result = 3.0 * e;
        assert_relative_eq!(result.as_mill_joule(), 6.0);
    }

    #[test]
    fn test_f64_div_energy() {
        let e = Energy::from_joule(2.0);
        let result = 6.0 / e;
        assert_relative_eq!(result.as_joule(), 3.0);

        let e = Energy::from_mill_joule(2000.0);
        let result = 6.0 / e;
        assert_relative_eq!(result.as_joule(), 3.0);
    }

    #[test]
    fn test_energy_operations_with_different_types() {
        // 测试不同能量类型之间的运算
        let e1 = Energy::from_joule(1000.0);
        let e2 = Energy::from_kilo_joule(1.0);
        let e3 = Energy::from_mega_joule(0.001);

        // 加法
        let sum = e1 + e2 + e3;
        assert_relative_eq!(sum.as_joule(), 3000.0);

        // 减法
        let diff = e1 - e2;
        assert_relative_eq!(diff.as_joule(), 0.0);

        // 乘法
        let product = e1 * 2.0;
        assert_relative_eq!(product.as_joule(), 2000.0);

        // 除法
        let quotient = e1 / 2.0;
        assert_relative_eq!(quotient.as_joule(), 500.0);
    }

    #[test]
    fn test_energy_comprehensive_arithmetic_operations() {
        // 测试所有类型的 Energy 与 f64 的运算

        // 测试从不同单位创建的 Energy 与 f64 的加法
        let e_joule = Energy::from_joule(10.0);
        let result = e_joule + 5.0;
        assert_relative_eq!(result.as_joule(), 15.0);

        let e_mill = Energy::from_mill_joule(10.0);
        let result = e_mill + 5.0;
        assert_relative_eq!(result.as_mill_joule(), 15.0);

        let e_micro = Energy::from_micro_joule(10.0);
        let result = e_micro + 5.0;
        assert_relative_eq!(result.as_micro_joule(), 15.0);

        let e_nano = Energy::from_nano_joule(10.0);
        let result = e_nano + 5.0;
        assert_relative_eq!(result.as_nano_joule(), 15.0);

        let e_kilo = Energy::from_kilo_joule(10.0);
        let result = e_kilo + 5.0;
        assert_relative_eq!(result.as_kilo_joule(), 15.0);

        let e_mega = Energy::from_mega_joule(10.0);
        let result = e_mega + 5.0;
        assert_relative_eq!(result.as_mega_joule(), 15.0);

        let e_ev = Energy::from_electron_volt(10.0);
        let result = e_ev + 5.0;
        assert_relative_eq!(result.as_electron_volt(), 15.0);

        let e_kev = Energy::from_kilo_electron_volt(10.0);
        let result = e_kev + 5.0;
        assert_relative_eq!(result.as_kilo_electron_volt(), 15.0);

        let e_mev = Energy::from_mega_electron_volt(10.0);
        let result = e_mev + 5.0;
        assert_relative_eq!(result.as_mega_electron_volt(), 15.0);

        // 测试从不同单位创建的 Energy 与 f64 的减法
        let e_joule = Energy::from_joule(20.0);
        let result = e_joule - 5.0;
        assert_relative_eq!(result.as_joule(), 15.0);

        let e_mill = Energy::from_mill_joule(20.0);
        let result = e_mill - 5.0;
        assert_relative_eq!(result.as_mill_joule(), 15.0);

        let e_micro = Energy::from_micro_joule(20.0);
        let result = e_micro - 5.0;
        assert_relative_eq!(result.as_micro_joule(), 15.0);

        let e_nano = Energy::from_nano_joule(20.0);
        let result = e_nano - 5.0;
        assert_relative_eq!(result.as_nano_joule(), 15.0);

        let e_kilo = Energy::from_kilo_joule(20.0);
        let result = e_kilo - 5.0;
        assert_relative_eq!(result.as_kilo_joule(), 15.0);

        let e_mega = Energy::from_mega_joule(20.0);
        let result = e_mega - 5.0;
        assert_relative_eq!(result.as_mega_joule(), 15.0);

        let e_ev = Energy::from_electron_volt(20.0);
        let result = e_ev - 5.0;
        assert_relative_eq!(result.as_electron_volt(), 15.0);

        let e_kev = Energy::from_kilo_electron_volt(20.0);
        let result = e_kev - 5.0;
        assert_relative_eq!(result.as_kilo_electron_volt(), 15.0);

        let e_mev = Energy::from_mega_electron_volt(20.0);
        let result = e_mev - 5.0;
        assert_relative_eq!(result.as_mega_electron_volt(), 15.0);
    }

    #[test]
    fn test_energy_physical_operations() {
        // 测试 Energy 与物理量的运算

        // Energy ÷ Duration = Power
        let energy = Energy::from_joule(100.0);
        let time = std::time::Duration::from_secs(5);
        let power = energy / time;
        assert_relative_eq!(power.as_watt(), 20.0);

        // Energy ÷ Distance = Force
        let energy = Energy::from_joule(60.0);
        let distance = Distance::from_m(3.0);
        let force = energy / distance;
        assert_relative_eq!(force.as_newton(), 20.0);

        // 测试不同单位的 Energy 进行物理运算
        let energy_mill = Energy::from_mill_joule(100000.0); // 100 J
        let time = std::time::Duration::from_secs(5);
        let power = energy_mill / time;
        assert_relative_eq!(power.as_watt(), 20.0);

        let energy_kilo = Energy::from_kilo_joule(0.06); // 60 J
        let distance = Distance::from_m(3.0);
        let force = energy_kilo / distance;
        assert_relative_eq!(force.as_newton(), 20.0);

        let energy_ev = Energy::from_electron_volt(100.0 / 1.602176634e-19); // 100 J
        let time = std::time::Duration::from_secs(5);
        let power = energy_ev / time;
        assert_relative_eq!(power.as_watt(), 20.0);
    }

    #[test]
    fn test_energy_electron_volt_conversions() {
        // 测试电子伏特相关的转换
        let e1 = Energy::from_electron_volt(1.0);
        let e2 = Energy::from_kilo_electron_volt(1.0);
        let e3 = Energy::from_mega_electron_volt(1.0);

        // 测试转换到焦耳
        assert_relative_eq!(e1.as_joule(), 1.602176634e-19, epsilon = 1e-25);
        assert_relative_eq!(e2.as_joule(), 1.602176634e-16, epsilon = 1e-22);
        assert_relative_eq!(e3.as_joule(), 1.602176634e-13, epsilon = 1e-19);

        // 测试转换到电子伏特
        assert_relative_eq!(e1.as_electron_volt(), 1.0);
        assert_relative_eq!(e2.as_electron_volt(), 1000.0);
        assert_relative_eq!(e3.as_electron_volt(), 1000000.0);

        // 测试转换到千电子伏特
        assert_relative_eq!(e1.as_kilo_electron_volt(), 0.001);
        assert_relative_eq!(e2.as_kilo_electron_volt(), 1.0);
        assert_relative_eq!(e3.as_kilo_electron_volt(), 1000.0);

        // 测试转换到兆电子伏特
        assert_relative_eq!(e1.as_mega_electron_volt(), 1e-6);
        assert_relative_eq!(e2.as_mega_electron_volt(), 0.001);
        assert_relative_eq!(e3.as_mega_electron_volt(), 1.0);
    }

    #[test]
    fn test_energy_all_unit_conversions() {
        let e = Energy::from_joule(1.0);

        // 测试所有单位的转换
        assert_relative_eq!(e.as_joule(), 1.0);
        assert_relative_eq!(e.as_mill_joule(), 1000.0);
        assert_relative_eq!(e.as_micro_joule(), 1000000.0);
        assert_relative_eq!(e.as_nano_joule(), 1000000000.0);
        assert_relative_eq!(e.as_kilo_joule(), 0.001);
        assert_relative_eq!(e.as_mega_joule(), 0.000001);

        // 从不同单位创建并测试转换
        let e_mill = Energy::from_mill_joule(1000.0);
        assert_relative_eq!(e_mill.as_joule(), 1.0);

        let e_micro = Energy::from_micro_joule(1000000.0);
        assert_relative_eq!(e_micro.as_joule(), 1.0);

        let e_nano = Energy::from_nano_joule(1000000000.0);
        assert_relative_eq!(e_nano.as_joule(), 1.0);

        let e_kilo = Energy::from_kilo_joule(0.001);
        assert_relative_eq!(e_kilo.as_joule(), 1.0);

        let e_mega = Energy::from_mega_joule(0.000001);
        assert_relative_eq!(e_mega.as_joule(), 1.0);
    }

    #[test]
    fn test_energy_comprehensive_as_methods() {
        // 测试从每个单位类型创建，然后调用所有 as_xxx 方法

        // 从 Joule 创建
        let e_joule = Energy::from_joule(1.0);
        assert_relative_eq!(e_joule.as_joule(), 1.0);
        assert_relative_eq!(e_joule.as_mill_joule(), 1e3);
        assert_relative_eq!(e_joule.as_micro_joule(), 1e6);
        assert_relative_eq!(e_joule.as_nano_joule(), 1e9);
        assert_relative_eq!(e_joule.as_kilo_joule(), 1e-3);
        assert_relative_eq!(e_joule.as_mega_joule(), 1e-6);
        assert_relative_eq!(e_joule.as_electron_volt(), 1.0 / 1.602176634e-19);
        assert_relative_eq!(e_joule.as_kilo_electron_volt(), 1.0 / 1.602176634e-16);
        assert_relative_eq!(e_joule.as_mega_electron_volt(), 1.0 / 1.602176634e-13);

        // 从 MillJoule 创建
        let e_mill = Energy::from_mill_joule(1.0);
        assert_relative_eq!(e_mill.as_joule(), 1e-3);
        assert_relative_eq!(e_mill.as_mill_joule(), 1.0);
        assert_relative_eq!(e_mill.as_micro_joule(), 1e3);
        assert_relative_eq!(e_mill.as_nano_joule(), 1e6);
        assert_relative_eq!(e_mill.as_kilo_joule(), 1e-6);
        assert_relative_eq!(e_mill.as_mega_joule(), 1e-9);
        assert_relative_eq!(e_mill.as_electron_volt(), 1e-3 / 1.602176634e-19);
        assert_relative_eq!(e_mill.as_kilo_electron_volt(), 1e-3 / 1.602176634e-16);
        assert_relative_eq!(e_mill.as_mega_electron_volt(), 1e-3 / 1.602176634e-13);

        // 从 MicroJoule 创建
        let e_micro = Energy::from_micro_joule(1.0);
        assert_relative_eq!(e_micro.as_joule(), 1e-6);
        assert_relative_eq!(e_micro.as_mill_joule(), 1e-3);
        assert_relative_eq!(e_micro.as_micro_joule(), 1.0);
        assert_relative_eq!(e_micro.as_nano_joule(), 1e3);
        assert_relative_eq!(e_micro.as_kilo_joule(), 1e-9);
        assert_relative_eq!(e_micro.as_mega_joule(), 1e-12);
        assert_relative_eq!(
            e_micro.as_electron_volt(),
            1e-6 / 1.602176634e-19,
            epsilon = 1e-8
        );
        assert_relative_eq!(
            e_micro.as_kilo_electron_volt(),
            1e-6 / 1.602176634e-16,
            epsilon = 1e-8
        );
        assert_relative_eq!(
            e_micro.as_mega_electron_volt(),
            1e-6 / 1.602176634e-13,
            epsilon = 1e-8
        );

        // 从 NanoJoule 创建
        let e_nano = Energy::from_nano_joule(1.0);
        assert_relative_eq!(e_nano.as_joule(), 1e-9);
        assert_relative_eq!(e_nano.as_mill_joule(), 1e-6);
        assert_relative_eq!(e_nano.as_micro_joule(), 1e-3);
        assert_relative_eq!(e_nano.as_nano_joule(), 1.0);
        assert_relative_eq!(e_nano.as_kilo_joule(), 1e-12);
        assert_relative_eq!(e_nano.as_mega_joule(), 1e-15);
        assert_relative_eq!(
            e_nano.as_electron_volt(),
            1e-9 / 1.602176634e-19,
            epsilon = 1e-8
        );
        assert_relative_eq!(
            e_nano.as_kilo_electron_volt(),
            1e-9 / 1.602176634e-16,
            epsilon = 1e-8
        );
        assert_relative_eq!(
            e_nano.as_mega_electron_volt(),
            1e-9 / 1.602176634e-13,
            epsilon = 1e-8
        );

        // 从 KiloJoule 创建
        let e_kilo = Energy::from_kilo_joule(1.0);
        assert_relative_eq!(e_kilo.as_joule(), 1e3);
        assert_relative_eq!(e_kilo.as_mill_joule(), 1e6);
        assert_relative_eq!(e_kilo.as_micro_joule(), 1e9);
        assert_relative_eq!(e_kilo.as_nano_joule(), 1e12);
        assert_relative_eq!(e_kilo.as_kilo_joule(), 1.0);
        assert_relative_eq!(e_kilo.as_mega_joule(), 1e-3);
        assert_relative_eq!(e_kilo.as_electron_volt(), 1e3 / 1.602176634e-19);
        assert_relative_eq!(e_kilo.as_kilo_electron_volt(), 1e3 / 1.602176634e-16);
        assert_relative_eq!(e_kilo.as_mega_electron_volt(), 1e3 / 1.602176634e-13);

        // 从 MegaJoule 创建
        let e_mega = Energy::from_mega_joule(1.0);
        assert_relative_eq!(e_mega.as_joule(), 1e6);
        assert_relative_eq!(e_mega.as_mill_joule(), 1e9);
        assert_relative_eq!(e_mega.as_micro_joule(), 1e12);
        assert_relative_eq!(e_mega.as_nano_joule(), 1e15);
        assert_relative_eq!(e_mega.as_kilo_joule(), 1e3);
        assert_relative_eq!(e_mega.as_mega_joule(), 1.0);
        assert_relative_eq!(e_mega.as_electron_volt(), 1e6 / 1.602176634e-19);
        assert_relative_eq!(e_mega.as_kilo_electron_volt(), 1e6 / 1.602176634e-16);
        assert_relative_eq!(e_mega.as_mega_electron_volt(), 1e6 / 1.602176634e-13);

        // 从 ElectronVolt 创建
        let e_ev = Energy::from_electron_volt(1.0);
        assert_relative_eq!(e_ev.as_joule(), 1.602176634e-19);
        assert_relative_eq!(e_ev.as_mill_joule(), 1.602176634e-16);
        assert_relative_eq!(e_ev.as_micro_joule(), 1.602176634e-13);
        assert_relative_eq!(e_ev.as_nano_joule(), 1.602176634e-10);
        assert_relative_eq!(e_ev.as_kilo_joule(), 1.602176634e-22);
        assert_relative_eq!(e_ev.as_mega_joule(), 1.602176634e-25);
        assert_relative_eq!(e_ev.as_electron_volt(), 1.0);
        assert_relative_eq!(e_ev.as_kilo_electron_volt(), 1e-3);
        assert_relative_eq!(e_ev.as_mega_electron_volt(), 1e-6);

        // 从 KiloElectronVolt 创建
        let e_kev = Energy::from_kilo_electron_volt(1.0);
        assert_relative_eq!(e_kev.as_joule(), 1.602176634e-16);
        assert_relative_eq!(e_kev.as_mill_joule(), 1.602176634e-13);
        assert_relative_eq!(e_kev.as_micro_joule(), 1.602176634e-10);
        assert_relative_eq!(e_kev.as_nano_joule(), 1.602176634e-7);
        assert_relative_eq!(e_kev.as_kilo_joule(), 1.602176634e-19);
        assert_relative_eq!(e_kev.as_mega_joule(), 1.602176634e-22);
        assert_relative_eq!(e_kev.as_electron_volt(), 1e3);
        assert_relative_eq!(e_kev.as_kilo_electron_volt(), 1.0);
        assert_relative_eq!(e_kev.as_mega_electron_volt(), 1e-3);

        // 从 MegaElectronVolt 创建
        let e_mev = Energy::from_mega_electron_volt(1.0);
        assert_relative_eq!(e_mev.as_joule(), 1.602176634e-13);
        assert_relative_eq!(e_mev.as_mill_joule(), 1.602176634e-10);
        assert_relative_eq!(e_mev.as_micro_joule(), 1.602176634e-7);
        assert_relative_eq!(e_mev.as_nano_joule(), 1.602176634e-4);
        assert_relative_eq!(e_mev.as_kilo_joule(), 1.602176634e-16);
        assert_relative_eq!(e_mev.as_mega_joule(), 1.602176634e-19);
        assert_relative_eq!(e_mev.as_electron_volt(), 1e6);
        assert_relative_eq!(e_mev.as_kilo_electron_volt(), 1e3);
        assert_relative_eq!(e_mev.as_mega_electron_volt(), 1.0);
    }

    #[test]
    fn test_energy_edge_cases() {
        // 测试边界情况
        let zero_energy = Energy::from_joule(0.0);
        assert!(zero_energy.is_zero());

        let negative_energy = Energy::from_joule(-100.0);
        assert!(!negative_energy.is_zero());
        assert_relative_eq!(negative_energy.as_joule(), -100.0);

        // 测试大数值
        let large_energy = Energy::from_mega_joule(1000.0);
        assert_relative_eq!(large_energy.as_joule(), 1e9);

        // 测试小数值
        let small_energy = Energy::from_nano_joule(1.0);
        assert_relative_eq!(small_energy.as_joule(), 1e-9);
    }

    #[test]
    fn test_energy_physical_quantity_trait() {
        let mut e = Energy::from_joule(10.0);

        // 测试 default_unit_value
        assert_relative_eq!(e.default_unit_value(), 10.0);

        // 测试 set_value
        e.set_value(20.0);
        assert_relative_eq!(e.as_joule(), 20.0);

        // 测试 is_zero
        assert!(!e.is_zero());
        e.set_value(0.0);
        assert!(e.is_zero());

        // 测试 as_any
        let any_ref = e.as_any();
        assert!(any_ref.is::<Energy>());
    }

    #[test]
    fn test_set_value() {
        let mut e = Energy::from_joule(1.0);
        e.set_value(2.0);
        assert_relative_eq!(e.as_joule(), 2.0);
    }

    #[test]
    fn test_energy_div_duration() {
        let energy = Energy::from_joule(100.0); // 100 J
        let time = std::time::Duration::from_secs(5); // 5 s
        let power: crate::physics::basic::Power = energy / time; // 20 W

        assert_relative_eq!(power.as_watt(), 20.0);
    }

    #[test]
    fn test_energy_div_distance() {
        let energy = Energy::from_joule(60.0); // 60 J
        let distance = Distance::from_m(3.0); // 3 m
        let force: Force = energy / distance; // 20 N

        assert_relative_eq!(force.as_newton(), 20.0);
    }
}
