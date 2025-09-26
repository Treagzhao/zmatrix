use std::ops::{Add, Div, Mul, Neg, Sub};
use std::time::Duration;
use super::*;
const G: f64 = 9.80665;
impl Acceleration {
    pub fn from_m_per_s2(v: f64) -> Self {
        Acceleration {
            default_type: AccelerationType::MPerSecond2,
            v,
        }
    }

    pub fn from_km_per_h2(v: f64) -> Self {
        Acceleration {
            default_type: AccelerationType::KmPerHour2,
            v,
        }
    }

    pub fn from_g(v: f64) -> Self {
        Acceleration {
            default_type: AccelerationType::G,
            v,
        }
    }

    pub fn as_m_per_s2(&self) -> f64 {
        match self.default_type {
            AccelerationType::MPerSecond2 => self.v,
            AccelerationType::KmPerHour2 => self.v / 12960.0,
            AccelerationType::G => self.v * G,
        }
    }

    pub fn as_km_per_h_2(&self) -> f64 {
        match self.default_type {
            AccelerationType::MPerSecond2 => self.v * 12960.0,
            AccelerationType::KmPerHour2 => self.v,
            AccelerationType::G => self.v * 127008.0,
        }
    }

    pub fn as_g(&self) -> f64 {
        match self.default_type {
            AccelerationType::MPerSecond2 => self.v / G,
            AccelerationType::KmPerHour2 => self.v / 127008.0,
            AccelerationType::G => self.v,
        }
    }
}

impl Default for Acceleration {
    fn default() -> Self {
        Acceleration::from_m_per_s2(0.0)
    }
}

impl Mul<Duration> for Acceleration {
    type Output = Velocity;
    fn mul(self, rhs: Duration) -> Self::Output {
        let a = self.as_m_per_s2();
        let v = a * rhs.as_secs_f64();
        Velocity::from_m_per_sec(v)
    }
}

// 引用版本：Acceleration * Duration -> Velocity
impl<'a> Mul<Duration> for &'a Acceleration {
    type Output = Velocity;
    fn mul(self, rhs: Duration) -> Self::Output { Velocity::from_m_per_sec(self.as_m_per_s2() * rhs.as_secs_f64()) }
}
impl<'a> Mul<&'a Duration> for Acceleration {
    type Output = Velocity;
    fn mul(self, rhs: &'a Duration) -> Self::Output { Velocity::from_m_per_sec(self.as_m_per_s2() * rhs.as_secs_f64()) }
}
impl<'a, 'b> Mul<&'b Duration> for &'a Acceleration {
    type Output = Velocity;
    fn mul(self, rhs: &'b Duration) -> Self::Output { Velocity::from_m_per_sec(self.as_m_per_s2() * rhs.as_secs_f64()) }
}


impl Sub for Acceleration {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_m_per_s2() - rhs.as_m_per_s2();
        Self::from_m_per_s2(v)
    }
}

// 引用-引用 与 混合引用：Acceleration 减法
impl<'a, 'b> Sub<&'b Acceleration> for &'a Acceleration {
    type Output = Acceleration;
    fn sub(self, rhs: &'b Acceleration) -> Self::Output { Acceleration::from_m_per_s2(self.as_m_per_s2() - rhs.as_m_per_s2()) }
}
impl<'a> Sub<&'a Acceleration> for Acceleration {
    type Output = Acceleration;
    fn sub(self, rhs: &'a Acceleration) -> Self::Output { Acceleration::from_m_per_s2(self.as_m_per_s2() - rhs.as_m_per_s2()) }
}
impl<'a> Sub<Acceleration> for &'a Acceleration {
    type Output = Acceleration;
    fn sub(self, rhs: Acceleration) -> Self::Output { Acceleration::from_m_per_s2(self.as_m_per_s2() - rhs.as_m_per_s2()) }
}

impl Sub<f64> for Acceleration {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        Acceleration {
            v,
            default_type: self.default_type,
        }
    }
}

impl Add for Acceleration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_m_per_s2() + rhs.as_m_per_s2();
        Self::from_m_per_s2(v)
    }
}

// 引用-引用 与 混合引用：Acceleration 加法
impl<'a, 'b> Add<&'b Acceleration> for &'a Acceleration {
    type Output = Acceleration;
    fn add(self, rhs: &'b Acceleration) -> Self::Output { Acceleration::from_m_per_s2(self.as_m_per_s2() + rhs.as_m_per_s2()) }
}
impl<'a> Add<&'a Acceleration> for Acceleration {
    type Output = Acceleration;
    fn add(self, rhs: &'a Acceleration) -> Self::Output { Acceleration::from_m_per_s2(self.as_m_per_s2() + rhs.as_m_per_s2()) }
}
impl<'a> Add<Acceleration> for &'a Acceleration {
    type Output = Acceleration;
    fn add(self, rhs: Acceleration) -> Self::Output { Acceleration::from_m_per_s2(self.as_m_per_s2() + rhs.as_m_per_s2()) }
}

impl Add<f64> for Acceleration {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        Acceleration {
            v,
            default_type: self.default_type,
        }
    }
}


impl Mul<f64> for Acceleration {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_m_per_s2() * rhs;
        Self::from_m_per_s2(v)
    }
}
impl PhysicalQuantity for Acceleration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn is_zero(&self) -> bool {
        self.v == 0.0
    }
    fn default_unit_value(&self) -> f64 {
        self.as_m_per_s2()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Div<f64> for Acceleration {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_m_per_s2() / rhs;
        Self::from_m_per_s2(v)
    }
}

impl Mul<Acceleration> for f64 {
    type Output = Acceleration;
    fn mul(self, rhs: Acceleration) -> Self::Output {
        rhs * self
    }
}

impl Div<Acceleration> for f64 {
    type Output = Acceleration;
    fn div(self, rhs: Acceleration) -> Self::Output {
        let v = self / rhs.v;
        Acceleration {
            default_type: rhs.default_type,
            v: v,
        }
    }
}

impl Mul<Coef> for Acceleration {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_m_per_s2() * rhs.get_value();
        Self::from_m_per_s2(v)
    }
}

impl Div<Coef> for Acceleration {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_m_per_s2() / rhs.get_value();
        Self::from_m_per_s2(v)
    }
}

impl Neg for Acceleration {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = -self.as_m_per_s2();
        Self::from_m_per_s2(v)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_acceleration() {
        let a1 = Acceleration::from_m_per_s2(1000.0);
        assert_eq!(a1.v, 1000.0);
        assert_eq!(a1.default_type, AccelerationType::MPerSecond2);

        let a2 = Acceleration::from_km_per_h2(1000.0);
        assert_eq!(a2.v, 1000.0);
        assert_eq!(a2.default_type, AccelerationType::KmPerHour2);

        let a3 = Acceleration::from_g(1.5);
        assert_eq!(a3.v, 1.5);
        assert_eq!(a3.default_type, AccelerationType::G);

        let a3 = Acceleration::default();
        assert_eq!(a3.v, 0.0);
        assert_eq!(a3.default_type, AccelerationType::MPerSecond2);

        let a2 = Acceleration::from_km_per_h2(3600.0);
        assert_eq!(a2.default_unit_value(), 0.277777777777777777778);

        let mut a3 = Acceleration::from_g(1.5);
        a3.set_value(1.0);
        assert_eq!(a3.v, 1.0);

    }

    #[test]
    fn convert() {
        let a1 = Acceleration::from_m_per_s2(1.0);
        assert_eq!(a1.as_m_per_s2(), 1.0);
        assert_eq!(a1.as_km_per_h_2(), 12960.0);
        assert_relative_eq!(a1.as_g(),0.10197162129779283);

        let a2 = Acceleration::from_km_per_h2(3600.0);
        assert_eq!(a2.as_m_per_s2(), 0.277777777777777777778);
        assert_eq!(a2.as_km_per_h_2(), 3600.0);
        assert_eq!(a2.as_g(), 0.02834467120181406);

        let a3 = Acceleration::from_g(1.0);
        assert_eq!(a3.as_m_per_s2(), 9.80665);
        assert_eq!(a3.as_km_per_h_2(), 127008.0);
        assert_eq!(a3.as_g(), 1.0);
    }

    #[test]
    fn test_acceleration_to_velocity() {
        let a = Acceleration::from_m_per_s2(1000.0);
        let d = Duration::from_secs(1);
        let v = a * d;
        assert_eq!(v.as_m_per_sec(), 1000.0);
    }
    #[test]
    fn test_acceleration_sub() {
        {
            let a1 = Acceleration::from_m_per_s2(1000.0);
            let a2 = Acceleration::from_m_per_s2(1000.0);
            let a3 = a1 - a2;
            assert_eq!(a3.as_m_per_s2(), 0.0);

            let a1 = Acceleration::from_m_per_s2(10.0);
            let a2 = Acceleration::from_km_per_h2(100.0);
            let a3 = a1 - a2;
            assert_relative_eq!(a3.as_m_per_s2(), 9.992283950617284,epsilon = 1e-8);

            let a1 = Acceleration::from_m_per_s2(10.0);
            let a2 = Acceleration::from_g(1.0);
            let a3 = a1 - a2;
            assert_relative_eq!(a3.as_m_per_s2(), 0.19335,epsilon = 1e-8);
        }
        {
            let a1 = Acceleration::from_m_per_s2(1000.0);
            let a2 = a1 - 100.0;
            assert_eq!(a2.as_m_per_s2(), 900.0);

            let a1 = Acceleration::from_km_per_h2(1000.0);
            let a2 = a1 - 100.0;
            assert_eq!(a2.as_km_per_h_2(), 900.0);

            let a1 = Acceleration::from_g(2.0);
            let a2 = a1 - 1.5;
            assert_relative_eq!(a2.as_m_per_s2(), 4.903325,epsilon = 1e-8);
        }
    }

    #[test]
    fn test_acceleration_add() {
        {
            let a1 = Acceleration::from_m_per_s2(1000.0);
            let a2 = Acceleration::from_m_per_s2(1000.0);
            let a3 = a1 + a2;
            assert_eq!(a3.as_m_per_s2(), 2000.0);

            let a1 = Acceleration::from_m_per_s2(10.0);
            let a2 = Acceleration::from_km_per_h2(1000.0);
            let a3 = a1 + a2;
            assert_relative_eq!(a3.as_m_per_s2(),10.0771604938271604,epsilon = 1e-8);

            let a1 = Acceleration::from_km_per_h2(1000.0);
            let a2 = Acceleration::from_m_per_s2(10.0);
            let a3 = a1 + a2;
            assert_relative_eq!(a3.as_m_per_s2(),10.0771604938271604,epsilon = 1e-8);

            let a1 = Acceleration::from_m_per_s2(10.0);
            let a2 = Acceleration::from_g(1.0);
            let a3 = a1 + a2;
            assert_relative_eq!(a3.as_m_per_s2(), 19.80665,epsilon = 1e-8);
        }
        {
            let a1 = Acceleration::from_m_per_s2(1000.0);
            let a2 = a1 + 1.0;
            assert_relative_eq!(a2.as_m_per_s2(), 1001.0);

            let a1 = Acceleration::from_km_per_h2(1000.0);
            let a2 = a1 + 1.0;
            assert_relative_eq!(a2.as_km_per_h_2(),1001.0);

            let a1 = Acceleration::from_g(1.5);
            let a2 = a1 + 1.5;
            assert_relative_eq!(a2.as_g(), 3.0);
        }
    }
    #[test]
    fn test_to_mul() {
        let d1 = Acceleration::from_m_per_s2(1000.0);
        let d2 = d1 * 2.0;
        assert_eq!(d2.as_m_per_s2(), 2000.0);
        let d1 = Acceleration::from_m_per_s2(1000.0);
        let d2 = d1 * Coef::new(2.0);
        assert_eq!(d2.as_m_per_s2(), 2000.0);
    }

    #[test]
    fn test_to_div() {
        let d1 = Acceleration::from_m_per_s2(1000.0);
        let d2 = d1 / 2.0;
        assert_eq!(d2.as_m_per_s2(), 500.0);
        let d1 = Acceleration::from_m_per_s2(1000.0);
        let d2 = d1 / Coef::new(2.0);
        assert_eq!(d2.as_m_per_s2(), 500.0);
    }

    #[test]
    fn test_is_zero() {
        let d1 = Acceleration::from_m_per_s2(1000.0);
        assert!(!d1.is_zero());
        let d1 = Acceleration::from_m_per_s2(0.0);
        assert!(d1.is_zero());
    }
    #[test]
    fn test_as_any() {
        let d1 = Acceleration::from_m_per_s2(1000.0);
        let d2 = d1.as_any().downcast_ref::<Acceleration>().unwrap();
    }

    #[test]
    fn test_f64_mul_acceleration() {
        // f64 * Acceleration 测试
        let a = Acceleration::from_m_per_s2(2.0);
        let result = 3.0 * a;
        assert_relative_eq!(result.as_m_per_s2(), 6.0);

        let a = Acceleration::from_km_per_h2(1000.0);
        let result = 2.0 * a;
        assert_relative_eq!(result.as_km_per_h_2(), 2000.0);

        let a = Acceleration::from_g(1.0);
        let result = 2.0 * a;
        assert_relative_eq!(result.as_g(), 2.0);
    }

    #[test]
    fn test_f64_div_acceleration() {
        // f64 / Acceleration 测试
        let a = Acceleration::from_m_per_s2(2.0);
        let result = 6.0 / a;
        assert_relative_eq!(result.as_m_per_s2(), 3.0);

        let a = Acceleration::from_km_per_h2(1000.0);
        let result = 2000.0 / a;
        assert_relative_eq!(result.as_km_per_h_2(), 2.0);

        let a = Acceleration::from_g(2.0);
        let result = 4.0 / a;
        assert_relative_eq!(result.as_g(), 2.0);
    }

    #[test]
    fn test_acceleration_ref_ops() {
        let a1 = Acceleration::from_m_per_s2(2.0);
        let a2 = Acceleration::from_km_per_h2(12960.0); // 1 m/s2
        let s = &a1 + &a2;
        assert_relative_eq!(s.as_m_per_s2(), 3.0);

        let d = &a1 - &a2;
        assert_relative_eq!(d.as_m_per_s2(), 1.0);

        let v = &a1 * &std::time::Duration::from_secs(2);
        assert_relative_eq!(v.as_m_per_sec(), 4.0);

        // 混合引用：加/减
        let s2 = a1 + &a2;
        assert_relative_eq!(s2.as_m_per_s2(), 3.0);
        let s3 = &a1 + a2;
        assert_relative_eq!(s3.as_m_per_s2(), 3.0);
        let a2b = Acceleration::from_km_per_h2(12960.0);
        let d2 = a1 - &a2b;
        assert_relative_eq!(d2.as_m_per_s2(), 1.0);
        let d3 = &a1 - a2b;
        assert_relative_eq!(d3.as_m_per_s2(), 1.0);

        // 混合引用：* Duration
        let dur = std::time::Duration::from_secs(2);
        let v2 = a1 * &dur;
        assert_relative_eq!(v2.as_m_per_sec(), 4.0);
        let v3 = &a1 * dur;
        assert_relative_eq!(v3.as_m_per_sec(), 4.0);
    }

    #[test]
    fn test_acceleration_neg() {
        // 测试正值的负号
        let a1 = Acceleration::from_m_per_s2(5.0);
        let neg_a1 = -a1;
        assert_relative_eq!(neg_a1.as_m_per_s2(), -5.0);

        // 测试负值的负号
        let a2 = Acceleration::from_km_per_h2(-10.0);
        let neg_a2 = -a2;
        assert_relative_eq!(neg_a2.as_km_per_h_2(), 10.0);

        // 测试零值
        let a3 = Acceleration::from_g(0.0);
        let neg_a3 = -a3;
        assert_relative_eq!(neg_a3.as_g(), 0.0);

        // 测试不同单位的负号操作
        let a4 = Acceleration::from_m_per_s2(9.80665); // 1g
        let neg_a4 = -a4;
        assert_relative_eq!(neg_a4.as_g(), -1.0);

        // 测试大数值
        let a5 = Acceleration::from_km_per_h2(12960.0); // 1 m/s2
        let neg_a5 = -a5;
        assert_relative_eq!(neg_a5.as_m_per_s2(), -1.0);
    }
}