use std::fs;
use std::fs::DirEntry;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;


pub fn find_file_in_dir(file_name: &str, dir_path: &Path) -> io::Result<DirEntry> {
    for entry in fs::read_dir(&dir_path)? {
        let entry = entry?;

        dbg!(&entry.file_name());

        if entry.file_name().eq(file_name) {
            return Ok(entry);
        }
    }

    Err(Error::new(ErrorKind::NotFound, "File not found"))
}