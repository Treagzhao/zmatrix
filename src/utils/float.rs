use std::ops::Neg;

// 符号函数
pub fn sgn2_64(xin: f64) -> i32 {
    if xin > 0.0 {
        1
    } else if xin == 0.0 {
        0
    } else {
        -1
    }
}

pub fn limit_float<T>(fin: T, f_bound: T) -> T
where
    T: Default + Copy + PartialOrd + Neg<Output=T>,
{
    let f_value: T;

    // 输入值 fin，限幅阈值 f_bound，必须大于 0
    if fin > f_bound {
        // 大于上限值
        // 高于上限则取上限
        f_value = f_bound;
    } else if fin < -f_bound {
        // 小于下限值
        // 低于下限则取下限
        f_value = -f_bound;
    } else {
        // 其他情况
        // 在上下限之间保持不变
        f_value = fin;
    }

    f_value
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;

    #[test]
    #[test]
    fn test_sgn2_64() {
        // 测试正数
        assert_eq!(sgn2_64(10.0), 1);
        // 测试负数
        assert_eq!(sgn2_64(-5.0), -1);
        // 测试零
        assert_eq!(sgn2_64(0.0), 0);
    }

    #[test]
    fn test_limit_float() {
        assert_relative_eq!(limit_float(0.0, 1.0), 0.0);
        assert_relative_eq!(limit_float(-0.0, 1.0), 0.0);
        assert_relative_eq!(limit_float(1.0, 1.0), 1.0);
        assert_relative_eq!(limit_float(-1.0, 1.0), -1.0);
        assert_relative_eq!(limit_float(1.8, 1.0), 1.0);
        assert_relative_eq!(limit_float(-1.8, 1.0), -1.0);
        assert_relative_eq!(limit_float(0.5, 1.0), 0.5);
    }
}

