use crate::arg::Arg;
use std::collections::HashMap;

#[derive(Clone)]
pub struct App {
    name: String,
    version: Option<String>,
    about: Option<String>,
    args: Vec<Arg>,
    subcommands: HashMap<String, App>,
}

impl App {
    pub fn new(name: &str) -> Self {
        App {
            name: name.to_string(),
            version: None,
            about: None,
            args: Vec::new(),
            subcommands: HashMap::new(),
        }
    }

    pub fn version(mut self, ver: &str) -> Self {
        self.version = Some(ver.to_string());
        self
    }

    pub fn about(mut self, desc: &str) -> Self {
        self.about = Some(desc.to_string());
        self
    }

    pub fn arg<F>(mut self, name: &str, build: F) -> Self
    where
        F: FnOnce(Arg) -> Arg,
    {
        let arg = Arg::new(name);
        let arg = build(arg);
        self.args.push(arg);
        self
    }

    pub fn subcommand<F>(mut self, name: &str, build: F) -> Self
    where
        F: FnOnce(App) -> App,
    {
        let cmd = App::new(name);
        let cmd = build(cmd);
        self.subcommands.insert(name.to_string(), cmd);
        self
    }
}
