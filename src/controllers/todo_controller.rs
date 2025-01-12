use crate::framework::shared::input_validator::InputValidator;
use crate::framework::shared::responser::response_generator::SnarkyResponder;
use crate::models::todo_model::{CreateTodoDto, UpdateTodoDto};
use crate::modules::AppModules;
use crate::services::todo_service::ITodoService;
use actix_web::web::Json;
use actix_web::{
    http::StatusCode,
    web::{self},
    Responder,
};
use bson::oid::ObjectId;
use shaku_actix::Inject;

pub struct TodoController;

impl TodoController {
    pub async fn create_todo(
        Json(body): web::Json<CreateTodoDto>,
        todo_service: Inject<AppModules, dyn ITodoService>,
    ) -> impl Responder {
        if let Some(e) = InputValidator::validate(&body) {
            return SnarkyResponder::error()
                .message(e)
                .code(StatusCode::BAD_REQUEST)
                .build();
        }
        match todo_service.create_todo(body).await {
            Ok(todo) => SnarkyResponder::success()
                .payload(todo)
                .code(StatusCode::CREATED)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::BAD_REQUEST)
                .build(),
        }
    }

    pub async fn get_todo(
        path: web::Path<String>,
        todo_service: Inject<AppModules, dyn ITodoService>,
    ) -> impl Responder {
        let (id) = path.into_inner();
        let object_id = ObjectId::parse_str(&id).unwrap();
        match todo_service.get_todo(object_id).await {
            Ok(todo) => SnarkyResponder::success()
                .payload(todo)
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::NOT_FOUND)
                .build(),
        }
    }

    pub async fn get_user_todos(
        path: web::Path<String>,
        todo_service: Inject<AppModules, dyn ITodoService>,
    ) -> impl Responder {
        let (user_id) = path.into_inner();

        let object_id = ObjectId::parse_str(user_id).unwrap();
        match todo_service.get_user_todos(object_id).await {
            Ok(todos) => SnarkyResponder::success()
                .payload(todos)
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::INTERNAL_SERVER_ERROR)
                .build(),
        }
    }

    pub async fn update_todo(
        path: web::Path<String>,
        update_dto: web::Json<UpdateTodoDto>,
        todo_service: Inject<AppModules, dyn ITodoService>,
    ) -> impl Responder {
        let (id) = path.into_inner();
        let object_id = ObjectId::parse_str(id).unwrap();
        match todo_service
            .update_todo(object_id, update_dto.into_inner())
            .await
        {
            Ok(todo) => SnarkyResponder::success()
                .payload(todo)
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::BAD_REQUEST)
                .build(),
        }
    }

    pub async fn delete_todo(
        path: web::Path<String>,
        todo_service: Inject<AppModules, dyn ITodoService>,
    ) -> impl Responder {
        let (id) = path.into_inner();
        let object_id = ObjectId::parse_str(id).unwrap();
        match todo_service.delete_todo(object_id).await {
            Ok(_) => SnarkyResponder::success()
                .message("Todo deleted successfully")
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::NOT_FOUND)
                .build(),
        }
    }

    pub async fn get_all_todos(
        todo_service: Inject<AppModules, dyn ITodoService>,
    ) -> impl Responder {
        match todo_service.get_all_todos().await {
            Ok(todos) => SnarkyResponder::success()
                .payload(todos)
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::INTERNAL_SERVER_ERROR)
                .build(),
        }
    }
}
