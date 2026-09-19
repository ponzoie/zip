use std::fs::File;
use std::io::{Result,prelude::*};
mod zip;

use zip::header:: {
    LocalFileHeader,
    CentralDirectoryHeader,
    EndOfCentralDirectoryRecord,
};
fn main() {
    let _ = write_to_binfile();
}


fn write_to_binfile() -> Result<()>{

    let filedata = "hello zip".as_bytes();
    let path = "sample.zip";
    let file_name = b"sample.zip";
    let mut file = File::create(path)?;
    let header = LocalFileHeader::new(file_name,filedata);

    let central_header = CentralDirectoryHeader::new(file_name,filedata);

    // local file header 書き込み
    header.write(&mut file)?;
    
    // filedata 書き込み
    file.write_all(filedata)?;
    let central_header_offset = file.stream_position()? as u32;

    central_header.write(&mut file)?;
    let after_central_directory = file.stream_position()? as u32;
    let central_directory_size = after_central_directory - central_header_offset;

    let end_record = EndOfCentralDirectoryRecord::new(central_directory_size,central_header_offset);
    end_record.write(&mut file)?;

    Ok(())
}

