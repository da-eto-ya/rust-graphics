extern crate image;

use image::ImageBuffer;
use image::imageops::flip_vertical;

fn main() {
    // load/create image
    let mut img = ImageBuffer::new(100, 100);

    // image processing
    let red_pixel = image::Rgb([0xff, 0x00, 0x00]);
    let white_pixel = image::Rgb([0xff, 0xff, 0xff]);

    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = white_pixel;
    }

    img.put_pixel(52, 41, red_pixel);
    img = flip_vertical(&img);

    // save image
    let _ = img.save("output.png");
}
