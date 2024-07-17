use std::path::Path;

pub trait MetaData {
    fn inject(&self, path_buf: impl AsRef<Path>);
}