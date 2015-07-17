extern crate image;

use image::{
    ImageBuffer,
    Pixel,
};
use std::ops::{
    Deref,
    DerefMut,
};
use geometry::*;

pub trait Painting<P> where P: Pixel {
    fn line(&mut self, v0: Vec2Du, v1: Vec2Du, color: P) -> &mut Self;
    fn triangle(&mut self, v0: Vec2Du, v1: Vec2Du, v2: Vec2Du, color: P) -> &mut Self;
}

impl<P, Container> Painting<P> for ImageBuffer<P, Container>
where P: Pixel + 'static,
      Container: Deref<Target=[P::Subpixel]> + DerefMut,
      P::Subpixel: 'static {

    fn line(&mut self, v0: Vec2Du, v1: Vec2Du, color: P) -> &mut Self {
        if v0.x == v1.x && v0.y == v1.y {
            self.put_pixel(v0.x, v0.y, color);
        } else {
            let transpose = (v0.x as i32 - v1.x as i32).abs() < (v0.y as i32 - v1.y as i32).abs();
            let reverse = (!transpose && v0.x > v1.x) || (transpose && v0.y > v1.y);
            let a0 = if transpose {if reverse {v1.y} else {v0.y}} else {if reverse {v1.x} else {v0.x}};
            let a1 = if transpose {if reverse {v0.y} else {v1.y}} else {if reverse {v0.x} else {v1.x}};
            let b0 = if transpose {if reverse {v1.x} else {v0.x}} else {if reverse {v1.y} else {v0.y}};
            let b1 = if transpose {if reverse {v0.x} else {v1.x}} else {if reverse {v0.y} else {v1.y}};
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

    fn triangle(&mut self, v0: Vec2Du, v1: Vec2Du, v2: Vec2Du, color: P) -> &mut Self {
        self.line(v0, v1, color).line(v1, v2, color).line(v2, v0, color)
    }
}
