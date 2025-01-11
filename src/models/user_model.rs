use bson::oid::ObjectId;
use serde::Serializer;
use serde_derive::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserModel {
    #[serde(serialize_with = "serialize_oid")]
    #[serde(rename = "id")]
    pub id: ObjectId,
    pub email: String,
    pub first_name: String,
    pub password: String,
}

fn serialize_oid<S>(id: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&id.to_string())
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
