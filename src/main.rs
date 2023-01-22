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
    for (x, y) in loaded_image.coordinates() {
        if trans_pixels.contains(&loaded_image.get_pixel(x, y)) {
            println!("Contains transparent pixel at {}, {}", x, y);
        }
    }

    let path = Path::new(r"/path/to/image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, loaded_image.get_width(), loaded_image.get_height()); // Width is 2 pixels and height is 1.
    let mut writer = encoder.write_header().unwrap();
    let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(&data).unwrap(); // Save
}
