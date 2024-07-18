use std::{fs::create_dir, path::PathBuf};

use anyhow::Error;
use clap::Parser;
use glob::glob;


#[derive(Debug, Parser)]
#[command(name = "dump", bin_name = "dump")]
pub struct Command {
    #[arg(value_name = "FILES")]
    pub paths: Vec<String>,
    #[arg(short = 'o', long = "out")]
    pub output_path: Option<String>,
}


impl Command {
    pub fn get_items(&self) -> anyhow::Result<Vec<PathBuf>> {
        let mut items = Vec::new();
        for matcher in self.paths.iter() {
            for entry in glob(matcher)? {
                let path = entry?;
                if !path.is_file() {
                    continue;
                }
                items.push(path);
            }
        }
        Ok(items)
    }

    pub fn get_output_path(&self) -> anyhow::Result<PathBuf> {
        if let Some(path) = &self.output_path {
            return Ok(PathBuf::from(path));
        }
        let default_paths = vec![
            PathBuf::from("output"),
            PathBuf::from("out"),
            PathBuf::from("dump"),
        ];
        for path in default_paths {
            if dir_check_and_create(&path) {
                return Ok(path);
            }
        }
        Err(Error::msg("No output path provided and default paths are occupied"))
    }
}

fn dir_check_and_create(path: &PathBuf) -> bool {
    if !path.exists() {
        create_dir(path).unwrap();
        return true;
    }
    path.is_dir()
}