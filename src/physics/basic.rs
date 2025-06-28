use std::any::Any;
use serde::Serialize;

mod vector;
mod distance;
mod velocity;
mod acceleration;
mod angular;
mod angular_velocity;
mod coef;
mod angular_acceleration;
mod area;

pub trait PhysicalQuantity: Any {
    fn as_any(&self) -> &dyn Any;
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
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Coef {
    v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vector3<T: PhysicalQuantity> {
    x: T,
    y: T,
    z: T,
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

impl PhysicalQuantity for Area {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl PhysicalQuantity for Distance {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl PhysicalQuantity for Velocity {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl PhysicalQuantity for Acceleration {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl PhysicalQuantity for Angular {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl PhysicalQuantity for AngularVelocity {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl PhysicalQuantity for Coef {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PhysicalQuantity for AngularAcceleration {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
#[cfg(test)]
mod tests {
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

        let g :&dyn PhysicalQuantity = &AngularAcceleration::from_rad_per_second2(1.23);
        let d = g.as_any().downcast_ref::<AngularAcceleration>().unwrap();
        assert_eq!(d.default_type, AngularAccelerationType::RadperSecond2);
    }
}