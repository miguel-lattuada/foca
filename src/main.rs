mod core;
mod interface;

use clap::Parser;
use interface::{
    cli::Cli, cli::Commands, cli_executor::CliExecutor, executor_trait::InterfaceExecutor, file_executor::FileExecutor,
};

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Cli(command) => {
            CliExecutor::new(command).execute()
        }
        Commands::File(file) => {
            FileExecutor::new(file).execute()
        }
    }
}
