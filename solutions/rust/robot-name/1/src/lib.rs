use once_cell::sync::Lazy;
use std::collections::HashSet;

use rand::Rng;

pub struct Robot(String);

static mut NAMES: Lazy<HashSet<String>> = Lazy::new(HashSet::new);

impl Robot {
    pub fn new() -> Self {
        unsafe {
            let name = loop {
                let new_name = Self::generate_name();
                if NAMES.insert(new_name.clone()) {
                    break new_name;
                }
            };
            Robot(name)
        }
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        unsafe {
            NAMES.remove(&self.0);
            let name = loop {
                let new_name = Self::generate_name();
                if NAMES.insert(new_name.clone()) {
                    break new_name;
                }
            };
            self.0 = name;
        }
    }

    fn generate_name() -> String {
        let mut rng = rand::thread_rng();
        let mut name = String::new();
        name.push(rng.gen_range(b'A'..=b'Z') as char);
        name.push(rng.gen_range(b'A'..=b'Z') as char);
        name.push(rng.gen_range(b'0'..=b'9') as char);
        name.push(rng.gen_range(b'0'..=b'9') as char);
        name.push(rng.gen_range(b'0'..=b'9') as char);
        name
    }
}
