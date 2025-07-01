use crate::arg::Arg;
use std::{collections::HashMap, process};

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

    fn print_help(&self) {
        println!("Usage: {} [OPTIONS] [SUBCOMMAND]", self.name);
        if let Some(desc) = &self.about {
            println!("\n{}", desc);
        }

        if !self.args.is_empty() {
            println!("\nOptions:");
            for arg in &self.args {
                let mut flags = String::new();
                if let Some(s) = arg.short {
                    flags.push_str(&format!("-{}, ", s));
                }
                if let Some(l) = &arg.long {
                    flags.push_str(&format!("--{}", l));
                }
                println!("  {:<15} {}", flags, arg.help.clone().unwrap_or_default());
            }
        }

        if !self.subcommands.is_empty() {
            println!("\nSubcommands:");
            for (name, sub) in &self.subcommands {
                println!("  {:<15} {}", name, sub.about.clone().unwrap_or_default());
            }
        }

        process::exit(0);
    }
}
