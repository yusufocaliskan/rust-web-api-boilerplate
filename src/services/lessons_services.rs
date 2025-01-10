use async_trait::async_trait;
use colored::Colorize;
use shaku::{Component, Interface};

#[async_trait]
pub trait ILessonsService: Interface {
    fn find_all(&self);
    fn lessons(&self);
}

#[derive(Component)]
#[shaku(interface=ILessonsService)]
pub struct LessonsService {}

impl ILessonsService for LessonsService {
    fn find_all(&self) {
        println!("Hello from unit --Lessons--> Services");
    }
    fn lessons(&self) {
        println!("{}", "Lessons".blue());
    }
}
