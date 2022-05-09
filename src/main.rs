use std::fs;

mod header;

fn main() {
    let contents = fs::read("data/dice.qoi").expect("Something went wrong reading the file");
    let header = header::parse_header(contents[..14].to_vec());
    header::validate_header(header);
    println!("Valid header")
}
