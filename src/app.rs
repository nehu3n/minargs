use crate::{arg::Arg, matches::Matches};
use std::{collections::HashMap, env, process};

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

    pub fn parse(self) -> Matches {
        let mut args = env::args().skip(1);
        let mut values = HashMap::new();
        let mut flags = HashMap::new();
        let mut positional_args =
            self.args.iter().filter(|a| a.short.is_none() && a.long.is_none()).peekable();

        let mut specs = HashMap::new();
        for arg in &self.args {
            if let Some(short) = arg.short {
                specs.insert(format!("-{}", short), arg);
            }
            if let Some(long) = &arg.long {
                specs.insert(format!("--{}", long), arg);
            }
            specs.insert(arg.name.clone(), arg);
        }

        while let Some(token) = args.next() {
            if token == "--help" || token == "-h" {
                self.print_help();
            }

            if token.starts_with("--") || token.starts_with('-') {
                if let Some(arg) = specs.get(&token) {
                    if arg.takes_value {
                        if let Some(val) = args.next() {
                            values.insert(arg.name.clone(), val);
                        }
                    } else {
                        flags.insert(arg.name.clone(), true);
                    }
                }
            } else if let Some(pos_arg) = positional_args.next() {
                values.insert(pos_arg.name.clone(), token);
            }
        }

        for arg in &self.args {
            if !values.contains_key(&arg.name) {
                if let Some(default) = &arg.default {
                    values.insert(arg.name.clone(), default.clone());
                } else if arg.required {
                    eprintln!("Missing required argument: {}", arg.name);
                    process::exit(1);
                }
            }
        }

        Matches { values, flags, subcommand: None, sub_matches: None }
    }
}
