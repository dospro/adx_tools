use std::fs::File;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use std::collections::VecDeque;

use super::{get_u16, get_u32};
use super::sample_decoder::Decoder;
use super::adx_buffer::AdxBuffer;
use crate::adx_buffer::AdxBufferCopy;


pub struct AdxFile {
    encoded_buffer: Vec<u8>,
    cri_text: String,
    file_code: u16,
    offset: u16,
    encoding_type: u8,
    block_size: u8,
    sample_bit_depth: u8,
    pub channel_count: u8,
    pub sample_rate: u32,
    sample_count: u32,
    highpass_frequency: u16,
    version: u8,
    has_loop: bool,
    loop_byte_start: u32,
    loop_byte_end: u32,
}


type BoxResult<T> = Result<T, Box<dyn Error>>;

impl AdxFile {
    pub fn new(filename: &str) -> BoxResult<Self> {
        let mut file = File::open(filename)?;
        let mut header: [u8; 20] = [0; 20];
        file.read(&mut header[..])?;
        let file_code = get_u16(&header[0..2]);
        if file_code != 0x8000 {
            println!("This may not be an ADX file. Code {:X?}", file_code);
        }
        let offset = get_u16(&header[2..4]);
        let encoding_type = header[4];
        let block_size = header[5];
        let sample_bit_depth = header[6];
        let channel_count = header[7];
        let sample_rate = get_u32(&header[0x8..0xC]);
        let sample_count = get_u32(&header[0xC..0x10]);
        let highpass_frequency = get_u16(&header[0x10..0x12]);
        let version = header[0x12];

        let mut has_loop = false;
        let mut loop_start = 0;
        let mut loop_end = 0;

        if version == 3 {
            file.seek(SeekFrom::Start(0x18))?;
            let mut loop_header: [u8; 20] = [0; 20];
            file.read(&mut loop_header[..])?;
            has_loop = get_u32(&loop_header[0..4]) != 0;
            loop_start = get_u32(&loop_header[4..8]);
            loop_end = get_u32(&loop_header[12..16]);
        } else if version == 4 {
            file.seek(SeekFrom::Start(0x24))?;
            let mut loop_header: [u8; 20] = [0; 20];
            file.read(&mut loop_header[..])?;
            has_loop = get_u32(&loop_header[0..4]) != 0;
            loop_start = get_u32(&loop_header[4..8]);
            loop_end = get_u32(&loop_header[12..16]);
            if loop_end == 0 {
                has_loop = false;
            }
        }

        file.seek(SeekFrom::Start(offset as u64 - 2))?;
        let mut cri_text = String::new();
        file.by_ref().take(6).read_to_string(&mut cri_text)?;

        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)?;


        return Ok(Self {
            encoded_buffer: buffer,
            cri_text,
            file_code,
            offset,
            encoding_type,
            block_size,
            sample_bit_depth,
            channel_count,
            sample_rate,
            sample_count,
            highpass_frequency,
            version,
            has_loop,
            loop_byte_start: loop_start,
            loop_byte_end: loop_end,
        });
    }

    pub fn print_info(&self) {
        println!("ADX file version: {}", self.version);
        println!("Channels: {}", self.channel_count);
        println!("Offset: {}", self.offset);
        println!("Block size: {}", self.block_size);
        println!("Sample bit rate: {}", self.sample_bit_depth);
        println!("Samples per channel: {}", self.sample_count);
        println!("Sample rate: {}", self.sample_rate);
        println!("Highpass Frequency: {}", self.highpass_frequency);
        println!("Buffer size: {}KB", (self.sample_count as u32 * self.channel_count as u32) / 1024);
        println!("Loop enabled: {}", self.has_loop);
        println!("Loop Start: {}", self.loop_byte_start);
        println!("Loop End: {}", self.loop_byte_end);
        println!("File code: {:X?}", self.file_code);
    }

    pub fn get_decoder(&self) -> BoxResult<AdxBuffer> {
        let decoders: Vec<Decoder> = std::iter::repeat(0)
            .take(self.channel_count as usize)
            .map(|_n| Decoder::new(self.highpass_frequency as u32, self.sample_rate))
            .collect();

        Ok(AdxBuffer {
            encoded_buffer: &self.encoded_buffer[..],
            cache: VecDeque::new(),
            buffer_offset: 0,
            channels: self.channel_count,
            has_loop: self.has_loop,
            loop_byte_start: self.loop_byte_start,
            loop_byte_end: self.loop_byte_end,
            decoders,
        })
    }

    pub fn get_into_iterator(&self) -> BoxResult<AdxBufferCopy> {
        let decoders: Vec<Decoder> = std::iter::repeat(0)
            .take(self.channel_count as usize)
            .map(|_n| Decoder::new(self.highpass_frequency as u32, self.sample_rate))
            .collect();

        Ok(AdxBufferCopy {
            encoded_buffer: self.encoded_buffer.clone(),
            cache: VecDeque::new(),
            buffer_offset: 0,
            channels: self.channel_count,
            has_loop: self.has_loop,
            loop_byte_start: self.loop_byte_start,
            loop_byte_end: self.loop_byte_end,
            decoders,
        })
    }
}