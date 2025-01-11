use crate::constants::services::service_errors::ServiceError;
use crate::framework::database::IDatabaseService;
use crate::models::user_model::UserModel;
use async_trait::async_trait;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::error::Error;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait IUserRepository: Interface {
    async fn create(&self, user: UserModel) -> Result<Option<UserModel>, Error>;
    async fn update(&self, user: UserModel) -> Result<Option<UserModel>, Error>;
    async fn delete(&self, id: &ObjectId) -> Result<Option<UserModel>, Error>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<UserModel>, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, ServiceError>;
    async fn find_all(&self) -> Result<Vec<UserModel>, Error>;
}

#[derive(Component)]
#[shaku(interface = IUserRepository)]
pub struct UserRepository {
    #[shaku(inject)]
    database: Arc<dyn IDatabaseService>,
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn create(&self, user: UserModel) -> Result<Option<UserModel>, Error> {
        let collection = self.database.get_database().collection("users");
        match collection.insert_one(user.clone()).await {
            Ok(_) => Ok(Some(user)),
            Err(e) => Err(e),
        }
    }
    async fn update(&self, user: UserModel) -> Result<Option<UserModel>, Error> {
        todo!()
    }
    async fn delete(&self, id: &ObjectId) -> Result<Option<UserModel>, Error> {
        todo!()
    }
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<UserModel>, Error> {
        todo!()
    }
    async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, ServiceError> {
        if email.is_empty() {
            return Err(ServiceError::Validation("Email connot be empty".into()));
        }
        let collection = self.database.get_database().collection("users");
        match collection.find_one(doc! {"email": email}).await {
            Ok(user) => Ok(user),
            Err(e) => Err(ServiceError::ConnotCreate("User Connot be created".into())),
        }
    }

    async fn find_all(&self) -> Result<Vec<UserModel>, Error> {
        todo!()
    }
}
