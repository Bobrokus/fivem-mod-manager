use std::env::current_dir;
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;

use crate::*;
use common::constants::*;

pub fn gtautil_exe_full_path() -> PathBuf {
    current_dir().unwrap().join(GTAUTIL_EXE_RELATIVE_PATH)
}

pub fn extract_archive<P: AsRef<OsStr>>(archive_path: P, output_dir_path: P) -> io::Result<Child> {
    Command::new(gtautil_exe_full_path())
        .arg(EXTRACT_ARCHIVE_COMMAND)
        .arg(INPUT_FLAG)
        .arg(archive_path)
        .arg(OUTPUT_FLAG)
        .arg(output_dir_path)
        .spawn()
}

pub fn create_archive<P: AsRef<OsStr>>(archive_path: P, output_dir_path: P) -> io::Result<Child> {
    Command::new(current_dir().unwrap().join(gtautil_exe_full_path()))
        .arg(CREATE_ARCHIVE_COMMAND)
        .arg(INPUT_FLAG)
        .arg(archive_path)
        .arg(OUTPUT_FLAG)
        .arg(output_dir_path)
        .spawn()
    
}

pub fn test_util() -> io::Result<Child> {
    dbg!(gtautil_exe_full_path());

    Command::new(current_dir().unwrap().join(gtautil_exe_full_path()))
        .current_dir(current_dir().unwrap())
        .arg(EXTRACT_ARCHIVE_COMMAND)
        .arg(INPUT_FLAG)
        .arg(TEST_RPF_FILE_NAME)
        .arg(OUTPUT_FLAG)
        .arg(TEMP_DIR_PATH.to_string() + TEST_RPF_EXTRACTED_PATH_RELATIVE_TO_TEMP)
        .spawn()
}