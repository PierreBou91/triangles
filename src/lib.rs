use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub time: usize,
    pub lat: f64,
    pub lon: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let r = 6371.0; // km
        let d_lat = (other.lat - self.lat).to_radians();
        let d_lon = (other.lon - self.lon).to_radians();
        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();

        let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
            + (d_lon / 2.0).sin() * (d_lon / 2.0).sin() * lat1.cos() * lat2.cos();
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        r * c
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.time.cmp(&other.time))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Point {}

pub type Trace = Vec<Point>;

pub fn igc_file_to_trace(igc_file: String) -> Trace {
    let f = File::open(igc_file).unwrap();
    let buf_reader = BufReader::new(f);

    let mut trace = Trace::new();

    buf_reader
        .split(b'\n') // returns an iterator over byte slices split by newline
        // .par_bridge()
        .filter_map(Result::ok)
        .filter(|line| line.starts_with(b"B")) // compare with byte slice
        .for_each(|line| {
            let p = Point {
                time: String::from_utf8_lossy(&line[1..7])
                    .parse::<usize>()
                    .unwrap(),
                lat: lat_to_decimal(&line[7..15]),
                lon: lon_to_decimal(&line[15..24]),
            };
            trace.push(p);
            // println!("p {:#?}", p);
        });
    trace.sort();
    trace
}

fn lon_to_decimal(gps: &[u8]) -> f64 {
    let degrees = String::from_utf8_lossy(&gps[0..3]).parse::<f64>().unwrap();
    let minutes = String::from_utf8_lossy(&gps[3..5]).parse::<f64>().unwrap();
    let thousands_of_mins = String::from_utf8_lossy(&gps[6..8]).parse::<f64>().unwrap();
    let east_west = if gps[8] == b'E' { 1.0 } else { -1.0 };
    east_west
        * (((degrees + (minutes + thousands_of_mins / 1000.0) / 60.0) * 1000000.0).round()
            / 1000000.0)
}

fn lat_to_decimal(gps: &[u8]) -> f64 {
    let degrees = String::from_utf8_lossy(&gps[0..2]).parse::<f64>().unwrap();
    let minutes = String::from_utf8_lossy(&gps[2..4]).parse::<f64>().unwrap();
    let thousands_of_mins = String::from_utf8_lossy(&gps[5..7]).parse::<f64>().unwrap();
    let north_south = if gps[7] == b'N' { 1.0 } else { -1.0 };
    north_south
        * (((degrees + (minutes + thousands_of_mins / 1000.0) / 60.0) * 1000000.0).round()
            / 1000000.0)
}

pub fn triangle_perimeter(
    trace: &[Point],
    points_1: usize,
    points_2: usize,
    points_3: usize,
) -> f64 {
    let distance1 = trace[points_1].distance(&trace[points_2]);
    let distance2 = trace[points_2].distance(&trace[points_3]);
    let distance3 = trace[points_3].distance(&trace[points_1]);
    distance1 + distance2 + distance3
}
