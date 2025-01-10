use crate::framework::database::IDatabase;
use async_trait::async_trait;
use colored::Colorize;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait ILessonsService: Interface {
    async fn find_all(&self);
    fn lessons(&self);
}

#[derive(Component)]
#[shaku(interface=ILessonsService)]
pub struct LessonsService {
    #[shaku(inject)]
    database: Arc<dyn IDatabase>,
}

#[async_trait]
impl ILessonsService for LessonsService {
    async fn find_all(&self) {
        let db = self.database.instance();

        db.create_collection("Test")
            .await
            .expect("TODO: panic message");
        println!("Hello from unit --Lessons--> Services");
    }
    fn lessons(&self) {
        println!("{}", "Lessons".blue());
    }
}
