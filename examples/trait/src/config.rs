use minargs::{App, FromArgs};

pub struct Config {
    pub input: String,
    pub verbose: bool,
    pub dry_run: bool,
}

impl FromArgs for Config {
    fn from_args() -> Self {
        let matches = App::new("mytrait")
            .version("0.1.0")
            .about("Trait-based CLI parser demo")
            .arg("input", |a| a.required(true).help("Input file"))
            .arg("verbose", |a| {
                a.short('v').long("verbose").takes_value(false).help("Verbose mode")
            })
            .arg("dry_run", |a| {
                a.short('d').long("dry-run").takes_value(false).help("Simulates the execution")
            })
            .parse();

        let input = matches.get("input").unwrap().to_string();
        let verbose = matches.has("verbose");
        let dry_run = matches.has("dry_run");

        Config { input, verbose, dry_run }
    }
}
