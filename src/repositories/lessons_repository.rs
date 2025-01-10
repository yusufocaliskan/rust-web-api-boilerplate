use async_trait::async_trait;
use shaku::{Component, Interface};

#[async_trait]
pub trait ILessonsRepository: Interface {
    fn delete_all(&self);
}

#[derive(Component)]
#[shaku(interface=ILessonsRepository)]
pub struct LessonsRepository {}

impl ILessonsRepository for LessonsRepository {
    fn delete_all(&self) {
        println!("Delete: Lessonn Respository");
    }
}
