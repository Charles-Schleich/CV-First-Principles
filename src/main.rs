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


fn main() {
    let x = (-2i16 + i16::MAX) as u16 ;
    // let x= (px.0[0] + i16::MAX) as u16;

    println!("Hello, world! {}",x);

    // let image_path = "images/lena.jpg";
    let image_path = "images/lenna_original.png";

    let image = image::open(&image_path)
        .expect("No image found at provided path");
 
    let self_filter = image.filter3x3(&SOBEL_HORIZ).to_luma8();
    display_image("c", &self_filter, 512, 512);
    // let image_rgb = image.to_rgb8();
    // display_image("c", &image_rgb, 512, 512);

    let grayscale_image=image.clone().to_luma8();
    let blurred_image = imageproc::filter::gaussian_blur_f32(&grayscale_image,1.0);
    let hz_sobel= imageproc::gradients::horizontal_sobel(&grayscale_image);

    // hz_sobel.save("path");

    let mut img_buffer= ImageBuffer::<Luma<u16>, Vec<u16>>::new(hz_sobel.width(), hz_sobel.height());
    for (x,y,px) in  hz_sobel.enumerate_pixels(){
        // let pix= (px.0[0] + i16::MAX) as u16;
        let pix= px.0[0] as u16 ;
        let lumapx= Luma::<u16>([pix;1]);
        img_buffer.put_pixel(x, y, lumapx);
    };
    display_image("c", &img_buffer, 512, 512);


    let ver_sobel = imageproc::gradients::vertical_sobel(&blurred_image);

    let mut img_buffer= ImageBuffer::<Luma<u16>, Vec<u16>>::new(ver_sobel.width(), ver_sobel.height());
    for (x,y,px) in  ver_sobel.enumerate_pixels(){
        // let pix= (px.0[0] + i16::MAX) as u16;
        let pix= px.0[0] as u16 ;
        let lumapx= Luma::<u16>([pix;1]);
        img_buffer.put_pixel(x, y, lumapx);
    };
    display_image("c", &img_buffer, 512, 512);

    // let sobelgrads= imageproc::gradients::sobel_gradients(&blurred_image);
    // display_image("c", &sobelgrads, 512, 512);

    // let window = create_window("image", Default::default()).unwrap();
    // window.set_image("image-001", horizontal_sobel).unwrap();
  
}
