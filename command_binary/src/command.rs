
use clap::Parser;
use glob::glob;


#[derive(Debug, Parser)]
#[command(name = "dump", bin_name = "dump")]
pub struct Command {
    #[arg(short, long)]
    pub paths: Vec<String>,
    #[arg(short = 'o', long = "out")]
    pub output_path: Option<String>,
}


impl Command {
    pub fn get_items(&self) -> anyhow::Result<Vec<PathBuf>> {
        self.paths.iter().for_each(|path| {
            PathBuf::from(path);
            for entry in glob(path)? {
                let path = entry?;
                path.ancestors()
            }
        });
    }
}