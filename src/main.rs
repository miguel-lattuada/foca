mod batch;
mod builder;
mod interface;
mod threading;

use clap::Parser;
use interface::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Cli(command) => {
            // TODO: use command interface executor
        }
        Commands::File(file) => {
            // TODO: use file interface executor
        }
    }
    

    // println!("{:?}", cli.url);

    // let matches = command!()
    //     .arg(arg!(-u --url <URL> "define load test URL").required(true))
    //     .arg(arg!(-r --rate <RATE> "requests per second").default_value("5"))
    //     .arg(arg!(-d --duration <DURATION> "load test duration in seconds").default_value("10"))
    //     .get_matches();

    // let mut builder = LoadTestBuilder::new();

    // if let Some(url) = matches.get_one::<String>("url") {
    //     builder.url(url.to_owned());
    // }

    // if let Some(rate) = matches.get_one::<String>("rate") {
    //     builder.rate(rate.parse::<u8>().unwrap());
    // }

    // if let Some(duration) = matches.get_one::<String>("duration") {
    //     builder.duration(duration.parse::<u8>().unwrap());
    // }

    // builder.execute();
}
