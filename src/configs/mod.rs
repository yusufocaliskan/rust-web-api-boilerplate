use std::collections::HashMap;
use std::env;

#[derive(Clone)]
pub struct Configs {
    pub settings: HashMap<String, String>,
}

impl Configs {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let mut settings = HashMap::new();
        for (key, value) in env::vars() {
            settings.insert(key, value);
        }

        Configs { settings }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }
}
