use std::fs;

mod data;
mod header;

fn main() {
    let path = "data/dice.qoi";
    // let path = "data/wikipedia_008.qoi";
    let contents = fs::read(path).expect("Something went wrong reading the file");
    let header = header::parse_header(contents[..14].to_vec());
    let header = header::validate_header(header);
    println!("Valid header");
    let rest = contents[14..].to_vec();
    data::parse_data(rest, header);
}
