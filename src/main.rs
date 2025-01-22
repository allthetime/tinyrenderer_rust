pub mod geometry;
pub mod model;

use geometry::Vec2i;
use tgaimage::{self, TGAColor, TGAImage};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let white: TGAColor = TGAColor::rgba(255, 255, 255, 255);

    let model = model::Model::new("./obj/african_head.obj");

    let frame: &mut TGAImage = &mut TGAImage::new(WIDTH as usize, HEIGHT as usize, 4);
    model.triangle_raster(frame, white);

    frame.flip_vertically();
    frame.write_tga_file("output.tga", true);
}
