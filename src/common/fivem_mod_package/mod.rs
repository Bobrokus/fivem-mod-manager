pub mod metadata;
pub mod package;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::path::Path;

use crate::common::constants::*;

pub use metadata::*;
pub use package::*;

fn register_package(rpf_archive_path: &Path) -> io::Result<Package> {
    let metadata = Metadata::from_rpf_archive(rpf_archive_path)?;
    let package = Package {
        path: rpf_archive_path.to_path_buf(),
        metadata: metadata,
    };

    let mut mods_toml_str = String::new();
    let mut mods_toml_file = OpenOptions::new().read(true).open(MODS_TOML_PATH).unwrap();

    mods_toml_file.read_to_string(&mut mods_toml_str).unwrap();

    let a: HashMap<String, Package> = toml::from_str(&mods_toml_str).unwrap();

    dbg!(&a);

    a.insert(package.path.file_stem().unwrap().to_str(), package);
}
