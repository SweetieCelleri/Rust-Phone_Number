use rust_phone_number::parser::load_contacts_from_file;
use rust_phone_number::trie::Trie;

fn main() {
    let contacts = load_contacts_from_file("data/01_simple.json")
        .expect("Failed to load contacts");

    let mut trie = Trie::new();

    for contact in contacts {
        trie.insert(&contact.nb, &contact.name);
    }

    println!("{:?}", trie.collect_all());
}