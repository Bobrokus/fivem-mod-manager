use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use super::Metadata;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Package {
    pub path: PathBuf,
    pub metadata: Metadata
}