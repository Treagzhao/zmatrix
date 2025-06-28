use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;
use super::*;
impl Coef {
    pub fn new(v: f64) -> Self {
        Coef { v }
    }
    pub fn get_value(&self) -> f64 {
        self.v
    }
}

impl Add for Coef {
    type Output = Coef;
    fn add(self, rhs: Self) -> Self::Output {
        Coef { v: self.v + rhs.v }
    }
}

impl Sub for Coef {
    type Output = Coef;
    fn sub(self, rhs: Self) -> Self::Output {
        Coef { v: self.v - rhs.v }
    }
}

impl Mul for Coef {
    type Output = Coef;
    fn mul(self, rhs: Self) -> Self::Output {
        Coef { v: self.v * rhs.v }
    }
}

impl Sub<Coef> for f64{
    type Output = Coef;
    fn sub(self, rhs: Coef) -> Self::Output {
        Coef { v: self - rhs.v }
    }
}

impl Mul<Duration> for Coef {
    type Output = Coef;

    fn mul(self, rhs: Duration) -> Self::Output {
        let v = self.get_value() * rhs.as_secs_f64();
        Coef { v }
    }
}

impl Div for Coef {
    type Output = Coef;
    fn div(self, rhs: Self) -> Self::Output {
        Coef { v: self.v / rhs.v }
    }
}

impl Add<f64> for Coef {
    type Output = Coef;
    fn add(self, rhs: f64) -> Self::Output {
        Coef { v: self.v + rhs }
    }
}

impl Sub<f64> for Coef {
    type Output = Coef;
    fn sub(self, rhs: f64) -> Self::Output {
        Coef { v: self.v - rhs }
    }
}

impl Mul<f64> for Coef {
    type Output = Coef;
    fn mul(self, rhs: f64) -> Self::Output {
        Coef { v: self.v * rhs }
    }
}

impl Div<f64> for Coef {
    type Output = Coef;
    fn div(self, rhs: f64) -> Self::Output {
        Coef { v: self.v / rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let coef = Coef::new(1.0);
        assert_eq!(coef.v, 1.0);
    }

    #[test]
    fn test_get_value() {
        let coef = Coef::new(1.0);
        assert_eq!(coef.get_value(), 1.0);
    }

    #[test]
    fn test_calculate_coef() {
        let coef = Coef::new(1.0);
        let result = coef + Coef::new(2.0);
        assert_eq!(result.get_value(), 3.0);

        let coef = Coef::new(2.0);
        let result = coef + 2.0;
        assert_eq!(result.get_value(), 4.0);

        let coef = Coef::new(2.0);
        let result = coef - Coef::new(3.0);
        assert_eq!(result.get_value(), -1.0);

        let coef = Coef::new(2.0);
        let result = coef - 1.0;
        assert_eq!(result.get_value(), 1.0);

        let coef = Coef::new(2.0);
        let result = coef * Coef::new(3.0);
        assert_eq!(result.get_value(), 6.0);

        let coef = Coef::new(2.0);
        let result = coef * 1.5;
        assert_eq!(result.get_value(), 3.0);

        let coef = Coef::new(2.0);
        let result = coef / Coef::new(0.5);
        assert_eq!(result.get_value(), 4.0);

        let coef = Coef::new(2.0);
        let result = coef / Coef::new(0.5);

        assert_eq!(result.get_value(), 4.0);
    }

    #[test]
    fn test_add() {
        let coef = Coef::new(1.0);
        let result = coef + Coef::new(2.0);
        assert_eq!(result.get_value(), 3.0);

        let coef = Coef::new(2.0);
        let result = coef + 2.0;
        assert_eq!(result.get_value(), 4.0);
    }
    #[test]
    fn test_sub() {
        let coef = Coef::new(1.0);
        let result = coef - Coef::new(2.0);
        assert_eq!(result.get_value(), -1.0);

        let coef = Coef::new(2.0);
        let result = coef - 6.0;
        assert_eq!(result.get_value(), -4.0);

        let coef = Coef::new(1.0);
        let result = 2.0 - coef;
        assert_eq!(result.get_value(), 1.0);
    }

    #[test]
    fn test_mul() {
        let coef = Coef::new(1.0);
        let result = coef * Coef::new(2.0);
        assert_eq!(result.get_value(), 2.0);

        let coef = Coef::new(2.0);
        let result = coef * 3.0;
        assert_eq!(result.get_value(), 6.0);

        let coef = Coef::new(2.0);
        let result = coef * Duration::from_secs(10);
        assert_eq!(result.get_value(), 20.0);
    }

    #[test]
    fn test_div() {
        let coef = Coef::new(1.0);
        let result = coef / Coef::new(2.0);
        assert_eq!(result.get_value(), 0.5);

        let coef = Coef::new(2.0);
        let result = coef / 4.0;
        assert_eq!(result.get_value(), 0.5);
    }
}