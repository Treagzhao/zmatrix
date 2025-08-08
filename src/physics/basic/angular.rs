use super::*;
use std::f64::consts::{PI, TAU};
use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;

impl Angular {
    pub fn from_rad(rad: f64) -> Self {
        Self {
            default_type: AngularType::Rad,
            v: rad,
        }
    }

    pub fn from_deg(deg: f64) -> Self {
        Self {
            default_type: AngularType::Deg,
            v: deg,
        }
    }

    pub fn as_rad(&self) -> f64 {
        return match self.default_type {
            AngularType::Rad => self.v,
            AngularType::Deg => self.v * PI / 180.0,
        };
    }

    pub fn as_deg(&self) -> f64 {
        return match self.default_type {
            AngularType::Rad => self.v * 180.0 / PI,
            AngularType::Deg => self.v,
        };
    }

    pub fn sin(&self) -> f64 {
        self.as_rad().sin()
    }

    pub fn tan(&self) -> f64 {
        self.as_rad().tan()
    }

    pub fn cos(&self) -> f64 {
        self.as_rad().cos()
    }
    // 输出的结果在-PI ~ PI之间
    pub fn mod_to_round_half(&self) -> Self {
        let x = self.as_rad();
        let period = 2.0 * PI;
        Angular::from_rad(x - ((x + PI) / period).floor() * period)
    }
    //   输出在   0 ~ 2PI  之间
    pub fn mod_to_round(&self) -> Self {
        let v = self.as_rad() - TAU * f64::floor(self.as_rad() / TAU);
        Self::from_rad(v)
    }

    pub fn atan2(a: f64, b: f64) -> Self {
        let v = f64::atan2(a, b);
        Self::from_rad(v)
    }

    pub fn atan(a: f64) -> Self {
        let v = f64::atan(a);
        Self::from_rad(v)
    }

    pub fn asin(v: f64) -> Self {
        let rad = f64::asin(v);
        Self::from_rad(rad)
    }

    pub fn acos(v:f64) ->   Self{
        let rad = f64::acos(v);
        Self::from_rad(rad)
    }
}

impl From<Coef> for Angular {
    // 角度也可以看做一个无量纲数，因为角度可以看做是弧长跟半径的比值，所以它们是可以相互转化的。
    fn from(value: Coef) -> Self {
        Angular::from_rad(value.get_value())
    }
}

impl Add for Angular {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_deg() + rhs.as_deg();
        Angular::from_deg(v)
    }
}

impl Add<f64> for Angular {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AngularType::Rad => Angular::from_rad(self.v + rhs),
            AngularType::Deg => Angular::from_deg(self.v + rhs),
        };
    }
}

impl Sub for Angular {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_deg() - rhs.as_deg();
        Angular::from_deg(v)
    }
}

impl Sub<f64> for Angular {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AngularType::Rad => Angular::from_rad(self.v - rhs),
            AngularType::Deg => Angular::from_deg(self.v - rhs),
        };
    }
}

impl Div<Duration> for Angular {
    type Output = AngularVelocity;
    fn div(self, rhs: Duration) -> Self::Output {
        let v = self.as_rad() / rhs.as_secs_f64();
        AngularVelocity::from_rad_per_second(v)
    }
}

impl Mul<f64> for Angular {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_rad() * rhs;
        Self::from_rad(v)
    }
}

impl Div<f64> for Angular {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_rad() / rhs;
        Self::from_rad(v)
    }
}

impl Mul<Angular> for f64 {
    type Output = Angular;
    fn mul(self, rhs: Angular) -> Self::Output {
        rhs * self
    }
}

impl Div<Angular> for f64 {
    type Output = Angular;
    fn div(self, rhs: Angular) -> Self::Output {
        let v = self / rhs.as_rad();
        Angular::from_rad(v)
    }
}

impl Mul<Coef> for Angular {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_rad() * rhs.get_value();
        Self::from_rad(v)
    }
}

impl Div<Coef> for Angular {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_rad() / rhs.get_value();
        Self::from_rad(v)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_angular() {
        let angular = Angular::from_rad(1.0);
        assert_eq!(angular.v, 1.0);
        assert_eq!(angular.default_type, AngularType::Rad);

        let angular = Angular::from_rad(6.15 * PI);
        assert_eq!(angular.default_type, AngularType::Rad);
        assert_relative_eq!(angular.v, 6.15 *PI, epsilon = 1.0e-6);

        let angular = Angular::from_deg(1.0);
        assert_eq!(angular.default_type, AngularType::Deg);
        assert_eq!(angular.v, 1.0);
    }
    #[test]
    fn test_angular_convert() {
        let angular = Angular::from_deg(1.0);
        assert_relative_eq!(angular.as_deg(),1.0);
        assert_relative_eq!(angular.as_rad(),0.0174532925,epsilon = 1e-8);

        let angular = Angular::from_rad(2.0 * PI);
        assert_relative_eq!(angular.as_deg(),360.0);
        assert_relative_eq!(angular.as_rad(),2.0*PI,epsilon = 1e-8);

        let c: Coef = Coef::new(0.5 * PI);
        let angular: Angular = Angular::from(c);
        assert_relative_eq!(angular.as_deg(),90.0);
    }

    #[test]
    fn test_angular_sin_cos() {
        let angular = Angular::from_deg(30.0);
        assert_relative_eq!(angular.sin(),0.5,epsilon = 1e-8);
        assert_relative_eq!(angular.cos(),0.866025403784,epsilon = 1e-8);

        let angular = Angular::from_rad(PI / 6.0);
        assert_relative_eq!(angular.sin(),0.5,epsilon = 1e-8);
        assert_relative_eq!(angular.cos(),0.866025403784,epsilon = 1e-8);

        let angular = Angular::from_rad(PI / 4.0);
        assert_relative_eq!(angular.tan(),1.0,epsilon = 1e-8);
    }

    #[test]
    fn test_angular_add() {
        {
            let theta1 = Angular::from_rad(PI * 0.5);
            let theta2 = Angular::from_rad(PI * 0.5);
            let theta3 = theta1 + theta2;
            assert_relative_eq!(PI,theta3.as_rad(),epsilon = 1e-8);

            let theta1 = Angular::from_deg(30.0);
            let theta2 = Angular::from_rad(PI / 6.0);
            let theta3 = theta1 + theta2;
            assert_relative_eq!(60.0,theta3.as_deg(),epsilon = 1e-8);

            let theta1 = Angular::from_rad(PI / 6.0);
            let theta2 = Angular::from_deg(30.0);
            let theta3 = theta1 + theta2;
            assert_relative_eq!(60.0,theta3.as_deg(),epsilon = 1e-8);

            let theta1 = Angular::from_deg(30.0);
            let theta2 = Angular::from_deg(90.0);
            let theta3 = theta1 + theta2;
            assert_relative_eq!(2.0*PI / 3.0,theta3.as_rad(),epsilon = 1e-8);
        }
        {
            let theta1 = Angular::from_deg(30.0);
            let theta2 = theta1 + 30.0;
            assert_relative_eq!(60.0,theta2.as_deg(),epsilon = 1e-8);

            let theta1 = Angular::from_rad(PI * 0.25);
            let theta2 = theta1 + PI * 0.25;
            assert_relative_eq!(90.0,theta2.as_deg(),epsilon = 1e-8);
        }
    }

    #[test]
    fn test_angular_sub() {
        {
            let theta1 = Angular::from_rad(PI * 1.0);
            let theta2 = Angular::from_rad(PI * 0.5);
            let theta3 = theta1 - theta2;
            assert_relative_eq!(PI*0.5,theta3.as_rad(),epsilon = 1e-8);

            let theta1 = Angular::from_deg(60.0);
            let theta2 = Angular::from_rad(PI / 6.0);
            let theta3 = theta1 - theta2;
            assert_relative_eq!(30.0,theta3.as_deg(),epsilon = 1e-8);

            let theta1 = Angular::from_rad(PI / 6.0);
            let theta2 = Angular::from_deg(15.0);
            let theta3 = theta1 - theta2;
            assert_relative_eq!(15.0,theta3.as_deg(),epsilon = 1e-8);

            let theta1 = Angular::from_deg(90.0);
            let theta2 = Angular::from_deg(30.0);
            let theta3 = theta1 - theta2;
            assert_relative_eq!(PI / 3.0,theta3.as_rad(),epsilon = 1e-8);
        }
        {
            let theta1 = Angular::from_deg(30.0);
            let theta2 = theta1 - 10.0;
            assert_relative_eq!(20.0,theta2.as_deg(),epsilon = 1e-8);

            let theta1 = Angular::from_rad(PI * 0.5);
            let theta2 = theta1 - PI * 0.25;
            assert_relative_eq!(45.0,theta2.as_deg(),epsilon = 1e-8);
        }
    }

    #[test]
    fn test_angular_mod_round() {
        let theta1 = Angular::from_deg(372.0);
        let theta2 = theta1.mod_to_round_half();
        assert_relative_eq!(12.0, theta2.as_deg(),epsilon = 1e-8);

        let theta1 = Angular::from_deg(315.0);
        let theta2 = theta1.mod_to_round_half();
        assert_relative_eq!(-1.0 * PI / 4.0, theta2.as_rad(),epsilon = 1e-8);

        let theta1 = Angular::from_deg(372.0);
        let theta2 = theta1.mod_to_round();
        assert_relative_eq!(12.0, theta2.as_deg(),epsilon = 1e-8);

        let theta1 = Angular::from_deg(315.0);
        let theta2 = theta1.mod_to_round();
        assert_relative_eq!(315.0, theta2.as_deg(),epsilon = 1e-8);
    }

    #[test]
    fn test_angular_to_angular_velocity() {
        let theta1 = Angular::from_rad(PI * 0.5);
        let omg1 = theta1 / Duration::from_secs(2);
        assert_eq!(omg1.default_type, AngularVelocityType::RadperSecond);
        assert_eq!(omg1.v, PI / 4.0);
    }

    #[test]
    fn test_to_mul() {
        let d1 = Angular::from_rad(1000.0);
        let d2 = d1 * 2.0;
        assert_eq!(d2.as_rad(), 2000.0);
        let d1 = Angular::from_rad(1000.0);
        let d2 = d1 * Coef::new(2.0);
        assert_eq!(d2.as_rad(), 2000.0);
    }

    #[test]
    fn test_to_div() {
        let d1 = Angular::from_rad(1000.0);
        let d2 = d1 / 2.0;
        assert_eq!(d2.as_rad(), 500.0);
        let d1 = Angular::from_rad(1000.0);
        let d2 = d1 / Coef::new(2.0);
        assert_eq!(d2.as_rad(), 500.0);
    }

    #[test]
    fn test_atan() {
        let sin = 0.5;
        let cos = 0.8660254037844386;
        let theta = Angular::atan2(sin, cos);
        assert_relative_eq!(theta.as_deg(), 30.0,epsilon = 1e-8);

        let value = 1.0f64;
        let theta = Angular::atan(value);
        assert_relative_eq!(theta.as_deg(), 45.0,epsilon = 1e-8);

        let value = 0.5f64;
        let theta = Angular::asin(value);
        assert_relative_eq!(theta.as_deg(), 30.0,epsilon = 1e-8);
    }

    #[test]
    fn test_acos() {
        // Test normal cases
        let cases = vec![
            (1.0, 0.0),   // acos(1) = 0 radians
            (0.5, PI/3.0), // acos(0.5) = π/3 radians (60 degrees)
            (0.0, PI/2.0), // acos(0) = π/2 radians (90 degrees)
            (-0.5, 2.0*PI/3.0), // acos(-0.5) = 2π/3 radians (120 degrees)
            (-1.0, PI),    // acos(-1) = π radians (180 degrees)
        ];

        for (input, expected_rad) in cases {
            let angle = Angular::acos(input);
            assert_relative_eq!(angle.as_rad(), expected_rad, epsilon = 1e-8);
            // Verify the conversion to degrees works as expected
            assert_relative_eq!(angle.as_deg(), expected_rad * 180.0 / PI, epsilon = 1e-8);
        }

        // Test edge cases
        {
            // Test input exactly at 1.0
            let angle = Angular::acos(1.0);
            assert_relative_eq!(angle.as_rad(), 0.0, epsilon = 1e-8);
        }
        {
            // Test input exactly at -1.0
            let angle = Angular::acos(-1.0);
            assert_relative_eq!(angle.as_rad(), PI, epsilon = 1e-8);
        }

        // Test invalid inputs (should panic as per f64::acos behavior)
        let invalid_cases = vec![1.00001, -1.00001, 2.0, -2.0, f64::NAN, f64::INFINITY];
        for invalid_input in invalid_cases {
            println!("{:?}",invalid_input);
            let angle = Angular::acos(invalid_input);
            assert_eq!(true,angle.v.is_nan());
        }

        // Test precision cases (values very close to boundaries)
        let epsilon = 1e-10;
        {
            let angle = Angular::acos(1.0 - epsilon);
            assert!(angle.as_rad() > 0.0 && angle.as_rad() < 1.5e-5);
        }
        {
            let angle = Angular::acos(-1.0 + epsilon);
            assert!(angle.as_rad() > PI - 1.5e-5 && angle.as_rad() < PI);
        }
    }

    #[test]
    fn test_f64_mul_angular() {
        // f64 * Angular 测试
        let a = Angular::from_rad(PI);
        let result = 2.0 * a;
        assert_relative_eq!(result.as_rad(), 2.0 * PI);

        let a = Angular::from_deg(90.0);
        let result = 2.0 * a;
        assert_relative_eq!(result.as_deg(), 180.0);
    }

    #[test]
    fn test_f64_div_angular() {
        // f64 / Angular 测试
        let a = Angular::from_rad(PI);
        let result = 2.0 * PI / a;
        assert_relative_eq!(result.as_rad(), 2.0);

        let a = Angular::from_deg(180.0);
        let result = 180.0 / a;
        assert_relative_eq!(result.as_deg(), 1.0);
    }
}