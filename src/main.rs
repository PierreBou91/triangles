use clap::Parser;
use triangles::{igc_file_to_trace, triangle_perimeter};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// igc input file to calculate the perimeter of
    file: String,

    /// Comma separated list of 3 points to for a triangle
    points: String,
}

fn main() {
    let args = Args::parse();

    let trace = igc_file_to_trace(args.file);

    let points: Vec<usize> = args
        .points
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    assert!(points.len() == 3, "Must provide 3 points");

    let perimeter = triangle_perimeter(&trace, points[0], points[1], points[2]);

    println!(
        "Point 1: {:?}\nPoint 2: {:?}\nPoint 3: {:?}",
        trace[points[0]], trace[points[1]], trace[points[2]]
    );

    println!("Perimeter: {:.3} km", perimeter);
}
