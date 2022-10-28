mod batch;
mod threading;
mod builder;

use builder::LoadTestBuilder;
use clap::{arg, command};

fn main() {
    let matches = command!()
        .arg(arg!(-u --url <URL> "define load test URL").required(true))
        .arg(arg!(-r --rate <RATE> "requests per second").default_value("5"))
        .arg(arg!(-d --duration <DURATION> "load test duration in seconds").default_value("10"))
        .get_matches();

    let mut builder = LoadTestBuilder::new();

    if let Some(url) = matches.get_one::<String>("url") {
        builder.url(url.to_owned());
    }

    if let Some(rate) = matches.get_one::<String>("rate") {
        builder.rate(rate.parse::<u8>().unwrap());
    }

    if let Some(duration) = matches.get_one::<String>("duration") {
        builder.duration(duration.parse::<u8>().unwrap());
    }

    builder.execute();
}
