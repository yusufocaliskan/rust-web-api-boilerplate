use crate::framework::database::IDatabaseService;
use crate::models::todo_model::TodoModel;
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::error::Error;
use mongodb::Collection;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait ITodoRepository: Interface {
    async fn create(&self, todo: TodoModel) -> Result<TodoModel, Error>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<TodoModel>, Error>;
    async fn find_by_user_id(&self, user_id: &ObjectId) -> Result<Vec<TodoModel>, Error>;
    async fn update(&self, id: &ObjectId, todo: TodoModel) -> Result<Option<TodoModel>, Error>;
    async fn delete(&self, id: &ObjectId) -> Result<Option<TodoModel>, Error>;
    async fn find_all(&self) -> Result<Vec<TodoModel>, Error>;
}

#[derive(Component)]
#[shaku(interface = ITodoRepository)]
pub struct TodoRepository {
    #[shaku(inject)]
    database: Arc<dyn IDatabaseService>,
}

#[async_trait]
impl ITodoRepository for TodoRepository {
    async fn create(&self, todo: TodoModel) -> Result<TodoModel, Error> {
        let collection = self.database.get_database().collection("todos");
        collection.insert_one(todo.clone()).await?;
        Ok(todo)
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<TodoModel>, Error> {
        let collection = self.database.get_database().collection("todos");
        collection.find_one(doc! { "_id": id }).await
    }

    async fn find_by_user_id(&self, user_id: &ObjectId) -> Result<Vec<TodoModel>, Error> {
        let collection = self
            .database
            .get_database()
            .collection::<TodoModel>("todos");
        let cursor = collection.find(doc! { "user_id": user_id }).await?;
        let todos: Vec<TodoModel> = cursor.try_collect().await?;
        Ok(todos)
    }

    async fn update(&self, id: &ObjectId, todo: TodoModel) -> Result<Option<TodoModel>, Error> {
        let collection: Collection<TodoModel> = self.database.get_database().collection("todos");
        let filter = doc! { "_id": id };
        let update_at = DateTime::from_chrono(Utc::now());
        let update = doc! {
            "$set": {
                "title": &todo.title,
                "description": &todo.description,
                "completed": &todo.completed,
                "updated_at": update_at
            }
        };

        match collection.find_one_and_update(filter, update).await? {
            Some(_) => Ok(Some(todo)),
            None => Ok(None),
        }
    }

    async fn delete(&self, id: &ObjectId) -> Result<Option<TodoModel>, Error> {
        let collection = self
            .database
            .get_database()
            .collection::<TodoModel>("todos");
        Ok(collection
            .find_one_and_delete(doc! { "_id": id.to_string() })
            .await?)
    }

    async fn find_all(&self) -> Result<Vec<TodoModel>, Error> {
        let collection = self.database.get_database().collection("todos");
        let cursor = collection.find(doc! {}).await?;
        cursor.try_collect().await
    }
}
