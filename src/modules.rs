use crate::framework::database::DatabaseService;
use crate::repositories::todo_repository::TodoRepository;
use crate::repositories::user_repository::UserRepository;
use crate::services::auth_service::AuthService;
use crate::services::lessons_services::LessonsService;
use crate::services::roles_services::RoleService;
use crate::services::todo_service::TodoService;
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
            AuthService,
            TodoService,

            //Respositories
            UserRepository,
            TodoRepository,
        ],
        providers = []

    }
}
