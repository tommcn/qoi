use criterion::{black_box, criterion_group, criterion_main, Criterion};

use qoi::data;
use qoi::header;
use std::fs;

fn helper_parse(contents: Vec<u8>) -> Vec<data::Pixel> {
    let header = header::parse_header(contents[..14].to_vec());
    let rest = contents[14..].to_vec();
    let decoded = data::parse_data(rest, header.clone());
    return decoded;
}
fn helper_all() -> Vec<data::Pixel> {
    let path = "data/wikipedia_008.qoi";
    let contents = fs::read(path).expect("Something went wrong reading the file");
    let header = header::parse_header(contents[..14].to_vec());
    let rest = contents[14..].to_vec();
    let decoded = data::parse_data(rest, header.clone());
    return decoded;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let path = "data/wikipedia_008.qoi";
    let contents = fs::read(path).expect("Something went wrong reading the file");
    c.bench_function("parse", |b| {
        b.iter(|| helper_parse(black_box(contents.clone())))
    });
    c.bench_function("parse and rw", |b| b.iter(|| helper_all()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
