use crate::physics::basic::{Coef, HeatFlux, HeatFluxType, Vector3};
use crate::utils::float;
impl Vector3<HeatFlux> {
    pub fn limit_float(&self, threshold: f64, heat_flux_type: HeatFluxType) -> Self {
        let (x, y, z) = match heat_flux_type {
            HeatFluxType::WattPerSquareMeter => (
                float::limit_float(self.x.as_w_per_m2(), threshold),
                float::limit_float(self.y.as_w_per_m2(), threshold),
                float::limit_float(self.z.as_w_per_m2(), threshold),
            ),
            HeatFluxType::KiloWattPerSquareMeter => (
                float::limit_float(self.x.as_kw_per_m2(), threshold),
                float::limit_float(self.y.as_kw_per_m2(), threshold),
                float::limit_float(self.z.as_kw_per_m2(), threshold),
            ),
            HeatFluxType::MilliWattPerSquareCentiMeter => (
                float::limit_float(self.x.as_mw_per_cm2(), threshold),
                float::limit_float(self.y.as_mw_per_cm2(), threshold),
                float::limit_float(self.z.as_mw_per_cm2(), threshold),
            ),
        };
        match heat_flux_type {
            HeatFluxType::WattPerSquareMeter => Self {
                x: HeatFlux::from_w_per_m2(x),
                y: HeatFlux::from_w_per_m2(y),
                z: HeatFlux::from_w_per_m2(z),
            },
            HeatFluxType::KiloWattPerSquareMeter => Self {
                x: HeatFlux::from_kw_per_m2(x),
                y: HeatFlux::from_kw_per_m2(y),
                z: HeatFlux::from_kw_per_m2(z),
            },
            HeatFluxType::MilliWattPerSquareCentiMeter => Self {
                x: HeatFlux::from_mw_per_cm2(x),
                y: HeatFlux::from_mw_per_cm2(y),
                z: HeatFlux::from_mw_per_cm2(z),
            },
        }
    }

    /// 转成无量纲方向向量（保留各分量的数值）。
    ///
    /// 注意：这里丢掉的是量纲，不是归一化 —— 结果需自行 `normalize` 才是单位向量。
    pub fn to_vector3_coef(&self, heat_flux_type: HeatFluxType) -> Vector3<Coef> {
        match heat_flux_type {
            HeatFluxType::WattPerSquareMeter => {
                let (x, y, z) = (
                    self.x.as_w_per_m2(),
                    self.y.as_w_per_m2(),
                    self.z.as_w_per_m2(),
                );
                Vector3::<Coef>::from_array([x, y, z])
            }
            HeatFluxType::KiloWattPerSquareMeter => {
                let (x, y, z) = (
                    self.x.as_kw_per_m2(),
                    self.y.as_kw_per_m2(),
                    self.z.as_kw_per_m2(),
                );
                Vector3::<Coef>::from_array([x, y, z])
            }
            HeatFluxType::MilliWattPerSquareCentiMeter => {
                let (x, y, z) = (
                    self.x.as_mw_per_cm2(),
                    self.y.as_mw_per_cm2(),
                    self.z.as_mw_per_cm2(),
                );
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, heat_flux_type: HeatFluxType) -> Vector3<HeatFlux> {
        let (x, y, z) = (
            coef.x.get_value(),
            coef.y.get_value(),
            coef.z.get_value(),
        );
        match heat_flux_type {
            HeatFluxType::WattPerSquareMeter => Vector3::new(
                HeatFlux::from_w_per_m2(x),
                HeatFlux::from_w_per_m2(y),
                HeatFlux::from_w_per_m2(z),
            ),
            HeatFluxType::KiloWattPerSquareMeter => Vector3::new(
                HeatFlux::from_kw_per_m2(x),
                HeatFlux::from_kw_per_m2(y),
                HeatFlux::from_kw_per_m2(z),
            ),
            HeatFluxType::MilliWattPerSquareCentiMeter => Vector3::new(
                HeatFlux::from_mw_per_cm2(x),
                HeatFlux::from_mw_per_cm2(y),
                HeatFlux::from_mw_per_cm2(z),
            ),
        }
    }

    pub fn from_array_with_unit(array: [f64; 3], heat_flux_type: HeatFluxType) -> Vector3<HeatFlux> {
        match heat_flux_type {
            HeatFluxType::WattPerSquareMeter => Vector3::new(
                HeatFlux::from_w_per_m2(array[0]),
                HeatFlux::from_w_per_m2(array[1]),
                HeatFlux::from_w_per_m2(array[2]),
            ),
            HeatFluxType::KiloWattPerSquareMeter => Vector3::new(
                HeatFlux::from_kw_per_m2(array[0]),
                HeatFlux::from_kw_per_m2(array[1]),
                HeatFlux::from_kw_per_m2(array[2]),
            ),
            HeatFluxType::MilliWattPerSquareCentiMeter => Vector3::new(
                HeatFlux::from_mw_per_cm2(array[0]),
                HeatFlux::from_mw_per_cm2(array[1]),
                HeatFlux::from_mw_per_cm2(array[2]),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_limit_float() {
        let v = Vector3::new(
            HeatFlux::from_w_per_m2(100.0),
            HeatFlux::from_w_per_m2(-5000.0),
            HeatFlux::from_w_per_m2(1500.0),
        );
        let r = v.limit_float(2000.0, HeatFluxType::WattPerSquareMeter);
        assert_relative_eq!(r.x.as_w_per_m2(), 100.0, epsilon = 1e-9);
        assert_relative_eq!(r.y.as_w_per_m2(), -2000.0, epsilon = 1e-9);
        assert_relative_eq!(r.z.as_w_per_m2(), 1500.0, epsilon = 1e-9);

        // 用 kW/m² 视图限幅：阈值含义随之改变
        let r2 = v.limit_float(2.0, HeatFluxType::KiloWattPerSquareMeter);
        assert_relative_eq!(r2.y.as_w_per_m2(), -2000.0, epsilon = 1e-9);
    }

    #[test]
    fn test_coef_round_trip() {
        let v = Vector3::new(
            HeatFlux::from_w_per_m2(1361.0),
            HeatFlux::from_w_per_m2(230.0),
            HeatFlux::from_w_per_m2(0.0),
        );
        let coef = v.to_vector3_coef(HeatFluxType::WattPerSquareMeter);
        assert_relative_eq!(coef.x.get_value(), 1361.0, epsilon = 1e-9);

        let back = Vector3::<HeatFlux>::from_vector_coef(coef, HeatFluxType::WattPerSquareMeter);
        assert_relative_eq!(back.x.as_w_per_m2(), 1361.0, epsilon = 1e-9);
        assert_relative_eq!(back.y.as_w_per_m2(), 230.0, epsilon = 1e-9);
    }

    #[test]
    fn test_from_array_with_unit() {
        let v = Vector3::<HeatFlux>::from_array_with_unit(
            [1.0, 2.0, 3.0],
            HeatFluxType::MilliWattPerSquareCentiMeter,
        );
        // 1 mW/cm² = 10 W/m²
        assert_relative_eq!(v.x.as_w_per_m2(), 10.0, epsilon = 1e-9);
        assert_relative_eq!(v.y.as_w_per_m2(), 20.0, epsilon = 1e-9);
        assert_relative_eq!(v.z.as_w_per_m2(), 30.0, epsilon = 1e-9);
    }

    #[test]
    fn test_all_unit_branches() {
        // 覆盖 limit_float 的三个单位分支
        let v = Vector3::new(
            HeatFlux::from_w_per_m2(10.0),
            HeatFlux::from_w_per_m2(-9000.0),
            HeatFlux::from_w_per_m2(20.0),
        );

        // kW/m² 视图：阈值 5 kW/m² = 5000 W/m²
        let r = v.limit_float(5.0, HeatFluxType::KiloWattPerSquareMeter);
        assert_relative_eq!(r.y.as_w_per_m2(), -5000.0, epsilon = 1e-9);

        // mW/cm² 视图：阈值 100 mW/cm² = 1000 W/m²
        let r = v.limit_float(100.0, HeatFluxType::MilliWattPerSquareCentiMeter);
        assert_relative_eq!(r.y.as_w_per_m2(), -1000.0, epsilon = 1e-9);

        // 覆盖 to_vector3_coef / from_vector_coef 的三个单位分支
        for t in [
            HeatFluxType::WattPerSquareMeter,
            HeatFluxType::KiloWattPerSquareMeter,
            HeatFluxType::MilliWattPerSquareCentiMeter,
        ] {
            let c = v.to_vector3_coef(t);
            let back = Vector3::<HeatFlux>::from_vector_coef(c, t);
            assert_relative_eq!(back.y.as_w_per_m2(), -9000.0, epsilon = 1e-6);
        }

        // 覆盖 from_array_with_unit 的另外两个分支
        for (t, expect) in [
            (HeatFluxType::WattPerSquareMeter, 1.0),
            (HeatFluxType::KiloWattPerSquareMeter, 1000.0),
        ] {
            let u = Vector3::<HeatFlux>::from_array_with_unit([1.0, 2.0, 3.0], t);
            assert_relative_eq!(u.x.as_w_per_m2(), expect, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_vector3_heat_flux_is_vector_quantity() {
        // 确认可在向量层使用：is_zero、to_array、加减
        let zero = Vector3::new(
            HeatFlux::default(),
            HeatFlux::default(),
            HeatFlux::default(),
        );
        assert!(zero.is_zero());

        let v = Vector3::new(
            HeatFlux::from_w_per_m2(1361.0),
            HeatFlux::from_w_per_m2(230.0),
            HeatFlux::from_w_per_m2(0.0),
        );
        assert!(!v.is_zero());
        assert_eq!(v.to_array(), [1361.0, 230.0, 0.0]);

        let sum = v + Vector3::new(
            HeatFlux::from_w_per_m2(100.0),
            HeatFlux::from_w_per_m2(0.0),
            HeatFlux::from_w_per_m2(0.0),
        );
        assert_relative_eq!(sum.x.as_w_per_m2(), 1461.0, epsilon = 1e-9);
    }
}
