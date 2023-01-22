use bmp::Pixel;
use png;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn get_transparent_pixels() -> Vec<Pixel> {
    let mut transparent_vec = Vec::new();

    let transparent_byte: Vec<u8> = vec![0xFF, 0xFC, 0xF9, 0xFE, 0xFB, 0xF8, 0xFD, 0xFA, 0xF7];

    for color in transparent_byte {
        transparent_vec.push(Pixel {
            r: color,
            g: 0,
            b: color,
        });
    }

    return transparent_vec;
}

fn main() {
    let trans_pixels = get_transparent_pixels();
    let loaded_image = bmp::open("D:\\git\\pngify\\assets\\00018.bmp").unwrap();
    let mut serialized_bytes: Vec<u8> = Vec::new();

    println!(
        "Loaded image is (wxh): {}x{}",
        loaded_image.get_width(),
        loaded_image.get_height()
    );

    for (x, y) in loaded_image.coordinates() {
        if trans_pixels.contains(&loaded_image.get_pixel(x, y)) {}

        let alpha = if trans_pixels.contains(&loaded_image.get_pixel(x, y)) {
            0
        } else {
            255
        };

        let pixel = loaded_image.get_pixel(x, y);
        serialized_bytes.push(pixel.r);
        serialized_bytes.push(pixel.g);
        serialized_bytes.push(pixel.b);
        serialized_bytes.push(alpha);
    }

    let path = Path::new(r".\output.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, loaded_image.get_width(), loaded_image.get_height());
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&serialized_bytes).unwrap();
}
