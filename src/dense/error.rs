use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct OperationError {
    pub message: String,
}

impl OperationError {
    pub(crate) fn new(message: &str) -> OperationError {
        OperationError {
            message: message.to_string(),
        }
    }
}

impl Display for OperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// 让本错误可以参与 `?` 传播到 `Box<dyn Error>` 或 `anyhow::Error`。
///
/// 缺少这个实现时，调用方只能 `unwrap()` 或手工 `map_err`，
/// 库的返回 `Result` 等于白返回值。
impl std::error::Error for OperationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_error_new_sets_message() {
        let err = OperationError::new("test message");
        assert_eq!(err.message, String::from("test message"));
    }

    #[test]
    fn test_operation_error_new_empty_message() {
        let err = OperationError::new("");
        assert_eq!(err.message, String::from(""));
    }

    #[test]
    fn test_display_matches_message() {
        let err = OperationError::new("index out of bounds");
        assert_eq!(format!("{}", err), "index out of bounds");
    }

    #[test]
    fn test_can_propagate_as_std_error() {
        // 能被 `?` 转成 Box<dyn Error>，且可向下转型取回原类型
        fn fallible() -> Result<(), Box<dyn std::error::Error>> {
            Err(OperationError::new("boom"))?
        }
        let e = fallible().unwrap_err();
        assert_eq!(e.to_string(), "boom");
        assert!(e.downcast_ref::<OperationError>().is_some());
    }
}

pub struct ValueError {
    pub message: String,
}
