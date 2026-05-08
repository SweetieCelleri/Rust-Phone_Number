use rust_phone_number::parser::load_contacts_from_file;
use rust_phone_number::trie::Trie;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "data/04_common_parts.json";

    let contacts = load_contacts_from_file(input_path)?;

    let mut trie = Trie::new();

    for contact in contacts {
        trie.insert(&contact.nb, &contact.name);
    }

    let puml = trie.to_plantuml();

    let stem = std::path::Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("nom de fichier invalide")?;
    let output_path = format!("graph/{}.puml", stem);

    std::fs::write(&output_path, puml)?;

    println!("Fichier généré : {}", output_path);

    Ok(())
}