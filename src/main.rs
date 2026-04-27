use rust_phone_number::parser::load_contacts_from_file;

fn main() {
    let contacts = load_contacts_from_file("data/01_simple.json").expect("Failed to load contacts");

    println!("{:#?}", contacts);
}
