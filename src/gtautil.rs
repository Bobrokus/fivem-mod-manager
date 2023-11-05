use std::env::current_dir;
use std::io;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

use crate::common::functions::gtautil_exe_full_path;
use crate::*;
use common::constants::*;

pub fn extract_archive(archive_path: &Path, output_dir_path: &Path) -> io::Result<Child> {
    Command::new(gtautil_exe_full_path())
        .arg(EXTRACT_ARCHIVE_COMMAND)
        .arg(INPUT_FLAG)
        .arg(archive_path)
        .arg(OUTPUT_FLAG)
        .arg(output_dir_path)
        .stdout(Stdio::piped())
        .spawn()
}

pub fn create_archive(archive_path: &Path, output_dir_path: &Path) -> io::Result<Child> {
    Command::new(current_dir().unwrap().join(gtautil_exe_full_path()))
        .arg(CREATE_ARCHIVE_COMMAND)
        .arg(INPUT_FLAG)
        .arg(archive_path)
        .arg(OUTPUT_FLAG)
        .arg(output_dir_path)
        .stdout(Stdio::piped())
        .spawn()
}
