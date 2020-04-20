use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Read, Write, BufReader, Seek, SeekFrom};

extern crate clap;

use clap::{Arg, App, SubCommand};

use afs_extract::AfsFile;

fn main() {
    let matches = App::new("Afs Extract")
        .version("1.0")
        .author("Ruben Gutierrez <dospro@gmail.com>")
        .about("Tool for extracting ADX sound files from AFS files")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input afs file to read")
            .required(true)
            .index(0))
        .arg(Arg::with_name("output_dir")
            .short("od")
            .long("output_dir")
            .help("Sets a custom ADX output folder")
            .takes_value(true))
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();

    //check file exists
    if fs::metadata(filename).is_err() {
        println!("Cannot open {}. Please check the file exists", filename);
        return;
    }

    // get file name without extension
    let path = Path::new(filename);
    let file_stem = String::from(path.file_stem().unwrap().to_string_lossy());

    // Create dir
    let files_path = format!("./{}", file_stem);
    if fs::metadata(&files_path).is_err() {
        println!("Extracting into new folder {}", files_path);
        fs::create_dir(format!("./{}", files_path));
    }

    // Open file
    let mut file = File::open(filename).unwrap();
    let mut afs_file = AfsFile::new(BufReader::new(file)).unwrap();

    // Do extraction
    for entry in afs_file.into_iter() {
        //println!("File {} in position {} has size {}", entry.filename, entry.offset, entry.size);
        if entry.len() == 0 {
            println!("Empty file");
        } else if entry[0] != 0x00 && entry[1] != 0x80 {
            println!("File {} may not be an ADX, skipping", "dummy");
        } else {
            println!("File {} is an ADX, extracting", "dummy");
            let mut adx_file = File::create("dummy").unwrap();
            adx_file.write_all(&entry);
        }
    }
    println!("Done");
}
