use std::{env, fs};
use std::path::Path;
use image::{DynamicImage, GenericImageView, ImageReader, Pixel};

fn convert_1bpp(img: DynamicImage) -> Vec<u8> {
    let (width, height) = img.dimensions();

    let mut result = Vec::new();
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
                result.push(byte);
                byte = 0;
                bit = 0;
            }
        }
        if bit != 0 {
            result.push(byte);
        }
    }

    result
}

fn convert_2bpp(img: DynamicImage) -> Vec<u8> {
    let (width, height) = img.dimensions();

    let mut result = Vec::new();
    for y in 0..height {
        let mut byte = 0u8;
        let mut bit = 0;

        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            // Arbitrary ARQ4 palette! Why not?
            let bits = match pixel.channels() {
                [255, 255, 255, _] => 0,
                [103, 114, 169, _] => 1,
                [58, 50, 119, _] => 2,
                [0, 0, 0, _] => 3,
                _ => {
                    println!("cargo::warning=unknown color encountered when processing an image");
                    0
                }
            };

            byte |= bits << (6 - bit);

            bit += 2;
            if bit == 8 {
                result.push(byte);
                byte = 0;
                bit = 0;
            }
        }
        if bit != 0 {
            result.push(byte);
        }
    }

    result
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    println!("cargo::rerun-if-changed=resources/1bpp");

    for entry in fs::read_dir("resources/1bpp").unwrap() {
        let path = entry.unwrap().path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "png") {
            let img = ImageReader::open(path.clone()).unwrap().decode().unwrap();

            let new_path = Path::new(&out_dir)
                .join(path.with_extension("raw").file_name().unwrap());

            fs::write(new_path, convert_1bpp(img)).unwrap();
        }
    }

    println!("cargo::rerun-if-changed=resources/2bpp");

    for entry in fs::read_dir("resources/2bpp").unwrap() {
        let path = entry.unwrap().path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "png") {
            let img = ImageReader::open(path.clone()).unwrap().decode().unwrap();

            let new_path = Path::new(&out_dir)
                .join(path.with_extension("raw").file_name().unwrap());

            fs::write(new_path, convert_2bpp(img)).unwrap();
        }
    }
}
