use std::fs::File;
use std::io::{Result,prelude::*};

fn main() {
    let _ = write_to_binfile();
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
    file_name_length                :u16,
    // extra_field_length
    file_name                       :Vec<u8>,
    extra_field                     :Vec<u8>,
}

impl LocalFileHeader {
    fn new(file_name: &[u8],filedata: &[u8]) -> Self {
        Self {
            // 固定長30bytes + file_name.len() + extra_fieled_len() + filedata.len()
            local_file_header_signature     :0x04034b50,
            version_needed_to_extract       :10,
            general_purpose_bit_flag        :0,
            compression_method              :0,
            last_mod_file_time              :0,
            last_mod_file_date              :0,
            crc_32                          :crc32(filedata),
            compressed_size                 :filedata.len() as u32,
            uncompressed_size               :filedata.len() as u32,
            file_name_length                :file_name.len() as u16,
            // extra_field_length
            file_name                       :file_name.to_vec(),
            extra_field                     :vec![],
        }
    }
    fn write<W:Write>(&self,writer: &mut W) -> Result<()> {
        writer.write_all(&self.local_file_header_signature.to_le_bytes())?;
        writer.write_all(&self.version_needed_to_extract.to_le_bytes())?;
        writer.write_all(&self.general_purpose_bit_flag.to_le_bytes())?;
        writer.write_all(&self.compression_method.to_le_bytes())?;
        writer.write_all(&self.last_mod_file_time.to_le_bytes())?;
        writer.write_all(&self.last_mod_file_date.to_le_bytes())?;
        writer.write_all(&self.crc_32.to_le_bytes())?;
        writer.write_all(&self.compressed_size.to_le_bytes())?;
        writer.write_all(&self.uncompressed_size.to_le_bytes())?;
        writer.write_all(&self.file_name_length.to_le_bytes())?;

        writer.write_all(&(self.extra_field.len() as u16).to_le_bytes())?;

        writer.write_all(&self.file_name)?;
        writer.write_all(&self.extra_field)?;
        Ok(())
    }
}


struct CentralDirectoryHeader {
    central_file_header_signature   :u32,
    version_made_by                 :u16,
    version_needed_to_extract       :u16,
    general_purpose_bit_flag        :u16,
    compression_method              :u16,
    last_mod_file_time              :u16,
    last_mod_file_date              :u16,
    crc_32                          :u32,
    compressed_size                 :u32,
    uncompressed_size               :u32,
    file_name_length                :u16,
    // extra_field_length              :u16,
    // file_comment_length             :u16,
    disk_number_start               :u16,
    internal_file_attributes        :u16,
    external_file_attributes        :u32,
    relative_offset_of_local_header :u32,


    file_name                       :Vec<u8>,
    extra_field                     :Vec<u8>,
    file_comment                    :Vec<u8>,
}

impl CentralDirectoryHeader {

    fn new(file_name: &[u8],filedata: &[u8]) -> Self {
        Self {
            // 固定長46bytes + file_name + extra + comment 
            central_file_header_signature   :0x02014b50,
            version_made_by                 :0x000A,
            version_needed_to_extract       :10,
            general_purpose_bit_flag        :0x0000,
            compression_method              :0,
            last_mod_file_time              :0,
            last_mod_file_date              :0,
            crc_32                          :crc32(filedata),
            compressed_size                 :filedata.len() as u32,
            uncompressed_size               :filedata.len() as u32,
            file_name_length                :file_name.len() as u16,
            // extra_field_length              :u16,
            // file_comment_length             :u16,
            disk_number_start               :0,
            internal_file_attributes        :0x0001,
            external_file_attributes        :0,
            relative_offset_of_local_header :0,


            file_name                       :file_name.to_vec(),
            extra_field                     :vec![],
            file_comment                    :vec![],
        }
    }
    fn write<W:Write>(&self,writer: &mut W) -> Result<()> {
        writer.write_all(&self.central_file_header_signature.to_le_bytes())?;
        writer.write_all(&self.version_made_by.to_le_bytes())?;
        writer.write_all(&self.version_needed_to_extract.to_le_bytes())?;
        writer.write_all(&self.general_purpose_bit_flag.to_le_bytes())?;
        writer.write_all(&self.compression_method.to_le_bytes())?;
        writer.write_all(&self.last_mod_file_time.to_le_bytes())?;
        writer.write_all(&self.last_mod_file_date.to_le_bytes())?;
        writer.write_all(&self.crc_32.to_le_bytes())?;
        writer.write_all(&self.compressed_size.to_le_bytes())?;
        writer.write_all(&self.uncompressed_size.to_le_bytes())?;
        writer.write_all(&self.file_name_length.to_le_bytes())?;

        writer.write_all(&(self.extra_field.len() as u16).to_le_bytes())?;
        writer.write_all(&(self.file_comment.len() as u16).to_le_bytes())?;

        writer.write_all(&self.disk_number_start.to_le_bytes())?;
        writer.write_all(&self.internal_file_attributes.to_le_bytes())?;
        writer.write_all(&self.external_file_attributes.to_le_bytes())?;
        writer.write_all(&self.relative_offset_of_local_header.to_le_bytes())?;


        writer.write_all(&self.file_name)?;
        writer.write_all(&self.extra_field)?;
        writer.write_all(&self.file_comment)?;

        Ok(())
    }
}


struct EndOfCentralDirectoryRecord {
    end_of_central_dir_signature                                :u32,
    number_of_this_disk                                         :u16,
    number_of_the_disk_with_the_start_of_the_central_directory  :u16,
    total_number_of_entries_in_the_central_directory_on_this_disk:u16,
    total_number_of_entries_in_the_central_directory            :u16,
    size_of_the_central_directory                               :u32,
    offset_of_start_of_central_directory_with_respect_to_the_starting_disk_number :u32,
    // zip_file_comment_length :16u,
    zip_file_comment                                            :Vec<u8>,
}

impl EndOfCentralDirectoryRecord {

    fn new(central_directory_size:u32,central_directory_offset:u32) -> Self {
        Self {
            end_of_central_dir_signature                        :0x06054b50,
            number_of_this_disk                                 :0,
            number_of_the_disk_with_the_start_of_the_central_directory :0,
            total_number_of_entries_in_the_central_directory_on_this_disk:1,
            total_number_of_entries_in_the_central_directory    :1,
            size_of_the_central_directory                       :central_directory_size,
            offset_of_start_of_central_directory_with_respect_to_the_starting_disk_number :central_directory_offset,
            // zip_file_comment_length :16u,
            zip_file_comment :vec![],
        }
    }
    fn write<W:Write>(&self,writer: &mut W) -> Result<()> {
        writer.write_all(&self.end_of_central_dir_signature.to_le_bytes())?;
        writer.write_all(&self.number_of_this_disk.to_le_bytes())?;
        writer.write_all(&self.number_of_the_disk_with_the_start_of_the_central_directory.to_le_bytes())?;
        writer.write_all(&self.total_number_of_entries_in_the_central_directory_on_this_disk.to_le_bytes())?;
        writer.write_all(&self.total_number_of_entries_in_the_central_directory.to_le_bytes())?;
        writer.write_all(&self.size_of_the_central_directory.to_le_bytes())?;
        writer.write_all(&self.offset_of_start_of_central_directory_with_respect_to_the_starting_disk_number.to_le_bytes())?;

        
        writer.write_all(&(self.zip_file_comment.len() as u16).to_le_bytes())?;

        writer.write_all(&self.zip_file_comment)?;

        Ok(())
    }
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