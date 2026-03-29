use image;
use std::env;
use std::path::Path;
use crate::processor::{compress, image_to_ascii};

pub mod processor;

fn main(){
    let args: Vec<String> = env::args().collect();

    let path = Path::new(args.last().expect("You need a image path as argument to execute de program"));

    let image = match image::open(path){
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
}
