use crate::physics::basic::{Coef, MagneticInduction, MagneticInductionType, Vector3};

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
}
