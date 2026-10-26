use std::any::Any;
use std::ops::{Add, Div, Sub};
use crate::physics::basic::{Coef, PhysicalQuantity, Temperature, TemperatureDifference};

/// 摄氏温标与开尔文之间的偏移量：`T_K = T_C + 273.15`
const KELVIN_OFFSET: f64 = 273.15;

/// 华氏（及兰氏）温标与开尔文的刻度比例：1 °F 的间隔 = 5/9 K
const FAHRENHEIT_RATIO: f64 = 5.0 / 9.0;

impl Default for Temperature {
    /// 默认值为绝对零度 0 K。
    fn default() -> Self {
        Self::from_kelvin(0.0)
    }
}

impl PhysicalQuantity for Temperature {
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// 只有绝对零度才算"零"。
    ///
    /// 不能沿用库内通用的 `self.v == 0.0` 写法：`0 °C` 内部是 `273.15 K`，
    /// 而 `0 K` 才是真正的零。这里判的是开尔文值。
    fn is_zero(&self) -> bool {
        self.as_kelvin() == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_kelvin()
    }

    /// 与 [`default_unit_value`](Self::default_unit_value) 对称：写入的是开尔文值。
    fn set_value(&mut self, value: f64) {
        self.kelvin = value;
    }

    fn unit_display_name(&self) -> String {
        "Temperature (K)".to_string()
    }
}

impl Temperature {
    pub fn from_kelvin(v: f64) -> Self {
        Self { kelvin: v }
    }

    pub fn from_celsius(v: f64) -> Self {
        Self {
            kelvin: v + KELVIN_OFFSET,
        }
    }

    pub fn from_fahrenheit(v: f64) -> Self {
        Self {
            kelvin: (v - 32.0) * FAHRENHEIT_RATIO + KELVIN_OFFSET,
        }
    }

    /// 兰氏温标（°R），绝对温标，1 °R = 5/9 K。航空领域偶见使用。
    pub fn from_rankine(v: f64) -> Self {
        Self {
            kelvin: v * FAHRENHEIT_RATIO,
        }
    }

    pub fn as_kelvin(&self) -> f64 {
        self.kelvin
    }

    pub fn as_celsius(&self) -> f64 {
        self.kelvin - KELVIN_OFFSET
    }

    pub fn as_fahrenheit(&self) -> f64 {
        (self.kelvin - KELVIN_OFFSET) / FAHRENHEIT_RATIO + 32.0
    }

    pub fn as_rankine(&self) -> f64 {
        self.kelvin / FAHRENHEIT_RATIO
    }
}

// 温度 - 温度 = 温差
//
// 这是唯一有意义的温度减法。返回 TemperatureDifference 而非 Temperature，
// 使得结果只能参与温差允许的运算（可与热导相乘等），不会退化成绝对温度。
impl Sub for Temperature {
    type Output = TemperatureDifference;
    fn sub(self, rhs: Self) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}

impl<'a, 'b> Sub<&'b Temperature> for &'a Temperature {
    type Output = TemperatureDifference;
    fn sub(self, rhs: &'b Temperature) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}
impl<'a> Sub<&'a Temperature> for Temperature {
    type Output = TemperatureDifference;
    fn sub(self, rhs: &'a Temperature) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}
impl<'a> Sub<Temperature> for &'a Temperature {
    type Output = TemperatureDifference;
    fn sub(self, rhs: Temperature) -> Self::Output {
        TemperatureDifference::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}

// 温度 + 温差 = 温度（加热）
impl Add<TemperatureDifference> for Temperature {
    type Output = Temperature;
    fn add(self, rhs: TemperatureDifference) -> Self::Output {
        Temperature::from_kelvin(self.as_kelvin() + rhs.as_kelvin())
    }
}

impl<'a> Add<TemperatureDifference> for &'a Temperature {
    type Output = Temperature;
    fn add(self, rhs: TemperatureDifference) -> Self::Output {
        Temperature::from_kelvin(self.as_kelvin() + rhs.as_kelvin())
    }
}
impl<'a> Add<&'a TemperatureDifference> for Temperature {
    type Output = Temperature;
    fn add(self, rhs: &'a TemperatureDifference) -> Self::Output {
        Temperature::from_kelvin(self.as_kelvin() + rhs.as_kelvin())
    }
}
impl<'a, 'b> Add<&'b TemperatureDifference> for &'a Temperature {
    type Output = Temperature;
    fn add(self, rhs: &'b TemperatureDifference) -> Self::Output {
        Temperature::from_kelvin(self.as_kelvin() + rhs.as_kelvin())
    }
}

// 温度 - 温差 = 温度（冷却）
impl Sub<TemperatureDifference> for Temperature {
    type Output = Temperature;
    fn sub(self, rhs: TemperatureDifference) -> Self::Output {
        Temperature::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}

impl<'a> Sub<TemperatureDifference> for &'a Temperature {
    type Output = Temperature;
    fn sub(self, rhs: TemperatureDifference) -> Self::Output {
        Temperature::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}
impl<'a> Sub<&'a TemperatureDifference> for Temperature {
    type Output = Temperature;
    fn sub(self, rhs: &'a TemperatureDifference) -> Self::Output {
        Temperature::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}
impl<'a, 'b> Sub<&'b TemperatureDifference> for &'a Temperature {
    type Output = Temperature;
    fn sub(self, rhs: &'b TemperatureDifference) -> Self::Output {
        Temperature::from_kelvin(self.as_kelvin() - rhs.as_kelvin())
    }
}

// 温度 / 温度 = 无量纲比（如 Carnot 效率 1 - Tc/Th）
//
// 必须用开尔文：摄氏温度相除没有意义（20/10 ≠ 293.15/283.15）。
impl Div for Temperature {
    type Output = Coef;
    fn div(self, rhs: Self) -> Self::Output {
        Coef::new(self.as_kelvin() / rhs.as_kelvin())
    }
}

impl<'a, 'b> Div<&'b Temperature> for &'a Temperature {
    type Output = Coef;
    fn div(self, rhs: &'b Temperature) -> Self::Output {
        Coef::new(self.as_kelvin() / rhs.as_kelvin())
    }
}
impl<'a> Div<&'a Temperature> for Temperature {
    type Output = Coef;
    fn div(self, rhs: &'a Temperature) -> Self::Output {
        Coef::new(self.as_kelvin() / rhs.as_kelvin())
    }
}
impl<'a> Div<Temperature> for &'a Temperature {
    type Output = Coef;
    fn div(self, rhs: Temperature) -> Self::Output {
        Coef::new(self.as_kelvin() / rhs.as_kelvin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_construction_and_units() {
        // 水的冰点
        let t = Temperature::from_celsius(0.0);
        assert_relative_eq!(t.as_kelvin(), 273.15, epsilon = 1e-9);
        assert_relative_eq!(t.as_fahrenheit(), 32.0, epsilon = 1e-9);

        // 水的沸点
        let t = Temperature::from_celsius(100.0);
        assert_relative_eq!(t.as_kelvin(), 373.15, epsilon = 1e-9);
        assert_relative_eq!(t.as_fahrenheit(), 212.0, epsilon = 1e-9);

        // 绝对零度
        let t = Temperature::from_celsius(-273.15);
        assert_relative_eq!(t.as_kelvin(), 0.0, epsilon = 1e-9);
        assert_relative_eq!(t.as_rankine(), 0.0, epsilon = 1e-9);

        // 兰氏：1 °R = 5/9 K
        let t = Temperature::from_rankine(491.67);
        assert_relative_eq!(t.as_kelvin(), 273.15, epsilon = 1e-9);
        assert_relative_eq!(
            Temperature::from_kelvin(273.15).as_rankine(),
            491.67,
            epsilon = 1e-9
        );
    }

    #[test]
    fn test_round_trip() {
        // 任意单位构造后，转一圈回到原值
        for v in [-40.0, 0.0, 20.0, 37.0, 100.0] {
            let t = Temperature::from_celsius(v);
            assert_relative_eq!(t.as_celsius(), v, epsilon = 1e-9);

            let t2 = Temperature::from_fahrenheit(t.as_fahrenheit());
            assert_relative_eq!(t2.as_celsius(), v, epsilon = 1e-9);

            let t3 = Temperature::from_kelvin(t.as_kelvin());
            assert_relative_eq!(t3.as_celsius(), v, epsilon = 1e-9);
        }

        // -40 是摄氏与华氏的唯一交点
        let t = Temperature::from_celsius(-40.0);
        assert_relative_eq!(t.as_fahrenheit(), -40.0, epsilon = 1e-9);
    }

    #[test]
    fn test_temperature_minus_temperature() {
        let t1 = Temperature::from_celsius(30.0);
        let t2 = Temperature::from_celsius(20.0);
        let dt = t1 - t2;
        assert_relative_eq!(dt.as_kelvin(), 10.0, epsilon = 1e-9);

        // 不同单位构造也能正确相减（内部统一 K）
        let t3 = Temperature::from_fahrenheit(50.0); // 10 °C
        let dt2 = t1 - t3;
        assert_relative_eq!(dt2.as_kelvin(), 20.0, epsilon = 1e-9);

        // 负温差：冷却
        let dt3 = t2 - t1;
        assert_relative_eq!(dt3.as_kelvin(), -10.0, epsilon = 1e-9);

        // 引用变体
        assert_relative_eq!((&t1 - &t2).as_kelvin(), 10.0, epsilon = 1e-9);
        assert_relative_eq!((t1 - &t2).as_kelvin(), 10.0, epsilon = 1e-9);
        assert_relative_eq!((&t1 - t2).as_kelvin(), 10.0, epsilon = 1e-9);
    }

    #[test]
    fn test_add_and_sub_difference() {
        let t = Temperature::from_celsius(20.0);
        let dt = TemperatureDifference::from_kelvin(15.0);

        // 加热
        let heated = t + dt;
        assert_relative_eq!(heated.as_celsius(), 35.0, epsilon = 1e-9);

        // 冷却
        let cooled = t - dt;
        assert_relative_eq!(cooled.as_celsius(), 5.0, epsilon = 1e-9);

        // 引用变体
        assert_relative_eq!((&t + dt).as_celsius(), 35.0, epsilon = 1e-9);
        assert_relative_eq!((t + &dt).as_celsius(), 35.0, epsilon = 1e-9);
        assert_relative_eq!((&t + &dt).as_celsius(), 35.0, epsilon = 1e-9);
        assert_relative_eq!((&t - dt).as_celsius(), 5.0, epsilon = 1e-9);
        assert_relative_eq!((t - &dt).as_celsius(), 5.0, epsilon = 1e-9);
        assert_relative_eq!((&t - &dt).as_celsius(), 5.0, epsilon = 1e-9);

        // 往返：加热再冷却回到原值
        assert_relative_eq!(((t + dt) - dt).as_celsius(), 20.0, epsilon = 1e-9);
    }

    #[test]
    fn test_ratio_uses_kelvin() {
        // Carnot 效率：η = 1 - Tc/Th
        let th = Temperature::from_celsius(500.0); // 773.15 K
        let tc = Temperature::from_celsius(27.0);  // 300.15 K

        let ratio = tc / th;
        assert_relative_eq!(ratio.get_value(), 300.15 / 773.15, epsilon = 1e-12);
        let efficiency = 1.0 - ratio.get_value();
        assert!(efficiency > 0.6 && efficiency < 0.62);

        // 关键：必须用开尔文比，摄氏比是错的
        assert_ne!(
            ratio.get_value(),
            27.0 / 500.0,
            "温度比必须基于开尔文，摄氏直接相除无效"
        );

        // 引用变体
        assert_relative_eq!((&tc / &th).get_value(), ratio.get_value(), epsilon = 1e-12);
        assert_relative_eq!((tc / &th).get_value(), ratio.get_value(), epsilon = 1e-12);
        assert_relative_eq!((&tc / th).get_value(), ratio.get_value(), epsilon = 1e-12);
    }

    #[test]
    fn test_is_zero_only_at_absolute_zero() {
        // 绝对零度才是零
        assert!(Temperature::from_kelvin(0.0).is_zero());

        // 0 °C 不是零 —— 通用实现会误判，这里必须为 false
        assert!(!Temperature::from_celsius(0.0).is_zero());

        // 0 °F 也不是零
        assert!(!Temperature::from_fahrenheit(0.0).is_zero());
    }

    #[test]
    fn test_physical_quantity_contract() {
        let mut t = Temperature::default();
        assert_relative_eq!(t.as_kelvin(), 0.0, epsilon = 1e-12);

        // set_value 与 default_unit_value 必须对称（都是开尔文）
        t.set_value(300.0);
        assert_relative_eq!(t.default_unit_value(), 300.0, epsilon = 1e-12);
        assert_relative_eq!(t.as_kelvin(), 300.0, epsilon = 1e-12);
        assert_relative_eq!(t.as_celsius(), 26.85, epsilon = 1e-9);

        assert_eq!(t.unit_display_name(), "Temperature (K)");

        // as_any 向下转型
        let any = t.as_any();
        assert!(any.downcast_ref::<Temperature>().is_some());
    }

    #[test]
    fn test_ordering() {
        let cold = Temperature::from_celsius(-10.0);
        let warm = Temperature::from_celsius(30.0);
        assert!(cold < warm);
        assert!(warm > cold);
        assert_eq!(
            Temperature::from_celsius(20.0),
            Temperature::from_celsius(20.0)
        );
    }
}
