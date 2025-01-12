use crate::framework::shared::input_validator::InputValidator;
use crate::framework::shared::responser::response_generator::SnarkyResponder;
use crate::framework::utils::helpers::parse_object_id;
use crate::models::user_model::{CreateUserDto, UpdateUserDto};
use crate::modules::AppModules;
use crate::services::users_services::IUsersServices;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{web, Responder};
use shaku_actix::Inject;

pub struct UserController {}

impl UserController {
    pub async fn create_user(
        Json(body): Json<CreateUserDto>,
        user_service: Inject<AppModules, dyn IUsersServices>,
    ) -> impl Responder {
        if let Some(e) = InputValidator::validate(&body) {
            return SnarkyResponder::error()
                .message(e)
                .code(StatusCode::BAD_REQUEST)
                .build();
        }

        match user_service.create_user(body).await {
            Ok(user) => SnarkyResponder::success()
                .payload(user)
                .code(StatusCode::CREATED)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::INTERNAL_SERVER_ERROR)
                .build(),
        }
    }

    pub async fn update_user(
        path: web::Path<String>,
        Json(body): Json<UpdateUserDto>,
        user_service: Inject<AppModules, dyn IUsersServices>,
    ) -> impl Responder {
        let (id) = path.into_inner();

        let object_id = match parse_object_id(&id) {
            Ok(object_id) => object_id,
            Err(err) => return err,
        };

        if let Some(e) = InputValidator::validate(&body) {
            return SnarkyResponder::error()
                .message(e)
                .code(StatusCode::BAD_REQUEST)
                .build();
        }

        match user_service.update_user(object_id, body).await {
            Ok(user) => SnarkyResponder::success()
                .payload(user)
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::INTERNAL_SERVER_ERROR)
                .build(),
        }
    }

    pub async fn delete_user(
        path: web::Path<String>,
        user_service: Inject<AppModules, dyn IUsersServices>,
    ) -> impl Responder {
        let (id) = path.into_inner();

        let object_id = match parse_object_id(&id) {
            Ok(object_id) => object_id,
            Err(err) => return err,
        };

        match user_service.delete_user(object_id).await {
            Ok(_) => SnarkyResponder::success()
                .message("User deleted successfully")
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::INTERNAL_SERVER_ERROR)
                .build(),
        }
    }

    pub async fn get_user(
        path: web::Path<String>,
        user_service: Inject<AppModules, dyn IUsersServices>,
    ) -> impl Responder {
        let (id) = path.into_inner();

        let object_id = match parse_object_id(&id) {
            Ok(object_id) => object_id,
            Err(err) => return err,
        };

        match user_service.get_user(object_id).await {
            Ok(user) => SnarkyResponder::success()
                .payload(user)
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::INTERNAL_SERVER_ERROR)
                .build(),
        }
    }

    pub async fn get_all_users(
        user_service: Inject<AppModules, dyn IUsersServices>,
    ) -> impl Responder {
        match user_service.get_all_users().await {
            Ok(users) => SnarkyResponder::success()
                .payload(users)
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::INTERNAL_SERVER_ERROR)
                .build(),
        }
    }
}
