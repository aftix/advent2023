use clap::Parser;

include!(concat!(env!("OUT_DIR"), "/generate_days.dat"));

#[derive(Parser)]
struct Cli {
    name: String,
}

fn main() {
    let cli = Cli::parse();
    dispatch(cli.name.as_str());
}
