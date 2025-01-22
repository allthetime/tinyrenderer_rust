use tgaimage::{TGAColor, TGAImage};

// Function to draw a line on the image
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
