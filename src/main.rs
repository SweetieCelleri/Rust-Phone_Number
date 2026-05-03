use rust_phone_number::parser::load_contacts_from_file;
use rust_phone_number::trie::Trie;
use std::fs;

fn main() {
    let input_path = "data/04_common_parts.json";

    let contacts = load_contacts_from_file(input_path)
        .expect("Failed to load contacts");

    let mut trie = Trie::new();

    for contact in contacts {
        trie.insert(&contact.nb, &contact.name);
    }

    let puml = trie.to_plantuml();

    let output_path = format!(
        "graph/{}.puml",
        input_path
            .split('/')
            .next_back()
            .unwrap()
            .replace(".json", "")
    );

    fs::write(&output_path, puml)
        .expect("Failed to write file");

    println!("File generated in {}", output_path);
}