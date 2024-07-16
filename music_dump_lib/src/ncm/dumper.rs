use std::{fs::File, io::Write, path::PathBuf};

use crate::{MetaData, NcmDecoder, NcmMusic};

use rayon::prelude::*;

pub struct NcmDumper {
    music_list: Vec<PathBuf>,
}

impl NcmDumper {
    pub fn dump_all(self) -> anyhow::Result<()> {
        self.music_list.into_par_iter()
            .for_each(|path_buf| {
                Self::dump(path_buf).unwrap();
            });
        Ok(())
    }

    fn dump(path_buf: PathBuf) -> anyhow::Result<()> {
        let mut decoder = NcmDecoder::new(path_buf);
        let NcmMusic { 
            metadata, 
            path_buf, 
            audio_data } 
                = decoder.decode()?;
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