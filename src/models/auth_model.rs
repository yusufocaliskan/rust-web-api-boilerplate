use crate::models::user_model::UserModel;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email must not exceed 255 characters"))]
    pub email: String,

    #[validate(length(min = 6, max = 30, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: UserModel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

pub type TokenClaims = Claims;

impl Claims {
    pub fn new(sub: String, email: String, exp: usize) -> Self {
        Self { sub, email, exp }
    }
}
