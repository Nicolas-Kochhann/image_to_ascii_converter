use image;
// use std::env;
use crate::processor::{compress, image_to_ascii};

pub mod processor;

fn main(){
    // let args: Vec<String> = env::args().collect();

    let image = match image::open("placeholders/mario.png"){
        Ok(img) => img,
        Err(e) => {
            println!("Open image error: {}", e);
            return;
        }
    };

    let mut image_buffer = image.to_luma8();

    image_buffer = compress(& image_buffer,200, 200);
    
    let ascii_image = image_to_ascii(& image_buffer);

    print!("{}", ascii_image);
    println!("Dimensions {}x{}", image_buffer.width(), image_buffer.height());
}
