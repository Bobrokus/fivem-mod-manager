use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use roxmltree::Document;
use roxmltree::Node;

use crate::gtautil_adapter::extract_archive;

use super::constants::ASSEMBLY_XML_FILE_NAME;
use super::constants::TEMP_DIR_PATH;
use super::functions::*;

#[derive(Debug, Default)]
pub struct Author {
    pub display_name: String,
    pub links: HashMap<String, String>,
}

#[derive(Debug, Default)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub tag: String,
}

#[derive(Debug, Default)]
pub struct Add {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
}

#[derive(Debug, Default)]
pub struct FivemModPackageMetadata {
    pub name: String,
    pub version: Version,
    pub author: Author,
    pub content: Vec<Add>,
}

fn find_first_element_in_xml_document<'a>(
    document: &'a Document<'a>,
    element_name: &str,
) -> Option<Node<'a, 'a>> {
    return document
        .descendants()
        .find(|&x| x.tag_name().name() == element_name);
}

impl FivemModPackageMetadata {
    pub fn new() -> FivemModPackageMetadata {
        FivemModPackageMetadata {
            name: String::new(),
            version: Version {
                major: 0,
                minor: 0,
                tag: String::new(),
            },
            author: Author {
                display_name: String::new(),
                links: HashMap::new(),
            },
            content: Vec::new(),
        }
    }

    pub fn from_assembly_xml(path: &Path) -> io::Result<FivemModPackageMetadata> {
        if path.file_name() != Some(OsStr::new("assembly.xml")) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Path is not a assembly.xml file",
            ));
        }

        let assembly_xml_string = fs::read_to_string(path)?;
        let xml_document = roxmltree::Document::parse(assembly_xml_string.as_str()).unwrap();

        dbg!(&xml_document);

        let mut package = FivemModPackageMetadata::new();

        let mut version = Version::default();

        let version_element_children = find_first_element_in_xml_document(&xml_document, "version")
            .unwrap()
            .children();

        for child_element in version_element_children {
            if !child_element.is_element() {
                continue;
            }

            let child_element_text_option = child_element.text();

            if child_element_text_option == None {
                continue;
            }

            let child_element_text = child_element_text_option.unwrap();

            let child_element_name = child_element.tag_name().name();

            match child_element_name {
                "major" => {
                    version.major = child_element_text.parse().unwrap();
                }
                "minor" => version.minor = child_element_text.parse().unwrap(),
                "tag" => {
                    version.tag = child_element_text.parse().unwrap();
                }
                _ => {}
            }
        }

        let mut author = Author::default();

        let author_element_children = find_first_element_in_xml_document(&xml_document, "author")
            .unwrap()
            .children();

        for child_element in author_element_children {
            if !child_element.is_element() {
                continue;
            }

            let child_element_text_option = child_element.text();

            if child_element_text_option == None {
                continue;
            }

            let child_element_text = child_element_text_option.unwrap();

            let child_element_name = child_element.tag_name().name();

            match child_element_name {
                "displayName" => author.display_name = child_element_text.to_string(),

                _ => {
                    author.links.insert(
                        child_element_name.to_string(),
                        child_element_text.to_string(),
                    );
                }
            }
        }

        let mut content = Vec::<Add>::new();

        let content_element_children = find_first_element_in_xml_document(&xml_document, "content")
            .unwrap()
            .children();

        for child_element in content_element_children {
            dbg!(child_element);
            
            if !child_element.is_element() {
                continue;
            }

            let source_path = PathBuf::from(child_element.attribute("source").unwrap().to_string());
            let target_path = PathBuf::from(child_element.text().unwrap().to_string());
            
            content.push(Add {
                source_path,
                target_path,
            });
        }

        package.name = find_first_element_in_xml_document(&xml_document, "name")
            .unwrap()
            .text()
            .unwrap()
            .to_string();

        package.version = version;

        package.author = author;

        package.content = content;

        Ok(dbg!(package))
    }

    pub fn from_extracted_rpf(path: &Path) -> io::Result<FivemModPackageMetadata> {
        FivemModPackageMetadata::from_assembly_xml(
            find_file_in_dir(ASSEMBLY_XML_FILE_NAME, &path)
                .unwrap()
                .path()
                .as_path(),
        )
    }

    pub fn from_rpf_archive(path: &Path) -> io::Result<FivemModPackageMetadata> {
        let output_path = Path::new(TEMP_DIR_PATH).join(path.file_name().unwrap());
        extract_archive(path, &output_path.as_path());

        FivemModPackageMetadata::from_extracted_rpf(&output_path.as_path())
    }
}
