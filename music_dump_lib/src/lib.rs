mod ncm;
mod error;
mod crypt;
mod meta_data;

use std::{fs::File, io::BufReader, path::Path};

use crypt::*;

use lofty::{config::WriteOptions, file::{AudioFile, TaggedFile, TaggedFileExt}, probe::Probe, tag::{Accessor, TagExt}};
pub use ncm::*;
pub use error::NcmDecodeError;
pub use meta_data::LoftyMetaData;

#[test]
fn test() {
    let path = Path::new("../target/output/1.mp3");
    let path_bak = Path::new("../target/output/1.bak.mp3");

    let mut bad = read_mpeg(path);
    let bak = read_mpeg(path_bak);
    
    let bak_tag = bak.tag(lofty::tag::TagType::Id3v2).unwrap();
    let bad_tag = bad.tag_mut(lofty::tag::TagType::Id3v2).unwrap();

    assert_eq!(bak_tag.pictures().len(), 1);
    assert_eq!(bad_tag.pictures().len(), 0);

    // let pictures = bak_tag.pictures();

    // bad_tag.set_picture(0, pictures[0].clone());

    // bak_tag.save_to_path(path, WriteOptions::default()).unwrap();
}

fn read_mpeg(path: impl AsRef<Path>) -> TaggedFile {
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);

    Probe::new(reader)
        .guess_file_type()
        .expect("Failed to guess file type")
        .read()
        .expect("Failed to read file")
}