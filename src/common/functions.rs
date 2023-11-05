use std::env::current_dir;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use super::constants::*;

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

pub fn gtautil_exe_full_path() -> PathBuf {
    current_dir().unwrap().join(GTAUTIL_EXE_RELATIVE_PATH)
}

pub fn ensure_dir(path: &Path) -> io::Result<()> {
    println!("Ensuring dir: {:#?}", path);
    fs::create_dir_all(path)
}

pub fn ensure_file(path: &Path) {
    println!("Ensuring file: {:#?}", path);
    OpenOptions::new().write(true).create_new(true).open(path);
}
