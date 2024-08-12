mod command;

use clap::Parser;
pub use command::Command;
use music_dump_lib::NcmDumper;

fn main() -> anyhow::Result<()> {
    let command = Command::parse();
    let music_list = command.get_items()?;
    let output_directory = command.get_output_path()?;
    if !output_directory.exists() {
        return Err(anyhow::Error::msg("Output directory does not exist"));
    }
    let ncm_dumper = NcmDumper::new(music_list, output_directory);
    ncm_dumper.dump_all()?;
    Ok(())
}
