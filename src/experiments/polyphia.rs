

// Idea to implement a filter to achieve the effect in this video 
// https://www.youtube.com/watch?v=EVSqUl-FtCI

// 1.  Start with single images: if Red Channel higher Keep red channel and
// 2a. Blue or Green channel higher -> Raise each channel to equal max of (R,G,B) to get a grayscale output 
// 2b. Blue or Green channel higher -> Lower each channel to equal min of (R,G,B) to get a grayscale output 

// 3a. Implement video processing Step using ffmpeg bindings or gstreamer
//     Might have to do some converstions with image proc

// 4a. see if i can use Rust CUDA in order to speed this up 
//  https://github.com/Rust-GPU/Rust-CUDA

pub fn polyphia_main(){

}