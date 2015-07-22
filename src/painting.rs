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
    fn line(&mut self, v0: Vec2Di, v1: Vec2Di, color: P) -> &mut Self;
    fn triangle(&mut self, v0: Vec2Di, v1: Vec2Di, v2: Vec2Di, color: P) -> &mut Self;
}

impl<P, Container> Painting<P> for ImageBuffer<P, Container>
where P: Pixel + 'static,
      Container: Deref<Target=[P::Subpixel]> + DerefMut,
      P::Subpixel: 'static {

    fn line(&mut self, v0: Vec2Di, v1: Vec2Di, color: P) -> &mut Self {
        if v0.x == v1.x && v0.y == v1.y {
            self.put_pixel(v0.x as u32, v0.y as u32, color);
        } else {
            let transpose = (v0.x - v1.x).abs() < (v0.y - v1.y).abs();
            let reverse = (!transpose && v0.x > v1.x) || (transpose && v0.y > v1.y);
            let a0 = if transpose {if reverse {v1.y} else {v0.y}} else {if reverse {v1.x} else {v0.x}};
            let a1 = if transpose {if reverse {v0.y} else {v1.y}} else {if reverse {v0.x} else {v1.x}};
            let b0 = if transpose {if reverse {v1.x} else {v0.x}} else {if reverse {v1.y} else {v0.y}};
            let b1 = if transpose {if reverse {v0.x} else {v1.x}} else {if reverse {v0.y} else {v1.y}};
            let da = a1 - a0;
            let derr = 2 * if b1 > b0 {b1 - b0} else {b0 - b1};
            let mut err = 0;
            let mut a = a0;
            let mut b = b0;
            let db = if b1 > b0 {1} else {-1};

            while a <= a1 {
                if transpose {
                    self.put_pixel(b as u32, a as u32, color);
                } else {
                    self.put_pixel(a as u32, b as u32, color);
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

    fn triangle(&mut self, v0: Vec2Di, v1: Vec2Di, v2: Vec2Di, color: P) -> &mut Self {
        let mut vs = Vec::new();
        vs.push_all(&[v0, v1, v2]);
        vs.sort_by(|a, b| a.y.cmp(&b.y));

        for v in &[[vs[0], vs[1], vs[2]], [vs[2], vs[1], vs[0]]] {
            if v[1].y != v[0].y {
                let p0 = (v[1].x - v[0].x) as f64 / (v[1].y - v[0].y) as f64;
                let p1 = (v[2].x - v[0].x) as f64 / (v[2].y - v[0].y) as f64;

                for y in cmp::min(v[0].y, v[1].y)..cmp::max(v[0].y, v[1].y)+1 {
                    let x0 = v[0].x + (p0 * (y - v[0].y) as f64) as i32;
                    let x1 = v[0].x + (p1 * (y - v[0].y) as f64) as i32;

                    for x in cmp::min(x0, x1)..cmp::max(x0, x1)+1 {
                        self.put_pixel(x as u32, y as u32, color);
                    }
                }
            }
        }

        self
    }
}
