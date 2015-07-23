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
    fn triangle(&mut self, v0: Vec3Di, v1: Vec3Di, v2: Vec3Di, color: P, zbuffer: &mut Vec<Vec<i32>>) -> &mut Self;
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

    fn triangle(&mut self, v0: Vec3Di, v1: Vec3Di, v2: Vec3Di, color: P, zbuffer: &mut Vec<Vec<i32>>) -> &mut Self {
        let mut vs = Vec::new();
        vs.push_all(&[v0, v1, v2]);
        vs.sort_by(|a, b| a.y.cmp(&b.y));

        for v in &[[vs[0], vs[1], vs[2]], [vs[2], vs[1], vs[0]]] {

            println!("v: {:?}", v);

            if v[1].y != v[0].y {
                let p1 = (v[1].x - v[0].x) as f64 / (v[1].y - v[0].y) as f64;
                let p2 = (v[2].x - v[0].x) as f64 / (v[2].y - v[0].y) as f64;
                let pz1 = (v[1].z - v[0].z) as f64 / (v[1].y - v[0].y) as f64;
                let pz2 = (v[2].z - v[0].z) as f64 / (v[2].y - v[0].y) as f64;

                println!("p1: {:?} p2: {:?} pz1: {:?} pz2: {:?}", p1, p2, pz1, pz2);

                for y in cmp::min(v[0].y, v[1].y)..cmp::max(v[0].y, v[1].y)+1 {
                    let x1 = v[0].x + (p1 * (y - v[0].y) as f64) as i32;
                    let x2 = v[0].x + (p2 * (y - v[0].y) as f64) as i32;
                    let z1 = v[0].z + (pz1 * (y - v[0].y) as f64) as i32;
                    let z2 = v[0].z + (pz2 * (y - v[0].y) as f64) as i32;

                    println!("y: {:?} x1: {:?} x2: {:?} z1: {:?} z2: {:?}", y, x1, x2, z1, z2);

                    if x1 != x2 {
                        let pz = (z2 - z1) as f64 / (x2 - x1) as f64;

                        println!("pz: {:?}", pz);

                        for x in cmp::min(x1, x2)..cmp::max(x1, x2)+1 {
                            let z = z1 + (pz * (x - x1) as f64) as i32;

                            println!("x: {:?} (pz * (x - x1)): {:?} z: {:?}", x, (pz * (x - x1) as f64), z);

                            if zbuffer[x as usize][y as usize] < z {
                                self.put_pixel(x as u32, y as u32, color);
                                zbuffer[x as usize][y as usize] = z;
                            }
                        }
                    } else {
                        let z = cmp::min(z1, z2);

                        println!("x1 = x2: {:?} z: {:?}", x1, z);

                        if zbuffer[x1 as usize][y as usize] < z {
                            self.put_pixel(x1 as u32, y as u32, color);
                            zbuffer[x1 as usize][y as usize] = z;
                        }
                    }
                }
            }
        }

        self
    }
}
