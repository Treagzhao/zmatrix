use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;
use super::*;

const LIGHT_SPEED: f64 = 299792458.0;
const LIGHT_YEAR_TO_METER: f64 = 9460730472580800.0;

impl Distance {
    pub fn from_m(v: f64) -> Distance {
        Distance {
            default_type: DistanceType::M,
            v,
        }
    }

    pub fn from_km(v: f64) -> Distance {
        Distance {
            default_type: DistanceType::KM,
            v,
        }
    }

    pub fn from_light_year(v: f64) -> Distance {
        Distance {
            default_type: DistanceType::LightYear,
            v,
        }
    }

    pub fn as_m(&self) -> f64 {
        match self.default_type {
            DistanceType::M => self.v,
            DistanceType::KM => self.v * 1000.0,
            DistanceType::LightYear => self.v * LIGHT_YEAR_TO_METER,
        }
    }

    pub fn as_km(&self) -> f64 {
        match self.default_type {
            DistanceType::M => self.v / 1000.0,
            DistanceType::KM => self.v,
            DistanceType::LightYear => self.v * LIGHT_YEAR_TO_METER / 1000.0,
        }
    }

    pub fn as_light_year(&self) -> f64 {
        match self.default_type {
            DistanceType::M => self.v / LIGHT_YEAR_TO_METER,
            DistanceType::KM => self.v / LIGHT_YEAR_TO_METER * 1000.0,
            DistanceType::LightYear => self.v,
        }
    }
}

impl Div<Duration> for Distance {
    type Output = Velocity;

    fn div(self, rhs: Duration) -> Self::Output {
        let v = self.as_m() / rhs.as_secs_f64();
        Velocity::from_m_per_sec(v)
    }
}
impl Add for Distance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_m() + rhs.as_m();
        Self::from_m(v)
    }
}

// 引用-引用 与 混合引用：Distance 加法
impl<'a, 'b> Add<&'b Distance> for &'a Distance {
    type Output = Distance;
    fn add(self, rhs: &'b Distance) -> Self::Output { Distance::from_m(self.as_m() + rhs.as_m()) }
}
impl<'a> Add<&'a Distance> for Distance {
    type Output = Distance;
    fn add(self, rhs: &'a Distance) -> Self::Output { Distance::from_m(self.as_m() + rhs.as_m()) }
}
impl<'a> Add<Distance> for &'a Distance {
    type Output = Distance;
    fn add(self, rhs: Distance) -> Self::Output { Distance::from_m(self.as_m() + rhs.as_m()) }
}

impl Add<f64> for Distance {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        Distance {
            v,
            default_type: self.default_type,
        }
    }
}

impl Sub for Distance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_m() - rhs.as_m();
        Self::from_m(v)
    }
}

// 引用-引用 与 混合引用：Distance 减法
impl<'a, 'b> Sub<&'b Distance> for &'a Distance {
    type Output = Distance;
    fn sub(self, rhs: &'b Distance) -> Self::Output { Distance::from_m(self.as_m() - rhs.as_m()) }
}
impl<'a> Sub<&'a Distance> for Distance {
    type Output = Distance;
    fn sub(self, rhs: &'a Distance) -> Self::Output { Distance::from_m(self.as_m() - rhs.as_m()) }
}
impl<'a> Sub<Distance> for &'a Distance {
    type Output = Distance;
    fn sub(self, rhs: Distance) -> Self::Output { Distance::from_m(self.as_m() - rhs.as_m()) }
}

impl Sub<f64> for Distance {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        Distance {
            v,
            default_type: self.default_type,
        }
    }
}

impl Mul<f64> for Distance {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_m() * rhs;
        Self::from_m(v)
    }
}

impl Mul<Area> for Distance {
    type Output = Volume;
    fn mul(self, rhs: Area) -> Self::Output {
        let v = self.as_m() * rhs.as_m2();
        Volume::from_m3(v)
    }
}

impl Mul for Distance {
    type Output = Area;
    fn mul(self, rhs: Self) -> Self::Output {
        let v = self.as_m() * rhs.as_m();
        Area::from_m2(v)
    }
}

// 引用-引用 与 混合引用：Distance * Distance -> Area
impl<'a, 'b> Mul<&'b Distance> for &'a Distance {
    type Output = Area;
    fn mul(self, rhs: &'b Distance) -> Self::Output { Area::from_m2(self.as_m() * rhs.as_m()) }
}
impl<'a> Mul<&'a Distance> for Distance {
    type Output = Area;
    fn mul(self, rhs: &'a Distance) -> Self::Output { Area::from_m2(self.as_m() * rhs.as_m()) }
}
impl<'a> Mul<Distance> for &'a Distance {
    type Output = Area;
    fn mul(self, rhs: Distance) -> Self::Output { Area::from_m2(self.as_m() * rhs.as_m()) }
}
impl Div<f64> for Distance {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_m() / rhs;
        Self::from_m(v)
    }
}

impl Mul<Distance> for f64 {
    type Output = Distance;
    fn mul(self, rhs: Distance) -> Self::Output {
        rhs * self
    }
}

impl Div<Distance> for f64 {
    type Output = Distance;
    fn div(self, rhs: Distance) -> Self::Output {
        let v = self / rhs.v;
        Distance {
            default_type: rhs.default_type,
            v: v,
        }
    }
}

impl Mul<Coef> for Distance {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_m() * rhs.get_value();
        Self::from_m(v)
    }
}

impl Div<Coef> for Distance {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_m() / rhs.get_value();
        Self::from_m(v)
    }
}

impl Mul<Momentum> for Distance {
    type Output = AngularMomentum;

    fn mul(self, rhs: Momentum) -> Self::Output {
        let v = self.default_unit_value() * rhs.default_unit_value();
        AngularMomentum::from_kg_m2_per_second(v)
    }
}

impl Div for Distance {
    type Output = Coef;
    fn div(self, rhs: Self) -> Self::Output {
        let v = self.as_m() / rhs.as_m();
        Coef::new(v)
    }
}

// 引用-引用 与 混合引用：Distance / Distance -> Coef
impl<'a, 'b> Div<&'b Distance> for &'a Distance {
    type Output = Coef;
    fn div(self, rhs: &'b Distance) -> Self::Output { Coef::new(self.as_m() / rhs.as_m()) }
}
impl<'a> Div<&'a Distance> for Distance {
    type Output = Coef;
    fn div(self, rhs: &'a Distance) -> Self::Output { Coef::new(self.as_m() / rhs.as_m()) }
}
impl<'a> Div<Distance> for &'a Distance {
    type Output = Coef;
    fn div(self, rhs: Distance) -> Self::Output { Coef::new(self.as_m() / rhs.as_m()) }
}

// 距离 ÷ 速度 = 时间
impl Div<Velocity> for Distance {
    type Output = std::time::Duration;
    fn div(self, rhs: Velocity) -> Self::Output {
        let time_value = self.as_m() / rhs.as_m_per_sec();
        std::time::Duration::from_secs_f64(time_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_distance() {
        let d1 = Distance::from_m(1000.0);
        assert_eq!(d1.v, 1000.0);
        assert_eq!(d1.default_type, DistanceType::M);
        let d2 = Distance::from_km(1000.0);
        assert_eq!(d2.v, 1000.0);
        assert_eq!(d2.default_type, DistanceType::KM);
    }

    #[test]
    fn test_distance_as() {
        let d1 = Distance::from_km(1000.0);
        assert_eq!(d1.as_km(), 1000.0);
        assert_eq!(d1.as_m(), 1000.0 * 1000.0);
        assert_relative_eq!(d1.as_light_year(), 1.0570008340246155 * 1e-10);

        let d2 = Distance::from_m(1000.0);
        assert_eq!(d2.as_m(), 1000.0);
        assert_eq!(d2.as_km(), 1.0);
        assert_relative_eq!(d2.as_light_year(), 1.0570008340246155 * 1e-13);

        let d3 = Distance::from_light_year(1.0);
        assert_eq!(d3.as_m(), 9_460_730_472_580_800.0);
        assert_eq!(d3.as_km(), 9_460_730_472_580_800.0 / 1000.0);
    }

    #[test]
    fn test_to_velocity() {
        let d1 = Distance::from_m(1000.0);
        let duration = Duration::from_secs(5);
        let v = d1 / duration;
        assert_eq!(v.as_m_per_sec(), 200.0);

        let d1 = Distance::from_km(1200.0);
        let duration = Duration::from_secs(60 * 60 * 24);
        let v = d1 / duration;
        assert_eq!(v.as_km_per_h(), 50.0);
    }

    #[test]
    fn test_change() {
        let d1 = Distance::from_m(2.0);
        let d2 = Distance::from_m(4.0);
        let a = d1 * d2;
        assert_eq!(a.default_type, AreaType::M2);
        assert_eq!(a.as_m2(), 8.0);

        let d1 = Distance::from_m(20.0);
        let m1 = Momentum::from_kg_m_s(10.0);
        let a1 = d1 * m1;
        assert_eq!(a1.default_type, AngularMomentumType::KgM2perSecond);
        assert_eq!(a1.as_kg_m2_per_second(), 200.0);

        let d1 = Distance::from_m(20.0);
        let a1 = Area::from_m2(10.0);
        let d3 = d1 * a1;
        assert_eq!(d3.as_m3(), 200.0);
    }

    #[test]
    fn test_to_add() {
        {
            let d1 = Distance::from_m(1000.0);
            let d2: Distance = Distance::from_m(1000.0);
            let d3 = d1 + d2;
            assert_eq!(d3.as_m(), 2000.0);

            let d1 = Distance::from_km(2000.0);
            let d2: Distance = Distance::from_m(1000.0);
            let d3 = d1 + d2;
            assert_eq!(d3.as_km(), 2001.0);

            let d1 = Distance::from_m(1000.0);
            let d2: Distance = Distance::from_km(3000.0);
            let d3 = d1 + d2;
            assert_eq!(d3.as_km(), 3001.0);
            let d1 = Distance::from_km(300.0);
            let d2 = Distance::from_km(100.0);
            let d3 = d1 + d2;
            assert_eq!(d3.as_m(), 400000.0);

            let d1 = Distance::from_light_year(1.8);
            let d2: Distance = Distance::from_light_year(1.1);
            let d3 = d1 + d2;
            assert_eq!(d3.as_light_year(), 2.9);
        }
        {
            let d1 = Distance::from_m(1000.0);
            let d2 = d1 + 1000.0;
            assert_eq!(d2.as_m(), 2000.0);

            let d1 = Distance::from_km(1000.0);
            let d2 = d1 + 1000.0;
            assert_eq!(d2.as_km(), 2000.0);

            let d1 = Distance::from_light_year(1.8);
            let d2 = d1 + 1.9;
            assert_eq!(d2.as_light_year(), 3.7);
        }
    }

    #[test]
    fn test_to_sub() {
        {
            let d1 = Distance::from_m(1000.0);
            let d2: Distance = Distance::from_m(1000.0);
            let d3 = d1 - d2;
            assert_eq!(d3.as_m(), 0.0);

            let d1 = Distance::from_km(1000.0);
            let d2: Distance = Distance::from_m(1000.0);
            let d3 = d1 - d2;
            assert_eq!(d3.as_m(), 999000.0);

            let d1 = Distance::from_km(1000.0);
            let d2: Distance = Distance::from_km(1000.0);
            let d3 = d1 - d2;
            assert_eq!(d3.as_m(), 0.0);
        }
        {
            let d1 = Distance::from_m(1000.0);
            let d2 = d1 - 1000.0;
            assert_eq!(d2.as_m(), 0.0);

            let d1 = Distance::from_km(2000.0);
            let d2 = d1 - 1000.0;
            assert_eq!(d2.as_km(), 1000.0);

            let d1 = Distance::from_light_year(1.8);
            let d2 = d1 - 0.9;
            assert_eq!(d2.as_light_year(), 0.9);
        }
    }

    #[test]
    fn test_to_mul() {
        let d1 = Distance::from_m(1000.0);
        let d2 = d1 * 2.0;
        assert_eq!(d2.as_m(), 2000.0);
        let d1 = Distance::from_m(1000.0);
        let d2 = d1 * Coef::new(2.0);
        assert_eq!(d2.as_m(), 2000.0);
    }

    #[test]
    fn test_to_div() {
        let d1 = Distance::from_m(1000.0);
        let d2 = d1 / 2.0;
        assert_eq!(d2.as_m(), 500.0);
        let d1 = Distance::from_m(1000.0);
        let d2 = d1 / Coef::new(2.0);
        assert_eq!(d2.as_m(), 500.0);

        let d1 = Distance::from_m(1000.0);
        let d2 = Distance::from_m(200.0);
        let c = d1 / d2;
        assert_eq!(c.get_value(), 5.0);
    }

    #[test]
    fn test_f64_mul_distance() {
        // f64 * Distance 测试
        let d = Distance::from_m(2.0);
        let result = 3.0 * d;
        assert_relative_eq!(result.as_m(), 6.0);

        let d = Distance::from_km(1.0);
        let result = 2.0 * d;
        assert_relative_eq!(result.as_km(), 2.0);

        let d = Distance::from_light_year(0.5);
        let result = 2.0 * d;
        assert_relative_eq!(result.as_light_year(), 1.0);
    }

    #[test]
    fn test_f64_div_distance() {
        // f64 / Distance 测试
        let d = Distance::from_m(2.0);
        let result = 6.0 / d;
        assert_relative_eq!(result.as_m(), 3.0);

        let d = Distance::from_km(1.0);
        let result = 2.0 / d;
        assert_relative_eq!(result.as_km(), 2.0);

        let d = Distance::from_light_year(0.5);
        let result = 1.0 / d;
        assert_relative_eq!(result.as_light_year(), 2.0);
    }

    #[test]
    fn test_distance_div_velocity() {
        let distance = Distance::from_m(100.0); // 100 m
        let velocity = Velocity::from_m_per_sec(20.0); // 20 m/s
        let time = distance / velocity; // 5 s
        
        assert_relative_eq!(time.as_secs_f64(), 5.0);
    }

    #[test]
    fn test_distance_ref_ops() {
        let d1 = Distance::from_m(2.0);
        let d2 = Distance::from_km(1.0); // 1000 m
        let s = &d1 + &d2;
        assert_relative_eq!(s.as_m(), 1002.0);

        let sub = &d2 - &d1;
        assert_relative_eq!(sub.as_m(), 998.0);

        let area = &d1 * &d2;
        assert_relative_eq!(area.as_m2(), 2000.0);

        let coef = &d2 / &d1;
        assert_relative_eq!(coef.get_value(), 500.0);

        // 混合引用：加/减/乘/除
        let s2 = d1 + &d2;
        assert_relative_eq!(s2.as_m(), 1002.0);
        let s3 = &d1 + d2;
        assert_relative_eq!(s3.as_m(), 1002.0);

        let d2b = Distance::from_km(1.0);
        let sub2 = d2b - &d1;
        assert_relative_eq!(sub2.as_m(), 998.0);
        let sub3 = &d2b - d1;
        assert_relative_eq!(sub3.as_m(), 998.0);

        let d1b = Distance::from_m(2.0);
        let a2 = d1b * &Distance::from_km(1.0);
        assert_relative_eq!(a2.as_m2(), 2000.0);
        let a3 = &d1b * Distance::from_km(1.0);
        assert_relative_eq!(a3.as_m2(), 2000.0);

        let c2 = d2b / &Distance::from_m(2.0);
        assert_relative_eq!(c2.get_value(), 500.0);
        let c3 = &d2b / Distance::from_m(2.0);
        assert_relative_eq!(c3.get_value(), 500.0);
    }
}