use std::fs::File;
use std::io::{Result,prelude::*};

fn main() {
}


struct LocalFileHeader {
    local_file_header_signature     :u32,
    version_needed_to_extract       :u16,
    general_purpose_bit_flag        :u16,
    compression_method              :u16,
    last_mod_file_time              :u16,
    last_mod_file_date              :u16,
    crc_32                          :u32,
    compressed_size                 :u32,
    uncompressed_size               :u32,
    file_name                       :Vec<u8>,
    extra_field_length              :Vec<u8>,
}

impl LocalFileHeader {
    fn.write<W:write>(&self,writer: &mut W) -> Result<()> {
        writer.write_all(&self.local_file_header_signature.to_le_bytes())?;
        writer.write_all(&self.version_needed_to_extract.to_le_bytes())?;
        writer.write_all(&self.general_purpose_bit_flag.to_le_bytes())?;
        writer.write_all(&self.compression_method.to_le_bytes())?;
        writer.write_all(&self.last_mod_file_date.to_le_bytes())?;
        writer.write_all(&self.last_mod_file_time.to_le_bytes())?;
        writer.write_all(&self.crc32.to_le_bytes())?;
        writer.write_all(&self.compressed_size.to_le_bytes())?;
        writer.write_all(&self.uncompressed_size.to_le_bytes())?;

        writer.write_all(&(self.file_name.len() as u16).to_le_bytes())?;
        writer.write_all(&(self.extra_filed.len() as u16).to_le_bytes())?;

        writer.write_all(&self.file_name.to_le_bytes())?;
        writer.write_all(&self.extra_field_length.to_le_bytes())?;
        Ok(())
    }
}
fn write_to_binfile() -> Result<()>{

    let filedata = "hello zip".as_bytes();

    let leader = LocalFileHeader {
        local_file_header_signature: 0x04034b50,
        version_needed_to_extract: 10,
        general_purpose_bit_flag: 0,
        compression_method: 0,
        last_mod_file_time: 0,
        last_mod_file_date: 0,
        crc_32: crc32(filedata),
        compressed_size: filedata.len(),
        uncompressed_size: filedata.len(),
        file_name: b"sample".to_vec(),
        extra_filed: vec![],
    };

    let mut file = File::create("sample.zip")?;
    
    // local file header 書き込み
    header.write(&mut file);

    // filedata 書き込み
    file.write_all(filedata);
}


// https://note.com/dreamy_stilt3370/n/n9b9739ce53c8
fn crc32(data: &[u8]) -> u32 {
    let init_crc  = 0xffffffff;
    let crc32_poly = 0xEDB88320; // 次数のアレ
    let xor = 0xffffffff;

    let mut crc = init_crc;


    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if (crc & 1) != 0 {
                (crc >> 1) ^ crc32_poly
            } else {
                crc >> 1
            };
        }
    }

    crc ^ xor
}