use bson::oid::ObjectId;
use serde_derive::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub id: ObjectId,
    pub email: String,
    pub first_name: String,
    pub password: String,
}

impl UserModel {
    pub fn new(id: ObjectId, email: String, first_name: String, password: String) -> Self {
        Self {
            id,
            email,
            first_name,
            password,
        }
    }
}

//DTO
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateUserDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    pub first_name: String,
    pub password: String,
}
