#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub short: Option<char>,
    pub long: Option<String>,
    pub help: Option<String>,
    pub required: bool,
    pub takes_value: bool,
    pub default: Option<String>,
}

impl Arg {
    pub fn new(name: &str) -> Self {
        Arg {
            name: name.to_string(),
            short: None,
            long: None,
            help: None,
            required: false,
            takes_value: true,
            default: None,
        }
    }

    pub fn short(mut self, c: char) -> Self {
        self.short = Some(c);
        self
    }

    pub fn long(mut self, l: &str) -> Self {
        self.long = Some(l.to_string());
        self
    }

    pub fn help(mut self, text: &str) -> Self {
        self.help = Some(text.to_string());
        self
    }

    pub fn required(mut self, yes: bool) -> Self {
        self.required = yes;
        self
    }

    pub fn default(mut self, val: &str) -> Self {
        self.default = Some(val.to_string());
        self
    }

    pub fn takes_value(mut self, yes: bool) -> Self {
        self.takes_value = yes;
        self
    }
}
