use core::panic;
use std::{num::Wrapping, path::Path};

use crate::header;

enum ChunkTypes {
    QoiOpRgb,
    QoiOpRgba,
    QoiOpIndex,
    QoiOpDiff,
    QoiOpLuma,
    QoiOpRun,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub fn parse_data(data: Vec<u8>, header: header::Header) {
    let mut decoded: Vec<Pixel> = Vec::with_capacity(64 as usize);
    let mut i = 0;
    let mut byte = data[i];
    let mut prev = Pixel {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    let mut seen: Vec<Pixel> = vec![prev; 64];
    loop {
        let chunk_type = get_chunk_type(byte);
        let cur_pixel: Pixel;

        match chunk_type {
            ChunkTypes::QoiOpRgb => {
                cur_pixel = parse_qoi_op_rgb(data[i..(i + 4)].to_vec(), prev);
                i += 4;
            }
            ChunkTypes::QoiOpRgba => {
                cur_pixel = parse_qoi_op_rgba(data[i..(i + 5)].to_vec());
                i += 5;
            }
            ChunkTypes::QoiOpIndex => {
                cur_pixel = parse_qoi_op_index(data[i], seen.clone());
                i += 1;
            }
            ChunkTypes::QoiOpDiff => {
                cur_pixel = parse_qoi_op_diff(data[i], prev);
                i += 1;
            }
            ChunkTypes::QoiOpLuma => {
                cur_pixel = parse_qoi_op_luma(data[i..(i + 2)].to_vec(), prev);
                i += 2;
            }
            ChunkTypes::QoiOpRun => {
                let mut res = parse_qoi_op_run(data[i], prev);
                decoded.append(&mut res);
                cur_pixel = decoded.pop().unwrap();

                i += 1;
            }
        };
        decoded.push(cur_pixel);
        prev = cur_pixel;

        let seen_idx = ((prev.r as u16) * 3
            + (prev.g as u16) * 5
            + (prev.b as u16) * 7
            + (prev.a as u16) * 11)
            % 64;

        seen[seen_idx as usize] = prev;
        if decoded.len() >= (header.height * header.width) as usize {
            break;
        }

        byte = data[i]
    }
    println!("Done parsing, saving...");
    save_image(header, decoded);
    println!("Saved");
}

fn save_image(header: header::Header, decoded: Vec<Pixel>) -> bool {
    let h = header.height;
    let w = header.width;
    let mut data: Vec<u8> = vec![0; 0];

    for p in decoded {
        data.push(p.r);
        data.push(p.g);
        data.push(p.b);
        data.push(p.a);
    }
    let c: &[u8] = &data;
    image::save_buffer(&Path::new("img-gen.png"), c, w, h, image::ColorType::Rgba8)
        .expect("wrong data size");
    return true;
}

fn parse_qoi_op_rgb(data: Vec<u8>, previous: Pixel) -> Pixel {
    let r = data[1];
    let g = data[2];
    let b = data[3];
    return Pixel {
        r,
        g,
        b,
        a: previous.a,
    };
}

fn parse_qoi_op_rgba(data: Vec<u8>) -> Pixel {
    let r = data[1];
    let g = data[2];
    let b = data[3];
    let a = data[4];
    return Pixel { r, g, b, a };
}

fn parse_qoi_op_index(data: u8, seen: Vec<Pixel>) -> Pixel {
    return seen[data as usize];
}

fn parse_qoi_op_diff(data: u8, previous: Pixel) -> Pixel {
    let dr: i16 = (((data as i16) & 0b00110000) >> 4) - 2;
    let dg: i16 = (((data as i16) & 0b00001100) >> 2) - 2;
    let db: i16 = (((data as i16) & 0b00000011) >> 0) - 2;

    let r = Wrapping((previous.r as u16) + (((data >> 4) & 0x03) as u16)) - Wrapping(2);
    let g = Wrapping((previous.g as u16) + (((data >> 2) & 0x03) as u16)) - Wrapping(2);
    let b = Wrapping((previous.b as u16) + (((data >> 0) & 0x03) as u16)) - Wrapping(2);

    return Pixel {
        r: (r.0 % 255) as u8,
        g: (g.0 % 255) as u8,
        b: (b.0 % 255) as u8,
        a: (previous.a),
    };
}

fn parse_qoi_op_luma(data: Vec<u8>, previous: Pixel) -> Pixel {
    let dg = Wrapping(data[0] & 0b00111111) - Wrapping(32);
    let dr = dg - Wrapping(8) + Wrapping((data[1] >> 4) & 0b00001111);
    let db = dg - Wrapping(8) + Wrapping(data[1] & 0b00001111);
    return Pixel {
        r: (Wrapping(previous.r) + dr).0,
        g: (Wrapping(previous.g) + dg).0,
        b: (Wrapping(previous.b) + db).0,
        a: (previous.a),
    };
}

fn parse_qoi_op_run(byte: u8, previous: Pixel) -> Vec<Pixel> {
    return vec![previous; ((byte & 0b00111111) + 1) as usize];
}

fn get_chunk_type(byte: u8) -> ChunkTypes {
    match byte {
        0b11111110 => return ChunkTypes::QoiOpRgb,
        0b11111111 => return ChunkTypes::QoiOpRgba,
        _ => match byte >> 6 {
            0b00 => return ChunkTypes::QoiOpIndex,
            0b01 => return ChunkTypes::QoiOpDiff,
            0b10 => return ChunkTypes::QoiOpLuma,
            0b11 => return ChunkTypes::QoiOpRun,
            _ => panic!("Unknow chunk type, {}", byte >> 6),
        },
    }
}
