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
    let m = match load_model_obj("obj/african_head.obj") {
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
    	if v.x > max_x {
    		max_x = v.x;
    	}
    	if v.x < min_x {
    		min_x = v.x;
    	}
    	if v.y > max_y {
    		max_y = v.y;
    	}
    	if v.y < min_y {
    		min_y = v.y;
    	}
	}    
    
    let ratio = (width - 2 * pad) as f64 / (max_x - min_x);
    let height = ((max_y - min_y) * ratio + 2.0 * pad as f64) as u32;
    
    let mut img = ImageBuffer::new(width, height);
    
    for v in m.verts.iter() {
    	let x = pad + ((v.x - min_x) * ratio) as u32;
    	let y = pad + ((v.y - min_y) * ratio) as u32;
    	img.line(x, y, x, y, white);
    }

    img = flip_vertical(&img);
    
    // save image
    let _ = img.save("output.ppm");
}
