use minargs::App;

fn main() {
    let matches = App::new("mytool")
        .version("0.1.0")
        .about("Ultra-light CLI parser demo")
        .arg("input", |a| a.required(true).help("Path to input file"))
        .arg("verbose", |a| a.short('v').long("verbose").takes_value(false))
        .subcommand("init", |cmd| {
            cmd.about("Initialize a project").arg("path", |a| a.help("Optional path"))
        })
        .parse();

    match matches.subcommand() {
        Some("init") => {
            let sub = matches.sub_matches().unwrap();
            println!("Init path: {:?}", sub.get("path"));
        }
        _ => {
            println!("Input: {}", matches.get("input").unwrap());
            if matches.has("verbose") {
                println!("Verbose mode enabled.");
            }
        }
    }
}
