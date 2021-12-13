

// Idea to implement a filter to achieve the effect in this video 
// https://www.youtube.com/watch?v=EVSqUl-FtCI

// 1.  Start with single images: if Red Channel higher Keep red channel and
// 2a. Blue or Green channel higher -> Raise each channel to equal max of (R,G,B) to get a grayscale output 
// 2b. Blue or Green channel higher -> Lower each channel to equal min of (R,G,B) to get a grayscale output 

// 3a. Implement video processing Step using ffmpeg bindings or gstreamer
//     Might have to do some converstions with image proc

// 4a. see if i can use Rust CUDA in order to speed this up 
//  https://github.com/Rust-GPU/Rust-CUDA


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

pub fn polyphia_main(){

    let out_path = "./images/colour_images/l'imperatrice_out.jpg";
    let image = image::open("./images/colour_images/l'imperatrice.jpg").unwrap();
    // let rgb_image = image.as_rgb8().unwrap();
    let mut rgb_image = image.into_rgb8();
    for (x,y,px) in rgb_image.enumerate_pixels_mut(){
        let [mut r,mut g,mut b]= px.0;
        if r > g && r > b { // red dominant
             if g > b {
                b=g;
            } else {
                g=b;
            }
            g = mute_col(g,0.9);
            b = mute_col(b,0.9);
            // r = mute_col(r,0.5);
        } else if g >= r && g >= b { // green dominant 
            r = g;
            b = g;
        } else if b >= r && b >= b { // blue dominant
            r = b;
            g = b;
        }

        px.0 = [r,g,b];
    }

    rgb_image.save(out_path).unwrap();

}