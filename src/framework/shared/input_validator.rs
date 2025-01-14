use actix_web::web::Bytes;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use validator::Validate;

pub fn validate_inputs<T>(body: &Bytes) -> Result<T, HashMap<String, Vec<String>>>
where
    T: Validate + DeserializeOwned,
{
    let mut error_map = HashMap::new();

    //is bodyepty?
    if body.is_empty() {
        error_map.insert(
            "danger".to_string(),
            vec!["Body cannot be empty".to_string()],
        );
        return Err(error_map);
    }

    //  JSON body into the generic type T
    let data: T = match serde_json::from_slice(&body) {
        Ok(val) => val,
        Err(e) => {
            error_map.insert("danger".to_string(), vec![format!("{}", e.to_string())]);
            return Err(error_map);
        }
    };

    //validate input
    if let Err(validation_errors) = data.validate() {
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
            return Err(error_map);
        }
    }

    Ok(data)
}
