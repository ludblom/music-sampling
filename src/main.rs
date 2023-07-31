use config::ClientConfig;

use crate::parser::Matches;

mod config;
mod parser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut config = ClientConfig::new();
    config.load_config()?;

    let mut matches: Matches = Matches::new();
    let size = match matches.find_data_to_be_downsampled(config.music_locations) {
        Ok(size) => size,
        Err(_) => panic!("Horrible.."),
    };

    println!("Total: {}", size);
    for m in matches.to_be_downsampled {
        println!("{:?}", m);
    }

    Ok(())
}
