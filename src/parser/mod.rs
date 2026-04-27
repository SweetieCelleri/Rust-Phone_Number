use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Contact {
    pub nb: String,
    pub name: String,
}
