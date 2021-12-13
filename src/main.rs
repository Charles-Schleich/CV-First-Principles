use imageproc::window::display_image;
use std::env;

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
