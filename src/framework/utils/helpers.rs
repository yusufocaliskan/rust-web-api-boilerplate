use validator::Validate;

//Validaate given dto
//returns the first one
pub fn validate_inputs<T: Validate>(data: &T) -> Option<String> {
    if let Err(validation_error) = data.validate() {
        for (_, field_errors) in validation_error.field_errors() {
            if let Some(error) = field_errors.first() {
                return Some(format!("{}", error));
            }
        }
    }
    None
}
