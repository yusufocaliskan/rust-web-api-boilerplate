use crate::framework::database::DatabaseService;
use crate::repositories::user_repository::UserRepository;
use crate::services::lessons_services::LessonsService;
use crate::services::roles_services::RoleService;
use crate::services::unit_services::UnitService;
use crate::services::users_services::UsersService;
use shaku::module;

module! {
    pub AppModules {
        components = [

            //services
            UnitService,
            RoleService,
            DatabaseService,
            LessonsService,
            UsersService,

            //Respositories
            UserRepository,
        ],
        providers = []

    }
}
