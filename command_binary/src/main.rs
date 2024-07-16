mod command;

use clap::Parser;
pub use command::Command;
use music_dump_lib::NcmDumper;

fn main() -> anyhow::Result<()> {
    let command = Command::parse();
    let music_list = command.get_items()?;
    let ncm_dumper = NcmDumper::new(music_list);
    ncm_dumper.dump_all()?;
    Ok(())
}