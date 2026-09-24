use std::fs::File;
use std::io::{Result,prelude::*};
use std::env;

use zip::zip::header:: {
    MetaData,
    LocalFileHeader,
    CentralDirectoryHeader,
    EndOfCentralDirectoryRecord,
};
fn main() {

    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("input file {}",&path);

    let filedata = std::fs::read(path).unwrap();
    let compressed_filedata = zip::compression::deflate::deflate(&filedata);

    let _ = make_zip(path,&filedata,&compressed_filedata).unwrap();
}


fn make_zip(path:&String,filedata:&[u8] , compressed_filedata:&[u8]) -> Result<()>{

    let filename = (*path).as_bytes();
    let compressed_path = format!("{}.zip",path);
    let mut file = File::create(compressed_path)?;

    let metadata = MetaData {
        filename,
        compression_method :0 as u16, // no compression
        crc_32:zip::crc::crc32(compressed_filedata),
        compressed_size :compressed_filedata.len() as u32,
        uncompressed_size :filedata.len() as u32,
    };
    let header = LocalFileHeader::new(&metadata);
    let local_header_offset= file.stream_position()? as u32;
    let central_header = CentralDirectoryHeader::new(&metadata,local_header_offset);

    // local file header 書き込み
    header.write(&mut file)?;
    
    // filedata 書き込み
    file.write_all(compressed_filedata)?;
    let central_header_offset = file.stream_position()? as u32;

    central_header.write(&mut file)?;
    let after_central_directory = file.stream_position()? as u32;
    let central_directory_size = after_central_directory - central_header_offset;

    let end_record = EndOfCentralDirectoryRecord::new(central_directory_size,central_header_offset);
    end_record.write(&mut file)?;

    Ok(())
}

