use clap::{arg, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get(GetArgs),
    Set {
        key: String,
        value: String,
    }
}
#[derive(Parser, Debug, Clone)]
struct GetArgs {
    #[arg()]
    key: String,
}

fn main() {
    let args = Args::parse();
    
    match args.cmd {
        Commands::Get(value) => print!("Get: {:?}", value),
        Commands::Set{key, value} => println!("Set: key={}, value={}", key, value)
    }
}