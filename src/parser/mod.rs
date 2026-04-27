use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Contact {
    pub nb: String,
    pub name: String,
}

pub fn parse_contacts(json_content: &str) -> Result<Vec<Contact>, serde_json::Error> {
    serde_json::from_str(json_content)
}

pub fn load_contacts_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<Contact>, Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(path)?;
    let contacts = parse_contacts(&json_content)?;
    Ok(contacts)
}