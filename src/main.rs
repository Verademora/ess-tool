#![allow(dead_code)]
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fs::File,
    io::{self, BufReader, Error, Read},
};

pub mod lib {

    #[derive(Debug, Default)]
    pub struct SystemTime {
        pub year: u16,
        pub month: u16,
        pub day_of_week: u16,
        pub day: u16,
        pub hour: u16,
        pub minute: u16,
        pub second: u16,
        pub millisecond: u16,
    }

    #[derive(Debug, Default)]
    pub struct FileHeader {
        pub file_id: String,
        pub major_version: u8,
        pub minor_version: u8,
        pub system_time: SystemTime,
    }

    impl SystemTime {
        pub fn new(fields: &[u16]) -> Self {
            if fields.len() != 8 {
                panic!("Provided vector is not the right size");
            }

            Self {
                year: fields[0],
                month: fields[1],
                day_of_week: fields[2],
                day: fields[3],
                hour: fields[4],
                minute: fields[5],
                second: fields[6],
                millisecond: fields[7],
            }
        }
    }
}

pub const HEADER_SIZE: usize = 30;
pub const ID_SIZE: usize = 12;

fn get_header(reader: &mut dyn Read) -> lib::FileHeader {
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

    let system_time = lib::SystemTime::new(&system_time_v);

    lib::FileHeader {
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
