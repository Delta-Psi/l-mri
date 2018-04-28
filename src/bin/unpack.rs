extern crate l_mri;
extern crate csv;

use std::env;
use std::io::{Read, Write};
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    let dat = &args[1];
    let output_dir = Path::new(&args[2]);

    fs::DirBuilder::new().recursive(true).create(&output_dir).unwrap();

    let mut buf = Vec::<u8>::new();
    fs::File::open(&dat).unwrap().read_to_end(&mut buf).unwrap();

    let buf = &buf;
    let (mut buf, hdr) = l_mri::header(buf).unwrap();
    println!("Parsing {} files...", hdr.count);

    let metadata: Vec<_> = (0 .. hdr.count).map(|_| {
        let (nbuf, fm) = l_mri::metadata(buf).unwrap();
        buf = nbuf;
        fm
    }).collect();

    let path = output_dir.join("metadata.csv");
    let file = fs::File::create(path).unwrap();
    let mut metadata_writer = csv::Writer::from_writer(file);

    for (i, file) in metadata.iter().enumerate() {
        let offset = file.offset as usize;
        let end = if i == metadata.len()-1 {
            buf.len()
        } else {
            metadata[i+1].offset as usize
        };
        
        let path = output_dir.join(&file.name);
        println!("Writing {}, size {}...", path.display(), end - offset);
        let mut file = fs::File::create(path).unwrap();
        file.write_all(&buf[offset .. end]).unwrap();

        metadata_writer.write_record(&[metadata[i].name.clone(), metadata[i].x.to_string()]).unwrap();
    }

    println!("Flushing metadata...");
    metadata_writer.flush().unwrap();
}
