#![allow(dead_code)]
use std::{
    fs::File,
    io::{self, BufReader, Read, Error},
};

pub mod lib {
    pub struct SystemTime {
        pub year: u32,
        pub month: u32,
        pub day_of_week: u32,
        pub day: u32,
        pub hour: u32,
        pub minute: u32,
        pub second: u32,
        pub millisecond: u32,
    }


    pub struct FileHeader {
        pub file_id: String,
        pub major_version: u8,
        pub minor_version: u8,
        pub system_time: SystemTime,
    }

}

pub const HEADER_SIZE: usize = 30;
pub const ID_SIZE: usize = 12;

fn get_header(reader: &mut dyn Read) -> lib::FileHeader {

    let mut header_buffer = [0; HEADER_SIZE];
    let _ = reader.read(&mut header_buffer);

    let file_id = String::new();
    for b in &header_buffer[..12] {
        dbg!(b);
    }
    

    let major_version = 0;
    let minor_version = 0;
    let system_time = lib::SystemTime {
        year: 0,
        month: 0,
        day_of_week: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 0,
        millisecond: 0,
    };

    let k = lib::FileHeader {
        file_id,
        major_version,
        minor_version,
        system_time,
    };
    k
}

fn main() -> io::Result<()> {
    let file = File::open("test.ess")?;
    let mut reader_buffer = BufReader::new(file);
    
    let header = get_header(&mut reader_buffer);

    Ok(())
}
