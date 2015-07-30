use num::traits::Float;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::ops::{Add, Sub, Mul};
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

impl<T> Mul<Vec3D<T>> for Vec3D<T> where T: Mul<T, Output = T> + Add<T, Output = T> {
    type Output = T;

    fn mul(self, other: Vec3D<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T> Vec3D<T> where T: Mul<T, Output = T> + Sub<T, Output = T> + Copy {
    pub fn cross(&self, other: Vec3D<T>) -> Vec3D<T> {
        Vec3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl<T> Vec3D<T> where T: PartialOrd {
    pub fn min_bound(&mut self, v: Vec3D<T>) -> &mut Self {
        if v.x < self.x { self.x = v.x; }
        if v.y < self.y { self.y = v.y; }
        if v.z < self.z { self.z = v.z; }
        self
    }

    pub fn max_bound(&mut self, v: Vec3D<T>) -> &mut Self {
        if v.x > self.x { self.x = v.x; }
        if v.y > self.y { self.y = v.y; }
        if v.z > self.z { self.z = v.z; }
        self
    }
}

impl<T> Vec3D<T> where T: Float {
    pub fn normalized(self) -> Vec3D<T> {
        let l = (self * self).sqrt();

        if l == T::zero() {
            self
        } else {
            Vec3D {
                x: self.x / l,
                y: self.y / l,
                z: self.z / l
            }
        }
    }
}

impl Vec3Df {
    pub fn to_i32(&self) -> Vec3Di {
        Vec3Di { x: self.x as i32, y: self.y as i32, z: self.z as i32 }
    }

    pub fn scale(&self, s: f64) -> Self {
        Vec3Df { x: self.x * s, y: self.y * s, z: self.z * s }
    }
}

pub type Vec2Di = Vec2D<i32>;
pub type Vec2Du = Vec2D<u32>;
pub type Vec2Df = Vec2D<f64>;

pub type Vec3Di = Vec3D<i32>;
pub type Vec3Du = Vec3D<u32>;
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
