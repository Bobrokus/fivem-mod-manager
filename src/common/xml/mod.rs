use std::collections::HashMap;

pub struct XmlElement {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub content: Vec<XmlElement>,
}

pub enum XmlNode {
    Element(XmlElement),
    Value(String),
}