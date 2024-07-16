use std::{fs::File, io::Write};

use crate::{MetaData, NcmMusic};

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