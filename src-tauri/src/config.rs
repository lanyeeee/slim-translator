pub struct Config {
    pub from: String,
    pub to: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            from: "auto".to_string(),
            to: "zh".to_string(),
        }
    }
}
