use crate::framework::shared::{
    input_validator::validate_inputs, responser::response_generator::SnarkyResponder,
};
use crate::models::auth_model::LoginDto;
use crate::modules::AppModules;
use crate::services::auth_service::IAuthService;
use actix_web::web::{Bytes, Json};
use actix_web::{http::StatusCode, Responder};
use serde_json::Value;
use shaku_actix::Inject;
use validator::Validate;

pub struct AuthController;

impl AuthController {
    pub async fn login(
        body: Bytes, // Raw body
        auth_service: Inject<AppModules, dyn IAuthService>,
    ) -> impl Responder {
        let login_value: LoginDto = match validate_inputs::<LoginDto>(&body) {
            Ok(data) => data,
            Err(errors) => {
                return SnarkyResponder::error()
                    .error_payload(errors)
                    .code(StatusCode::BAD_REQUEST)
                    .build();
            }
        };

        match auth_service.login(login_value).await {
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
