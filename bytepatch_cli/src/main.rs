use std::fs::File;
use std::io::{BufReader, Read};
use scroll::Pread;

use bytepatch_core::lua::Header;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("bytecode.bin")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let offset = &mut 0;
    let header: Header = buffer.gread_with(offset, scroll::LE).unwrap();
    println!("Header = {:#?}", header);

    Ok(())
}
