use std::any::Any;
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::physics::basic::{MagneticInduction, MagneticInductionType, PhysicalQuantity};
lazy_static! {
    static ref TESLA_CONVERT:HashMap<MagneticInductionType,f64> = [
        (MagneticInductionType::Tesla, 1.0),
        (MagneticInductionType::MillTesla,1e-3),
        (MagneticInductionType::MicroTesla,1e-6),
        (MagneticInductionType::NanoTesla,1e-9),
    ]
   .iter()
   .cloned()
   .collect();

    static ref GAUSS_CONVERT:HashMap<MagneticInductionType,f64> = [
        (MagneticInductionType::KiloGauss,1e3),
        (MagneticInductionType::Gauss, 1.0),
        (MagneticInductionType::MillGauss,1e-3),
    ] .iter()
   .cloned()
   .collect();
}

impl MagneticInduction {
    pub fn from_tesla(v: f64) -> Self {
        Self {
            default_type: MagneticInductionType::Tesla,
            v,
        }
    }

    pub fn from_gauss(v: f64) -> Self {
        Self {
            default_type: MagneticInductionType::Gauss,
            v,
        }
    }

    pub fn from_micro_tesla(v: f64) -> Self {
        Self {
            default_type: MagneticInductionType::MicroTesla,
            v,
        }
    }

    pub fn from_mill_tesla(v: f64) -> Self {
        Self {
            default_type: MagneticInductionType::MillTesla,
            v,
        }
    }

    pub fn from_nano_tesla(v: f64) -> Self {
        Self {
            default_type: MagneticInductionType::NanoTesla,
            v,
        }
    }

    pub fn from_mill_gauss(v: f64) -> Self {
        Self {
            default_type: MagneticInductionType::MillGauss,
            v,
        }
    }
    pub fn from_kilo_gauss(v: f64) -> Self {
        Self {
            default_type: MagneticInductionType::KiloGauss,
            v,
        }
    }

    pub fn as_tesla(&self) -> f64 {
        match self.default_type {
            MagneticInductionType::Tesla | MagneticInductionType::MicroTesla | MagneticInductionType::MillTesla | MagneticInductionType::NanoTesla => {
                convert_tesla_value(self.v, self.default_type, MagneticInductionType::Tesla).unwrap()
            }
            _ => {
                let v = convert_gauss_value(self.v, self.default_type, MagneticInductionType::Gauss).unwrap();
                v * 1e-4
            }
        }
    }
    pub fn as_gauss(&self) -> f64 {
        match self.default_type {
            MagneticInductionType::Gauss | MagneticInductionType::MillGauss | MagneticInductionType::KiloGauss => {
                convert_gauss_value(self.v, self.default_type, MagneticInductionType::Gauss).unwrap()
            },
            _ => {
                let v = convert_tesla_value(self.v, self.default_type, MagneticInductionType::Tesla).unwrap();
                v * 1e4
            }
        }
    }

    pub fn as_milli_tesla(&self) -> f64 {
       match self.default_type {
           MagneticInductionType::Gauss | MagneticInductionType::MillGauss | MagneticInductionType::KiloGauss =>{
               let v = convert_gauss_value(self.v, self.default_type, MagneticInductionType::Gauss).unwrap();
               v * 1e-1
           },
           _ => {
               let v = convert_tesla_value(self.v, self.default_type, MagneticInductionType::Tesla).unwrap();
               v * 1e3
           }
       }
    }

    pub fn as_micro_tesla(&self) -> f64 {
        match self.default_type {
            MagneticInductionType::Gauss | MagneticInductionType::MillGauss | MagneticInductionType::KiloGauss =>{
                let v = convert_gauss_value(self.v, self.default_type, MagneticInductionType::Gauss).unwrap();
                v * 1e2
            },
            _ => {
                let v = convert_tesla_value(self.v, self.default_type, MagneticInductionType::Tesla).unwrap();
                v * 1e6
            }
        }
    }
}

fn convert_gauss_value(v: f64, from: MagneticInductionType, to: MagneticInductionType) -> Option<f64> {
    let from_base = GAUSS_CONVERT.get(&from)?;
    let to_base = GAUSS_CONVERT.get(&to)?;
    Some(v * from_base / to_base)
}

fn convert_tesla_value(v: f64, from: MagneticInductionType, to: MagneticInductionType) -> Option<f64> {
    let from_base = TESLA_CONVERT.get(&from)?;
    let to_base = TESLA_CONVERT.get(&to)?;
    Some(v * from_base / to_base)
}

impl PhysicalQuantity for MagneticInduction {
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

impl Default for MagneticInduction {
    fn default() -> Self {
        MagneticInduction::from_tesla(0.0)
    }
}



#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    #[test]
    fn test_default(){
        let magnetic_induction = MagneticInduction::default();
        assert_relative_eq!(magnetic_induction.as_tesla(), 0.0);
    }
    #[test]
    fn test_magnetic_induction_from() {
        let magnetic_induction = MagneticInduction::from_tesla(1.0);
        assert_eq!(magnetic_induction.default_type, MagneticInductionType::Tesla);
        assert_eq!(magnetic_induction.v, 1.0);

        let magnetic_induction = MagneticInduction::from_gauss(1.0);
        assert_eq!(magnetic_induction.default_type, MagneticInductionType::Gauss);
        assert_eq!(magnetic_induction.v, 1.0);

        let magnetic_induction = MagneticInduction::from_micro_tesla(1.0);
        assert_eq!(magnetic_induction.default_type, MagneticInductionType::MicroTesla);
        assert_eq!(magnetic_induction.v, 1.0);

        let magnetic_induction = MagneticInduction::from_mill_tesla(1.0);
        assert_eq!(magnetic_induction.default_type, MagneticInductionType::MillTesla);
        assert_eq!(magnetic_induction.v, 1.0);

        let magnetic_induction = MagneticInduction::from_nano_tesla(1.0);
        assert_eq!(magnetic_induction.default_type, MagneticInductionType::NanoTesla);
        assert_eq!(magnetic_induction.v, 1.0);

        let magnetic_induction = MagneticInduction::from_mill_gauss(1.0);
        assert_eq!(magnetic_induction.default_type, MagneticInductionType::MillGauss);
        assert_eq!(magnetic_induction.v, 1.0);

        let magnetic_induction = MagneticInduction::from_kilo_gauss(1.0);
        assert_eq!(magnetic_induction.default_type, MagneticInductionType::KiloGauss);
        assert_eq!(magnetic_induction.v, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_convert_gauss_value_panic() {
        let result = convert_gauss_value(1.0, MagneticInductionType::Tesla, MagneticInductionType::KiloGauss).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_convert_gauss_value_panic2() {
        let result = convert_gauss_value(1.0, MagneticInductionType::KiloGauss, MagneticInductionType::Tesla).unwrap();
    }

    #[test]
    fn test_convert_gauss_value() {
        let result = convert_gauss_value(1.0, MagneticInductionType::Gauss, MagneticInductionType::KiloGauss).unwrap();
        assert_relative_eq!(1e-3,result);
        let result = convert_gauss_value(1.0, MagneticInductionType::MillGauss, MagneticInductionType::Gauss).unwrap();
        assert_relative_eq!(1e-3,result);
        let result = convert_gauss_value(1.0, MagneticInductionType::MillGauss, MagneticInductionType::KiloGauss).unwrap();
        assert_relative_eq!(1e-6,result);
        let result = convert_gauss_value(1.0, MagneticInductionType::Gauss, MagneticInductionType::MillGauss).unwrap();
        assert_relative_eq!(1e3,result);
        let result = convert_gauss_value(1.0, MagneticInductionType::KiloGauss, MagneticInductionType::Gauss).unwrap();
        assert_relative_eq!(1e3,result);
        let result = convert_gauss_value(1.0, MagneticInductionType::KiloGauss, MagneticInductionType::MillGauss).unwrap();
        assert_relative_eq!(1e6,result);
    }

    #[test]
    #[should_panic]
    fn test_convert_tesla_value_panic() {
        let result = convert_tesla_value(1.0, MagneticInductionType::Tesla, MagneticInductionType::KiloGauss).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_convert_tesla_value_panic2() {
        let result = convert_tesla_value(1.0, MagneticInductionType::KiloGauss, MagneticInductionType::Tesla).unwrap();
    }

    #[test]
    fn test_convert_tesla_value() {
        let result = convert_tesla_value(1.0, MagneticInductionType::NanoTesla, MagneticInductionType::MicroTesla).unwrap();
        assert_relative_eq!(1e-3,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::MicroTesla, MagneticInductionType::MillTesla).unwrap();
        assert_relative_eq!(1e-3,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::MillTesla, MagneticInductionType::Tesla).unwrap();
        assert_relative_eq!(1e-3,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::NanoTesla, MagneticInductionType::MillTesla).unwrap();
        assert_relative_eq!(1e-6,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::MicroTesla, MagneticInductionType::Tesla).unwrap();
        assert_relative_eq!(1e-6,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::NanoTesla, MagneticInductionType::Tesla).unwrap();
        assert_relative_eq!(1e-9,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::MicroTesla, MagneticInductionType::NanoTesla).unwrap();
        assert_relative_eq!(1e3,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::MillTesla, MagneticInductionType::MicroTesla).unwrap();
        assert_relative_eq!(1e3,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::Tesla, MagneticInductionType::MillTesla).unwrap();
        assert_relative_eq!(1e3,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::Tesla, MagneticInductionType::MicroTesla).unwrap();
        assert_relative_eq!(1e6,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::MillTesla, MagneticInductionType::NanoTesla).unwrap();
        assert_relative_eq!(1e6,result);

        let result = convert_tesla_value(1.0, MagneticInductionType::Tesla, MagneticInductionType::NanoTesla).unwrap();
        assert_relative_eq!(1e9,result);
    }

    #[test]
    fn test_as_tesla() {
        let magnetic_induction = MagneticInduction::from_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_tesla(), 1.0);
        let magnetic_induction = MagneticInduction::from_micro_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_tesla(), 1e-6);
        let magnetic_induction = MagneticInduction::from_gauss(1.0);
        assert_relative_eq!(magnetic_induction.as_tesla(), 1e-4);
        let magnetic_induction = MagneticInduction::from_kilo_gauss(1.0);
        assert_relative_eq!(magnetic_induction.as_tesla(), 0.1);
    }

    #[test]
    fn test_as_any(){
        let magnetic_induction = MagneticInduction::from_tesla(1.0);
        let any = magnetic_induction.as_any();
        let a = any.downcast_ref::<MagneticInduction>().unwrap();
    }

    #[test]
    fn test_is_zero(){
        let magnetic_induction = MagneticInduction::from_tesla(1.0);
        assert!(!magnetic_induction.is_zero());

        let magnetic_induction = MagneticInduction::from_tesla(0.0);
        assert!(magnetic_induction.is_zero());
    }

    #[test]
    fn test_as_gauss(){
        let magnetic_induction = MagneticInduction::from_gauss(1.0);
        assert_relative_eq!(magnetic_induction.as_gauss(), 1.0);

        let magnetic_induction = MagneticInduction::from_mill_gauss(1000.0);
        assert_relative_eq!(magnetic_induction.as_gauss(), 1.0);

        let magnetic_induction = MagneticInduction::from_kilo_gauss(1.0);
        assert_relative_eq!(magnetic_induction.as_gauss(), 1e3);

        let magnetic_induction = MagneticInduction::from_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_gauss(), 1e4);

        let magnetic_induction = MagneticInduction::from_mill_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_gauss(), 10.0);

        let magnetic_induction = MagneticInduction::from_micro_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_gauss(), 1e-2);

        let magnetic_induction = MagneticInduction::from_nano_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_gauss(), 1e-5);
    }

    #[test]
    fn test_as_milli_tesla(){
        let magnetic_induction = MagneticInduction::from_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_milli_tesla(), 1e3);

        let magnetic_induction = MagneticInduction::from_mill_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_milli_tesla(), 1.0);

        let magnetic_induction = MagneticInduction::from_micro_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_milli_tesla(), 1e-3);

        let magnetic_induction = MagneticInduction::from_nano_tesla(1.0);
        assert_relative_eq!(magnetic_induction.as_milli_tesla(), 1e-6);

        let magnetic_induction = MagneticInduction::from_gauss(1.0);
        assert_relative_eq!(magnetic_induction.as_milli_tesla(), 1e-1);

        let magnetic_induction = MagneticInduction::from_mill_gauss(1.0);
        assert_relative_eq!(magnetic_induction.as_milli_tesla(), 1e-4);

        let magnetic_induction = MagneticInduction::from_kilo_gauss(1.0);
        assert_relative_eq!(magnetic_induction.as_milli_tesla(), 1e2);

    }

    #[test]
    fn test_as_micro_tesla() {
        // Tesla 系列单位测试
       let m = MagneticInduction::from_micro_tesla(1.0);
        assert_relative_eq!(m.as_micro_tesla(), 1.0);
        let m = MagneticInduction::from_mill_tesla(1.0);
        assert_relative_eq!(m.as_micro_tesla(), 1e3);
        let m = MagneticInduction::from_tesla(1.0);
        assert_relative_eq!(m.as_micro_tesla(), 1e6);
        let m = MagneticInduction::from_nano_tesla(1.0);
        assert_relative_eq!(m.as_micro_tesla(), 1e-3);

        // Gauss 系列单位测试
       let m = MagneticInduction::from_mill_gauss(1.0);
        assert_relative_eq!(m.as_micro_tesla(), 1e-1);
        let m = MagneticInduction::from_gauss(1.0);
        assert_relative_eq!(m.as_micro_tesla(), 1e2);
        let m = MagneticInduction::from_kilo_gauss(1.0);
        assert_relative_eq!(m.as_micro_tesla(), 1e5);

        // 边界值测试
        let zero_case = MagneticInduction {
            v: 0.0,
            default_type: MagneticInductionType::Gauss,
        };
        assert_eq!(zero_case.as_micro_tesla(), 0.0);
    }
}