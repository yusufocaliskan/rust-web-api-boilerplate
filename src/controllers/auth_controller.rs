use crate::framework::shared::responser::response_generator::SnarkyResponder;
use crate::models::auth_model::LoginDto;
use crate::modules::AppModules;
use crate::services::auth_service::IAuthService;
use actix_web::{http::StatusCode, web, Responder};
use shaku_actix::Inject;

pub struct AuthController;

impl AuthController {
    pub async fn login(
        login_dto: web::Json<LoginDto>,
        auth_service: Inject<AppModules, dyn IAuthService>,
    ) -> impl Responder {
        match auth_service.login(login_dto.into_inner()).await {
            Ok(token) => SnarkyResponder::success()
                .payload(token)
                .code(StatusCode::OK)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::UNAUTHORIZED)
                .build(),
        }
    }
} 