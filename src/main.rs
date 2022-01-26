use imageproc::{gradients::VERTICAL_SOBEL, window::display_image};
use imageproc::*;
use image::*;
use show_image::{ImageView, ImageInfo, create_window};
use std::env;
use image::GenericImageView;


const KERNEL: [f32;9] = 
    [-1.0f32, -1.0, -1.0,
     -1.0, 8.0, -1.0,
     -1.0, -1.0, -1.0];

const SOBEL_HORIZ: [f32;9] = 
     [1.0, 0.0, -1.0,
      2.0, 0.0, -2.0,
      1.0, 0.0, -1.0];


mod binary_proc;
use binary_proc::binaryproc_main;

mod filters;
use filters::filterproc_main;

mod experiments;
use experiments::polyphia_main;

fn main() {
    println!("Hello, world!");
    
    // binaryproc_main();
    // filterproc_main();
    // display_image("", &image, 500, 500);
    polyphia_main();
}
