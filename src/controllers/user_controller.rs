use crate::framework::database::IDatabaseService;
use crate::framework::shared::responser::response_generator::SnarkyResponder;
use crate::framework::utils::helpers::validate_inputs;
use crate::models::user_model::{CreateUserDto, UserModel};
use crate::modules::AppModules;
use crate::services::roles_services::IRoleService;
use crate::services::unit_services::IUnitService;
use crate::services::users_services::IUsersServices;
use crate::AppState;
use actix_web::http::StatusCode;
use actix_web::web::{Html, Json};
use actix_web::{web, HttpResponse, Responder};
use bson::oid::ObjectId;
use shaku_actix::Inject;

pub struct UserController {}

impl UserController {
    pub async fn create_user(
        Json(body): Json<CreateUserDto>,
        user_service: Inject<AppModules, dyn IUsersServices>,
    ) -> impl Responder {
        //check validations
        if let Some(e) = validate_inputs(&body) {
            return SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::CREATED)
                .build();
        }

        let user = UserModel::new(ObjectId::new(), body.email, body.first_name, body.password);

        match user_service.create_user(user).await {
            Ok(created_user) => SnarkyResponder::success()
                .payload(created_user)
                .code(StatusCode::CREATED)
                .build(),
            Err(e) => SnarkyResponder::error()
                .message(e.to_string())
                .code(StatusCode::INTERNAL_SERVER_ERROR)
                .build(),
        }
    }

    pub async fn get_an_object_id() -> impl Responder {
        let id = ObjectId::new();
        SnarkyResponder::success()
            .payload({ id })
            .code(StatusCode::CREATED)
            .build()
    }
    pub async fn find_all(
        state: web::Data<AppState>,

        unit_services: Inject<AppModules, dyn IUnitService>,
        roles_services: Inject<AppModules, dyn IRoleService>,
        database: Inject<AppModules, dyn IDatabaseService>,
    ) -> Html {
        // HTTP response'u hemen döndür
        unit_services.find_all();
        roles_services.roles();

        Html::new("<p>test</p>")
    }

    pub async fn delete_by_id(state: web::Data<AppState>) -> impl Responder {
        // println!("Silav deleted {:#?}", Self::find_by_user_id());
        HttpResponse::Ok().body("Deleted all")
    }
}
