use crate::geometry::{self, Vec3f};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use tgaimage::{TGAColor, TGAImage};

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

    pub fn draw_model_with_lines(&self, image: &mut TGAImage, color: TGAColor) {
        let (width, height) = (image.width(), image.height());

        for face in &self.faces {
            for i in 0..3 {
                let v0 = &self.verts[face[i as usize] as usize];
                let v1 = &self.verts[face[((i + 1) % 3) as usize] as usize];
                let x0 = ((v0.x + 1.0) * width as f32 / 2.0) as usize;
                let y0 = ((v0.y + 1.0) * height as f32 / 2.0) as usize;
                let x1 = ((v1.x + 1.0) * width as f32 / 2.0) as usize;
                let y1 = ((v1.y + 1.0) * height as f32 / 2.0) as usize;
                geometry::draw_line(x0, y0, x1, y1, image, color);
            }
        }
    }

    // Function to rasterize triangles and draw them on the image
    pub fn triangle_raster(&self, image: &mut TGAImage, color: TGAColor) {
        // Define the light direction
        let light_direction = Vec3f::new(0.0, 0.0, -1.0);

        // Get the dimensions of the image
        let (width, height) = (image.width(), image.height());

        // Iterate over each face of the model
        for face in &self.faces {
            let mut screen_coords: Vec<geometry::Vec2i> = Vec::new();
            let mut world_coords: Vec<geometry::Vec3f> = Vec::new();
            // Convert each vertex of the face to screen coordinates and store world coordinates
            for i in 0..3 {
                let v = &self.verts[face[i as usize] as usize];
                let x = ((v.x + 1.0) * width as f32 / 2.0) as usize;
                let y = ((v.y + 1.0) * height as f32 / 2.0) as usize;
                screen_coords.push(geometry::Vec2i {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
                world_coords.push(*v);
            }
            // Calculate the normal of the face
            let n = (world_coords[2] - world_coords[0])
                .cross(&(world_coords[1] - world_coords[0]))
                .normalize();
            // Calculate the intensity of the light on the face
            let intensity = n * light_direction;
            // Convert screen coordinates to an array
            let screen_coords: [geometry::Vec2i; 3] = screen_coords.try_into().unwrap();
            // If the intensity is positive, draw the triangle
            if intensity > 0.0 {
                geometry::draw_triangle(
                    &screen_coords,
                    image,
                    TGAColor::rgba(
                        (intensity * 255.0) as u8,
                        (intensity * 255.0) as u8,
                        (intensity * 255.0) as u8,
                        255,
                    ),
                );
            }
        }
    }
}
