use minargs::App;

fn main() {
    let matches = App::new("minargs_demo")
        .arg("input", |a| a.required(true))
        .arg("verbose", |a| a.short('v').long("verbose").takes_value(false))
        .parse();

    let input = matches.get("input").unwrap();
    println!("Input: {input}");

    if matches.has("verbose") {
        println!("Verbose mode enabled.");
    }
}
