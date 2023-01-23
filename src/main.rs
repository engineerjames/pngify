use bmp::Pixel;
use png::{self, Compression};
use std::env;
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

fn get_serialized_bytes(image: bmp::Image, trans_pixels: Vec<Pixel>) -> Vec<u8> {
    let mut serialized_bytes: Vec<u8> = Vec::new();

    for (x, y) in image.coordinates() {
        let alpha = if trans_pixels.contains(&image.get_pixel(x, y)) {
            0
        } else {
            255
        };

        let pixel = image.get_pixel(x, y);
        serialized_bytes.push(pixel.r);
        serialized_bytes.push(pixel.g);
        serialized_bytes.push(pixel.b);
        serialized_bytes.push(alpha);
    }

    return serialized_bytes;
}

fn create_png_from_serialized_bytes(
    serialized_bytes: Vec<u8>,
    out_path: &Path,
    width: u32,
    height: u32,
) {
    let file = File::create(out_path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(Compression::Best);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&serialized_bytes).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("pngify: Will convert Astonia-formatted BMP files to transparency-enabled PNGs.");
        println!("Program usage: `pngify <path_to_file> <output_directory>`");
        return;
    }

    let bmp_file_path = Path::new(&args[1]);
    let out_file_path = Path::new(&args[2]);
    let bmp_file_name = bmp_file_path.file_name().unwrap().to_str().unwrap();
    let png_file_path = out_file_path.to_str().unwrap().to_owned()
        + &bmp_file_name[..bmp_file_name.len() - 4]
        + ".png";

    let trans_pixels = get_transparent_pixels();
    let loaded_image = bmp::open(bmp_file_path).unwrap();
    let height = loaded_image.get_height();
    let width = loaded_image.get_width();
    let serialized_bytes = get_serialized_bytes(loaded_image, trans_pixels);

    let path = Path::new(png_file_path.as_str());

    create_png_from_serialized_bytes(serialized_bytes, path, width, height);
}
