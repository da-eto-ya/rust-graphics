extern crate image;

use image::{
    ImageBuffer,
    Pixel,
};
use std::cmp;
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
        let mut v = Vec::new();
        v.push_all(&[v0, v1, v2]);
        v.sort_by(|a, b| a.y.cmp(&b.y));

        if v[1].y > v[0].y {
            let p0 = (v[1].x - v[0].x) as f64 / (v[1].y - v[0].y) as f64;
            let p1 = (v[2].x - v[0].x) as f64 / (v[2].y - v[0].y) as f64;

            for y in (v[0].y..v[1].y) {
                let a0 = Vec2Du { x: v[0].x + (p0 * (y - v[0].y) as f64) as u32, y: y };
                let a1 = Vec2Du { x: v[0].x + (p1 * (y - v[0].y) as f64) as u32, y: y };

                println!("{:?} {:?} {:?} {:?}", v, y, a0, a1);

                self.line(
                    a0,
                    a1,
                    color
                );
            }
        }

        self
    }
}
