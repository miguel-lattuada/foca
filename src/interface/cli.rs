use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "foca",
    author = "Miguel Lattuada <miguellattuada@outlook.com>",
    version = "0.0.1",
    about = "Coomand line tool to perfom load tests",
    long_about = None,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Use options to execute the load test
    Cli(Command),
    /// Use a config file to execute the load test
    File(File),
}

#[derive(Args)]
pub struct Command {
    /// Url to load test
    #[arg(short, long)]
    pub url: String,

    /// Duration in seconds of the load test
    #[arg(short, long)]
    pub duration: u8,

    /// Amount of requests sent per second
    #[arg(short, long)]
    pub rate: u8,
}

#[derive(Args)]
pub struct File {
    /// yaml config file
    #[arg(short, long)]
    pub yaml: Option<String>,

    /// json config file
    #[arg(short, long)]
    pub json: Option<String>,
}
