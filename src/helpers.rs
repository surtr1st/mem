use crate::constants::{DEFAULT_FILE, DEFAULT_PATH, HOME};

#[derive(Clone, Copy)]
pub struct MemorizeHelper;

impl MemorizeHelper {
    pub fn use_env(env: &str) -> String {
        std::env::var(env).expect("should been read `HOME` value")
    }

    pub fn use_default_path() -> String {
        let home_dir = Self::use_env(HOME);
        format!("{home_dir}/{DEFAULT_PATH}")
    }

    pub fn use_default_file() -> String {
        let home_dir = Self::use_env(HOME);
        format!("{home_dir}/{DEFAULT_PATH}/{DEFAULT_FILE}")
    }

    pub fn use_left_aligned(items: Vec<&str>) -> String {
        let mut formatted = String::new();
        for item in items {
            formatted += &format!("{:<20}\t", item);
        }
        format!("{}", formatted,)
    }
}
