#![feature(collections)]

extern crate image;
extern crate rand;

use rand::Rng;
use image::ImageBuffer;
use image::imageops::flip_vertical;
use painting::Painting;
use geometry::*;

mod painting;
mod geometry;


fn main() {
    // image processing
    let white = image::Rgb([0xff, 0xff, 0xff]);
    let red = image::Rgb([0xff, 0x00, 0x00]);

    let m = match load_model_obj("obj/phone.obj") {
        Ok(m) => m,
        Err(..) => panic!("couldn't read input file"),
    };

    let width = 1920;
    let pad = 10;

    let mut min_x = m.verts[0].x;
    let mut min_y = m.verts[0].y;
    let mut max_x = m.verts[0].x;
    let mut max_y = m.verts[0].y;

    for v in m.verts.iter() {
        if v.x < min_x { min_x = v.x; }
        if v.y < min_y { min_y = v.y; }
        if v.x > max_x { max_x = v.x; }
        if v.y > max_y { max_y = v.y; }
    }

    let ratio = (width - 2 * pad) as f64 / (max_x - min_x);
    let height = ((max_y - min_y) * ratio + 2.0 * pad as f64) as u32;

    let mut img = ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();

    for f in m.faces.iter() {
        let npoly = f.len();

        if npoly >= 3 {
            let v0 = m.verts[f[0]];
            let col = image::Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);

            for i in 1..(npoly - 1) {
                let v1 = m.verts[f[i]];
                let v2 = m.verts[f[i + 1]];
                img.triangle(
                    Vec2Du {
                        x: pad + ((v0.x - min_x) * ratio) as u32,
                        y: pad + ((v0.y - min_y) * ratio) as u32
                    },
                    Vec2Du {
                        x: pad + ((v1.x - min_x) * ratio) as u32,
                        y: pad + ((v1.y - min_y) * ratio) as u32
                    },
                    Vec2Du {
                        x: pad + ((v2.x - min_x) * ratio) as u32,
                        y: pad + ((v2.y - min_y) * ratio) as u32
                    },
                    col
                );
            }
        }
    }

    img = flip_vertical(&img);

    // save image
    let _ = img.save("output/triangles.png");
}
