pub fn crc32(data: &[u8]) -> u32 {
    let init_crc  = 0xffffffff;
    let crc32_poly = 0xEDB88320;

    let mut crc = init_crc;


    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ crc32_poly;
            } else {
                crc >>= 1
            };
        }
    }

    !crc
}