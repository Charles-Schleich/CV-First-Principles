
use image::{GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage, buffer::ConvertBuffer};
use imageproc::window::display_image;
use std::env;
use show_image::{ImageView, ImageInfo, create_window};


pub fn find_centroid(){

    let image_path = "images/binary_shapes/blob.png";
    let out_path = "images/binary_shapes/blob_processed.png";
    

    let mut image = image::open(&image_path)
        .expect("No image found at provided path").to_luma8();

    println!("w: {}, h:{}",image.width(),image.height());
    
    let total_area= image.width()*image.height();
    println!("total_area :{}",total_area);
    let mut shape_area=0;
    for (x,y,px) in image.enumerate_pixels(){
        if px.0[0] > 128 {
            shape_area=shape_area+1;
        }
    }
    println!("shape_area :{}",shape_area);
    let ratio = shape_area as f64 /total_area as f64;
    println!("ratio :{}",ratio*100.0);
    let centroid @ (x,y) =centroid(&image,shape_area);
    // 
    // let Luma<T: Primitive>(pub [T; 1]);
    let mut image_colour:RgbImage =image.convert();
    add_red_dot_centroid(&mut image_colour,x,y);

    // Create a window with default options and display the image.
    // let window = create_window("image", Default::default()).unwrap();
    // window.set_image("image-001", image_colour).unwrap();
    image_colour.save(out_path).unwrap();
}

// pub type GrayImage = ImageBuffer<Luma<u8>, Vec<u8>>;
fn centroid(image:&GrayImage, zeroth_moment:i32) -> (u32,u32){
    let mut sumx= 0;
    let mut sumy= 0;
    for (x,y,px) in image.enumerate_pixels(){
        if px.0[0] > 128 {
            sumx += x;
        }
        if px.0[0] > 128 {
            sumy += y;
        }
    }

    let x_center = (sumx as i32 /zeroth_moment) as u32;
    let y_center = (sumy as i32 /zeroth_moment) as u32;
    println!("Centroid {} {}",x_center ,y_center);
    (x_center,y_center)
}

fn add_red_dot_centroid(image_colour:&mut RgbImage, x:u32,y:u32){
    let x = x as i32;
    let y = y as i32;
    let px = Rgb([255,0,0]);
    let size_x = -2..2;
    for i in size_x {
        let size_y = -2..2;
        for j in size_y {
            image_colour.put_pixel((x+i) as u32, (y+j) as u32, px);
        }
    }

}