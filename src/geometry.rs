use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::ops::{Add, Sub};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add<Vec2D<T>> for Vec2D<T> where T: Add<T, Output = T> {
    type Output = Vec2D<T>;

    fn add(self, other: Vec2D<T>) -> Vec2D<T> {
        Vec2D { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T> Sub<Vec2D<T>> for Vec2D<T> where T: Sub<T, Output = T> {
    type Output = Vec2D<T>;

    fn sub(self, other: Vec2D<T>) -> Vec2D<T> {
        Vec2D { x: self.x - other.x, y: self.y - other.y }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Vec3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Add<Vec3D<T>> for Vec3D<T> where T: Add<T, Output = T> {
    type Output = Vec3D<T>;

    fn add(self, other: Vec3D<T>) -> Vec3D<T> {
        Vec3D { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<T> Sub<Vec3D<T>> for Vec3D<T> where T: Sub<T, Output = T> {
    type Output = Vec3D<T>;

    fn sub(self, other: Vec3D<T>) -> Vec3D<T> {
        Vec3D { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

pub type Vec3Df = Vec3D<f64>;

#[derive(Debug)]
pub struct Model {
    pub verts: Vec<Vec3Df>,
    pub faces: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct Facet {
	pub v: usize,
	pub t: usize,
	pub n: usize,
}

fn parse_facet_obj(str: &str) -> Facet {
	let idx: Vec<&str> = str.split('/').collect();
	
	if idx.len() == 1 {
		Facet { v: idx[0].parse::<usize>().unwrap(), t: 0, n: 0 }
	} else if idx.len() >= 3 {
		Facet {
			v: idx[0].parse::<usize>().unwrap(),
			t: if idx[1] == "" {0} else {idx[1].parse::<usize>().unwrap()},
			n: idx[2].parse::<usize>().unwrap()
		}
	} else {
		Facet { v: 0, t: 0, n: 0 }
	}
}

pub fn load_model_obj<P>(path: P) -> Result<Model, io::Error> where P: AsRef<Path> {
    let f = match File::open(&path) {
        Err(why)    => return Err(why),
        Ok(f)       => f,
    };

    let reader = BufReader::new(&f);
    let mut vs = Vec::new();
    let mut fs = Vec::new();
    
    for line in reader.lines().filter_map(|res| res.ok()) {
    	let v: Vec<&str> = line.split(' ').collect();
    	
		if v.len() >= 4 && v[0] == "v" {
			let coords: Vec<f64> = v.tail().iter().map(|s| s.trim()).filter_map(
				|s| if s == "" {None} else {Some(s.parse::<f64>().unwrap())}
				).collect();
			
			if coords.len() >= 3 {
				vs.push(Vec3Df { x: coords[0], y: coords[1], z: coords[2] });
			}
		} else if v.len() >= 4 && v[0] == "f" {
			// TODO: add error handling (0 vertex -> Result<Facet>)
			let verts: Vec<usize> = v.tail().iter().map(|s| s.trim()).filter_map(
				|s| if s == "" {None} else {Some(parse_facet_obj(s).v - 1)}
				).collect();
			
			if verts.len() >= 2 {
				fs.push(verts);
			}
		}
    }

    Ok(Model { verts: vs, faces: fs })
}

