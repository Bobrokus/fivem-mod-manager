#[derive(Debug, Default)]
pub struct Author {
    pub display_name: String,
    pub action_link: String,
    pub web: String,
}

#[derive(Debug, Default)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub tag: String,
}

#[derive(Debug, Default)]
pub struct Add {
    pub source_path: String,
    pub target_path: String
}

#[derive(Debug, Default)]
pub struct FivemModPackage {
    pub name: String,
    pub version: Version,
    pub author: Author,
    pub content: Vec<Add>
}

impl FivemModPackage {
    pub fn new() -> FivemModPackage {
        FivemModPackage {
            name: String::new(),
            version: Version {
                major: 0,
                minor: 0,
                tag: String::new(),
            },
            author: Author {
                display_name: String::new(),
                action_link: String::new(),
                web: String::new(),
            },
            content: Vec::new(),
        }
    }
}