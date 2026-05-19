use std::sync::RwLock;

static FLT64_ZERO: RwLock<f64> = RwLock::new(1e-14);

pub fn get_flt64_zero() -> f64 {
    *FLT64_ZERO.read().unwrap()
}

pub fn set_flt64_zero(value: f64) {
    *FLT64_ZERO.write().unwrap() = value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_flt64_zero() {
        // 默认值为 1e-14
        assert_eq!(get_flt64_zero(), 1e-14);

        // 设置为其他值
        set_flt64_zero(1e-10);
        assert_eq!(get_flt64_zero(), 1e-10);

        // 设置为 0
        set_flt64_zero(0.0);
        assert_eq!(get_flt64_zero(), 0.0);

        // 设置为较大值
        set_flt64_zero(1.0);
        assert_eq!(get_flt64_zero(), 1.0);

        // 恢复默认值
        set_flt64_zero(1e-14);
        assert_eq!(get_flt64_zero(), 1e-14);
    }
}