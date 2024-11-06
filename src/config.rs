use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Config {
    version: &'static str,
}
lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config {
        version: "1.0"
    };
}

impl Config {
    pub fn to_json_string() -> String {
        serde_json::to_string(&CONFIG.clone()).unwrap()
    }

    pub fn autotick() -> bool {
        true
    }
}

//待实现：Dolphin的属性、等级的modifiers
