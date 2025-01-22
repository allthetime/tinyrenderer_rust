use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Model {
    verts: Vec<Vec3f>,
    faces: Vec<Vec<i32>>,
}

impl Model {
    // Constructor to load model from a file
    pub fn new(filename: &str) -> Self {
        let path = Path::new(filename);
        let file = File::open(path).expect("Unable to open file");
        let reader = BufReader::new(file);

        let mut verts = Vec::new();
        let mut faces = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            if line.starts_with("v ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let x = parts[1].parse::<f32>().unwrap();
                let y = parts[2].parse::<f32>().unwrap();
                let z = parts[3].parse::<f32>().unwrap();
                verts.push(Vec3f { x, y, z });
            } else if line.starts_with("f ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let face = parts[1..]
                    .iter()
                    .map(|&s| s.split('/').next().unwrap().parse::<i32>().unwrap() - 1)
                    .collect();
                faces.push(face);
            }
        }

        Model { verts, faces }
    }

    // Return the number of vertices
    pub fn nverts(&self) -> usize {
        self.verts.len()
    }

    // Return the number of faces
    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    // Return a vertex by index
    pub fn vert(&self, i: usize) -> &Vec3f {
        &self.verts[i]
    }

    // Return a face by index
    pub fn face(&self, idx: usize) -> &Vec<i32> {
        &self.faces[idx]
    }
}
