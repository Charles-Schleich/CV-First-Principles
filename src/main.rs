use imageproc::window::display_image;
use std::env;

fn main() {
    println!("Hello, world!");

    let image_path = "images/lena.jpg";

    let image = image::open(&image_path)
        .expect("No image found at provided path")
        .to_rgba();

    display_image("", &image, 500, 500);

}
