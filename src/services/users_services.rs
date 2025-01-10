use async_trait::async_trait;
use colored::Colorize;
use shaku::{Component, Interface};

#[async_trait]
pub trait IUsersServices: Interface {
    fn find_all(&self);
    fn lessons(&self);
}

#[derive(Component)]
#[shaku(interface=IUsersServices)]
pub struct UsersService {}

impl IUsersServices for UsersService {
    fn find_all(&self) {
        println!("Hello from unit --Users--> Services");
    }

    fn lessons(&self) {
        println!("{}", "Users".blue());
    }
}
