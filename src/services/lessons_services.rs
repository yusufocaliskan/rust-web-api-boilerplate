use crate::framework::database::IDatabaseService;
use async_trait::async_trait;
use bson::doc;
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
    database: Arc<dyn IDatabaseService>,
}

#[async_trait]
impl ILessonsService for LessonsService {
    async fn find_all(&self) {
        let collection = self.database.get_database().create_collection("test").await;
    }
    fn lessons(&self) {
        println!("{}", "Lessons".blue());
    }
}
