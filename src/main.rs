#![feature(collections)]

extern crate image;

use image::ImageBuffer;
use image::imageops::flip_vertical;
use painting::Painting;
use geometry::load_model_obj;

mod painting;
mod geometry;


fn main() {
    // image processing
    let white = image::Rgb([0xff, 0xff, 0xff]);
    let m = match load_model_obj("obj/phone.obj") {
        Ok(m) => m,
        Err(..) => panic!("couldn't read input file"),
    };

    let width = 1024;
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

    for f in m.faces.iter() {
        let npoly = f.len();

        for i in 0..npoly {
            let v = m.verts[f[i]];
            let u = m.verts[f[(i + 1) % npoly]];
            let x0 = pad + ((v.x - min_x) * ratio) as u32;
            let y0 = pad + ((v.y - min_y) * ratio) as u32;
            let x1 = pad + ((u.x - min_x) * ratio) as u32;
            let y1 = pad + ((u.y - min_y) * ratio) as u32;
            img.line(x0, y0, x1, y1, white);
        }
    }

    img = flip_vertical(&img);

    // save image
    let _ = img.save("output.ppm");
}
