extern crate image;

use image::{
    ImageBuffer,
    Pixel,
};
use image::imageops::flip_vertical;
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
        if x0 == x1 && y0 == y1 {
            self.put_pixel(x0, y0, color);
        } else {
            let transpose = (x0 as i32 - x1 as i32).abs() < (y0 as i32 - y1 as i32).abs();
            let reverse = (!transpose && x0 > x1) || (transpose && y0 > y1);
            let a0 = if transpose {if reverse {y1} else {y0}} else {if reverse {x1} else {x0}};
            let a1 = if transpose {if reverse {y0} else {y1}} else {if reverse {x0} else {x1}};
            let b0 = if transpose {if reverse {x1} else {x0}} else {if reverse {y1} else {y0}};
            let b1 = if transpose {if reverse {x0} else {x1}} else {if reverse {y0} else {y1}};

            for a in a0..a1+1 {
                let t = (a - a0) as f64 / (a1 - a0) as f64;
                let b = (b0 as f64 * (1.0 - t) + b1 as f64 * t) as u32;

                if transpose {
                    self.put_pixel(b, a, color);
                } else {
                    self.put_pixel(a, b, color);
                }
            }
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

    for _ in 0..1000000 {
        img.line(13, 20, 80, 40, white);
        img.line(20, 13, 40, 80, red);
        img.line(80, 40, 13, 20, red);
    }

    img = flip_vertical(&img);

    // save image
    let _ = img.save("output.png");
}
