#[macro_use]
extern crate nom;

use std::env;
use std::io::{Read, Write};
use std::fs;
use std::path::Path;
use nom::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Header {
    count: u32,
}

named!(header<Header>,
       do_parse!(
                        tag!(b"LNK\x00")    >>
            count:      le_u32              >>
            padding:    take!(8)            >>
            (Header {
                count,
            })
      )
);

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileMetadata {
    offset: u32,
    x: u32,
    name: String,
}

named!(file_metadata<FileMetadata>,
       do_parse!(
           offset:      le_u32      >>
           x:           le_u32      >>
           name:        take!(24)   >>
           (FileMetadata {
               offset,
               x,
               name: String::from_utf8_lossy(name).trim_right_matches('\x00').to_string(),
           })
        )
);

fn main() {
    let args: Vec<_> = env::args().collect();
    let dat = &args[1];
    let output_dir = Path::new(&args[2]);

    fs::DirBuilder::new().recursive(true).create(&output_dir).unwrap();

    let mut buf = Vec::<u8>::new();
    fs::File::open(&dat).unwrap().read_to_end(&mut buf).unwrap();

    let buf = &buf;
    let (mut buf, hdr) = header(buf).unwrap();
    println!("Parsing {} files...", hdr.count);

    let metadata: Vec<_> = (0 .. hdr.count).map(|_| {
        let (nbuf, fm) = file_metadata(buf).unwrap();
        buf = nbuf;
        fm
    }).collect();

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
    }
}
