extern crate image;

use image::{
    ImageBuffer,
    Pixel,
};
use std::ops::{
    Deref,
    DerefMut,
};

trait Painting<P> where P: Pixel {
    fn line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: P) -> &mut Self;
}

impl<P, Container> Painting<P> for ImageBuffer<P, Container>
where P: Pixel + 'static,
      Container: Deref<Target=[P::Subpixel]> + DerefMut,
      P::Subpixel: 'static {

    fn line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: P) -> &mut Self {
        let mut t = 0.0;
        let step = 0.1;

        while t < 1.0 {
            let x = ((x0 as f64) * (1.0 - t) + (x1 as f64) * t) as u32;
            let y = ((y0 as f64) * (1.0 - t) + (y1 as f64) * t) as u32;
            self.put_pixel(x, y, color);
            t += step;
        }

        self
    }
}

fn main() {
    // load/create image
    let mut img = ImageBuffer::new(100, 100);

    // image processing
    let red = image::Rgb([0xff, 0x00, 0x00]);
    let white = image::Rgb([0xff, 0xff, 0xff]);
    img.line(0, 0, 99, 99, white)
        .line(10, 10, 12, 99, red)
        .line(99, 0, 99, 9, white);

    // save image
    let _ = img.save("output.png");
}
