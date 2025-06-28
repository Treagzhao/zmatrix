use super::*;
use std::f64::consts::PI;
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

    pub fn cos(&self) -> f64 {
        self.as_rad().cos()
    }

    pub fn mod_to_round(&self) -> Self {
        let v = self.as_deg();
        let v = v % 360.0;
        Self::from_deg(v)
    }
}

impl Add for Angular {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut v = self.as_deg() + rhs.as_deg();
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
        let mut v = self.as_deg() - rhs.as_deg();
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
        let mut v = self.as_rad() / rhs.as_secs_f64();
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
    }

    #[test]
    fn test_angular_sin_cos() {
        let angular = Angular::from_deg(30.0);
        assert_relative_eq!(angular.sin(),0.5,epsilon = 1e-8);
        assert_relative_eq!(angular.cos(),0.866025403784,epsilon = 1e-8);

        let angular = Angular::from_rad(PI / 6.0);
        assert_relative_eq!(angular.sin(),0.5,epsilon = 1e-8);
        assert_relative_eq!(angular.cos(),0.866025403784,epsilon = 1e-8);
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
        let theta2 = theta1.mod_to_round();
        assert_eq!(12.0, theta2.as_deg());
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
}