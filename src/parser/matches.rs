use std::{path::{PathBuf, Path}, fs::{self, DirEntry}};

use indicatif::ProgressBar;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Matches {
    pub to_be_downsampled: Vec<Match>,
}

#[derive(Debug)]
pub struct Match {
    pub folder: PathBuf,
    pub v0: Option<PathBuf>,
    pub v320: Option<PathBuf>,
}

impl Matches {
    pub fn new() -> Matches {
        Matches { to_be_downsampled: Vec::new() }
    }

    pub fn find_data_to_be_downsampled(&mut self, music_locations: Vec<PathBuf>) -> Result<usize> {
        let mut tot_folders: u64 = 0;

        for folder in &music_locations {
            tot_folders += Matches::count_subfolders(&folder);
        }

        let pb = ProgressBar::new(tot_folders);

        for config_location in music_locations {
            if let Ok(folders) = fs::read_dir(config_location) {
                for folder in folders {
                    if let Ok(folder) = folder {
                        let contains_flac = Matches::folder_contains_flac(&folder)?;
                        if contains_flac {
                            // TODO: Set user option for this
                            let found_v0: Option<PathBuf> = Matches::replace_folder_format(&folder, "V0");
                            let found_320: Option<PathBuf> = Matches::replace_folder_format(&folder, "320");

                            let found_flac: Match = Match {
                                folder: folder.path(),
                                v0: found_v0,
                                v320: found_320,
                            };
                            self.to_be_downsampled.push(found_flac);
                        }
                    }
                    pb.inc(1);
                }
            }
        }
        pb.set_message("Parsing complete");
        Ok(self.to_be_downsampled.len())
    }

    fn folder_contains_flac(dir_entry: &DirEntry) -> Result<bool> {
        if dir_entry.file_type()?.is_dir() {
            let dir_path = dir_entry.path();
            let entries = fs::read_dir(dir_path).unwrap();

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() &&
                   path.extension().is_some() &&
                   path.extension().unwrap() == "flac"
                {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn count_subfolders(locations: &Path) -> u64 {
        let subfolders: Vec<PathBuf> = fs::read_dir(locations)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.path())
            .collect();
        subfolders.len() as u64
    }

    fn replace_folder_format(folder: &DirEntry, replace_to: &str) -> Option<PathBuf> {
        let folder_name = folder
            .file_name()
            .to_string_lossy()
            .into_owned()
            .trim()
            .to_lowercase();

        let lowercase = &folder_name
            .trim()
            .to_lowercase();

        let re: Regex = match Regex::new(r"(flac)") {
            Ok(r)  => r,
            Err(_) => return None,
        };

        let hit = match re.find_iter(lowercase).last() {
            Some(h) => h,
            None    => return None
        };

        let mut start: String = folder_name[0..hit.start()].to_string().to_owned();
        let end = &folder_name[hit.end()..];

        start.push_str(replace_to);
        start.push_str(end);

        Some(Path::new(&start).to_path_buf())
    }
}
