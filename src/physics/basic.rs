use std::any::Any;
use std::ops::Div;
use crate::physics::basic;

mod vector;
mod distance;
mod velocity;
mod acceleration;
mod angular;
mod angular_velocity;
mod coef;
mod angular_acceleration;
mod area;
mod magnetic_induction;
pub mod mass;
mod angular_momentum;
mod momentum;
mod volume;

pub trait PhysicalQuantity: Any {
    fn as_any(&self) -> &dyn Any;

    fn is_zero(&self) -> bool;

    fn default_unit_value(&self) -> f64;

    fn set_value(&mut self, value: f64);
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DistanceType {
    M,
    KM,
    LightYear,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VelocityType {
    MPerSecond,
    KmPerHour,
    KmPerSecond,
    LightSpeed,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Distance {
    default_type: DistanceType,
    v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Velocity {
    default_type: VelocityType,
    v: f64,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Acceleration {
    default_type: AccelerationType,
    v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AccelerationType {
    MPerSecond2,
    KmPerHour2,
    G,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AngularType {
    Rad,
    Deg,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Angular {
    default_type: AngularType,
    v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AngularVelocityType {
    RadperSecond,
    DegPerSecond,
    RadperHour,
    DegperHour,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct AngularVelocity {
    default_type: AngularVelocityType,
    v: f64,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AngularAccelerationType {
    RadperSecond2,
    DegPerSecond2,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct AngularAcceleration {
    default_type: AngularAccelerationType,
    v: f64,
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct Coef {
    v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MassType {
    Kg,
    g,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Mass {
    default_type: MassType,
    pub v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AngularMomentumType {
    KgM2perSecond, // 每秒1千克1平方米
    KgKm2perSecond, // 每秒1千克1平方公里
}
//角动量
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct AngularMomentum {
    default_type: AngularMomentumType,
    pub v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MomentumType {
    KgMperSecond, // 每秒1千克1米
    KgKmperSecond, // 每秒1千克1千米
}
//动量
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Momentum {
    default_type: MomentumType,
    pub v: f64,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vector3<T: PhysicalQuantity + Default> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AreaType {
    M2,
    KM2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Area {
    default_type: AreaType,
    pub v: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MagneticInductionType {
    Gauss,
    Tesla,
    MillTesla, // 毫
    MicroTesla, // 微
    NanoTesla, //; 纳
    MillGauss, // 毫
    KiloGauss, // 千
}
//磁感应强度B，单位是特斯拉或者高斯
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MagneticInduction {
    default_type: MagneticInductionType,
    pub v: f64,
}


#[derive(Clone, Debug, PartialEq, Copy)]
pub enum VolumeType {
    M3,
    KM3,
}
//体积
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Volume {
    default_type: VolumeType,
    pub v: f64,
}

impl Default for Distance {
    fn default() -> Self {
        Distance::from_m(0.0)
    }
}

impl PhysicalQuantity for Distance {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn is_zero(&self) -> bool {
        self.v == 0.0
    }

    fn default_unit_value(&self) -> f64 {
        self.as_m()
    }

    fn set_value(&mut self, value: f64) {
        self.v = value;
    }
}

impl Default for Angular {
    fn default() -> Self {
        Angular::from_rad(0.0)
    }
}

impl PhysicalQuantity for Angular {
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

impl Default for AngularVelocity {
    fn default() -> Self {
        AngularVelocity::from_rad_per_second(0.0)
    }
}


impl Div for AngularMomentum {
    type Output = Coef;
    fn div(self, rhs: Self) -> Self::Output {
        let v = self.default_unit_value() / rhs.default_unit_value();
        Coef::new(v)
    }
}

impl Default for Coef {
    fn default() -> Self {
        Coef::new(1.0)
    }
}


impl Default for AngularAcceleration {
    fn default() -> Self {
        AngularAcceleration::from_rad_per_second2(0.0)
    }
}


impl<T: PhysicalQuantity + Default> Default for Vector3<T> {
    fn default() -> Self {
        Vector3 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}
#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_as_any() {
        let g: &dyn PhysicalQuantity = &Acceleration::from_g(1.23);
        let d = g.as_any().downcast_ref::<Acceleration>().unwrap();

        let g: &dyn PhysicalQuantity = &Area::from_m2(10.0);
        let d = g.as_any().downcast_ref::<Area>().unwrap();

        let g: &dyn PhysicalQuantity = &Distance::from_m(10.0);
        let d = g.as_any().downcast_ref::<Distance>().unwrap();
        assert_eq!(d.default_type, DistanceType::M);

        let g: &dyn PhysicalQuantity = &Velocity::from_m_per_sec(10.0);
        let d = g.as_any().downcast_ref::<Velocity>().unwrap();
        assert_eq!(d.default_type, VelocityType::MPerSecond);

        let g: &dyn PhysicalQuantity = &Acceleration::from_g(1.23);
        let d = g.as_any().downcast_ref::<Acceleration>().unwrap();
        assert_eq!(d.default_type, AccelerationType::G);

        let g: &dyn PhysicalQuantity = &Angular::from_rad(1.23);
        let d = g.as_any().downcast_ref::<Angular>().unwrap();
        assert_eq!(d.default_type, AngularType::Rad);

        let g: &dyn PhysicalQuantity = &AngularVelocity::from_rad_per_hour(1.23);
        let d = g.as_any().downcast_ref::<AngularVelocity>().unwrap();
        assert_eq!(d.default_type, AngularVelocityType::RadperHour);

        let g: &dyn PhysicalQuantity = &Coef::new(1.23);
        let d = g.as_any().downcast_ref::<Coef>().unwrap();
        assert_eq!(d.v, 1.23);

        let g: &dyn PhysicalQuantity = &AngularAcceleration::from_rad_per_second2(1.23);
        let d = g.as_any().downcast_ref::<AngularAcceleration>().unwrap();
        assert_eq!(d.default_type, AngularAccelerationType::RadperSecond2);
    }

    #[test]
    fn test_acceleration_default(){
        let a = Acceleration::default();
        assert_eq!(0.0,a.as_m_per_s2());
    }

    #[test]
    fn test_angular_velocity_default(){
        let a = AngularVelocity::default();
        assert_eq!(0.0,a.as_rad_per_hour());

        let a = AngularAcceleration::default();
        assert_eq!(0.0,a.as_rad_per_second2());
    }

    #[test]
    fn test_angular_is_zero(){
        let a = Angular::from_rad(0.0);
        assert_eq!(true,a.is_zero());
        let a = Angular::from_rad(1.0);
        assert_eq!(false,a.is_zero());
    }

    #[test]
    fn test_volume_clone(){
        let a = Volume::from_m3(3.8);
        let b = a.clone();
        assert_relative_eq!(a.as_km3(),b.as_km3(),epsilon=1e-8);
    }
}