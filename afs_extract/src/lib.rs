use std::error::Error;
use std::path::{Path, PathBuf};
use std::io::{Read, Write, Seek, SeekFrom};
use std::cell::RefCell;

type BoxResult<T> = Result<T, Box<Error>>;

struct ADXEntry {
    offset: u64,
    size: u64,
}

pub struct AfsFile<R>
    where R: Read + Seek {
    file_reader: R,
    entries: Vec<ADXEntry>,
}

impl<R> AfsFile<R>
    where R: Read + Seek {
    pub fn new(file_reader: R) -> BoxResult<AfsFile<R>> {
        let mut reader = file_reader;
        reader.seek(SeekFrom::Start(4))?; // Skip first 32 bits
        let mut buffer: [u8; 4] = [0; 4];
        reader.read(&mut buffer)?;
        let num_of_entries = u32::from_be_bytes(buffer);
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
            entries,
        })
    }

    pub fn get_adx_file_buffer(&mut self, index: u32) -> Option<Vec<u8>> {
        if let Some(entry) = self.entries.get(index as usize) {
            let mut buffer: Vec<u8> = Vec::new();
            self.file_reader.seek(SeekFrom::Start(entry.offset));
            self.file_reader.by_ref().take(entry.size).read(&mut buffer);
            Some(buffer)
        } else {
            None
        }
    }

    pub fn iter(&self) -> AfsIter<R> {
        self.into_iter()
//        AfsIter {
//            current_entry: 0,
//            afs_file: self,
//        }
    }

    pub fn iter_mut(&mut self) -> AfsIterMut<R> {
        AfsIterMut {
            current_entry: 0,
            afs_file: self,
        }
    }

    fn read_u32(file_reader: &mut R) -> BoxResult<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        file_reader.read(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }
}

impl<R> IntoIterator for AfsFile<R>
    where R: Read + Seek {
    type Item = Vec<u8>;
    type IntoIter = AfsIntoIterator<R>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current_entry: 0,
            afs_file: self,
        }
    }
}

pub struct AfsIntoIterator<R>
    where R: Read + Seek {
    current_entry: u32,
    afs_file: AfsFile<R>,
}

impl<R> Iterator for AfsIntoIterator<R>
    where R: Read + Seek {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.afs_file.get_adx_file_buffer(self.current_entry);
        self.current_entry += 1;
        entry
    }
}

pub struct AfsIter<'a, R>
    where R: Read + Seek {
    current_entry: u32,
    afs_file: &'a AfsFile<R>,
}

impl<'a, R> IntoIterator for &'a AfsFile<R>
    where R: Read + Seek {
    type Item = Vec<u8>;
    type IntoIter = AfsIter<'a, R>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, R> Iterator for AfsIter<'a, R>
    where R: Read + Seek {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
//        let entry_index = self.current_entry;
//        let entry = self.afs_file.borrow_mut().get_adx_file_buffer(entry_index);
//        self.current_entry += 1;
//        entry
        None
    }
}

pub struct AfsIterMut<'a, R>
    where R: Read + Seek {
    current_entry: u32,
    afs_file: &'a mut AfsFile<R>,
}

impl<'a, R> Iterator for AfsIterMut<'a, R>
    where R: Read + Seek {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry_index = self.current_entry;
        let entry = self.afs_file.get_adx_file_buffer(entry_index);
        self.current_entry += 1;
        entry
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
