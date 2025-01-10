use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    pub user_id: i32,
    pub email: String,
    pub first_name: String,
    pub password: String,
}
