use crate::framework::database::{IDatabase, IDatabaseProvider};
use crate::modules::AppModules;
use crate::services::roles_services::IRoleService;
use crate::services::unit_services::IUnitService;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use shaku_actix::{Inject, InjectProvided};

pub struct UserController {}

impl UserController {
    pub async fn add_new_user(state: web::Data<AppState>) {}

    pub async fn find_all(
        state: web::Data<AppState>,
        // services: web::Data<ServiceContainer>,
        unit_services: Inject<AppModules, dyn IUnitService>,
        roles_services: Inject<AppModules, dyn IRoleService>,
        database: Inject<AppModules, dyn IDatabase>,
        dbb: InjectProvided<AppModules, dyn IDatabaseProvider>,
    ) -> impl Responder {
        // HTTP response'u hemen döndür
        unit_services.find_all();
        roles_services.roles();

        dbb.database()
            .await
            .create_collection("poooooooooooooooooooo")
            .await
            .expect("TODO: panic message");
        // database.get_db().collection("tset")

        HttpResponse::Ok().body(format!("Hello, {}", state.app_name))
    }

    pub async fn delete_by_id(state: web::Data<AppState>) -> impl Responder {
        // println!("Silav deleted {:#?}", Self::find_by_user_id());
        HttpResponse::Ok().body("Deleted all")
    }
}
