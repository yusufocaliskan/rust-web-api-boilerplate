use crate::constants::services::service_errors::ServiceError;
use crate::models::user_model::{CreateUserDto, UpdateUserDto, UserModel};
use crate::repositories::user_repository::IUserRepository;
use async_trait::async_trait;
use bson::oid::ObjectId;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait IUsersServices: Interface {
    async fn create_user(&self, dto: CreateUserDto) -> Result<UserModel, ServiceError>;
    async fn update_user(
        &self,
        id: ObjectId,
        dto: UpdateUserDto,
    ) -> Result<UserModel, ServiceError>;
    async fn delete_user(&self, id: ObjectId) -> Result<(), ServiceError>;
    async fn get_user(&self, id: ObjectId) -> Result<UserModel, ServiceError>;
    async fn get_all_users(&self) -> Result<Vec<UserModel>, ServiceError>;
}

#[derive(Component)]
#[shaku(interface = IUsersServices)]
pub struct UsersService {
    #[shaku(inject)]
    user_repository: Arc<dyn IUserRepository>,
}

#[async_trait]
impl IUsersServices for UsersService {
    async fn create_user(&self, dto: CreateUserDto) -> Result<UserModel, ServiceError> {
        if self
            .user_repository
            .find_by_email(&dto.email)
            .await?
            .is_some()
        {
            return Err(ServiceError::AlreadyExists(
                "Email already exists".to_string(),
            ));
        }

        let user = UserModel::new(ObjectId::new(), dto.email, dto.first_name, dto.password);

        match self.user_repository.create(user).await? {
            Some(user) => Ok(user),
            None => Err(ServiceError::ConnotCreate("Could not create user".into())),
        }
    }

    async fn update_user(
        &self,
        id: ObjectId,
        dto: UpdateUserDto,
    ) -> Result<UserModel, ServiceError> {
        let mut user = self.get_user(id).await?;

        if let Some(email) = dto.email {
            user.email = email;
        }
        if let Some(first_name) = dto.first_name {
            user.first_name = first_name;
        }
        if let Some(password) = dto.password {
            user.password = password;
        }

        match self.user_repository.update(user).await? {
            Some(updated_user) => Ok(updated_user),
            None => Err(ServiceError::NotFound("User not found".into())),
        }
    }

    async fn delete_user(&self, id: ObjectId) -> Result<(), ServiceError> {
        match self.user_repository.delete(&id).await? {
            Some(_) => Ok(()),
            None => Err(ServiceError::NotFound("User not found".into())),
        }
    }

    async fn get_user(&self, id: ObjectId) -> Result<UserModel, ServiceError> {
        match self.user_repository.find_by_id(&id).await? {
            Some(user) => Ok(user),
            None => Err(ServiceError::NotFound("User not found".into())),
        }
    }

    async fn get_all_users(&self) -> Result<Vec<UserModel>, ServiceError> {
        Ok(self.user_repository.find_all().await?)
    }
}
