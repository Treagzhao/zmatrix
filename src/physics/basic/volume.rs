use std::any::Any;
use std::ops::{Add, Div, Mul, Sub};
use crate::physics::basic::{Area, Coef, Distance, DistanceType, PhysicalQuantity, Volume, VolumeType};

impl Default for Volume {
    fn default() -> Self {
        Volume::from_m3(0.0)
    }
}

impl PhysicalQuantity for Volume {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_m3()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Volume {
    pub fn from_m3(value: f64) -> Self {
        Self {
            default_type: VolumeType::M3,
            v: value,
        }
    }

    pub fn from_km3(value: f64) -> Self {
        Self {
            default_type: VolumeType::KM3,
            v: value,
        }
    }

    pub fn as_m3(&self) -> f64 {
        return match self.default_type {
            VolumeType::M3 => self.v,
            VolumeType::KM3 => self.v * 1e9,
        };
    }

    pub fn as_km3(&self) -> f64 {
        return match self.default_type {
            VolumeType::M3 => self.v * 1e-9,
            VolumeType::KM3 => self.v,
        };
    }
}

impl Add for Volume {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let v = self.as_m3() + rhs.as_m3();
        Self::from_m3(v)
    }
}

impl Add<f64> for Volume {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            VolumeType::M3 => {
                let v = self.as_m3() + rhs;
                Self::from_m3(v)
            }
            VolumeType::KM3 => {
                let v = self.as_km3() + rhs;
                Self::from_km3(v)
            }
        };
    }
}

impl Sub for Volume {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_m3() - rhs.as_m3();
        Self::from_m3(v)
    }
}

impl Sub<f64> for Volume {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        return match self.default_type {
            VolumeType::M3 => {
                let v = self.as_m3() - rhs;
                Self::from_m3(v)
            }
            VolumeType::KM3 => {
                let v = self.as_km3() - rhs;
                Self::from_km3(v)
            }
        };
    }
}

impl Mul<f64> for Volume {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_m3() * rhs;
        Self::from_m3(v)
    }
}

impl Mul for Volume {
    type Output = Area;
    fn mul(self, rhs: Self) -> Self::Output {
        let v = self.as_m3() * rhs.as_m3();
        Area::from_m2(v)
    }
}

impl Div for Volume {
    type Output = Coef;
    fn div(self, rhs: Self) -> Self::Output {
        let v = self.as_m3() / rhs.as_m3();
        Coef::new(v)
    }
}


impl Div<f64> for Volume {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let v = self.as_m3() / rhs;
        Self::from_m3(v)
    }
}

impl Mul<Volume> for f64 {
    type Output = Volume;
    fn mul(self, rhs: Volume) -> Self::Output {
        rhs * self
    }
}

impl Div<Volume> for f64 {
    type Output = Volume;
    fn div(self, rhs: Volume) -> Self::Output {
        let v = self / rhs.as_m3();
        Volume::from_m3(v)
    }
}

impl Div<Coef> for Volume {
    type Output = Volume;
    fn div(self, rhs: Coef) -> Self::Output {
        let v = self.as_m3() / rhs.get_value();
        Self::from_m3(v)
    }
}

impl Div<Distance> for Volume {
    type Output = Area;

    fn div(self, rhs: Distance) -> Self::Output {
        let v = self.as_m3() / rhs.as_m();
        Area::from_m2(v)
    }
}

impl Mul<Coef> for Volume {
    type Output = Self;
    fn mul(self, rhs: Coef) -> Self::Output {
        let v = self.as_m3() * rhs.get_value();
        Self::from_m3(v)
    }
}


#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_volume() {
        let volume = Volume::from_m3(1.0);
        assert_eq!(volume.as_m3(), 1.0);
        assert_eq!(volume.as_km3(), 1e-9);

        let volume = Volume::from_km3(1.0);
        assert_eq!(volume.as_m3(), 1e9);
        assert_eq!(volume.as_km3(), 1.0);
    }

    #[test]
    fn test_default() {
        let v = Volume::default();
        assert_eq!(v.as_m3(), 0.0);
        assert!(v.is_zero());

        let b: Box<dyn PhysicalQuantity> = Box::new(Volume::from_m3(10.0));
        let v = b.as_any().downcast_ref::<Volume>().unwrap();

        let mut v = Volume::from_km3(1.0);
        assert_eq!(v.default_unit_value(), 1e9);

        v.set_value(0.9);
        assert_eq!(v.v, 0.9);
    }

    #[test]
    fn test_to_add() {
        {
            let d1 = Volume::from_m3(1000.0);
            let d2 = Volume::from_m3(1000.0);
            let d3 = d1 + d2;
            assert_eq!(d3.as_m3(), 2000.0);

            let d1 = Volume::from_km3(2000.0);
            let d2 = Volume::from_m3(1000.0);
            let d3 = d1 + d2;
            assert_eq!(d3.as_km3(), 2000.000001);

            let d1 = Volume::from_m3(1000.0);
            let d2 = Volume::from_km3(3000.0);
            let d3 = d1 + d2;
            assert_relative_eq!(d3.as_m3(), 3000000001000.0);
            let d1 = Volume::from_km3(3.0);
            let d2 = Volume::from_km3(1.0);
            let d3 = d1 + d2;
            assert_eq!(d3.as_m3(), 4000000000.0);
        }
        {
            let d1 = Volume::from_m3(1000.0);
            let d2 = d1 + 1000.0;
            assert_eq!(d2.as_m3(), 2000.0);

            let d1 = Volume::from_km3(1000.0);
            let d2 = d1 + 1000.0;
            assert_eq!(d2.as_km3(), 2000.0);
        }
    }

    #[test]
    fn test_to_sub() {
        {
            let d1 = Volume::from_m3(1000.0);
            let d2 = Volume::from_m3(1000.0);
            let d3 = d1 - d2;
            assert_eq!(d3.as_m3(), 0.0);

            let d1 = Volume::from_km3(1.0);
            let d2 = Volume::from_m3(1000.0);
            let d3 = d1 - d2;
            assert_eq!(d3.as_m3(), 999999000.0);

            let d1 = Volume::from_km3(1000.0);
            let d2 = Volume::from_km3(1000.0);
            let d3 = d1 - d2;
            assert_eq!(d3.as_m3(), 0.0);
        }
        {
            let d1 = Volume::from_m3(1000.0);
            let d2 = d1 - 1000.0;
            assert_eq!(d2.as_m3(), 0.0);

            let d1 = Volume::from_km3(2000.0);
            let d2 = d1 - 1000.0;
            assert_eq!(d2.as_km3(), 1000.0);
        }
    }

    #[test]
    fn test_to_mul() {
        let d1 = Volume::from_m3(1000.0);
        let d2 = d1 * 2.0;
        assert_eq!(d2.as_m3(), 2000.0);
        let d1 = Volume::from_m3(1000.0);
        let d2 = d1 * Coef::new(2.0);
        assert_eq!(d2.as_m3(), 2000.0);
    }

    #[test]
    fn test_to_div() {
        let d1 = Volume::from_m3(1000.0);
        let d2 = d1 / 2.0;
        assert_eq!(d2.as_m3(), 500.0);
        let d1 = Volume::from_m3(1000.0);
        let d2 = d1 / Coef::new(2.0);
        assert_eq!(d2.as_m3(), 500.0);

        let d1 = Volume::from_m3(1000.0);
        let d2 = Volume::from_m3(200.0);
        let c = d1 / d2;
        assert_eq!(c.get_value(), 5.0);
    }

    #[test]
    fn test_convert() {
        let v = Volume::from_m3(1000.0);
        let d = Distance::from_m(10.0);
        let a = v / d;
        assert_relative_eq!(a.as_m2(),100.0);
    }

    #[test]
    fn test_f64_mul_volume() {
        // f64 * Volume 测试
        let v = Volume::from_m3(2.0);
        let result = 3.0 * v;
        assert_relative_eq!(result.as_m3(), 6.0);

        let v = Volume::from_km3(1.0);
        let result = 2.0 * v;
        assert_relative_eq!(result.as_km3(), 2.0);
    }

    #[test]
    fn test_f64_div_volume() {
        // f64 / Volume 测试
        let v = Volume::from_m3(2.0);
        let result = 6.0 / v;
        assert_relative_eq!(result.as_m3(), 3.0);

        let v = Volume::from_km3(1.0);
        let result = 2.0 / v;
        assert_relative_eq!(result.as_km3(), 2.0);
    }
}