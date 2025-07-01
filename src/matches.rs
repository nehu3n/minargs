use std::collections::HashMap;

#[derive(Debug)]
pub struct Matches {
    pub values: HashMap<String, String>,
    pub flags: HashMap<String, bool>,
    pub subcommand: Option<String>,
    pub sub_matches: Option<Box<Matches>>,
}

impl Matches {
    pub fn get(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(|s| s.as_str())
    }

    pub fn has(&self, name: &str) -> bool {
        *self.flags.get(name).unwrap_or(&false)
    }

    pub fn subcommand(&self) -> Option<&str> {
        self.subcommand.as_deref()
    }

    pub fn sub_matches(&self) -> Option<&Matches> {
        self.sub_matches.as_deref()
    }
}
