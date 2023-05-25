pub struct Config<'a> {
    pub api_key: &'a str
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Self {
            api_key: "API_KEY_HERE"
        }
    }
}