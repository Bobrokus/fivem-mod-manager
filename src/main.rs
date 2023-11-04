mod common;
mod gtautil_adapter;

use std::fs;
use std::path::PathBuf;

use common::constants::*;
use common::fivem_mod_package_metadata::FivemModPackageMetadata;

fn ensure_temp_dir_exists() {
    println!("Ensuring temp dir exists");
    fs::create_dir_all(TEMP_DIR_PATH).unwrap();
}

fn test_gtautil() {
    gtautil_adapter::test_util()
        .expect("Util Test Failed")
        .wait()
        .unwrap();

    let test_rpf_extracted_path =
        PathBuf::from(TEMP_DIR_PATH).join(TEST_RPF_EXTRACTED_PATH_RELATIVE_TO_TEMP);

    dbg!(FivemModPackageMetadata::from_extracted_rpf(&test_rpf_extracted_path));
}

fn main() {
    ensure_temp_dir_exists();
    test_gtautil();
}
