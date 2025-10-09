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
}

pub struct ValueError {
    pub message: String,
}
