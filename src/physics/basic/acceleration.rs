use std::ops::{Add, Div, Mul, Sub};
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

impl Mul<Duration> for Acceleration {
    type Output = Velocity;
    fn mul(self, rhs: Duration) -> Self::Output {
        let a = self.as_m_per_s2();
        let v = a * rhs.as_secs_f64();
        Velocity::from_m_per_sec(v)
    }
}


impl Sub for Acceleration {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        println!("{} {}", self.as_m_per_s2(), rhs.as_m_per_s2());
        let v = self.as_m_per_s2() - rhs.as_m_per_s2();
        Self::from_m_per_s2(v)
    }
}

impl Sub<f64> for Acceleration {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AccelerationType::MPerSecond2 => {
                let v = self.as_m_per_s2() - rhs;
                Self::from_m_per_s2(v)
            }
            AccelerationType::KmPerHour2 => {
                let v = self.as_km_per_h_2() - rhs;
                Self::from_km_per_h2(v)
            }
            AccelerationType::G => {
                let v = self.as_g() - rhs;
                Self::from_g(v)
            }
        };
    }
}

impl Add for Acceleration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_m_per_s2() + rhs.as_m_per_s2();
        Self::from_m_per_s2(v)
    }
}

impl Add<f64> for Acceleration {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AccelerationType::MPerSecond2 => {
                let v = self.as_m_per_s2() + rhs;
                Self::from_m_per_s2(v)
            }
            AccelerationType::KmPerHour2 => {
                let v = self.as_km_per_h_2() + rhs;
                Self::from_km_per_h2(v)
            }
            AccelerationType::G => {
                let v = self.as_g() + rhs;
                Self::from_g(v)
            }
        };
    }
}


impl Mul<f64> for Acceleration {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_m_per_s2() * rhs;
        Self::from_m_per_s2(v)
    }
}

impl Div<f64> for Acceleration {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_m_per_s2() / rhs;
        Self::from_m_per_s2(v)
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
}