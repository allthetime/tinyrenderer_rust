use std::ops::{Mul, Sub};
use tgaimage::{TGAColor, TGAImage};

// Vec2i represents a 2D vector with integer coordinates
#[derive(Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

// Vec3f represents a 3D vector with floating-point coordinates
#[derive(Default, Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    // Constructor for Vec3f
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }

    // Cross product of two 3D vectors
    pub fn cross(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Normalize the vector to have a length of 1
    pub fn normalize(&self) -> Vec3f {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3f {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

// Subtraction of two 3D vectors
impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Dot product of two 3D vectors
impl Mul for Vec3f {
    type Output = f32;

    fn mul(self, other: Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// Function to draw a line on the image using Bresenham's algorithm
pub fn draw_line(
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    image: &mut TGAImage,
    color: TGAColor,
) {
    // Convert coordinates to signed integers for calculations
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let mut x1 = x1 as i32;
    let mut y1 = y1 as i32;

    // Determine if the line is steep (more vertical than horizontal)
    let steep = (x0 - x1).abs() < (y0 - y1).abs();
    if steep {
        // Swap x and y coordinates if the line is steep
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        // Ensure that we draw from left to right
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    // Calculate differences and error term
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    // Draw the line using Bresenham's algorithm
    for x in x0..=x1 {
        if steep {
            // If the line is steep, plot y,x instead of x,y
            image.set(y as usize, x as usize, &color);
        } else {
            // Plot the point
            image.set(x as usize, y as usize, &color);
        }
        // Update the error term and y coordinate
        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}

// Function to compute barycentric coordinates
pub fn barycentric(pts: &[Vec2i; 3], p: &Vec2i) -> Vec3f {
    // Compute the cross product of two vectors in the triangle plane
    let u = Vec3f::cross(
        &Vec3f {
            x: (pts[2].x - pts[0].x) as f32,
            y: (pts[1].x - pts[0].x) as f32,
            z: (pts[0].x - p.x) as f32,
        },
        &Vec3f {
            x: (pts[2].y - pts[0].y) as f32,
            y: (pts[1].y - pts[0].y) as f32,
            z: (pts[0].y - p.y) as f32,
        },
    );

    // Check if the triangle is degenerate
    if u.z.abs() < 1.0 {
        return Vec3f {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        };
    }
    Vec3f {
        x: 1.0 - (u.x + u.y) / u.z,
        y: u.y / u.z,
        z: u.x / u.z,
    }
}

// Function to draw a filled triangle on the image
pub fn draw_triangle(pts: &[Vec2i; 3], image: &mut TGAImage, color: TGAColor) {
    let width = image.width() as i32;
    let height = image.height() as i32;

    // Initialize bounding box
    let mut bboxmin = Vec2i {
        x: width - 1,
        y: height - 1,
    };
    let mut bboxmax = Vec2i { x: 0, y: 0 };
    let clamp = Vec2i {
        x: width - 1,
        y: height - 1,
    };

    // Compute the bounding box of the triangle
    for pt in pts {
        bboxmin.x = bboxmin.x.min(pt.x).max(0);
        bboxmin.y = bboxmin.y.min(pt.y).max(0);

        bboxmax.x = bboxmax.x.max(pt.x).min(clamp.x);
        bboxmax.y = bboxmax.y.max(pt.y).min(clamp.y);
    }

    // Iterate over the bounding box and draw the triangle
    let mut p = Vec2i { x: 0, y: 0 };
    for x in bboxmin.x..=bboxmax.x {
        for y in bboxmin.y..=bboxmax.y {
            p.x = x;
            p.y = y;
            let bc_screen = barycentric(pts, &p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            image.set(p.x as usize, p.y as usize, &color);
        }
    }
}
