use std::{fs::File, io::Write, path::PathBuf};

use crate::{MetaData, NcmDecoder, NcmMusic};

pub struct NcmDumper {
    music_list: Vec<NcmMusic>,
}

impl NcmDumper {
    pub fn dump(&self, music: NcmMusic) -> Result<(), Box::<dyn std::error::Error>> {
        let NcmMusic { metadata, path_buf, audio_data } = music;
        let path = path_buf.as_path();
        let mut file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(&audio_data)?;
        metadata.inject(path_buf);
        Ok(())
    }
}

#[test]
fn dump_test() {
    let mut decoder = NcmDecoder::new(PathBuf::from("./src/test/8bite-honest.ncm"));
    let music = decoder.decode().unwrap();
    let dumper = NcmDumper {
        music_list: vec![],
    };
    dumper.dump(music).unwrap();

}