

// Idea to implement a filter to achieve the effect in this video 
// https://www.youtube.com/watch?v=EVSqUl-FtCI

// 1.  Start with single images: if Red Channel higher Keep red channel and
// 2a. Blue or Green channel higher -> Raise each channel to equal max of (R,G,B) to get a grayscale output 
// 2b. Blue or Green channel higher -> Lower each channel to equal min of (R,G,B) to get a grayscale output 

// 3a. Implement video processing Step using ffmpeg bindings or gstreamer
//     Might have to do some converstions with image proc
// https://github.com/meh/rust-ffmpeg

// 4a. see if i can use Rust CUDA in order to speed this up 
//  https://github.com/Rust-GPU/Rust-CUDA



// use palette::{encoding::Srgb, Hsl, Lch};
use palette::{FromColor, Hsl, Lch, Hue, Srgb};
// use palette::{cast};
pub fn write(input_path:&str) {
    // input_path:&str

    // let mut image_colour:RgbImage =image.convert();
    // add_red_dot_centroid(&mut image_colour,x_center,y_center);
    // image_colour.save(out_path).unwrap();
}

fn diff(a:u8, b:u8, percentage_over: f32) -> bool {
    let a_16 = a as f32;
    let b_16 = b as f32;
    if  a_16 > b_16 * (1.0+percentage_over) {
        true 
    } else {
        false
    } 
}

fn mute_col(col:u8, factor:f32) -> u8 {
    let col_32 = col as f32;
    let res= col_32 * factor;
    res as u8
}

pub fn rgb_only_attempt(){

    let out_path = "./images/colour_images/l'imperatrice_out.jpg";
    let image = image::open("./images/colour_images/l'imperatrice.jpg").unwrap();
    // let rgb_image = image.as_rgb8().unwrap();
    let mut rgb_image = image.into_rgb8();
    for (x,y,px) in rgb_image.enumerate_pixels_mut(){
        let [mut r,mut g,mut b]= px.0;
        if r > g && r > b { 
             if g > b {
                b=g;
            } else {
                g=b;
            }
            // g = mute_col(g,0.9);
            // b = mute_col(b,0.9);
            // r = mute_col(r,0.5);
        // this was not the right way to go about it
        // this does not turn into grey, 
        } else if g >= r && g >= b { // green dominant 
            r = b;
            g = b;
        } else if b >= r && b >= g { // blue dominant
            r = g;
            b = g;
        } else {
            println!("{} {} {}",r,g,b);
        }

        px.0 = [r,g,b];
    }

    rgb_image.save(out_path).unwrap();
}

pub fn rgb_hsl_attempt(){
    // pallet_attempt
    let out_path = "./images/colour_images/l'imperatrice_out_hsl.jpg";
    let mut image = image::open("./images/colour_images/l'imperatrice.jpg").expect("Cannot open image").to_rgb8();

    // let out_path = "./images/colour_images/basic_colour_image_out.png";
    // let mut image = image::open("./images/colour_images/basic_colour_image.png").expect("Cannot open image").to_rgb8();

    // let out_path = "./images/colour_images/one_line_basic_colour_image_out.png";
    // let mut image = image::open("./images/colour_images/one_line_basic_colour_image.png").expect("Cannot open image").to_rgb8();

    // let out_path = "./images/colour_images/red_blue_grad_out.png";
    // let mut image = image::open("./images/colour_images/red_blue_grad.png").expect("Cannot open image").to_rgb8();

    for (x, y, px) in image.enumerate_pixels_mut() {
        // pixel.0.
        let pixels = px.0;
        let color  = Srgb::from_components((pixels[0],pixels[1],pixels[2])).into_format::<f64>();
        let hsl = Hsl::from_color(color);
        let hue= hsl.hue;
        let sat= hsl.saturation;
        let lit= hsl.lightness;
        let hue_deg= hue.to_positive_degrees();
        let [mut r,mut g,mut b]= px.0;

        // println!("x:{} y:{} ___ h:{:?} s:{} l:{} ___ r:{} g:{} b:{} ",x, y, hue_deg, sat, lit, r, g, b);
        // println!("h:{:?} s:{} l:{}",hue_deg, sat, lit);
        // println!("r:{} g:{} b:{}",r,g,b);
        let red_bound = 10.0;
        if ((hue_deg>=(360.0-red_bound) && hue_deg<=360.0) || (hue_deg>=0.0 && hue_deg<=(red_bound)))
        && (lit<0.9)
        {
            println!("Not grey x:{} y:{} --- r:{} g:{} b:{} ",x, y, r, g, b);
            // println!("x:{} y:{}",x, y);
            // println!("hue:{:?}\nsat: {}\n lit:{}",hue_deg, sat, lit);
            // println!("  r:{} g:{} b:{}",r,g,b);
        } else{
            println!("Grey x:{} y:{}",x, y);
            let sum: u16 = r as u16 +g as u16+b as u16;
            let avg = (sum/3) as u8;
            r = avg;
            b = avg;
            g = avg;
        }
        px.0 = [r,g,b];
    }
    image.save(out_path).unwrap();

    // let blue_hue_end = 250;
    // // let blue_hue_midpoint = 240;
    // let blue_hue_start = 230;

    // // red hue 350->360   to  0->10
    // let red_hue_end = 350;
    // // let red_hue_midpoint = 0 / 360;
    // let red_hue_start = 230;
}




