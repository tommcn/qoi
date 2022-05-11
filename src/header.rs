#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    /**
     * Magic header, must equal "qoif"
     */
    pub magic: String,
    /**
     * Width in pixels
     */
    pub width: u32,
    /**
     * Height in pixels
     */
    pub height: u32,
    /**
     * 3 = RGB
     * 4 = RGBA
     */
    pub channels: u8,
    /**
     * 0 = sRGB with linear alpha
     * 1 = all channels linear
     */
    pub colorspace: u8, // 0 = sRGB with linear a; 1 = all channels linear
}

pub fn parse_header(header: Vec<u8>) -> Header {
    assert!(header.len() >= 14);

    let mut magic = String::from("");
    for i in 0..4 {
        let byte = header[i];
        magic.push(byte as char)
    }
    let width: u32 = ((header[4] as u32) << 24)
        + ((header[5] as u32) << 16)
        + ((header[6] as u32) << 8)
        + ((header[7] as u32) << 0);
    let height: u32 = ((header[8] as u32) << 24)
        + ((header[9] as u32) << 16)
        + ((header[10] as u32) << 8)
        + ((header[11] as u32) << 0);
    let channels = header[12];
    let colorspace = header[13];

    let header = Header {
        magic,
        width,
        height,
        channels,
        colorspace,
    };
    return header;
}

pub fn validate_header(header: Header) -> Header {
    if header.magic != "qoif" {
        panic!(
            "Header magic value is not equal to `qoif`, it is: {}",
            header.magic
        )
    }
    if !([3, 4].contains(&header.channels)) {
        panic!(
            "Header channels value is not in `[3, 4]`, it is: {}",
            header.channels
        )
    }
    if !([0, 1].contains(&header.colorspace)) {
        panic!(
            "Header colorspace value is not in `[0, 1]`, it is: {}",
            header.colorspace
        )
    }
    return header;
}
