extern crate image;

use image::ImageBuffer;
use image::imageops::flip_vertical;
use painting::Painting;

mod painting;

fn main() {
    // load/create image
    let mut img = ImageBuffer::new(100, 100);

    // image processing
    let red = image::Rgb([0xff, 0x00, 0x00]);
    let white = image::Rgb([0xff, 0xff, 0xff]);

    img.line(13, 20, 80, 40, white);
    img.line(20, 13, 40, 80, red);
    img.line(80, 40, 13, 20, red);

    img = flip_vertical(&img);

    // save image
    let _ = img.save("output.ppm");
}
