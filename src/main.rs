#![allow(dead_code)]
pub mod liboblivion;
use byteorder::{LittleEndian, ReadBytesExt};
use liboblivion::savefile::{FileHeader, SaveHeader, SystemTime};
use std::{
    fs::File,
    io::{self, BufReader, Read},
    mem,
};

pub const FILE_HEADER_SIZE: usize = 30;
pub const ID_SIZE: usize = 12;
pub const SYSTEM_TIME_SIZE: usize = mem::size_of::<SystemTime>() / 2;
pub const MAJOR_VERSION: u8 = 0;
pub const MINOR_VERSION: u8 = 125;

fn get_file_header(reader: &mut dyn Read) -> io::Result<FileHeader> {
    let mut id_buffer = [0; ID_SIZE];
    let _ = reader.read(&mut id_buffer);

    let mut file_id = String::new();
    for b in id_buffer {
        file_id.push(b as char);
    }

    if file_id != "TES4SAVEGAME".to_owned() {
        panic!("Invalid save file. Header ID mismatch");
    }

    let major_version = reader.read_u8()?;
    if major_version != MAJOR_VERSION {
        panic!("Invalid save file. Version mismatch");
    }

    let minor_version = reader.read_u8()?;
    if minor_version != MINOR_VERSION {
        panic!("Invalid save file. Version mismatch");
    }


    let mut system_time_v: Vec<u16> = Vec::new();
    for _ in 0..SYSTEM_TIME_SIZE {
        let x = reader.read_u16::<LittleEndian>()?;
        system_time_v.push(x);
    }

    let system_time = SystemTime::new(&system_time_v);

    Ok(FileHeader {
        file_id,
        major_version,
        minor_version,
        system_time,
    })
} 

fn get_save_header(reader: &mut dyn Read) -> io::Result<SaveHeader> {
    // let _ = reader.read(&mut header_buffer);
    let header_version = reader.read_u32::<LittleEndian>()?;
    let header_size = reader.read_u32::<LittleEndian>()?;
    let save_num = reader.read_u32::<LittleEndian>()?;
    let pc_name_len: usize = reader.read_u8()? as usize;
    let mut pc_name = String::new();
    for _ in 0..pc_name_len {
        let c = reader.read_u8()?;
        if c != 0 {
            pc_name.push(c as char);
        } else {
            break;
        }
    }

    // let _header_buffer = [0; 8];
    let pc_level = reader.read_u16::<LittleEndian>()?;

    let location_len: usize = reader.read_u8()? as usize;
    let mut pc_location = String::new();
    for _ in 0..location_len {
        let c = reader.read_u8()?;
        if c != 0 {
            pc_location.push(c as char);
        } else {
            break;
        }
    }

    Ok(SaveHeader {
        header_version,
        header_size,
        save_num,
        pc_name,
        ..SaveHeader::default()
    })
}

fn main() -> io::Result<()> {
    let file = File::open("test.ess")?;
    let mut reader_buffer = BufReader::new(file);

    let file_header = get_file_header(&mut reader_buffer).unwrap();
    let save_header = get_save_header(&mut reader_buffer);
    dbg!(file_header);
    dbg!(save_header);

    Ok(())
}
