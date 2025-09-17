use crate::physics::basic;
use std::any::Any;
use std::ops::Div;

mod acceleration;
mod angular;
mod angular_acceleration;
mod angular_momentum;
mod angular_velocity;
mod area;
mod coef;
mod distance;
mod energy;
mod force;
mod magnetic_angular_velocity;
mod magnetic_induction;
mod magnetic_moment;
pub mod mass;
mod momentum;
mod power;
mod torque;
mod vector;
mod velocity;
mod volume;

//支持向量的物理量
pub trait VectorQuantity: PhysicalQuantity {}

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
/// 距离物理量结构体
/// 
/// 表示空间中的距离，支持多种单位制
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Distance;
/// 
/// let distance = Distance::from_m(100.0);
/// println!("距离: {} 米", distance.as_m());
/// ```
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Distance {
    default_type: DistanceType,
    v: f64,
}
/// 速度物理量结构体
/// 
/// 表示物体运动的快慢和方向，支持多种单位制
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Velocity;
/// 
/// let velocity = Velocity::from_m_per_sec(10.0);
/// println!("速度: {} m/s", velocity.as_m_per_sec());
/// ```
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Velocity {
    default_type: VelocityType,
    v: f64,
}

/// 加速度物理量结构体
/// 
/// 表示速度变化的快慢，支持多种单位制
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Acceleration;
/// 
/// let acceleration = Acceleration::from_g(1.0);
/// println!("加速度: {} g", acceleration.as_g());
/// ```
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

/// 角度物理量结构体
/// 
/// 表示角度大小，支持弧度和度数单位
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Angular;
/// 
/// let angle = Angular::from_rad(std::f64::consts::PI);
/// println!("角度: {} 弧度", angle.as_rad());
/// ```
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Angular {
    default_type: AngularType,
    v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy, Default)]
pub enum AngularVelocityType {
    #[default]
    RadperSecond,
    DegPerSecond,
    RadperHour,
    DegperHour,
}

/// 角速度物理量结构体
/// 
/// 表示角度变化的快慢，支持多种单位制
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::AngularVelocity;
/// 
/// let angular_velocity = AngularVelocity::from_rad_per_second(1.0);
/// println!("角速度: {} rad/s", angular_velocity.as_rad_per_second());
/// ```
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

/// 角加速度物理量结构体
/// 
/// 表示角速度变化的快慢，支持多种单位制
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::AngularAcceleration;
/// 
/// let angular_acceleration = AngularAcceleration::from_rad_per_second2(1.0);
/// println!("角加速度: {} rad/s²", angular_acceleration.as_rad_per_second2());
/// ```
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct AngularAcceleration {
    default_type: AngularAccelerationType,
    v: f64,
}
/// 系数物理量结构体
/// 
/// 表示无量纲的系数或比例因子
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Coef;
/// 
/// let coefficient = Coef::new(0.5);
/// println!("系数: {}", coefficient.get_value());
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct Coef {
    v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MassType {
    Kg,
    g,
}

/// 质量物理量结构体
/// 
/// 表示物体的质量，支持千克和克单位
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Mass;
/// 
/// let mass = Mass::from_kg(1.0);
/// println!("质量: {} kg", mass.as_kg());
/// ```
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Mass {
    default_type: MassType,
    pub v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AngularMomentumType {
    KgM2perSecond,  // 每秒1千克1平方米
    KgKm2perSecond, // 每秒1千克1平方公里
    Nms,            // 牛顿·米·秒 (N·m·s)
    MillNms,        // 毫牛顿·米·秒 (mN·m·s)
    MicroNms,       // 微牛顿·米·秒 (μN·m·s)
    NanoNms,        // 纳牛顿·米·秒 (nN·m·s)
}
/// 角动量物理量结构体
/// 
/// 表示物体绕轴旋转的动量，是质量、速度和距离的乘积
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::AngularMomentum;
/// 
/// let angular_momentum = AngularMomentum::from_kg_m2_per_second(1.0);
/// println!("角动量: {} kg·m²/s", angular_momentum.as_kg_m2_per_second());
/// ```
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct AngularMomentum {
    default_type: AngularMomentumType,
    pub v: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MomentumType {
    KgMperSecond,  // 每秒1千克1米
    KgKmperSecond, // 每秒1千克1千米
}
/// 动量物理量结构体
/// 
/// 表示物体运动的动量，是质量和速度的乘积
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Momentum;
/// 
/// let momentum = Momentum::from_kg_m_s(1.0);
/// println!("动量: {} kg·m/s", momentum.as_kg_m_s());
/// ```
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Momentum {
    default_type: MomentumType,
    pub v: f64,
}

/// 三维向量结构体
/// 
/// 表示三维空间中的向量，包含x、y、z三个分量
/// 
/// # 类型参数
/// * `T` - 向量分量的物理量类型，必须实现 `VectorQuantity` 和 `Default` trait
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::{Vector3, Distance};
/// 
/// let position = Vector3 {
///     x: Distance::from_m(1.0),
///     y: Distance::from_m(2.0),
///     z: Distance::from_m(3.0),
/// };
/// println!("位置: ({}, {}, {})", position.x.as_m(), position.y.as_m(), position.z.as_m());
/// ```
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vector3<T: VectorQuantity + Default> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AreaType {
    M2,
    KM2,
}

/// 面积物理量结构体
/// 
/// 表示二维空间的面积大小，支持平方米和平方千米单位
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Area;
/// 
/// let area = Area::from_m2(100.0);
/// println!("面积: {} m²", area.as_m2());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Area {
    default_type: AreaType,
    pub v: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum MagneticInductionType {
    Gauss,
    #[default]
    Tesla,
    MillTesla,  // 毫
    MicroTesla, // 微
    NanoTesla,  //; 纳
    MillGauss,  // 毫
    KiloGauss,  // 千
}
/// 磁感应强度物理量结构体
/// 
/// 表示磁场强度，单位是特斯拉或高斯
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::MagneticInduction;
/// 
/// let magnetic_field = MagneticInduction::from_tesla(1.0);
/// println!("磁感应强度: {} T", magnetic_field.as_tesla());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MagneticInduction {
    default_type: MagneticInductionType,
    pub v: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MagneticMomentType {
    AM2,            // 安培·平方米 (A·m²)
    MillAM2,        // 毫安培·平方米 (mA·m²)
    MicroAM2,       // 微安培·平方米 (μA·m²)
    NanoAM2,        // 纳安培·平方米 (nA·m²)
    JPerTesla,      // 焦耳/特斯拉 (J/T)
    MillJPerTesla,  // 毫焦耳/特斯拉 (mJ/T)
    MicroJPerTesla, // 微焦耳/特斯拉 (μJ/T)
    NanoJPerTesla,  // 纳焦耳/特斯拉 (nJ/T)
}

/// 磁矩物理量结构体
/// 
/// 表示磁偶极子的磁矩，单位是安培·平方米或焦耳/特斯拉
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::MagneticMoment;
/// 
/// let magnetic_moment = MagneticMoment::from_am2(1.0);
/// println!("磁矩: {} A·m²", magnetic_moment.as_am2());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MagneticMoment {
    default_type: MagneticMomentType,
    pub v: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TorqueType {
    NM,      // 牛顿·米 (N·m)
    MillNM,  // 毫牛顿·米 (mN·m)
    MicroNM, // 微牛顿·米 (μN·m)
    NanoNM,  // 纳牛顿·米 (nN·m)
    KNM,     // 千牛顿·米 (kN·m)
    MNM,     // 兆牛顿·米 (MN·m)
}

/// 力矩物理量结构体
/// 
/// 表示力对物体产生的转动效应，单位是牛顿·米
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Torque;
/// 
/// let torque = Torque::from_nm(10.0);
/// println!("力矩: {} N·m", torque.as_nm());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Torque {
    default_type: TorqueType,
    pub v: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnergyType {
    Joule,            // 焦耳 (J)
    MillJoule,        // 毫焦耳 (mJ)
    MicroJoule,       // 微焦耳 (μJ)
    NanoJoule,        // 纳焦耳 (nJ)
    KiloJoule,        // 千焦耳 (kJ)
    MegaJoule,        // 兆焦耳 (MJ)
    ElectronVolt,     // 电子伏特 (eV)
    KiloElectronVolt, // 千电子伏特 (keV)
    MegaElectronVolt, // 兆电子伏特 (MeV)
}

/// 能量物理量结构体
/// 
/// 表示物体做功的能力，单位是焦耳
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Energy;
/// 
/// let energy = Energy::from_joule(100.0);
/// println!("能量: {} J", energy.as_joule());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Energy {
    default_type: EnergyType,
    pub v: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ForceType {
    Newton,      // 牛顿 (N)
    MillNewton,  // 毫牛顿 (mN)
    MicroNewton, // 微牛顿 (μN)
    NanoNewton,  // 纳牛顿 (nN)
    KiloNewton,  // 千牛顿 (kN)
    MegaNewton,  // 兆牛顿 (MN)
}

/// 力物理量结构体
/// 
/// 表示物体间的相互作用，单位是牛顿
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Force;
/// 
/// let force = Force::from_newton(10.0);
/// println!("力: {} N", force.as_newton());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Force {
    default_type: ForceType,
    pub v: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PowerType {
    Watt,       // 瓦特 (W)
    MillWatt,   // 毫瓦特 (mW)
    MicroWatt,  // 微瓦特 (μW)
    NanoWatt,   // 纳瓦特 (nW)
    KiloWatt,   // 千瓦特 (kW)
    MegaWatt,   // 兆瓦特 (MW)
    HorsePower, // 马力 (hp)
}

/// 功率物理量结构体
/// 
/// 表示单位时间内做功的多少，单位是瓦特
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Power;
/// 
/// let power = Power::from_watt(100.0);
/// println!("功率: {} W", power.as_watt());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Power {
    default_type: PowerType,
    pub v: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MagneticAngularVelocityType {
    TeslaRadPerSecond,      // 特斯拉·弧度/秒 (T·rad/s)
    MillTeslaRadPerSecond,  // 毫特斯拉·弧度/秒 (mT·rad/s)
    MicroTeslaRadPerSecond, // 微特斯拉·弧度/秒 (μT·rad/s)
    NanoTeslaRadPerSecond,  // 纳特斯拉·弧度/秒 (nT·rad/s)
    GaussRadPerSecond,      // 高斯·弧度/秒 (G·rad/s)
    MillGaussRadPerSecond,  // 毫高斯·弧度/秒 (mG·rad/s)
    KiloGaussRadPerSecond,  // 千高斯·弧度/秒 (kG·rad/s)
}

/// 磁角速度物理量结构体
/// 
/// 表示磁场中的角速度，单位是特斯拉·弧度/秒或高斯·弧度/秒
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::MagneticAngularVelocity;
/// 
/// let magnetic_angular_velocity = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
/// println!("磁角速度: {} T·rad/s", magnetic_angular_velocity.as_tesla_rad_per_second());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MagneticAngularVelocity {
    default_type: MagneticAngularVelocityType,
    pub v: f64,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum VolumeType {
    M3,
    KM3,
}
/// 体积物理量结构体
/// 
/// 表示三维空间的体积大小，支持立方米和立方千米单位
/// 
/// # 示例
/// ```
/// use zmatrix::physics::basic::Volume;
/// 
/// let volume = Volume::from_m3(1000.0);
/// println!("体积: {} m³", volume.as_m3());
/// ```
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

impl<T: VectorQuantity + Default> Default for Vector3<T> {
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
    use super::*;
    use approx::assert_relative_eq;
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
    fn test_acceleration_default() {
        let a = Acceleration::default();
        assert_eq!(0.0, a.as_m_per_s2());
    }

    #[test]
    fn test_angular_velocity_default() {
        let a = AngularVelocity::default();
        assert_eq!(0.0, a.as_rad_per_hour());

        let a = AngularAcceleration::default();
        assert_eq!(0.0, a.as_rad_per_second2());
    }

    #[test]
    fn test_angular_is_zero() {
        let a = Angular::from_rad(0.0);
        assert_eq!(true, a.is_zero());
        let a = Angular::from_rad(1.0);
        assert_eq!(false, a.is_zero());
    }

    #[test]
    fn test_volume_clone() {
        let a = Volume::from_m3(3.8);
        let b = a.clone();
        assert_relative_eq!(a.as_km3(), b.as_km3(), epsilon = 1e-8);
    }

    #[test]
    fn test_magnetic_moment_as_any() {
        let g: &dyn PhysicalQuantity = &MagneticMoment::from_am2(1.23);
        let d = g.as_any().downcast_ref::<MagneticMoment>().unwrap();
        assert_eq!(d.default_type, MagneticMomentType::AM2);
    }

    #[test]
    fn test_magnetic_moment_default() {
        let mm = MagneticMoment::default();
        assert_eq!(0.0, mm.as_am2());
    }

    #[test]
    fn test_magnetic_moment_is_zero() {
        let mm = MagneticMoment::from_am2(0.0);
        assert_eq!(true, mm.is_zero());
        let mm = MagneticMoment::from_am2(1.0);
        assert_eq!(false, mm.is_zero());
    }

    #[test]
    fn test_torque_as_any() {
        let g: &dyn PhysicalQuantity = &Torque::from_nm(1.23);
        let d = g.as_any().downcast_ref::<Torque>().unwrap();
        assert_eq!(d.default_type, TorqueType::NM);
    }

    #[test]
    fn test_torque_default() {
        let t = Torque::default();
        assert_eq!(0.0, t.as_nm());
    }

    #[test]
    fn test_torque_is_zero() {
        let t = Torque::from_nm(0.0);
        assert_eq!(true, t.is_zero());
        let t = Torque::from_nm(1.0);
        assert_eq!(false, t.is_zero());
    }

    #[test]
    fn test_energy_as_any() {
        let g: &dyn PhysicalQuantity = &Energy::from_joule(1.23);
        let d = g.as_any().downcast_ref::<Energy>().unwrap();
        assert_eq!(d.default_type, EnergyType::Joule);
    }

    #[test]
    fn test_energy_default() {
        let e = Energy::default();
        assert_eq!(0.0, e.as_joule());
    }

    #[test]
    fn test_energy_is_zero() {
        let e = Energy::from_joule(0.0);
        assert_eq!(true, e.is_zero());
        let e = Energy::from_joule(1.0);
        assert_eq!(false, e.is_zero());
    }
}

// 手动实现 VectorQuantity trait 给所有支持向量的物理量
impl VectorQuantity for Distance {}
impl VectorQuantity for Velocity {}
impl VectorQuantity for Acceleration {}
impl VectorQuantity for Angular {}
impl VectorQuantity for AngularVelocity {}
impl VectorQuantity for AngularAcceleration {}
impl VectorQuantity for Coef {}
impl VectorQuantity for AngularMomentum {}
impl VectorQuantity for Momentum {}
impl VectorQuantity for MagneticInduction {}
impl VectorQuantity for MagneticMoment {}
impl VectorQuantity for Torque {}
impl VectorQuantity for Force {}
impl VectorQuantity for MagneticAngularVelocity {}
