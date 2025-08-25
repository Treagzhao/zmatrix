use std::any::Any;
use std::ops::{Add, Div, Mul, Sub};
use crate::physics::basic::{Coef, Power, PowerType, PhysicalQuantity, Energy, Velocity, Force};
use approx::assert_relative_eq;

impl Default for Power {
    fn default() -> Self {
        Self::from_watt(0.0)
    }
}

impl PhysicalQuantity for Power {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_watt()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Power {
    pub fn from_watt(watt: f64) -> Self {
        Self {
            default_type: PowerType::Watt,
            v: watt,
        }
    }

    pub fn from_mill_watt(mill_watt: f64) -> Self {
        Self {
            default_type: PowerType::MillWatt,
            v: mill_watt,
        }
    }

    pub fn from_micro_watt(micro_watt: f64) -> Self {
        Self {
            default_type: PowerType::MicroWatt,
            v: micro_watt,
        }
    }

    pub fn from_nano_watt(nano_watt: f64) -> Self {
        Self {
            default_type: PowerType::NanoWatt,
            v: nano_watt,
        }
    }

    pub fn from_kilo_watt(kilo_watt: f64) -> Self {
        Self {
            default_type: PowerType::KiloWatt,
            v: kilo_watt,
        }
    }

    pub fn from_mega_watt(mega_watt: f64) -> Self {
        Self {
            default_type: PowerType::MegaWatt,
            v: mega_watt,
        }
    }

    pub fn from_horse_power(horse_power: f64) -> Self {
        Self {
            default_type: PowerType::HorsePower,
            v: horse_power,
        }
    }

    pub fn as_watt(&self) -> f64 {
        match self.default_type {
            PowerType::Watt => self.v,
            PowerType::MillWatt => self.v * 1e-3,
            PowerType::MicroWatt => self.v * 1e-6,
            PowerType::NanoWatt => self.v * 1e-9,
            PowerType::KiloWatt => self.v * 1e3,
            PowerType::MegaWatt => self.v * 1e6,
            PowerType::HorsePower => self.v * 745.7, // 1 hp = 745.7 W
        }
    }

    pub fn as_mill_watt(&self) -> f64 {
        match self.default_type {
            PowerType::Watt => self.v * 1e3,
            PowerType::MillWatt => self.v,
            PowerType::MicroWatt => self.v * 1e-3,
            PowerType::NanoWatt => self.v * 1e-6,
            PowerType::KiloWatt => self.v * 1e6,
            PowerType::MegaWatt => self.v * 1e9,
            PowerType::HorsePower => self.v * 745700.0,
        }
    }

    pub fn as_micro_watt(&self) -> f64 {
        match self.default_type {
            PowerType::Watt => self.v * 1e6,
            PowerType::MillWatt => self.v * 1e3,
            PowerType::MicroWatt => self.v,
            PowerType::NanoWatt => self.v * 1e-3,
            PowerType::KiloWatt => self.v * 1e9,
            PowerType::MegaWatt => self.v * 1e12,
            PowerType::HorsePower => self.v * 745700000.0,
        }
    }

    pub fn as_nano_watt(&self) -> f64 {
        match self.default_type {
            PowerType::Watt => self.v * 1e9,
            PowerType::MillWatt => self.v * 1e6,
            PowerType::MicroWatt => self.v * 1e3,
            PowerType::NanoWatt => self.v,
            PowerType::KiloWatt => self.v * 1e12,
            PowerType::MegaWatt => self.v * 1e15,
            PowerType::HorsePower => self.v * 745700000000.0,
        }
    }

    pub fn as_kilo_watt(&self) -> f64 {
        match self.default_type {
            PowerType::Watt => self.v * 1e-3,
            PowerType::MillWatt => self.v * 1e-6,
            PowerType::MicroWatt => self.v * 1e-9,
            PowerType::NanoWatt => self.v * 1e-12,
            PowerType::KiloWatt => self.v,
            PowerType::MegaWatt => self.v * 1e3,
            PowerType::HorsePower => self.v * 0.7457,
        }
    }

    pub fn as_mega_watt(&self) -> f64 {
        match self.default_type {
            PowerType::Watt => self.v * 1e-6,
            PowerType::MillWatt => self.v * 1e-9,
            PowerType::MicroWatt => self.v * 1e-12,
            PowerType::NanoWatt => self.v * 1e-15,
            PowerType::KiloWatt => self.v * 1e-3,
            PowerType::MegaWatt => self.v,
            PowerType::HorsePower => self.v * 0.0007457,
        }
    }

    pub fn as_horse_power(&self) -> f64 {
        match self.default_type {
            PowerType::Watt => self.v / 745.7,
            PowerType::MillWatt => self.v / 745700.0,
            PowerType::MicroWatt => self.v / 745700000.0,
            PowerType::NanoWatt => self.v / 745700000000.0,
            PowerType::KiloWatt => self.v / 0.7457,
            PowerType::MegaWatt => self.v / 0.0007457,
            PowerType::HorsePower => self.v,
        }
    }
}

impl Add for Power {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_watt() + rhs.as_watt();
        Self::from_watt(v)
    }
}

impl Add<f64> for Power {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.as_watt() + rhs;
        Self::from_watt(v)
    }
}

impl Sub for Power {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_watt() - rhs.as_watt();
        Self::from_watt(v)
    }
}

impl Sub<f64> for Power {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.as_watt() - rhs;
        Self::from_watt(v)
    }
}

impl Mul<f64> for Power {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_watt() * rhs;
        Self::from_watt(v)
    }
}

impl Div<f64> for Power {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_watt() / rhs;
        Self::from_watt(v)
    }
}

impl Mul<Power> for f64 {
    type Output = Power;
    fn mul(self, rhs: Power) -> Self::Output {
        let v = self * rhs.as_watt();
        Power::from_watt(v)
    }
}

impl Div<Power> for f64 {
    type Output = Power;
    fn div(self, rhs: Power) -> Self::Output {
        let v = self / rhs.as_watt();
        Power::from_watt(v)
    }
}

impl Mul<Coef> for Power {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_watt() * rhs.get_value();
        Self::from_watt(v)
    }
}

impl Div<Coef> for Power {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_watt() / rhs.get_value();
        Self::from_watt(v)
    }
}

// 功率 ÷ 力 = 速度
impl Div<Force> for Power {
    type Output = Velocity;
    fn div(self, rhs: Force) -> Self::Output {
        let velocity_value = self.as_watt() / rhs.as_newton();
        Velocity::from_m_per_sec(velocity_value)
    }
}

// 功率 ÷ 速度 = 力
impl Div<Velocity> for Power {
    type Output = Force;
    fn div(self, rhs: Velocity) -> Self::Output {
        let force_value = self.as_watt() / rhs.as_m_per_sec();
        Force::from_newton(force_value)
    }
}

// 功率与时间的乘积（得到能量）
impl Mul<std::time::Duration> for Power {
    type Output = Energy; // 能量，单位：焦耳
    fn mul(self, rhs: std::time::Duration) -> Self::Output {
        let energy_value = self.as_watt() * rhs.as_secs_f64();
        Energy::from_joule(energy_value)
    }
}

// 时间与功率的乘积（得到能量，满足交换律）
impl Mul<Power> for std::time::Duration {
    type Output = Energy; // 能量，单位：焦耳
    fn mul(self, rhs: Power) -> Self::Output {
        let energy_value = self.as_secs_f64() * rhs.as_watt();
        Energy::from_joule(energy_value)
    }
}

// 力与速度的乘积（得到功率）
impl Mul<Velocity> for Force {
    type Output = Power; // 功率，单位：瓦特
    fn mul(self, rhs: Velocity) -> Self::Output {
        let power_value = self.as_newton() * rhs.as_m_per_sec();
        Power::from_watt(power_value)
    }
}

// 速度与力的乘积（得到功率，满足交换律）
impl Mul<Force> for Velocity {
    type Output = Power; // 功率，单位：瓦特
    fn mul(self, rhs: Force) -> Self::Output {
        let power_value = self.as_m_per_sec() * rhs.as_newton();
        Power::from_watt(power_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::basic::{Energy, Force, Velocity};
    use std::time::Duration;

    #[test]
    fn test_power() {
        let p1 = Power::from_watt(1.0); // 1 W
        let p2 = Power::from_mill_watt(1000.0); // 1000 mW = 1 W
        let p3 = Power::from_micro_watt(1000000.0); // 1000000 μW = 1 W
        let p4 = Power::from_nano_watt(1000000000.0); // 1000000000 nW = 1 W
        let p5 = Power::from_kilo_watt(0.001); // 0.001 kW = 1 W
        let p6 = Power::from_mega_watt(0.000001); // 0.000001 MW = 1 W
        let p7 = Power::from_horse_power(1.0 / 745.7); // 1/745.7 hp ≈ 1 W

        assert_relative_eq!(p1.as_watt(), 1.0);
        assert_relative_eq!(p2.as_watt(), 1.0);
        assert_relative_eq!(p3.as_watt(), 1.0);
        assert_relative_eq!(p4.as_watt(), 1.0);
        assert_relative_eq!(p5.as_watt(), 1.0);
        assert_relative_eq!(p6.as_watt(), 1.0);
        assert_relative_eq!(p7.as_watt(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_power_as() {
        let p = Power::from_watt(1.0);
        
        assert_relative_eq!(p.as_mill_watt(), 1000.0);
        assert_relative_eq!(p.as_micro_watt(), 1000000.0);
        assert_relative_eq!(p.as_nano_watt(), 1000000000.0);
        assert_relative_eq!(p.as_kilo_watt(), 0.001);
        assert_relative_eq!(p.as_mega_watt(), 0.000001);
        assert_relative_eq!(p.as_horse_power(), 1.0 / 745.7, epsilon = 1e-10);
    }

    #[test]
    fn test_power_add() {
        let p1 = Power::from_watt(5.0);
        let p2 = Power::from_mill_watt(3000.0); // 3 W
        let result = p1 + p2;
        
        assert_relative_eq!(result.as_watt(), 8.0);
    }

    #[test]
    fn test_power_sub() {
        let p1 = Power::from_watt(10.0);
        let p2 = Power::from_mill_watt(2000.0); // 2 W
        let result = p1 - p2;
        
        assert_relative_eq!(result.as_watt(), 8.0);
    }

    #[test]
    fn test_power_mul() {
        let p = Power::from_watt(5.0);
        let result = p * 3.0;
        
        assert_relative_eq!(result.as_watt(), 15.0);
    }

    #[test]
    fn test_power_div() {
        let p = Power::from_watt(15.0);
        let result = p / 3.0;
        
        assert_relative_eq!(result.as_watt(), 5.0);
    }

    #[test]
    fn test_power_mul_duration() {
        let power = Power::from_watt(100.0); // 100 W
        let time = Duration::from_secs(5); // 5 s
        let energy: Energy = power * time; // 500 J
        
        assert_relative_eq!(energy.as_joule(), 500.0);
    }

    #[test]
    fn test_duration_mul_power() {
        let time = Duration::from_secs(5); // 5 s
        let power = Power::from_watt(100.0); // 100 W
        let energy: Energy = time * power; // 500 J
        
        assert_relative_eq!(energy.as_joule(), 500.0);
    }

    #[test]
    fn test_force_mul_velocity() {
        let force = Force::from_newton(10.0); // 10 N
        let velocity = Velocity::from_m_per_sec(5.0); // 5 m/s
        let power: Power = force * velocity; // 50 W
        
        assert_relative_eq!(power.as_watt(), 50.0);
    }

    #[test]
    fn test_velocity_mul_force() {
        let velocity = Velocity::from_m_per_sec(5.0); // 5 m/s
        let force = Force::from_newton(10.0); // 10 N
        let power: Power = velocity * force; // 50 W
        
        assert_relative_eq!(power.as_watt(), 50.0);
    }

    #[test]
    fn test_horse_power_conversion() {
        let hp = Power::from_horse_power(1.0); // 1 hp
        assert_relative_eq!(hp.as_watt(), 745.7, epsilon = 1e-10);
        
        let watt = Power::from_watt(745.7); // 745.7 W
        assert_relative_eq!(watt.as_horse_power(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_default_unit_value() {
        let p = Power::from_watt(5.0);
        assert_relative_eq!(p.default_unit_value(), 5.0);
    }

    #[test]
    fn test_is_zero() {
        let p1 = Power::from_watt(0.0);
        let p2 = Power::from_watt(1.0);
        
        assert!(p1.is_zero());
        assert!(!p2.is_zero());
    }

    #[test]
    fn test_set_value() {
        let mut p = Power::from_watt(5.0);
        p.set_value(10.0);
        assert_relative_eq!(p.as_watt(), 10.0);
    }

    #[test]
    fn test_power_div_force() {
        let power = Power::from_watt(50.0); // 50 W
        let force = Force::from_newton(10.0); // 10 N
        let velocity: Velocity = power / force; // 5 m/s
        
        assert_relative_eq!(velocity.as_m_per_sec(), 5.0);
    }

    #[test]
    fn test_power_div_velocity() {
        let power = Power::from_watt(50.0); // 50 W
        let velocity = Velocity::from_m_per_sec(5.0); // 5 m/s
        let force: Force = power / velocity; // 10 N
        
        assert_relative_eq!(force.as_newton(), 10.0);
    }
}
