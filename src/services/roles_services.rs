use async_trait::async_trait;
use colored::Colorize;
use shaku::{Component, Interface};

#[async_trait]
pub trait IRoleService: Interface {
    fn find_all(&self);
    fn roles(&self);
}

#[derive(Component)]
#[shaku(interface=IRoleService)]
pub struct RoleService {}

impl IRoleService for RoleService {
    fn find_all(&self) {
        println!("Hello from unit Role Services");
    }
    fn roles(&self) {
        println!("{}", "Rroles".blue());
    }
}
