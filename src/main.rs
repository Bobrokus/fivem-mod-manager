mod common;
mod gtautil;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

use common::constants::*;
use common::fivem_mod_package;
use common::functions::*;

use crate::common::fivem_mod_package::Package;

fn test_gtautil() {
    dbg!(gtautil_exe_full_path());

    let test_mod_metadata =
        fivem_mod_package::Metadata::from_rpf_archive(Path::new(TEST_RPF_PATH)).unwrap();

    dbg!(&test_mod_metadata);
}

fn main() {
    ensure_dir(Path::new(EXTRACTED_RPF_CACHE_PATH)).unwrap();
    ensure_file(Path::new(MODS_TOML_PATH));
    test_gtautil();
}
