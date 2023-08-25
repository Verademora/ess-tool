pub mod liboblivion;
use byteorder::{LittleEndian, ReadBytesExt};
use liboblivion::savefile::{
    FileHeader, SystemTime, SaveHeader,
};
use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, Read},
    mem,
};

pub const FILE_HEADER_SIZE: usize = 30;
pub const ID_SIZE: usize = 12;
pub const SYSTEM_TIME_SIZE: usize = mem::size_of::<SystemTime>();


fn get_file_header(reader: &mut dyn Read) -> Result<FileHeader, Box<dyn Error>> {
    // let mut header_buffer = [0; FILE_HEADER_SIZE];
    // let _ = reader.read(&mut header_buffer);
    let mut id_buffer = [0; ID_SIZE];
    let _ = reader.read(&mut id_buffer);

    let mut file_id = String::new();
    for b in id_buffer {
        file_id.push(b as char);
    }

    // let [major_version, minor_version] = &header_buffer[12..14] else { todo!() };
    let major_version = reader.read_u8()?;
    let minor_version = reader.read_u8()?;

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

fn get_save_header(reader: &mut dyn Read) -> SaveHeader {
    // let _ = reader.read(&mut header_buffer);
    let header_version = reader.read_u32::<LittleEndian>().unwrap();
    let header_size = reader.read_u32::<LittleEndian>().unwrap();
    let save_num = reader.read_u32::<LittleEndian>().unwrap();
    let pc_name_len: usize = reader.read_u8().unwrap() as usize;
    let mut pc_name = String::new();
    for _ in 0..pc_name_len {
        let c = reader.read_u8().unwrap();
        if c != 0 {
            pc_name.push(c as char);
        } else { break; }
        // pc_name_buffer.push(&reader.read_u8().unwrap());
    }
    
    let mut header_buffer = [0; 8];

    dbg!(header_version);
    dbg!(header_size);
    dbg!(pc_name);

    SaveHeader {
        header_version,
        header_size,
        save_num,
        ..SaveHeader::default()
    }
}

fn main() -> io::Result<()> {
    let file = File::open("test.ess")?;
    let mut reader_buffer = BufReader::new(file);

    let file_header = get_file_header(&mut reader_buffer);
    let save_header = get_save_header(&mut reader_buffer);
    dbg!(file_header);
    dbg!(save_header);

    Ok(())
}
