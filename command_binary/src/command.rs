
use std::path::PathBuf;

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
}