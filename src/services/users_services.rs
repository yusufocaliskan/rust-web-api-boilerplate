use crate::constants::services::service_errors::ServiceError;
use crate::framework::database::IDatabaseService;
use crate::models::user_model::UserModel;
use crate::repositories::user_repository::IUserRepository;
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait IUsersServices: Interface {
    async fn create_user(&self, user: UserModel) -> Result<Option<UserModel>, ServiceError>;
    async fn find_all(&self);
}

#[derive(Component)]
#[shaku(interface=IUsersServices)]
pub struct UsersService {
    #[shaku(inject)]
    database: Arc<dyn IDatabaseService>,
    #[shaku(inject)]
    user_repository: Arc<dyn IUserRepository>,
}

#[async_trait]
impl IUsersServices for UsersService {
    async fn create_user(&self, user: UserModel) -> Result<Option<UserModel>, ServiceError> {
        println!("---> Creating user ---?");

        if self
            .user_repository
            .find_by_email(&user.email)
            .await?
            .is_some()
        {
            return Err(ServiceError::AlreadyExists(
                "Email already exists".to_string(),
            ));
        }

        let create_user = self.user_repository.create(user).await;

        match create_user {
            Ok(create_user) => Ok(Some(create_user.expect("REASON"))),
            Err(e) => Err(ServiceError::ConnotCreate("connot be create".into())),
        }
    }

    async fn find_all(&self) {
        self.database
            .get_database()
            .create_collection("users")
            .await
            .expect("TODO: panic message");
    }
}
