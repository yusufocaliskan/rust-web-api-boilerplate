use crate::framework::database::{DatabaseInstance, DatabaseProvider};
use crate::repositories::lessons_repository::LessonsRepository;
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
            DatabaseInstance,
            LessonsService,
            UsersService,

            //repos
            LessonsRepository

        ],
        providers = [DatabaseProvider]
    }
}
