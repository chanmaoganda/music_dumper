use std::io::{self, Read, Seek};

pub struct NcmMetaData {
    tag: id3::Tag,
}

impl NcmMetaData {
    pub fn new<T>(reader: T) -> Result<Self, id3::Error>
        where T: Read + Seek {
        let tag = id3::Tag::read_from2(reader)?;
        Ok(Self {tag,})
    }
}

#[test]
pub fn test() -> io::Result<()>{
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut cursor = io::Cursor::new(a);
    cursor.seek(io::SeekFrom::Current(2))?;
    let mut buf = [0; 2];
    cursor.read(&mut buf)?;
    println!("{:?}", buf);
    cursor.seek(io::SeekFrom::Current(2))?;

    cursor.read(&mut buf)?;
    println!("{:?}", buf);
    println!("{}", cursor.position());
    Ok(())
}