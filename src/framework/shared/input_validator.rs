use std::collections::HashMap;
use validator::Validate;

pub struct InputValidator<T: Validate>(T);

impl<T: Validate> InputValidator<T> {
    pub fn validate(data: &T) -> Option<String> {
        if let Err(validation_errors) = data.validate() {
            for (_, field_errors) in validation_errors.field_errors() {
                if let Some(error) = field_errors.first() {
                    return Some(error.to_string());
                }
            }
        }
        None
    }
    // Returns all errors grouped by field
    pub fn validate_all(data: &T) -> Option<HashMap<String, Vec<String>>> {
        if let Err(validation_errors) = data.validate() {
            let mut error_map = HashMap::new();

            for (field, errors) in validation_errors.field_errors() {
                let error_messages: Vec<String> = errors
                    .iter()
                    .map(|error| {
                        error
                            .message
                            .as_ref()
                            .map_or_else(|| "Validation error".to_string(), |msg| msg.to_string())
                    })
                    .collect();

                if !error_messages.is_empty() {
                    error_map.insert(field.to_string(), error_messages);
                }
            }

            if !error_map.is_empty() {
                return Some(error_map);
            }
        }
        None
    }
}
