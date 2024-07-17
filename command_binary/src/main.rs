mod command;

use std::path::PathBuf;

use clap::Parser;
pub use command::Command;
use music_dump_lib::NcmDumper;

fn main() -> anyhow::Result<()> {
    let command = Command::parse();
    let music_list = command.get_items()?;
    let output_directory = command.output_path.unwrap();
    let ncm_dumper = NcmDumper::new(music_list, PathBuf::from(&output_directory));
    ncm_dumper.dump_all()?;
    Ok(())
}