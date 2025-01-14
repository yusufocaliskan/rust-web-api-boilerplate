use crate::framework::utils::helpers::serialize_oid;
use bson::oid::ObjectId;
use serde_derive::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserModel {
    #[serde(rename = "_id", serialize_with = "serialize_oid")]
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,

    #[validate(length(min = 1, message = "First name cannot be empty"))]
    pub first_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: Option<String>,
    #[validate(length(min = 1, message = "First name cannot be empty"))]
    pub first_name: Option<String>,
    pub password: Option<String>,
}
