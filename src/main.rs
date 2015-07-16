extern crate image;

use image::ImageBuffer;
use image::imageops::flip_vertical;
use painting::Painting;
use geometry::Vec3D;
use geometry::Model;
use geometry::load_model_obj;

mod painting;
mod geometry;


fn main() {
    // load/create image
    let mut img = ImageBuffer::new(100, 100);

    // image processing
    let red = image::Rgb([0xff, 0x00, 0x00]);
    let white = image::Rgb([0xff, 0xff, 0xff]);

    img.line(13, 20, 80, 40, white);
    img.line(20, 13, 40, 80, red);
    img.line(80, 40, 13, 20, red);

    img = flip_vertical(&img);

    // save image
    let _ = img.save("output.ppm");

    let v1 = Vec3D {x: 10.0, y: 12.0, z: -10.0};
    let v2 = Vec3D {x: 1.5, y: 1.2, z: -3.0};
    let v3 = v1 + v2;
    let v4 = v1 - v2;
    println!("{:?} {:?}", v3, v4);

    let m = load_model_obj("obj/african_head.obj");
    println!("{:?}", m);
}
