use std::fs;

mod data;
mod header;

fn main() {
    // let path = "data/dice.qoi";
    let path = "data/wikipedia_008.qoi";
    let contents = fs::read(path).expect("Something went wrong reading the file");
    let header = header::parse_header(contents[..14].to_vec());
    header::validate_header(header.clone());
    let rest = contents[14..].to_vec();
    let decoded = data::parse_data(rest, header.clone());
    data::save_image(header, decoded);
}
