use crate::framework::database::IDatabase;
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait IUnitService: Interface {
    fn find_all(&self);
}

#[derive(Component)]
#[shaku(interface=IUnitService)]
pub struct UnitService {
    #[shaku(inject)]
    pub database: Arc<dyn IDatabase>,
}

impl IUnitService for UnitService {
    fn find_all(&self) {
        self.database.test_db();
        self.database.get_db();
        println!("Hello from User service");
    }
}
