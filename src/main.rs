use rust_phone_number::parser::load_contacts_from_file;
use rust_phone_number::trie::Trie;
use std::fs;

fn main() {
    let contacts = load_contacts_from_file("data/01_simple.json")
        .expect("Failed to load contacts");

    let mut trie = Trie::new();

    for contact in contacts {
        trie.insert(&contact.nb, &contact.name);
    }

    let puml = trie.to_plantuml();

    fs::write("graph/output.puml", puml)
        .expect("Failed to write file");

    println!("File generated in graph/output.puml");
}