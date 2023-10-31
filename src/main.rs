use clap::Parser;
use triangles::{triangle_perimeter, Point, Trace};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// igc input file
    file: String,
}

const COMBINATIONS: [(usize, usize, usize); 4] = [(0, 1, 2), (0, 1, 3), (0, 2, 3), (1, 2, 3)];

fn main() {
    let args = Args::parse();

    let trace = triangles::igc_file_to_trace(args.file);

    let extremes = extreme_points(&trace);
    println!("Extremes: {:#?}", extremes);

    let mut max_perim: f64 = 0.0;
    let mut best_combin: (usize, usize, usize) = (0, 0, 0);

    for (i, j, k) in COMBINATIONS {
        let perim = triangle_perimeter(&trace, extremes[i].0, extremes[j].0, extremes[k].0);
        if perim > max_perim {
            max_perim = perim;
            best_combin = (i, j, k);
        }
        println!("Perimeter: {:.3} km", perim);
    }

    let (i, j, k) = best_combin;
    println!(
        "Best combination: {:?},{:?},{:?}",
        extremes[i].0, extremes[j].0, extremes[k].0
    );
    println!("Max perimeter: {:.3} km", max_perim);
    println!(
        "Distances: {:#?}",
        [
            extremes[i].1.distance(&extremes[j].1),
            extremes[i].1.distance(&extremes[k].1),
            extremes[j].1.distance(&extremes[k].1),
        ]
    );
}

fn extreme_points(trace: &Trace) -> Vec<(usize, Point)> {
    let mut max_lat = trace[0].lat;
    let mut max_lat_index = 0;
    let mut min_lat = trace[0].lat;
    let mut min_lat_index = 0;
    let mut max_lon = trace[0].lon;
    let mut max_lon_index = 0;
    let mut min_lon = trace[0].lon;
    let mut min_lon_index = 0;
    let mut extremes = Vec::<(usize, Point)>::new();
    for (index, point) in trace.iter().enumerate() {
        if point.lat > max_lat {
            max_lat = point.lat;
            max_lat_index = index;
        }
        if point.lat < min_lat {
            min_lat = point.lat;
            min_lat_index = index;
        }
        if point.lon > max_lon {
            max_lon = point.lon;
            max_lon_index = index;
        }
        if point.lon < min_lon {
            min_lon = point.lon;
            min_lon_index = index;
        }
    }
    extremes.push((max_lat_index, trace[max_lat_index].clone()));
    extremes.push((min_lat_index, trace[min_lat_index].clone()));
    extremes.push((max_lon_index, trace[max_lon_index].clone()));
    extremes.push((min_lon_index, trace[min_lon_index].clone()));
    extremes
}

// 1 2 3
// 1 2 4
// 1 3 4
// 2 3 4
