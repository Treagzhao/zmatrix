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

// 引用-引用 与 混合引用：Volume 加法
impl<'a, 'b> Add<&'b Volume> for &'a Volume {
    type Output = Volume;
    fn add(self, rhs: &'b Volume) -> Self::Output { Volume::from_m3(self.as_m3() + rhs.as_m3()) }
}
impl<'a> Add<&'a Volume> for Volume {
    type Output = Volume;
    fn add(self, rhs: &'a Volume) -> Self::Output { Volume::from_m3(self.as_m3() + rhs.as_m3()) }
}
impl<'a> Add<Volume> for &'a Volume {
    type Output = Volume;
    fn add(self, rhs: Volume) -> Self::Output { Volume::from_m3(self.as_m3() + rhs.as_m3()) }
}

impl Add<f64> for Volume {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let v = self.v + rhs;
        Volume {
            v,
            default_type: self.default_type,
        }
    }
}

impl Sub for Volume {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.as_m3() - rhs.as_m3();
        Self::from_m3(v)
    }
}

// 引用-引用 与 混合引用：Volume 减法
impl<'a, 'b> Sub<&'b Volume> for &'a Volume {
    type Output = Volume;
    fn sub(self, rhs: &'b Volume) -> Self::Output { Volume::from_m3(self.as_m3() - rhs.as_m3()) }
}
impl<'a> Sub<&'a Volume> for Volume {
    type Output = Volume;
    fn sub(self, rhs: &'a Volume) -> Self::Output { Volume::from_m3(self.as_m3() - rhs.as_m3()) }
}
impl<'a> Sub<Volume> for &'a Volume {
    type Output = Volume;
    fn sub(self, rhs: Volume) -> Self::Output { Volume::from_m3(self.as_m3() - rhs.as_m3()) }
}

impl Sub<f64> for Volume {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let v = self.v - rhs;
        Volume {
            v,
            default_type: self.default_type,
        }
    }
}

impl Mul<f64> for Volume {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.as_m3() * rhs;
        Self::from_m3(v)
    }
}



impl Div for Volume {
    type Output = Coef;
    fn div(self, rhs: Self) -> Self::Output {
        let v = self.as_m3() / rhs.as_m3();
        Coef::new(v)
    }
}

// 引用版本：Volume / Volume -> Coef
impl<'a, 'b> Div<&'b Volume> for &'a Volume {
    type Output = Coef;
    fn div(self, rhs: &'b Volume) -> Self::Output { Coef::new(self.as_m3() / rhs.as_m3()) }
}
impl<'a> Div<&'a Volume> for Volume {
    type Output = Coef;
    fn div(self, rhs: &'a Volume) -> Self::Output { Coef::new(self.as_m3() / rhs.as_m3()) }
}
impl<'a> Div<Volume> for &'a Volume {
    type Output = Coef;
    fn div(self, rhs: Volume) -> Self::Output { Coef::new(self.as_m3() / rhs.as_m3()) }
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
        let v = self / rhs.v;
        Volume {
            default_type: rhs.default_type,
            v: v,
        }
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

// 引用版本：Volume / Distance -> Area
impl<'a, 'b> Div<&'b Distance> for &'a Volume {
    type Output = Area;
    fn div(self, rhs: &'b Distance) -> Self::Output { Area::from_m2(self.as_m3() / rhs.as_m()) }
}
impl<'a> Div<&'a Distance> for Volume {
    type Output = Area;
    fn div(self, rhs: &'a Distance) -> Self::Output { Area::from_m2(self.as_m3() / rhs.as_m()) }
}
impl<'a> Div<Distance> for &'a Volume {
    type Output = Area;
    fn div(self, rhs: Distance) -> Self::Output { Area::from_m2(self.as_m3() / rhs.as_m()) }
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

    #[test]
    fn test_volume_ref_ops() {
        let v = Volume::from_m3(8.0);
        let w = Volume::from_km3(1.0); // 1e9 m3
        let s = &v + &w;
        assert_relative_eq!(s.as_m3(), 1_000_000_008.0);

        let d = &w - &v;
        assert_relative_eq!(d.as_m3(), 999_999_992.0);

        let a = &w / &Distance::from_m(1000.0);
        assert_relative_eq!(a.as_m2(), 1_000_000.0);

        // 混合引用覆盖：Volume +/- &Volume, &Volume +/- Volume, &Volume / Distance, Volume / &Distance
        let _ = v + &w;
        let _ = &v + w;
        let _ = v - &w;
        let _ = &v - w;
        let _ = &w / Distance::from_m(1000.0);
        let _ = w / &Distance::from_m(1000.0);
    }
}