extern crate byteorder;
extern crate csv;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};

struct FileData {
    filename: String,
    data: Vec<u8>,
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let metadata_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    let metadata = File::open(metadata_path).unwrap();
    let mut metadata_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(metadata);

    let mut output = File::create(output_path).unwrap();

    let mut files = Vec::<FileData>::new();

    for result in metadata_reader.records() {
        let record = result.unwrap();

        let filename = &record[0];
        assert!(filename.len() <= 24);
        let path = metadata_path.with_file_name(filename);
        println!("Reading {:?}...", path);

        let mut data = Vec::<u8>::new();
        File::open(path).unwrap().read_to_end(&mut data).unwrap();

        files.push(FileData {
            filename: filename.to_string(),
            data,
        });
    }

    println!("Writing header ({} files)...", files.len());
    output.write(b"\x4c\x4e\x4b\x00").unwrap();
    output.write_u32::<LittleEndian>(files.len() as u32).unwrap();
    output.write(&[0x00; 8]).unwrap();

    let mut offset: usize = 0;
    for file in files.iter() {
        output.write_u32::<LittleEndian>(offset as u32).unwrap();
        output.write_u32::<LittleEndian>(file.data.len() as u32 * 2).unwrap();
        let mut filename_data = file.filename.clone().into_bytes();
        while filename_data.len() < 24 {
            filename_data.push(0);
        }
        output.write(&filename_data).unwrap();

        offset += file.data.len();
    }

    for file in files.drain(..) {
        println!("Writing {}...", file.filename);

        output.write(&file.data).unwrap();
    }
}

