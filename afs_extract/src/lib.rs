use std::error::Error;
use std::io::{Read, Seek, SeekFrom};

type BoxResult<T> = Result<T, Box<dyn Error>>;

struct ADXEntry {
    offset: u64,
    size: u64,
}

pub struct AfsFile<R>
    where R: Read + Seek {
    file_reader: R,
    current_entry: usize,
    entries: Vec<ADXEntry>,
}

impl<R> AfsFile<R>
    where R: Read + Seek {
    pub fn new(file_reader: R) -> BoxResult<AfsFile<R>> {
        let mut reader = file_reader;
        reader.seek(SeekFrom::Start(4))?; // Skip first 32 bits
        let mut buffer: [u8; 4] = [0; 4];
        reader.read(&mut buffer)?;
        let num_of_entries = u32::from_le_bytes(buffer);
        println!("Num of entries {}", num_of_entries);
        let entries: Vec<ADXEntry> = (0..num_of_entries).map(|_i| {
            let position = AfsFile::read_u32(&mut reader).unwrap();
            let size = AfsFile::read_u32(&mut reader).unwrap();
            ADXEntry {
                offset: position as u64,
                size: size as u64,
            }
        }).collect();
        Ok(AfsFile {
            file_reader: reader,
            current_entry: 0,
            entries,
        })
    }

    fn read_u32(file_reader: &mut R) -> BoxResult<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        file_reader.read(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }
}

impl<R> Iterator for AfsFile<R>
    where R: Read + Seek {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.entries.get(self.current_entry) {
            let mut buffer: Vec<u8> = Vec::new();
            self.file_reader.seek(SeekFrom::Start(entry.offset));
            self.file_reader.by_ref().take(entry.size).read_to_end(&mut buffer);
            self.current_entry += 1;
            Some(buffer)
        } else {
            None
        }
    }
}
