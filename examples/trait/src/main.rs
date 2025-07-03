mod config;

use config::Config;
use minargs::FromArgs;

fn main() {
    let cfg = Config::from_args();
    if cfg.verbose {
        println!("Verbose mode enabled.");
    }
}
