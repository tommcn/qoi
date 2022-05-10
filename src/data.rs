use crate::header;

enum ChunkTypes {
    QOI_OP_RGB,
    QOI_OP_RGBA,
    QOI_OP_INDEX,
    QOI_OP_DIFF,
    QOI_OP_LUMA,
    QOI_OP_RUN,
}

#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub fn parse_data(data: Vec<u8>, header: header::Header) {
    let mut decoded: Vec<Pixel> = Vec::new();
    let mut i = 0;
    let mut byte = data[i];
    let mut prev = Pixel {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    loop {
        let chunk_type = get_chunk_type(byte);
        let cur_pixel: Pixel;

        match chunk_type {
            ChunkTypes::QOI_OP_RGB => {
                cur_pixel = parse_qoi_op_rgb(data[i..(i + 4)].to_vec(), prev);
                i += 4;
            }
            ChunkTypes::QOI_OP_RGBA => todo!(),
            ChunkTypes::QOI_OP_INDEX => todo!(),
            ChunkTypes::QOI_OP_DIFF => todo!(),
            ChunkTypes::QOI_OP_LUMA => todo!(),
            ChunkTypes::QOI_OP_RUN => todo!(),
        };
        decoded.push(cur_pixel);
        prev = cur_pixel;

        byte = data[i]
    }
}

fn parse_qoi_op_rgb(data: Vec<u8>, previous: Pixel) -> Pixel {
    let r = data[1];
    let g = data[2];
    let b = data[3];
    return Pixel {
        r: r,
        g: g,
        b: b,
        a: previous.a,
    };
}
fn get_chunk_type(byte: u8) -> ChunkTypes {
    match byte {
        0b11111110 => return ChunkTypes::QOI_OP_RGB,
        0b11111111 => return ChunkTypes::QOI_OP_RGBA,
        _ => match byte >> 6 {
            0b00 => return ChunkTypes::QOI_OP_INDEX,
            0b01 => return ChunkTypes::QOI_OP_DIFF,
            0b10 => return ChunkTypes::QOI_OP_LUMA,
            0b11 => return ChunkTypes::QOI_OP_RUN,
            _ => panic!("Unknow chunk type, {}", byte >> 6),
        },
    }
}
