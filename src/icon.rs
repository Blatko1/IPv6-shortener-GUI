use eframe::IconData;

pub fn generate_icon(width: u32, height: u32) -> IconData {
    let pixels = width * height;
    let data_size = width * height * 4;
    let mut rgba = Vec::with_capacity(data_size as usize);

    for p in 0..pixels {
        let r = (((p / width) as f32 / height as f32) * 255.0) as u8; // top to bottom
        let g = ((((255 - (p % width)) / width) as f32 / height as f32) * 255.0
            / 3.0) as u8; // bottom to top
        let b = (((p % height) as f32 / width as f32) * 255.0) as u8; // left to right
        let a = 255;
        rgba.push(r);
        rgba.push(g);
        rgba.push(b);
        rgba.push(a);
    }

    IconData {
        rgba,
        width,
        height,
    }
}
