#[macro_use]
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
    pub x: u32,
    pub name: String,
}

named!(pub metadata<Metadata>,
       do_parse!(
           offset:      le_u32      >>
           x:           le_u32      >>
           name:        take!(24)   >>
           (Metadata {
               offset,
               x,
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

