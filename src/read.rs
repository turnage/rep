use std::fs::File;
use std::io::Read;
use std::io::Error;

pub fn read(filename: &str) -> Result<Vec<u8>, Error> {
    let mut buffer = Vec::new();
    let mut file = File::open(filename)?;
    let _ = file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
