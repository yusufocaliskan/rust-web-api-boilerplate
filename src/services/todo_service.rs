use crate::constants::services::service_errors::ServiceError;
use crate::models::todo_model::{CreateTodoDto, TodoModel, UpdateTodoDto};
use crate::repositories::todo_repository::ITodoRepository;
use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::Utc;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait ITodoService: Interface {
    async fn create_todo(&self, dto: CreateTodoDto) -> Result<TodoModel, ServiceError>;
    async fn get_todo(&self, id: ObjectId) -> Result<TodoModel, ServiceError>;
    async fn get_user_todos(&self, user_id: ObjectId) -> Result<Vec<TodoModel>, ServiceError>;
    async fn update_todo(
        &self,
        id: ObjectId,
        dto: UpdateTodoDto,
    ) -> Result<TodoModel, ServiceError>;
    async fn delete_todo(&self, id: ObjectId) -> Result<Option<TodoModel>, ServiceError>;
    async fn get_all_todos(&self) -> Result<Vec<TodoModel>, ServiceError>;
}

#[derive(Component)]
#[shaku(interface = ITodoService)]
pub struct TodoService {
    #[shaku(inject)]
    todo_repository: Arc<dyn ITodoRepository>,
}

#[async_trait]
impl ITodoService for TodoService {
    async fn create_todo(&self, dto: CreateTodoDto) -> Result<TodoModel, ServiceError> {
        let user_id = ObjectId::parse_str(&dto.user_id)
            .map_err(|_| ServiceError::Validation("Invalid user ID".into()))?;

        let todo = TodoModel {
            id: ObjectId::new(),
            title: dto.title,
            description: dto.description,
            completed: false,
            user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.todo_repository
            .create(todo)
            .await
            .map_err(|e| ServiceError::Database(e.to_string()))
    }

    async fn get_todo(&self, id: ObjectId) -> Result<TodoModel, ServiceError> {
        match self.todo_repository.find_by_id(&id).await? {
            Some(todo) => Ok(todo),
            None => Err(ServiceError::NotFound("Todo not found".into())),
        }
    }

    async fn get_user_todos(&self, user_id: ObjectId) -> Result<Vec<TodoModel>, ServiceError> {
        self.todo_repository
            .find_by_user_id(&user_id)
            .await
            .map_err(|e| ServiceError::Database(e.to_string()))
    }

    async fn update_todo(
        &self,
        id: ObjectId,
        dto: UpdateTodoDto,
    ) -> Result<TodoModel, ServiceError> {
        let current_todo = self.get_todo(id).await?;

        let updated_todo = TodoModel {
            title: dto.title.unwrap_or(current_todo.title),
            description: dto.description.unwrap_or(current_todo.description),
            completed: dto.completed.unwrap_or(current_todo.completed),
            updated_at: Utc::now(),
            ..current_todo
        };

        match self
            .todo_repository
            .update(&id, updated_todo.clone())
            .await?
        {
            Some(_) => Ok(updated_todo),
            None => Err(ServiceError::NotFound("Todo not found".into())),
        }
    }

    async fn delete_todo(&self, id: ObjectId) -> Result<Option<TodoModel>, ServiceError> {
        self.todo_repository
            .delete(&id)
            .await
            .map_err(|e| ServiceError::Database(e.to_string()))
    }

    async fn get_all_todos(&self) -> Result<Vec<TodoModel>, ServiceError> {
        self.todo_repository
            .find_all()
            .await
            .map_err(|e| ServiceError::Database(e.to_string()))
    }
}
