pub mod afs_file;
pub mod adx_file;
mod adx_buffer;
mod sample_decoder;

fn get_u16(data: &[u8]) -> u16 {
    ((data[0] as u16) << 8) | (data[1] as u16)
}

fn get_u32(data: &[u8]) -> u32 {
    ((data[0] as u32) << 24) | ((data[1] as u32) << 16) | ((data[2] as u32) << 8) | (data[3] as u32)
}