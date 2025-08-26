use crate::physics::basic::{Coef, Mass, MassType, Vector3};

impl Vector3<Mass> {
    pub fn to_vector3_coef(&self, mass_type: MassType) -> Vector3<Coef> {
        match mass_type {
            MassType::Kg => {
                let (x, y, z) = (self.x.as_kg(), self.y.as_kg(), self.z.as_kg());
                Vector3::<Coef>::from_array([x, y, z])
            }
            MassType::g => {
                let (x, y, z) = (self.x.as_g(), self.y.as_g(), self.z.as_g());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, mass_type: MassType) -> Vector3<Mass> {
        let x = Mass {
            v: coef.x.v,
            default_type: mass_type,
        };
        let y = Mass {
            v: coef.y.v,
            default_type: mass_type,
        };
        let z = Mass {
            v: coef.z.v,
            default_type: mass_type,
        };
        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_to_vector3_coef_kg() {
        let mass_vec = Vector3::new(
            Mass::from_kg(1.0),
            Mass::from_kg(2.0),
            Mass::from_kg(3.0),
        );
        
        let coef_vec = mass_vec.to_vector3_coef(MassType::Kg);
        
        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_to_vector3_coef_g() {
        let mass_vec = Vector3::new(
            Mass::from_kg(1.0),    // 1 kg = 1000 g
            Mass::from_kg(0.5),    // 0.5 kg = 500 g
            Mass::from_kg(2.5),    // 2.5 kg = 2500 g
        );
        
        let coef_vec = mass_vec.to_vector3_coef(MassType::g);
        
        assert_relative_eq!(coef_vec.x, 1000.0);
        assert_relative_eq!(coef_vec.y, 500.0);
        assert_relative_eq!(coef_vec.z, 2500.0);
    }

    #[test]
    fn test_from_vector_coef_kg() {
        let coef_vec = Vector3::new(
            Coef::new(1.0),
            Coef::new(2.0),
            Coef::new(3.0),
        );

        let mass_vec = Vector3::<Mass>::from_vector_coef(coef_vec, MassType::Kg);

        assert_relative_eq!(mass_vec.x.as_kg(), 1.0);
        assert_relative_eq!(mass_vec.y.as_kg(), 2.0);
        assert_relative_eq!(mass_vec.z.as_kg(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_g() {
        let coef_vec = Vector3::new(
            Coef::new(1000.0),
            Coef::new(500.0),
            Coef::new(2500.0),
        );

        let mass_vec = Vector3::<Mass>::from_vector_coef(coef_vec, MassType::g);

        assert_relative_eq!(mass_vec.x.as_g(), 1000.0);
        assert_relative_eq!(mass_vec.y.as_g(), 500.0);
        assert_relative_eq!(mass_vec.z.as_g(), 2500.0);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_mass_vec = Vector3::new(
            Mass::from_kg(1.0),
            Mass::from_kg(2.0),
            Mass::from_kg(3.0),
        );

        let coef_vec = original_mass_vec.to_vector3_coef(MassType::Kg);
        let reconstructed_mass_vec = Vector3::<Mass>::from_vector_coef(coef_vec, MassType::Kg);

        assert_relative_eq!(original_mass_vec.x.as_kg(), reconstructed_mass_vec.x.as_kg());
        assert_relative_eq!(original_mass_vec.y.as_kg(), reconstructed_mass_vec.y.as_kg());
        assert_relative_eq!(original_mass_vec.z.as_kg(), reconstructed_mass_vec.z.as_kg());
    }
}
