use clap::{CommandFactory, FromArgMatches, Parser};
#[derive(Parser, Debug)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long, default_value = "default.txt")]
    config: String,
}

impl Opts {
    pub fn get_matches() -> Result<Self, clap::Error> {
        let version = get_version();
        let app = Opts::command().version(version);
        Opts::from_arg_matches(&app.get_matches())
    }
}

fn get_version() -> &'static str {
    "1.0.0"
}

fn main() {
    match Opts::get_matches() {
        Ok(opts) => {
            if opts.verbose {
                println!("Verbose mode is on")
            }
            println!("Config file: {}", opts.config);
        }
        Err(e) => eprintln!("Error parsing arguments: {}", e),
    }
}
