use clap::{Arg, Command};

fn main() {
    let matches = Command::new("clap_demo")
        .arg(Arg::new("input").required(true))
        .arg(Arg::new("verbose").short('v').long("verbose"))
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();
    println!("Input: {input}");

    if matches.contains_id("verbose") {
        println!("Verbose mode enabled.");
    }
}
