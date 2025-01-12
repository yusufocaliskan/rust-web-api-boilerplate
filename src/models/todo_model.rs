use crate::framework::utils::helpers::serialize_oid;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoModel {
    #[serde(rename = "_id", serialize_with = "serialize_oid")]
    pub id: ObjectId,
    pub title: String,
    pub description: String,
    pub completed: bool,

    #[serde(serialize_with = "serialize_oid")]
    pub user_id: ObjectId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTodoDto {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    pub description: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTodoDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
