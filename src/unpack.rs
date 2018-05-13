extern crate csv;
extern crate nom;

use nom::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub count: u32,
}

named!(pub header<Header>,
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
pub struct Metadata {
    pub offset: u32,
    pub size: u32,
    pub name: String,
}

named!(pub metadata<Metadata>,
       do_parse!(
           offset:      le_u32      >>
           size:        le_u32      >>
           name:        take!(24)   >>
           (Metadata {
               offset,
               size: size/2,
               name: String::from_utf8_lossy(name).trim_right_matches('\x00').to_string(),
           })
        )
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dat {
    pub header: Header,
    pub metadata: Vec<Metadata>,
    //pub files: Vec<Vec<u8>>,
}

named!(pub dat<Dat>,
       do_parse!(
            header:     header      >>
            metadata:   count!(metadata, header.count as usize)  >>
            (Dat {
                header,
                metadata,
                //files,
            })
       )
);

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
    let (mut buf, hdr) = header(buf).unwrap();
    println!("Parsing {} files...", hdr.count);

    let metadata: Vec<_> = (0 .. hdr.count).map(|_| {
        let (nbuf, fm) = metadata(buf).unwrap();
        buf = nbuf;
        fm
    }).collect();

    let path = output_dir.join("metadata.csv");
    let file = fs::File::create(path).unwrap();
    let mut metadata_writer = csv::Writer::from_writer(file);

    for (i, file) in metadata.iter().enumerate() {
        let offset = file.offset as usize;
        let end = offset + (file.size as usize);
        
        let path = output_dir.join(&file.name);
        println!("Writing {}, size {}...", path.display(), end - offset);
        let mut file = fs::File::create(path).unwrap();
        file.write_all(&buf[offset .. end]).unwrap();

        metadata_writer.write_record(&[metadata[i].name.clone()]).unwrap();
    }

    println!("Flushing metadata...");
    metadata_writer.flush().unwrap();
}
