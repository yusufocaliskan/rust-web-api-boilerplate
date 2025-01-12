use crate::constants::services::service_errors::ServiceError;
use crate::models::auth_model::{Claims, LoginDto, TokenResponse};
use crate::repositories::user_repository::IUserRepository;
use async_trait::async_trait;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use shaku::{Component, Interface};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_SECRET: &[u8] = b"test-key";

#[async_trait]
pub trait IAuthService: Interface {
    async fn login(&self, login_dto: LoginDto) -> Result<TokenResponse, ServiceError>;
}

#[derive(Component)]
#[shaku(interface = IAuthService)]
pub struct AuthService {
    #[shaku(inject)]
    user_repository: Arc<dyn IUserRepository>,
}

#[async_trait]

impl IAuthService for AuthService {
    async fn login(&self, login_dto: LoginDto) -> Result<TokenResponse, ServiceError> {
        let user = self
            .user_repository
            .find_by_email(&login_dto.email)
            .await?
            .ok_or(ServiceError::NotFound("User not found".into()))?;

        if user.password != login_dto.password {
            return Err(ServiceError::Validation("Invalid credentials".into()));
        }

        let cloned_user = user.clone();

        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + 24 * 3600;

        let claims = Claims::new(user.id.to_string(), user.email, expiration);

        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
            .map_err(|e| ServiceError::Internal(format!("Token generation failed: {}", e)))?;

        println!("Cloned user: {:?}", cloned_user);
        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            user: cloned_user,
        })
    }
}
