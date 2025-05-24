use std::{env, fs};
use std::path::Path;
use image::{GenericImageView, ImageReader};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    println!("cargo::rerun-if-changed=resources/1bpp");

    for entry in fs::read_dir("resources/1bpp").unwrap() {
        let path = entry.unwrap().path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "png") {
            let img = ImageReader::open(path.clone()).unwrap().decode().unwrap();
            let (width, height) = img.dimensions();

            let mut raw_data = Vec::new();
            for y in 0..height {
                let mut byte = 0u8;
                let mut bit = 0;
                for x in 0..width {
                    let pixel = img.get_pixel(x, y);
                    if pixel[1] == 255 {
                        byte |= 1 << (7 - bit);
                    }
                    bit += 1;
                    if bit == 8 {
                        raw_data.push(byte);
                        byte = 0;
                        bit = 0;
                    }
                }
                if bit != 0 {
                    raw_data.push(byte);
                }
            }

            fs::write(Path::new(&out_dir).join(path.with_extension("raw").file_name().unwrap()), &raw_data).unwrap();
        }
    }
}
