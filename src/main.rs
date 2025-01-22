pub mod geometry;
pub mod model;

use std::task::ready;

use tgaimage::{self, TGAColor, TGAImage};

fn main() {
    let white: TGAColor = TGAColor::rgba(255, 255, 255, 255);
    let red: TGAColor = TGAColor::rgba(255, 0, 0, 255);
    let mut image: TGAImage = TGAImage::new(100, 100, 4);
    geometry::draw_line(13, 20, 80, 40, &mut image, white);
    image.flip_vertically();
    image.write_tga_file("output.tga", true);
}
