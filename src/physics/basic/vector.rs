use super::*;

impl<T: PhysicalQuantity> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let vec = Vector3::new(Distance::from_m(1.0), Distance::from_m(2.0), Distance::from_m(3.0));
        assert_eq!(vec.x, Distance::from_m(1.0));
        assert_eq!(vec.y, Distance::from_m(2.0));
        assert_eq!(vec.z, Distance::from_m(3.0));
    }
}