
use image::{GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage, buffer::ConvertBuffer};
use imageproc::{rect::Region, window::display_image};
use std::env;
use show_image::{ImageView, ImageInfo, create_window};


pub fn find_centroid(image: &GrayImage)-> (u32,u32){
    // println!("w: {}, h:{}",image.width(),image.height());
    let total_area= image.width()*image.height();
    let mut shape_area=0;
    let mut sumx= 0;
    let mut sumy= 0;
    for (x,y,px) in image.enumerate_pixels(){
        if px.0[0] > 128 {
            shape_area=shape_area+1;
            sumx += x;
            sumy += y;
        }
    }

    let x_center = (sumx as i32 /shape_area) as u32;
    let y_center = (sumy as i32 /shape_area) as u32;
    println!("Centroid {} {}",x_center ,y_center);
    println!("Shape_area :{}",shape_area);
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

pub fn find_min_second_moment(image:&GrayImage, x_mid:u32, y_mid:u32) {
    let x_mid = x_mid as f64;
    let y_mid = y_mid as f64;
    let mut a:f64 = 0.0;
    let mut b:f64 = 0.0;
    let mut c:f64 = 0.0 ;
    for (i,j,px) in image.enumerate_pixels(){
        let i = i as f64;
        let j = j as f64;
        if px.0[0] > 128 {
            a += (i-x_mid).powi(2);
            b += (j-x_mid)*(j-y_mid);
            c += (j-y_mid).powi(2);
            // a += (x_mid).powi(2);
            // b += (x_mid)*(y_mid);
            // c += (y_mid).powi(2);
        }
    }
    b=b*2.0;
    let a_fl = a as f64;
    let b_fl = b as f64;
    let c_fl = c as f64;
    let theta = b_fl.atan2(a_fl-c_fl)/2.0;
    // E = asin^2(theta) - bsin(theta)cos(theta) + csin^2(theta);
    // tan2(theta) = b/(a-c)
    println!("theta = {}",theta.to_degrees());
}

pub fn write(input_path:&str) {
    // input_path:&str

    // let mut image_colour:RgbImage =image.convert();
    // add_red_dot_centroid(&mut image_colour,x_center,y_center);
    // image_colour.save(out_path).unwrap();
}


pub fn binaryproc_main(){
    let paths = std::fs::read_dir("./images/binary_shapes/").unwrap();
    for res_path in paths {
        if let Ok(path) = res_path {
            let pathbuf= path.path();
            let pathstr =pathbuf.as_path().to_str().unwrap();
            if !pathstr.contains("processed"){
                println!("\nName: {}", path.path().display());
                let image = image::open(&pathstr)
                    .expect("No image found at provided path").to_luma8();
                let (x,y) = find_centroid(&image);
                find_min_second_moment(&image,x,y);
            };
        };
    }

    // let out_path = "images/binary_shapes/blob_processed.png".into();
    // find_centroid(&image_path,&out_path);
}