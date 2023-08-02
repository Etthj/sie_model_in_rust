mod sie;

use std::io;
use sie::SIE;
use nalgebra::{Vector2, Point2};

fn main() {
    // Get user input for SIE parameters
    println!("Enter the Einstein radius:");
    let einstein_radius: f64 = read_input();

    println!("Enter the axis ratio (0 < axis_ratio <= 1):");
    let axis_ratio: f64 = read_input();

    println!("Enter the orientation angle (in radians):");
    let orientation: f64 = read_input();

    // Create the SIE model
    let sie_model = SIE::new(einstein_radius, axis_ratio, orientation);

    // Get user input for the position of the point source
    println!("Enter the x-coordinate of the point source:");
    let source_x: f64 = read_input();

    println!("Enter the y-coordinate of the point source:");
    let source_y: f64 = read_input();

    let source_position = Vector2::new(source_x, source_y);

    // Compute the deflection angle at the source position
    let deflection = sie_model.deflection_angle(source_position);

    // Compute the position of the lensed images
    let image_position_1 = Point2::from(source_position - deflection);
    let image_position_2 = Point2::from(source_position + deflection);

    // Print the results
    println!("Image 1: {:?}", image_position_1);
    println!("Image 2: {:?}", image_position_2);
}

fn read_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}