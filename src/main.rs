use imageproc::window::display_image;
use std::env;

mod binary_proc;
use binary_proc::find_centroid;
const sobel_3x3_x: [i8;9] = [
    -1,0,1,
    -2,0,2,
    -1,0,1
];

fn main() {
    println!("Hello, world!");

    let image_path = "images/lena.jpg";

    let image = image::open(&image_path)
        .expect("No image found at provided path")
        .to_rgba8();

    find_centroid();
    // display_image("", &image, 500, 500);
}
