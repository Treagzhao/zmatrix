use std::ops::{Div, Mul};
use crate::physics::basic::{AngularVelocity, Coef, MagneticAngularVelocity, MagneticInduction, MagneticInductionType, Vector3};

impl Mul<AngularVelocity> for Vector3<MagneticInduction>{
    type Output = Vector3<MagneticAngularVelocity>;

    fn mul(self, rhs: AngularVelocity) -> Self::Output {
        let x = self.x.as_nano_tesla() * rhs.as_rad_per_second();
        let y = self.y.as_nano_tesla() * rhs.as_rad_per_second();
        let z = self.z.as_nano_tesla() * rhs.as_rad_per_second();
        Vector3::new(
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(x),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(y),
            MagneticAngularVelocity::from_nano_tesla_rad_per_second(z),
        )
    }
}

impl Vector3<MagneticInduction> {
    pub fn to_vector3_coef(&self, magnetic_induction_type: MagneticInductionType) -> Vector3<Coef> {
        match magnetic_induction_type {
            MagneticInductionType::Tesla => {
                let (x, y, z) = (self.x.as_tesla(), self.y.as_tesla(), self.z.as_tesla());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticInductionType::Gauss => {
                let (x, y, z) = (self.x.as_gauss(), self.y.as_gauss(), self.z.as_gauss());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticInductionType::MillTesla => {
                let (x, y, z) = (self.x.as_milli_tesla(), self.y.as_milli_tesla(), self.z.as_milli_tesla());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticInductionType::MicroTesla => {
                let (x, y, z) = (self.x.as_micro_tesla(), self.y.as_micro_tesla(), self.z.as_micro_tesla());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticInductionType::NanoTesla => {
                let (x, y, z) = (self.x.as_nano_tesla(), self.y.as_nano_tesla(), self.z.as_nano_tesla());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticInductionType::MillGauss => {
                let (x, y, z) = (self.x.as_mill_gauss(), self.y.as_mill_gauss(), self.z.as_mill_gauss());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MagneticInductionType::KiloGauss => {
                let (x, y, z) = (self.x.as_kilo_gauss(), self.y.as_kilo_gauss(), self.z.as_kilo_gauss());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, magnetic_induction_type: MagneticInductionType) -> Vector3<MagneticInduction> {
        let x = MagneticInduction {
            v: coef.x.v,
            default_type: magnetic_induction_type,
        };
        let y = MagneticInduction {
            v: coef.y.v,
            default_type: magnetic_induction_type,
        };
        let z = MagneticInduction {
            v: coef.z.v,
            default_type: magnetic_induction_type,
        };
        Vector3::new(x, y, z)
    }

    pub fn from_array_with_unit(array: [f64; 3], magnetic_induction_type: MagneticInductionType) -> Vector3<MagneticInduction> {
        let [x, y, z] = array;
        let x = MagneticInduction {
            v: x,
            default_type: magnetic_induction_type,
        };
        let y = MagneticInduction {
            v: y,
            default_type: magnetic_induction_type,
        };
        let z = MagneticInduction {
            v: z,
            default_type: magnetic_induction_type,
        };
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_to_vector3_coef_tesla() {
        let magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_tesla(2.0),
            MagneticInduction::from_tesla(3.0),
        );
        
        let coef_vec = magnetic_vec.to_vector3_coef(MagneticInductionType::Tesla);
        
        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_to_vector3_coef_gauss() {
        let magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(1.0),    // 1 T = 10000 G
            MagneticInduction::from_tesla(0.5),    // 0.5 T = 5000 G
            MagneticInduction::from_tesla(0.1),    // 0.1 T = 1000 G
        );
        
        let coef_vec = magnetic_vec.to_vector3_coef(MagneticInductionType::Gauss);
        
        assert_relative_eq!(coef_vec.x, 10000.0);
        assert_relative_eq!(coef_vec.y, 5000.0);
        assert_relative_eq!(coef_vec.z, 1000.0);
    }

    #[test]
    fn test_from_vector_coef_tesla() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_vector_coef(coef_vec, MagneticInductionType::Tesla);

        assert_relative_eq!(magnetic_induction_vec.x.as_tesla(), 1.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_tesla(), 2.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_tesla(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_gauss() {
        let coef_vec = Vector3::new(
            Coef::new(10000.0),
            Coef::new(20000.0),
            Coef::new(30000.0),
        );

        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_vector_coef(coef_vec, MagneticInductionType::Gauss);

        assert_relative_eq!(magnetic_induction_vec.x.as_gauss(), 10000.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_gauss(), 20000.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_gauss(), 30000.0);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_magnetic_induction_vec = Vector3::new(
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_tesla(2.0),
            MagneticInduction::from_tesla(3.0),
        );

        let coef_vec = original_magnetic_induction_vec.to_vector3_coef(MagneticInductionType::Tesla);
        let reconstructed_magnetic_induction_vec = Vector3::<MagneticInduction>::from_vector_coef(coef_vec, MagneticInductionType::Tesla);

        assert_relative_eq!(original_magnetic_induction_vec.x.as_tesla(), reconstructed_magnetic_induction_vec.x.as_tesla());
        assert_relative_eq!(original_magnetic_induction_vec.y.as_tesla(), reconstructed_magnetic_induction_vec.y.as_tesla());
        assert_relative_eq!(original_magnetic_induction_vec.z.as_tesla(), reconstructed_magnetic_induction_vec.z.as_tesla());
    }

    #[test]
    fn test_to_vector3_coef_mill_tesla() {
        let magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(1.0),    // 1 T = 1000 mT
            MagneticInduction::from_tesla(0.5),    // 0.5 T = 500 mT
            MagneticInduction::from_tesla(0.001),  // 0.001 T = 1 mT
        );
        
        let coef_vec = magnetic_vec.to_vector3_coef(MagneticInductionType::MillTesla);
        
        assert_relative_eq!(coef_vec.x.get_value(), 1000.0);
        assert_relative_eq!(coef_vec.y.get_value(), 500.0);
        assert_relative_eq!(coef_vec.z.get_value(), 1.0);
    }

    #[test]
    fn test_to_vector3_coef_micro_tesla() {
        let magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(1.0),      // 1 T = 1,000,000 μT
            MagneticInduction::from_tesla(0.000001), // 0.000001 T = 1 μT
            MagneticInduction::from_tesla(0.000002), // 0.000002 T = 2 μT
        );
        
        let coef_vec = magnetic_vec.to_vector3_coef(MagneticInductionType::MicroTesla);
        
        assert_relative_eq!(coef_vec.x.get_value(), 1_000_000.0);
        assert_relative_eq!(coef_vec.y.get_value(), 1.0);
        assert_relative_eq!(coef_vec.z.get_value(), 2.0);
    }

    #[test]
    fn test_to_vector3_coef_nano_tesla() {
        let magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(1.0),        // 1 T = 1,000,000,000 nT
            MagneticInduction::from_tesla(0.000000001), // 0.000000001 T = 1 nT
            MagneticInduction::from_tesla(0.000000002), // 0.000000002 T = 2 nT
        );
        
        let coef_vec = magnetic_vec.to_vector3_coef(MagneticInductionType::NanoTesla);
        
        assert_relative_eq!(coef_vec.x.get_value(), 1_000_000_000.0);
        assert_relative_eq!(coef_vec.y.get_value(), 1.0);
        assert_relative_eq!(coef_vec.z.get_value(), 2.0);
    }

    #[test]
    fn test_to_vector3_coef_mill_gauss() {
        let magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(1.0),    // 1 T = 10,000 G = 10,000,000 mG
            MagneticInduction::from_tesla(0.5),    // 0.5 T = 5,000 G = 5,000,000 mG
            MagneticInduction::from_tesla(0.0001), // 0.0001 T = 1 G = 1,000 mG
        );
        
        let coef_vec = magnetic_vec.to_vector3_coef(MagneticInductionType::MillGauss);
        
        assert_relative_eq!(coef_vec.x.get_value(), 10_000_000.0);
        assert_relative_eq!(coef_vec.y.get_value(), 5_000_000.0);
        assert_relative_eq!(coef_vec.z.get_value(), 1_000.0);
    }

    #[test]
    fn test_to_vector3_coef_kilo_gauss() {
        let magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(1.0),    // 1 T = 10,000 G = 10 kG
            MagneticInduction::from_tesla(0.5),    // 0.5 T = 5,000 G = 5 kG
            MagneticInduction::from_tesla(0.1),    // 0.1 T = 1,000 G = 1 kG
        );
        
        let coef_vec = magnetic_vec.to_vector3_coef(MagneticInductionType::KiloGauss);
        
        assert_relative_eq!(coef_vec.x.get_value(), 10.0);
        assert_relative_eq!(coef_vec.y.get_value(), 5.0);
        assert_relative_eq!(coef_vec.z.get_value(), 1.0);
    }

    #[test]
    fn test_from_vector_coef_mill_tesla() {
        let coef_vec = Vector3::new(
            Coef::new(1000.0),
            Coef::new(500.0),
            Coef::new(1.0),
        );

        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_vector_coef(coef_vec, MagneticInductionType::MillTesla);

        assert_relative_eq!(magnetic_induction_vec.x.as_milli_tesla(), 1000.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_milli_tesla(), 500.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_milli_tesla(), 1.0);
    }

    #[test]
    fn test_from_vector_coef_micro_tesla() {
        let coef_vec = Vector3::new(
            Coef::new(1_000_000.0),
            Coef::new(1.0),
            Coef::new(2.0),
        );

        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_vector_coef(coef_vec, MagneticInductionType::MicroTesla);

        assert_relative_eq!(magnetic_induction_vec.x.as_micro_tesla(), 1_000_000.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_micro_tesla(), 1.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_micro_tesla(), 2.0);
    }

    #[test]
    fn test_from_vector_coef_nano_tesla() {
        let coef_vec = Vector3::new(
            Coef::new(1_000_000_000.0),
            Coef::new(1.0),
            Coef::new(2.0),
        );

        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_vector_coef(coef_vec, MagneticInductionType::NanoTesla);

        assert_relative_eq!(magnetic_induction_vec.x.as_nano_tesla(), 1_000_000_000.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_nano_tesla(), 1.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_nano_tesla(), 2.0);
    }

    #[test]
    fn test_from_vector_coef_mill_gauss() {
        let coef_vec = Vector3::new(
            Coef::new(10_000_000.0),
            Coef::new(5_000_000.0),
            Coef::new(1_000.0),
        );

        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_vector_coef(coef_vec, MagneticInductionType::MillGauss);

        assert_relative_eq!(magnetic_induction_vec.x.as_mill_gauss(), 10_000_000.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_mill_gauss(), 5_000_000.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_mill_gauss(), 1_000.0);
    }

    #[test]
    fn test_from_vector_coef_kilo_gauss() {
        let coef_vec = Vector3::new(
            Coef::new(10.0),
            Coef::new(5.0),
            Coef::new(1.0),
        );

        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_vector_coef(coef_vec, MagneticInductionType::KiloGauss);

        assert_relative_eq!(magnetic_induction_vec.x.as_kilo_gauss(), 10.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_kilo_gauss(), 5.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_kilo_gauss(), 1.0);
    }

    #[test]
    fn test_roundtrip_all_types() {
        // 测试所有类型的往返转换
        let types = vec![
            MagneticInductionType::Tesla,
            MagneticInductionType::Gauss,
            MagneticInductionType::MillTesla,
            MagneticInductionType::MicroTesla,
            MagneticInductionType::NanoTesla,
            MagneticInductionType::MillGauss,
            MagneticInductionType::KiloGauss,
        ];

        let original_magnetic_induction_vec = Vector3::new(
            MagneticInduction::from_tesla(1.0),
            MagneticInduction::from_tesla(2.0),
            MagneticInduction::from_tesla(3.0),
        );

        for magnetic_type in types {
            let coef_vec = original_magnetic_induction_vec.to_vector3_coef(magnetic_type);
            let reconstructed_magnetic_induction_vec = Vector3::<MagneticInduction>::from_vector_coef(coef_vec, magnetic_type);

            // 验证转换后的值在对应单位下是正确的
            match magnetic_type {
                MagneticInductionType::Tesla => {
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.x.as_tesla(), 1.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.y.as_tesla(), 2.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.z.as_tesla(), 3.0);
                }
                MagneticInductionType::Gauss => {
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.x.as_gauss(), 10000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.y.as_gauss(), 20000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.z.as_gauss(), 30000.0);
                }
                MagneticInductionType::MillTesla => {
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.x.as_milli_tesla(), 1000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.y.as_milli_tesla(), 2000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.z.as_milli_tesla(), 3000.0);
                }
                MagneticInductionType::MicroTesla => {
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.x.as_micro_tesla(), 1_000_000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.y.as_micro_tesla(), 2_000_000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.z.as_micro_tesla(), 3_000_000.0);
                }
                MagneticInductionType::NanoTesla => {
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.x.as_nano_tesla(), 1_000_000_000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.y.as_nano_tesla(), 2_000_000_000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.z.as_nano_tesla(), 3_000_000_000.0);
                }
                MagneticInductionType::MillGauss => {
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.x.as_mill_gauss(), 10_000_000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.y.as_mill_gauss(), 20_000_000.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.z.as_mill_gauss(), 30_000_000.0);
                }
                MagneticInductionType::KiloGauss => {
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.x.as_kilo_gauss(), 10.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.y.as_kilo_gauss(), 20.0);
                    assert_relative_eq!(reconstructed_magnetic_induction_vec.z.as_kilo_gauss(), 30.0);
                }
            }
        }
    }

    #[test]
    fn test_edge_cases() {
        // 测试零值
        let zero_magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
            MagneticInduction::from_tesla(0.0),
        );
        
        let coef_vec = zero_magnetic_vec.to_vector3_coef(MagneticInductionType::Tesla);
        assert_relative_eq!(coef_vec.x.get_value(), 0.0);
        assert_relative_eq!(coef_vec.y.get_value(), 0.0);
        assert_relative_eq!(coef_vec.z.get_value(), 0.0);

        // 测试负值
        let negative_magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(-1.0),
            MagneticInduction::from_tesla(-2.0),
            MagneticInduction::from_tesla(-3.0),
        );
        
        let coef_vec = negative_magnetic_vec.to_vector3_coef(MagneticInductionType::Tesla);
        assert_relative_eq!(coef_vec.x.get_value(), -1.0);
        assert_relative_eq!(coef_vec.y.get_value(), -2.0);
        assert_relative_eq!(coef_vec.z.get_value(), -3.0);

        // 测试很小的值
        let small_magnetic_vec = Vector3::new(
            MagneticInduction::from_tesla(1e-10),
            MagneticInduction::from_tesla(1e-15),
            MagneticInduction::from_tesla(1e-20),
        );
        
        let coef_vec = small_magnetic_vec.to_vector3_coef(MagneticInductionType::NanoTesla);
        assert_relative_eq!(coef_vec.x.get_value(), 1e-1); // 1e-10 T = 1e-1 nT
        assert_relative_eq!(coef_vec.y.get_value(), 1e-6); // 1e-15 T = 1e-6 nT
        assert_relative_eq!(coef_vec.z.get_value(), 1e-11); // 1e-20 T = 1e-11 nT
    }

    #[test]
    fn test_from_array_with_unit() {
        // 测试从数组和单位类型创建磁感应向量
        let array = [1.0, 2.0, 3.0];
        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_array_with_unit(array, MagneticInductionType::Tesla);
        
        assert_relative_eq!(magnetic_induction_vec.x.as_tesla(), 1.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_tesla(), 2.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_tesla(), 3.0);
        
        // 测试不同单位类型
        let array = [10000.0, 20000.0, 30000.0];
        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_array_with_unit(array, MagneticInductionType::Gauss);
        
        assert_relative_eq!(magnetic_induction_vec.x.as_gauss(), 10000.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_gauss(), 20000.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_gauss(), 30000.0);
        
        // 测试毫特斯拉单位
        let array = [1000.0, 2000.0, 3000.0];
        let magnetic_induction_vec = Vector3::<MagneticInduction>::from_array_with_unit(array, MagneticInductionType::MillTesla);
        
        assert_relative_eq!(magnetic_induction_vec.x.as_milli_tesla(), 1000.0);
        assert_relative_eq!(magnetic_induction_vec.y.as_milli_tesla(), 2000.0);
        assert_relative_eq!(magnetic_induction_vec.z.as_milli_tesla(), 3000.0);
    }
}
