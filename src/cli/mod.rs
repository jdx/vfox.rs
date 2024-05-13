use clap::{Parser, Subcommand};

mod install;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install(install::Install),
}

pub async fn run() {
    let args = Cli::parse();
    match args.command {
        Commands::Install(install) => install.run().await,
    }
}
