#![feature(collections)]

extern crate image;
extern crate num;

use image::ImageBuffer;
use image::imageops::flip_vertical;
use painting::Painting;
use geometry::*;

mod painting;
mod geometry;


fn main() {
    // image processing
    let m = match load_model_obj("obj/african_head.obj") {
        Ok(m) => m,
        Err(..) => panic!("couldn't read input file"),
    };

    let width = 1024 as i32;
    let pad = 10 as i32;

    // TODO: check m.verts size
    let mut min_coords = Vec3Df { x: m.verts[0].x, y: m.verts[0].y, z: m.verts[0].z };
    let mut max_coords = Vec3Df { x: m.verts[0].x, y: m.verts[0].y, z: m.verts[0].z };

    for &v in m.verts.iter() {
        min_coords.min_bound(v);
        max_coords.max_bound(v);
    }

    let ratio = (width - 2 * pad) as f64 / (max_coords.x - min_coords.x);
    let height = ((max_coords.y - min_coords.y) * ratio + 2.0 * pad as f64) as i32;

    let mut img = ImageBuffer::new(width as u32, height as u32);
    let light = (Vec3Df { x: 0.0, y: 0.0, z: -1.0 }).normalized();

    let mut zbuffer = vec![vec![std::i32::MIN; height as usize]; width as usize];
    let padding = Vec3Di { x: pad, y: pad, z: pad };

    for f in m.faces.iter() {
        let npoly = f.len();

        if npoly >= 3 {
            let v0 = m.verts[f[0]];
            let l = light * ((m.verts[f[2]] - v0).cross((m.verts[f[1]] - v0))).normalized();

            if l > 0.0 {
                let dens = (255.0 * l) as u8;
                let col = image::Rgb([dens, dens, dens]);

                for i in 1..(npoly - 1) {
                    let v1 = m.verts[f[i]];
                    let v2 = m.verts[f[i + 1]];

                    img.triangle(
                        padding + (v0 - min_coords).scale(ratio).to_i32(),
                        padding + (v1 - min_coords).scale(ratio).to_i32(),
                        padding + (v2 - min_coords).scale(ratio).to_i32(),
                        col,
                        &mut zbuffer
                    );
                }
            }
        }
    }

    img = flip_vertical(&img);

    // save image
    let _ = img.save("output/triangles.png");
}
