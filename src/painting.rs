extern crate image;

use image::{
    ImageBuffer,
    Pixel,
};
use std::ops::{
    Deref,
    DerefMut,
};

pub trait Painting<P> where P: Pixel {
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
            let da = (a1 - a0) as i32;
            let derr = 2 * if b1 > b0 {b1 - b0} else {b0 - b1} as i32;
            let mut err = 0;
            let mut a = a0;
            let mut b = b0 as i32;
            let db = if b1 > b0 {1} else {-1};

            while a <= a1 {
                if transpose {
                    self.put_pixel(b as u32, a, color);
                } else {
                    self.put_pixel(a, b as u32, color);
                }

                err += derr;

                if err > da {
                    b += db;
                    err -= 2 * da;
                }

                a += 1;
            }
        }

        self
    }
}

