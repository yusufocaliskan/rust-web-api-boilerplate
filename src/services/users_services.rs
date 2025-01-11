use crate::framework::database::IDatabaseService;
use async_trait::async_trait;
use colored::Colorize;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait IUsersServices: Interface {
    async fn find_all(&self);
    fn lessons(&self);
}

#[derive(Component)]
#[shaku(interface=IUsersServices)]
pub struct UsersService {
    #[shaku(inject)]
    database: Arc<dyn IDatabaseService>,
}

#[async_trait]
impl IUsersServices for UsersService {
    async fn find_all(&self) {
        self.database
            .get_database()
            .create_collection("users")
            .await
            .expect("TODO: panic message");
    }

    fn lessons(&self) {
        println!("{}", "Users".blue());
    }
}
