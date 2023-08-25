pub mod liboblivion;
use byteorder::{LittleEndian, ReadBytesExt};
use liboblivion::savefile::{FileHeader, SystemTime};
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

pub const HEADER_SIZE: usize = 30;
pub const ID_SIZE: usize = 12;

fn get_header(reader: &mut dyn Read) -> FileHeader {
    let mut header_buffer = [0; HEADER_SIZE];
    let _ = reader.read(&mut header_buffer);

    let mut file_id = String::new();
    for b in &header_buffer[..12] {
        file_id.push(*b as char);
    }

    let [major_version, minor_version] = &header_buffer[12..14] else { todo!() };

    let mut system_time_v: Vec<u16> = Vec::new();
    let time_slice = &header_buffer[14..30];
    for mut chunk in time_slice.chunks(2) {
        let x = chunk.read_u16::<LittleEndian>().unwrap();
        system_time_v.push(x);
    }

    let system_time = SystemTime::new(&system_time_v);

    FileHeader {
        file_id,
        major_version: *major_version,
        minor_version: *minor_version,
        system_time,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("test.ess")?;
    let mut reader_buffer = BufReader::new(file);

    let header = get_header(&mut reader_buffer);
    dbg!(header);

    Ok(())
}
