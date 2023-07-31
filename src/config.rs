use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::{fs, path::Path, io::{stdin, Write}};
use anyhow::{anyhow, Error, Result};

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "music-sampling";
const FILE_NAME: &str = "client.yml";

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientConfig {
    pub music_locations: Vec<PathBuf>,
}

pub struct ConfigPaths {
    pub config_file_path: PathBuf,
}

impl ClientConfig {
    pub fn new() -> ClientConfig {
        ClientConfig {
            music_locations: Vec::new(),
        }
    }

    pub fn get_or_build_paths(&self) -> Result<ConfigPaths> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                if !home_config_dir.exists() {
                    fs::create_dir(&home_config_dir)?;
                }

                if !app_config_dir.exists() {
                    fs::create_dir(&app_config_dir)?;
                }

                let config_file_path = &app_config_dir.join(FILE_NAME);

                let paths = ConfigPaths {
                    config_file_path: config_file_path.to_path_buf(),
                };

                Ok(paths)
            }
            None => Err(anyhow!("No $HOME directory found for client config")),
        }
    }

    pub fn load_config(&mut self) -> Result<()> {
        let paths = self.get_or_build_paths()?;
        if paths.config_file_path.exists() {
            let config_string = fs::read_to_string(&paths.config_file_path)?;
            let config_yml: ClientConfig = serde_yaml::from_str(&config_string)?;

            self.music_locations = config_yml.music_locations;

            Ok(())
        } else {
            println!("Config will be saved to {}", paths.config_file_path.display());
            println!("\nHow to setup config file:\n");
            println!("- Music Locations\n\tCopy and paste your full paths to your music libraries. When done, press enter");
            println!("\nIf you want to change your config, edit it manually in '~/.config/red-best/config.yml'\n");
            let music_locations = ClientConfig::get_music_location_from_input()?;

            let config_yml = ClientConfig {
                music_locations,
            };

            let content_yml = serde_yaml::to_string(&config_yml)?;

            let mut new_config = fs::File::create(&paths.config_file_path)?;
            write!(new_config, "{}", content_yml)?;

            self.music_locations = config_yml.music_locations;

            Ok(())
        }
    }

    fn get_music_location_from_input() -> Result<Vec<PathBuf>> {
        let mut location = String::new();
        let mut locations = Vec::new();

        loop {
            println!("\nEnter music library location (nothing to quit): ");
            stdin().read_line(&mut location)?;
            location = location.trim().to_string();
            let location_path = Path::new(&location);

            if location.eq("") {
                return Ok(locations);
            } else if !location_path.exists() {
                println!("That path do not exist, try again. Note: Tilde (~) do not work, type full path.");
                location.clear();
            } else {
                locations.push(location_path.to_path_buf());
                location.clear();
            }
        }
    }
}
