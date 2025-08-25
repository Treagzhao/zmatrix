use std::any::Any;
use std::ops::{Add, Div, Mul, Sub};
use crate::physics::basic::{Coef, Energy, EnergyType, Force, PhysicalQuantity, Distance, Mass, Velocity, Acceleration};
use approx::assert_relative_eq;

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
        match self.default_type {
            EnergyType::Joule => {
                let v = self.as_joule() + rhs;
                Self::from_joule(v)
            }
            EnergyType::MillJoule => {
                let v = self.as_mill_joule() + rhs;
                Self::from_mill_joule(v)
            }
            EnergyType::MicroJoule => {
                let v = self.as_micro_joule() + rhs;
                Self::from_micro_joule(v)
            }
            EnergyType::NanoJoule => {
                let v = self.as_nano_joule() + rhs;
                Self::from_nano_joule(v)
            }
            EnergyType::KiloJoule => {
                let v = self.as_kilo_joule() + rhs;
                Self::from_kilo_joule(v)
            }
            EnergyType::MegaJoule => {
                let v = self.as_mega_joule() + rhs;
                Self::from_mega_joule(v)
            }
            EnergyType::ElectronVolt => {
                let v = self.as_electron_volt() + rhs;
                Self::from_electron_volt(v)
            }
            EnergyType::KiloElectronVolt => {
                let v = self.as_kilo_electron_volt() + rhs;
                Self::from_kilo_electron_volt(v)
            }
            EnergyType::MegaElectronVolt => {
                let v = self.as_mega_electron_volt() + rhs;
                Self::from_mega_electron_volt(v)
            }
        }
    }
}

impl Sub<f64> for Energy {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        match self.default_type {
            EnergyType::Joule => {
                let v = self.as_joule() - rhs;
                Self::from_joule(v)
            }
            EnergyType::MillJoule => {
                let v = self.as_mill_joule() - rhs;
                Self::from_mill_joule(v)
            }
            EnergyType::MicroJoule => {
                let v = self.as_micro_joule() - rhs;
                Self::from_micro_joule(v)
            }
            EnergyType::NanoJoule => {
                let v = self.as_nano_joule() - rhs;
                Self::from_nano_joule(v)
            }
            EnergyType::KiloJoule => {
                let v = self.as_kilo_joule() - rhs;
                Self::from_kilo_joule(v)
            }
            EnergyType::MegaJoule => {
                let v = self.as_mega_joule() - rhs;
                Self::from_mega_joule(v)
            }
            EnergyType::ElectronVolt => {
                let v = self.as_electron_volt() - rhs;
                Self::from_electron_volt(v)
            }
            EnergyType::KiloElectronVolt => {
                let v = self.as_kilo_electron_volt() - rhs;
                Self::from_kilo_electron_volt(v)
            }
            EnergyType::MegaElectronVolt => {
                let v = self.as_mega_electron_volt() - rhs;
                Self::from_mega_electron_volt(v)
            }
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
