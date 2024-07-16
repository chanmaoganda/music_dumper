use std::path::PathBuf;

pub trait MetaData {
    fn inject(&self, path_buf: PathBuf);
}