use crate::framework::database::{IDatabase, IDatabaseProvider};
use crate::framework::shared::responser::response_generator::SnarkyResponder;
use crate::models::user_model::UserModel;
use crate::modules::AppModules;
use crate::services::roles_services::IRoleService;
use crate::services::unit_services::IUnitService;
use crate::AppState;
use actix_web::http::StatusCode;
use actix_web::web::{Html, Json};
use actix_web::{web, HttpResponse, Responder};
use shaku_actix::{Inject, InjectProvided};

pub struct UserController {}

impl UserController {
    pub async fn create_user(Json(body): Json<UserModel>) -> impl Responder {
        println!("body: {:?}", body);
        if body.email == "silav@bar.com" {
            return SnarkyResponder::success()
                .payload(body)
                .code(StatusCode::CREATED)
                .build();
        }

        SnarkyResponder::error()
            .code(StatusCode::INTERNAL_SERVER_ERROR)
            .build()
    }
    pub async fn find_all(
        state: web::Data<AppState>,
        // services: web::Data<ServiceContainer>,
        unit_services: Inject<AppModules, dyn IUnitService>,
        roles_services: Inject<AppModules, dyn IRoleService>,
        database: Inject<AppModules, dyn IDatabase>,
        dbb: InjectProvided<AppModules, dyn IDatabaseProvider>,
    ) -> Html {
        // HTTP response'u hemen döndür
        unit_services.find_all();
        roles_services.roles();

        dbb.database()
            .await
            .create_collection("poooooooooooooooooooo")
            .await
            .expect("TODO: panic message");
        // database.get_db().collection("tset")

        Html::new("<p>test</p>")
    }

    pub async fn delete_by_id(state: web::Data<AppState>) -> impl Responder {
        // println!("Silav deleted {:#?}", Self::find_by_user_id());
        HttpResponse::Ok().body("Deleted all")
    }
}
