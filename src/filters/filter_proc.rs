
use image::{GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage, buffer::ConvertBuffer};
use imageproc::{rect::Region, window::display_image};
use nalgebra::{Const, Matrix4, MatrixN, OMatrix};
use std::env;
use show_image::{ImageView, ImageInfo, create_window};
use std::f64::consts::PI;



pub fn filter_gaussian(image:RgbImage){
    
}

pub fn create_gaussian(width:u32,height:u32, sigma:f64) -> GrayImage{
    // let mut kernel = Vec::new();
    // let mut img = RgbImage::new(32, 32);
    let mut img = GrayImage::new(width, height);
    let mut vec_vec:Vec<Vec<f64>> = Vec::new();
    // This Shifts center of Gaussian Kernel to middle of image
    let w2 = (width/2) as f64;
    let h2 = (height/2) as f64;

    let mut maxval = 0.0;
    let mut location =(0,0);
    for i in 0..height {
        let mut inner : Vec<f64> = Vec::new();
        for j in 0..width {
            // inner.input
            let euler= std::f64::consts::E;
            let x = i as f64;
            let y = j as f64;
            // exp
            let exp = (-1.0/2.0)*(((x-w2)*(x-w2)+(y-h2)*(y-h2))/(sigma*sigma));
            // base 
            let denom = 2.0*PI*(sigma*sigma);
            let gaussian_value = (1.0/denom)*euler.powf(exp);
            let gaussian_value = gaussian_value.abs();
            inner.push(gaussian_value);
            if gaussian_value > maxval {
                maxval = gaussian_value;
                location = (i,j);
            };
        }
        vec_vec.push(inner);
    }
    println!("{} {:?}", maxval, location );

    // px = 255*val/max
    // normalize value
    for (i,vec) in vec_vec.iter().enumerate() {
        for (j,val) in vec.iter().enumerate() {
            let u8rep = (val*255.0/maxval) as u8;
            img.put_pixel(i as u32, j as u32, Luma([u8rep]));
        } 
    }
    img
}


pub fn filterproc_main(){
    
    // let paths = std::fs::read_dir("./images/colour_images/").unwrap();

    // for res_path in paths {
    //     if let Ok(path) = res_path {
    //         let pathbuf= path.path();
    //         let pathstr =pathbuf.as_path().to_str().unwrap();
    //         if !pathstr.contains("processed"){
    //             println!("\nName: {}", path.path().display());
    //             let image = image::open(&pathstr)
    //                 .expect("No image found at provided path").to_rgb8();
    //         };
    //     };
    // }

    // let w= 16;
    // let h= 16;
    let w= 25;
    let h= 25;
    // create_gaussian_nalgebra(4,4,4.0);
    let gaussian = create_gaussian(w,h, 4.0);
    let out_path = format!("images/kernel_output/gaussian_{}_{}.png",w,h);
    let image_colour:RgbImage = gaussian.convert();
    image_colour.save(out_path).unwrap();

}

// 512//2pi