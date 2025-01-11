use crate::framework::database::IDatabaseService;
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
    pub db: Arc<dyn IDatabaseService>,
}

impl IUnitService for UnitService {
    fn find_all(&self) {
        println!("Hello from User service");
    }
}
