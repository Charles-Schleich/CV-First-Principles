use imageproc::window::display_image;
use std::env;

mod binary_proc;
use binary_proc::binaryproc_main;
const sobel_3x3_x: [i8;9] = [
    -1,0,1,
    -2,0,2,
    -1,0,1
];

fn main() {
    println!("Hello, world!");
    
    // find_centroid();
    binaryproc_main();
    // display_image("", &image, 500, 500);
}
