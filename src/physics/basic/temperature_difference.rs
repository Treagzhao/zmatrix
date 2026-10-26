use std::any::Any;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::physics::basic::{Coef, PhysicalQuantity, Temperature, TemperatureDifference};

/// 华氏（及兰氏）温标与开尔文的刻度比例：1 °F 的间隔 = 5/9 K
const FAHRENHEIT_RATIO: f64 = 5.0 / 9.0;

impl Default for TemperatureDifference {
    fn default() -> Self {
        Self::from_kelvin(0.0)
    }
}

impl PhysicalQuantity for TemperatureDifference {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.as_kelvin() == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_kelvin()
    }

    fn set_value(&mut self, value: f64) {
        self.kelvin = value;
    }

    fn unit_display_name(&self) -> String {
        "TemperatureDifference (K)".to_string()
    }
}

impl TemperatureDifference {
    pub fn from_kelvin(v: f64) -> Self {
        Self { kelvin: v }
    }

    /// 摄氏温差。
    ///
    /// 注意：**`Δ°C` 与 `ΔK` 数值完全相同**，此处不做任何偏移。
    /// 这正是温差与温度的根本区别 —— `from_celsius(0.0)` 在这里表示
    /// "温差为零"，而不是"冰点"。
    pub fn from_celsius(v: f64) -> Self {
        Self { kelvin: v }
    }

    /// 华氏温差。`Δ°F` 与 `ΔK` 相差 `9/5`，无偏移。
    pub fn from_fahrenheit(v: f64) -> Self {
        Self {
            kelvin: v * FAHRENHEIT_RATIO,
        }
    }

    /// 兰氏温差。与华氏温差相同比例，无偏移。
    pub fn from_rankine(v: f64) -> Self {
        Self {
            kelvin: v * FAHRENHEIT_RATIO,
        }
    }

    pub fn as_kelvin(&self) -> f64 {
        self.kelvin
    }

    /// 摄氏温差，与开尔文数值相同。
    pub fn as_celsius(&self) -> f64 {
        self.kelvin
    }

    pub fn as_fahrenheit(&self) -> f64 {
        self.kelvin / FAHRENHEIT_RATIO
    }

    pub fn as_rankine(&self) -> f64 {
        self.kelvin / FAHRENHEIT_RATIO
    }
}

impl Add for TemperatureDifference {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_kelvin(self.as_kelvin() + rhs.as_kelvin())
    }
}

impl<'a, 'b> Add<&'b TemperatureDifference> for &'a TemperatureDifference {
    type Output = TemperatureDifference;
    fn add(self, rhs: &'b TemperatureDifference) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() + rhs.as_kelvin())
    }
}
impl<'a> Add<&'a TemperatureDifference> for TemperatureDifference {
    type Output = TemperatureDifference;
    fn add(self, rhs: &'a TemperatureDifference) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() + rhs.as_kelvin())
    }
}
impl<'a> Add<TemperatureDifference> for &'a TemperatureDifference {
    type Output = TemperatureDifference;
    fn add(self, rhs: TemperatureDifference) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() + rhs.as_kelvin())
    }
}

impl Sub for TemperatureDifference {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}

impl<'a, 'b> Sub<&'b TemperatureDifference> for &'a TemperatureDifference {
    type Output = TemperatureDifference;
    fn sub(self, rhs: &'b TemperatureDifference) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}
impl<'a> Sub<&'a TemperatureDifference> for TemperatureDifference {
    type Output = TemperatureDifference;
    fn sub(self, rhs: &'a TemperatureDifference) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}
impl<'a> Sub<TemperatureDifference> for &'a TemperatureDifference {
    type Output = TemperatureDifference;
    fn sub(self, rhs: TemperatureDifference) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}

impl Neg for TemperatureDifference {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_kelvin(-self.as_kelvin())
    }
}

// 温差 + 温度 = 温度
//
// 加法可交换，`ΔT + T` 与 `T + ΔT` 必须都成立，否则累加循环里
// 换个书写顺序就编译不过。实现在此（从温差一侧），温度一侧在
// temperature.rs 中。
impl Add<Temperature> for TemperatureDifference {
    type Output = Temperature;
    fn add(self, rhs: Temperature) -> Self::Output {
        Temperature::from_kelvin(rhs.as_kelvin() + self.as_kelvin())
    }
}

impl<'a> Add<Temperature> for &'a TemperatureDifference {
    type Output = Temperature;
    fn add(self, rhs: Temperature) -> Self::Output {
        Temperature::from_kelvin(rhs.as_kelvin() + self.as_kelvin())
    }
}
impl<'a> Add<&'a Temperature> for TemperatureDifference {
    type Output = Temperature;
    fn add(self, rhs: &'a Temperature) -> Self::Output {
        Temperature::from_kelvin(rhs.as_kelvin() + self.as_kelvin())
    }
}
impl<'a, 'b> Add<&'b Temperature> for &'a TemperatureDifference {
    type Output = Temperature;
    fn add(self, rhs: &'b Temperature) -> Self::Output {
        Temperature::from_kelvin(rhs.as_kelvin() + self.as_kelvin())
    }
}

// 温差可以按纯数缩放（与温度不同，这里没有偏移问题）
impl Mul<f64> for TemperatureDifference {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_kelvin(self.as_kelvin() * rhs)
    }
}

impl Div<f64> for TemperatureDifference {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_kelvin(self.as_kelvin() / rhs)
    }
}

impl Mul<TemperatureDifference> for f64 {
    type Output = TemperatureDifference;
    fn mul(self, rhs: TemperatureDifference) -> Self::Output {
        TemperatureDifference::from_kelvin(self * rhs.as_kelvin())
    }
}

impl Mul<Coef> for TemperatureDifference {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        Self::from_kelvin(self.as_kelvin() * rhs.get_value())
    }
}

impl Div<Coef> for TemperatureDifference {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        Self::from_kelvin(self.as_kelvin() / rhs.get_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_celsius_equals_kelvin() {
        // 温差的摄氏与开尔文数值完全相同（无偏移）
        let dt = TemperatureDifference::from_celsius(20.0);
        assert_relative_eq!(dt.as_kelvin(), 20.0, epsilon = 1e-12);
        assert_relative_eq!(dt.as_celsius(), 20.0, epsilon = 1e-12);

        // 温度则不同：20 °C = 293.15 K
        // 这个对比是两者必须分型的直接证据
        let t = crate::physics::basic::Temperature::from_celsius(20.0);
        assert_relative_eq!(t.as_kelvin(), 293.15, epsilon = 1e-9);
        assert_ne!(dt.as_kelvin(), t.as_kelvin());
    }

    #[test]
    fn test_fahrenheit_difference() {
        // 1 °C 的温升 = 1.8 °F 的温升
        let dt = TemperatureDifference::from_celsius(1.0);
        assert_relative_eq!(dt.as_fahrenheit(), 1.8, epsilon = 1e-9);

        let dt = TemperatureDifference::from_celsius(10.0);
        assert_relative_eq!(dt.as_fahrenheit(), 18.0, epsilon = 1e-9);

        // 反向
        let df = TemperatureDifference::from_fahrenheit(18.0);
        assert_relative_eq!(df.as_celsius(), 10.0, epsilon = 1e-9);

        // 零温差不带偏移：0 °F 温差就是 0 K
        assert_relative_eq!(TemperatureDifference::from_fahrenheit(0.0).as_kelvin(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn test_round_trip() {
        for v in [-30.0, 0.0, 5.0, 100.0] {
            let dt = TemperatureDifference::from_celsius(v);
            assert_relative_eq!(dt.as_celsius(), v, epsilon = 1e-9);
            assert_relative_eq!(
                TemperatureDifference::from_fahrenheit(dt.as_fahrenheit()).as_celsius(),
                v,
                epsilon = 1e-9
            );
            assert_relative_eq!(
                TemperatureDifference::from_rankine(dt.as_rankine()).as_celsius(),
                v,
                epsilon = 1e-9
            );
        }
    }

    #[test]
    fn test_arith() {
        let a = TemperatureDifference::from_celsius(20.0);
        let b = TemperatureDifference::from_celsius(5.0);

        // 温差累加
        assert_relative_eq!((a + b).as_celsius(), 25.0, epsilon = 1e-12);
        assert_relative_eq!((a - b).as_celsius(), 15.0, epsilon = 1e-12);

        // 缩放
        assert_relative_eq!((a * 2.0).as_celsius(), 40.0, epsilon = 1e-12);
        assert_relative_eq!((a / 2.0).as_celsius(), 10.0, epsilon = 1e-12);
        assert_relative_eq!((2.0 * a).as_celsius(), 40.0, epsilon = 1e-12);
        assert_relative_eq!((a * Coef::new(0.5)).as_celsius(), 10.0, epsilon = 1e-12);
        assert_relative_eq!((a / Coef::new(0.5)).as_celsius(), 40.0, epsilon = 1e-12);

        // 取负：冷却
        assert_relative_eq!((-a).as_celsius(), -20.0, epsilon = 1e-12);

        // 引用变体
        assert_relative_eq!((&a + &b).as_celsius(), 25.0, epsilon = 1e-12);
        assert_relative_eq!((a + &b).as_celsius(), 25.0, epsilon = 1e-12);
        assert_relative_eq!((&a + b).as_celsius(), 25.0, epsilon = 1e-12);
        assert_relative_eq!((&a - &b).as_celsius(), 15.0, epsilon = 1e-12);
        assert_relative_eq!((a - &b).as_celsius(), 15.0, epsilon = 1e-12);
        assert_relative_eq!((&a - b).as_celsius(), 15.0, epsilon = 1e-12);
    }

    #[test]
    fn test_mixed_units_arith() {
        // 摄氏温差 + 华氏温差：内部统一 K 后正确
        let dc = TemperatureDifference::from_celsius(10.0); // 10 K
        let df = TemperatureDifference::from_fahrenheit(9.0); // 5 K
        let sum = dc + df;
        assert_relative_eq!(sum.as_kelvin(), 15.0, epsilon = 1e-9);
        assert_relative_eq!(sum.as_celsius(), 15.0, epsilon = 1e-9);
        assert_relative_eq!(sum.as_fahrenheit(), 27.0, epsilon = 1e-9);
    }

    #[test]
    fn test_add_to_temperature_is_commutative() {
        let t = Temperature::from_celsius(20.0);
        let dt = TemperatureDifference::from_celsius(5.0);

        // 两种书写顺序必须给同一结果
        let a = t + dt;
        let b = dt + t;
        assert_relative_eq!(a.as_celsius(), 25.0, epsilon = 1e-9);
        assert_relative_eq!(b.as_celsius(), 25.0, epsilon = 1e-9);
        assert_relative_eq!(a.as_celsius(), b.as_celsius(), epsilon = 1e-12);

        // 引用变体
        assert_relative_eq!((&dt + t).as_celsius(), 25.0, epsilon = 1e-9);
        assert_relative_eq!((dt + &t).as_celsius(), 25.0, epsilon = 1e-9);
        assert_relative_eq!((&dt + &t).as_celsius(), 25.0, epsilon = 1e-9);
    }

    #[test]
    fn test_physical_quantity_contract() {
        let mut dt = TemperatureDifference::default();
        assert!(dt.is_zero());

        dt.set_value(12.0);
        assert_relative_eq!(dt.default_unit_value(), 12.0, epsilon = 1e-12);
        assert_relative_eq!(dt.as_kelvin(), 12.0, epsilon = 1e-12);
        assert!(!dt.is_zero());

        assert_eq!(dt.unit_display_name(), "TemperatureDifference (K)");

        let any = dt.as_any();
        assert!(any.downcast_ref::<TemperatureDifference>().is_some());
    }

    #[test]
    fn test_ordering_and_eq() {
        let small = TemperatureDifference::from_celsius(1.0);
        let large = TemperatureDifference::from_celsius(10.0);
        assert!(small < large);
        assert_eq!(
            TemperatureDifference::from_fahrenheit(9.0),
            TemperatureDifference::from_celsius(5.0)
        );
    }
}
