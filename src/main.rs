mod common;
mod gtautil_adapter;
mod rpf_mod;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::ops::Index;
use std::path::PathBuf;

use common::*;
use common::constants::*;
use common::fivem_mod_package::FivemModPackage;

use quick_xml::events::Event;

use crate::common::xml::XmlElement;
use crate::common::xml::XmlNode;

fn find_file_in_dir<P: AsRef<OsStr> + std::convert::AsRef<std::path::Path>>(file_name: &str, dir_path: P) -> io::Result<DirEntry> {

    for entry in fs::read_dir(&dir_path)? {
        let entry = entry?;
        
        dbg!(&entry.file_name());

        if entry.file_name().eq(file_name) {
            return Ok(entry);
        }
    }

    Err(Error::new(ErrorKind::NotFound, "File not found"))
}

fn main() {
    fs::create_dir_all(TEMP_DIR_PATH).unwrap();

    gtautil_adapter::test_util()
        .expect("Util Test Failed")
        .wait()
        .unwrap();

    let test_rpf_extracted_path = PathBuf::from(TEMP_DIR_PATH)
        .join(TEST_RPF_EXTRACTED_PATH_RELATIVE_TO_TEMP);

    let assembly_xml_file_dir_entry = find_file_in_dir(ASSEMBLY_XML_FILE_NAME, &test_rpf_extracted_path)
        .unwrap();

    let mut reader = quick_xml::Reader::from_file(&assembly_xml_file_dir_entry.path())
        .unwrap();

    reader.trim_text(true);

    // Parse the XML file

    let mut element_stack = Vec::<XmlElement>::new();
    let mut tree = Vec::<XmlNode>::new();

    let mut buf = Vec::new(); 

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            
            Ok(Event::Start(e)) => {
                let mut name = String::new();

                e.name()
                    .as_ref()
                    .read_to_string(&mut name)
                    .expect("Failed to read to string");

                let attributes = HashMap::new();

                for attribute in e.attributes() {
                    let attribute = attribute.unwrap();
                    attributes.insert(attribute.key.as_ref(), attribute.value);
                }
                
                let last_element = element_stack.last();
                
                match last_element {
                    Some(XmlElement) => {
                        element_stack.push(XmlElement {
                            name: name,
                            attributes: attributes,
                            content: Vec::new(),
                        });
                    },

                    None => {
                        
                    }
                }

                
            },
            Ok(Event::Text(e)) => {
                    let mut data = String::new();

                    e.as_ref()
                        .read_to_string(&mut data)
                        .unwrap();

                    data = data.trim().to_string();
                    
                    element_stack.pop().push((
                        curr_tag_name.clone(),
                        data
                    ));
            },

            Ok(Event::End(e)) => {
                element_stack.pop();
            }

            Ok(Event::Eof) => break,

            _ => {},
        }

        buf.clear();
    }

    dbg!(&elements);

    let package = FivemModPackage::new();

    package.name = elements.index("name");

    for element in elements.iter() {
        
    }
}