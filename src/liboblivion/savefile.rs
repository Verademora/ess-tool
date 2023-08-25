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

#[derive(Debug, Default)]
pub struct Screenshot {
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct SaveHeader {
    pub header_version: u32,
    pub header_size: u32,
    pub save_num: u32,
    pub pc_name: String,
    pub pc_level: u16,
    pub pc_location: String,
    pub game_days: f32,
    pub game_ticks: u32,
    pub game_time: SystemTime,
    pub screenshot: Screenshot,
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
