use image::{ImageBuffer, Luma, imageops::{self, FilterType::{Lanczos3}}};

const CHARACTERS: [char; 69] = [
    ' ', '.', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', 
    '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 
    'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', 
    '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'
];

type Image = ImageBuffer<Luma<u8>, Vec<u8>>;

pub fn image_to_ascii(image: &Image) -> String
{
    let mut ascii_image = String::new();

    let (width, _height) = image.dimensions();

    for (x, _y, pixel) in image.enumerate_pixels() {

        let [brighteness] = pixel.0;

        let index = (brighteness as usize) * (CHARACTERS.len() - 1) / 255;

        ascii_image += & String::from(CHARACTERS[index]);
        //ascii_image += & String::from(CHARACTERS[index]);

        if x == (width - 1) { ascii_image += "\n"; }
    }

    ascii_image
}

pub fn compress(image: &Image, nwidth: u32, nheight: u32) -> Image
{
    let (width, height) = image.dimensions();

    let aspect_ratio = (width as f64)/(height as f64);

    let real_width = nwidth as f64 * aspect_ratio;
    let real_width = real_width as u32;

    let new_image = imageops::resize(image, real_width, nheight, Lanczos3);

    new_image
}