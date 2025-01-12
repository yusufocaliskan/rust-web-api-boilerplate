use crate::constants::services::service_errors::ServiceError;
use crate::framework::database::IDatabaseService;
use crate::models::user_model::UserModel;
use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use mongodb::error::Error;
use mongodb::Collection;
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
        let collection: Collection<UserModel> = self.database.get_database().collection("users");
        let filter = doc! {"_id": user.id.to_string()};
        let update = doc! {
            "$set": {
                "email": &user.email,
                "first_name": &user.first_name,
                "password": &user.password,
            }
        };

        match collection.find_one_and_update(filter, update).await {
            Ok(_) => Ok(Some(user)),
            Err(e) => Err(e),
        }
    }

    async fn delete(&self, id: &ObjectId) -> Result<Option<UserModel>, Error> {
        let collection = self.database.get_database().collection("users");
        let filter = doc! {"_id": id.to_string()};

        collection.find_one_and_delete(filter).await
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<UserModel>, Error> {
        let collection: Collection<UserModel> = self.database.get_database().collection("users");
        let filter = doc! {"_id": id.to_string()};
        let found = collection.find_one(filter).await;
        found
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, ServiceError> {
        if email.is_empty() {
            return Err(ServiceError::Validation("Email cannot be empty".into()));
        }

        let collection = self.database.get_database().collection("users");
        match collection.find_one(doc! {"email": email}).await {
            Ok(user) => Ok(user),
            Err(e) => Err(ServiceError::Database(e.to_string())),
        }
    }

    async fn find_all(&self) -> Result<Vec<UserModel>, Error> {
        let collection = self
            .database
            .get_database()
            .collection::<UserModel>("users");

        let cursor = collection.find(doc! {}).await?;

        let users: Vec<UserModel> = cursor.try_collect().await?;
        println!("users --> {:#? }", users);

        Ok(users)
    }
}
