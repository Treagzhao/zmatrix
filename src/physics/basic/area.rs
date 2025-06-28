use std::ops::{Add, Div, Mul, Sub};
use super::*;
impl Area {
    pub fn from_m2(v: f64) -> Self {
        Self {
            default_type: AreaType::M2,
            v,
        }
    }

    pub fn from_km2(v: f64) -> Self {
        Self {
            default_type: AreaType::KM2,
            v,
        }
    }

    pub fn as_m2(&self) -> f64 {
        match self.default_type {
            AreaType::M2 => self.v,
            AreaType::KM2 => self.v * 1000.0 * 1000.0,
        }
    }

    pub fn as_km2(&self) -> f64 {
        match self.default_type {
            AreaType::M2 => self.v / 1000.0 / 1000.0,
            AreaType::KM2 => self.v,
        }
    }
}
impl Default for Area {
    fn default() -> Self {
        Area::from_m2(0.0)
    }
}

impl PhysicalQuantity for Area {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.v
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Div<Distance> for Area {
    type Output = Distance;

    fn div(self, rhs: Distance) -> Self::Output {
        let v = self.as_m2() / rhs.as_m();
        Distance::from_m(v)
    }
}
impl Add for Area {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_m2() + rhs.as_m2();
        Self::from_m2(v)
    }
}

impl Add<f64> for Area {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AreaType::M2 => {
                let v = self.as_m2() + rhs;
                Self::from_m2(v)
            }
            AreaType::KM2 => {
                let v = self.as_km2() + rhs;
                Self::from_km2(v)
            }
        };
    }
}

impl Sub for Area {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_m2() - rhs.as_m2();
        Self::from_m2(v)
    }
}

impl Sub<f64> for Area {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            AreaType::KM2 => {
                let v = self.as_km2() - rhs;
                Self::from_km2(v)
            }
            AreaType::M2 => {
                let v = self.as_m2() - rhs;
                Self::from_m2(v)
            }
        };
    }
}

impl Mul<f64> for Area {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_m2() * rhs;
        Self::from_m2(v)
    }
}

impl Mul<Distance> for Area {
    type Output = Volume;
    fn mul(self, rhs: Distance) -> Self::Output {
        let v = self.as_m2() * rhs.as_m();
        Volume::from_m3(v)
    }
}

impl Div<f64> for Area {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_m2() / rhs;
        Self::from_m2(v)
    }
}

impl Mul<Coef> for Area {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_m2() * rhs.get_value();
        Self::from_m2(v)
    }
}

impl Div<Area> for Area {
    type Output = Coef;
    fn div(self, rhs: Area) -> Self::Output {
        let v = self.as_m2() / rhs.as_m2();
        Coef::new(v)
    }
}

impl Div<Coef> for Area {
    type Output = Self;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_m2() / rhs.get_value();
        Self::from_m2(v)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_area_default() {
        let area = Area::default();
        assert_eq!(area.default_type, AreaType::M2);
        assert_eq!(area.v, 0.0);
    }
    #[test]
    fn test_area_from() {
        let area = Area::from_m2(0.5);
        assert_eq!(area.default_type, AreaType::M2);
        assert_eq!(area.v, 0.5);

        let area = Area::from_km2(0.5);
        assert_eq!(area.default_type, AreaType::KM2);
        assert_eq!(area.v, 0.5);
    }

    #[test]
    fn test_area_as() {
        let area = Area::from_m2(1000000.0);
        assert_eq!(area.as_km2(), 1.0);
        assert_eq!(area.as_m2(), 1000000.0);

        let area = Area::from_km2(10.0);
        assert_eq!(area.as_m2(), 10_000_000.0);
        assert_eq!(area.as_km2(), 10.0);

        let area1 = Area::from_m2(5.0);
        let area2 = Area::from_m2(2.5);
        let result = area1 / area2;
        assert_eq!(result.get_value(), 2.0);
    }

    #[test]
    fn test_convert() {
        let area = Area::from_m2(1000000.0);
        let distance = area / Distance::from_m(1000.0);
        assert_eq!(distance.as_m(), 1000.0);

        let area = Area::from_m2(200.0);
        let d = Distance::from_m(0.6);
        let result = area * d;
        assert_relative_eq!(result.as_m3(),120.0);
    }

    #[test]
    fn test_to_add() {
        let area = Area::from_m2(100.0);
        let area2 = Area::from_m2(100.0);
        let area3 = area + area2;
        assert_eq!(area3.as_m2(), 200.0);

        let area = Area::from_m2(100.0);
        let area2 = Area::from_km2(1.0);
        let area3 = area + area2;
        assert_eq!(area3.as_m2(), 1_000_100.0);

        let area = Area::from_m2(100.0);
        let area2 = area + 100.0;
        assert_eq!(area2.as_m2(), 200.0);

        let area = Area::from_km2(100.0);
        let area2 = area + 100.0;
        assert_eq!(area2.as_km2(), 200.0);
    }
    #[test]
    fn test_to_sub() {
        let area = Area::from_m2(150.0);
        let area2 = Area::from_m2(100.0);
        let area3 = area - area2;
        assert_eq!(area3.as_m2(), 50.0);

        let area = Area::from_m2(100.0);
        let area2 = Area::from_km2(1.0);
        let area3 = area2 - area;
        assert_eq!(area3.as_m2(), 999_900.0);

        let area = Area::from_m2(120.0);
        let area2 = area - 100.0;
        assert_eq!(area2.as_m2(), 20.0);

        let area = Area::from_km2(100.0);
        let area2 = area - 80.0;
        assert_eq!(area2.as_km2(), 20.0);
    }

    #[test]
    fn test_to_mul() {
        let area = Area::from_m2(100.0);
        let area2 = area * 2.0;
        assert_eq!(area2.as_m2(), 200.0);

        let area = Area::from_m2(100.0);
        let area2 = area * Coef::new(2.0);
        assert_eq!(area2.as_m2(), 200.0);
    }

    #[test]
    fn test_to_div() {
        let area = Area::from_m2(100.0);
        let area2 = area / 2.0;
        assert_eq!(area2.as_m2(), 50.0);

        let area = Area::from_m2(100.0);
        let area2 = area / Coef::new(2.0);
        assert_eq!(area2.as_m2(), 50.0);
    }

    #[test]
    fn test_as_any() {
        let area = Area::default();
        let any = area.as_any();
        let a: &Area = any.downcast_ref::<Area>().unwrap();
    }

    #[test]
    fn test_is_zero() {
        let area = Area::default();
        assert!(area.is_zero());
        let area = Area::from_m2(0.0);
        assert!(area.is_zero());
        let area = Area::from_m2(1.0);
        assert!(!area.is_zero());
    }

    #[test]
    fn test_default_unit_value() {
        let area = Area::from_m2(1.0);
        assert_eq!(area.default_unit_value(), 1.0);
    }

    #[test]
    fn test_set_value() {
        let mut area = Area::from_m2(1.0);
        area.set_value(2.0);
        assert_eq!(area.as_m2(), 2.0);
    }
}