use std::{
    collections::HashMap,
    fs::File,
    io::{self},
};
use triangles::{igc_file_to_trace, Trace};

fn main() -> io::Result<()> {
    let mut dataset = HashMap::<usize, Trace>::new();

    for igc_file in 1..=11 {
        let trace = igc_file_to_trace(format!("data/{}.igc", igc_file));

        dataset.insert(igc_file, trace.clone());

        serde_json::to_writer_pretty(
            File::create(format!("data/{}.json", igc_file)).expect("Unable to create file"),
            &trace,
        )
        .unwrap();
    }

    serde_json::to_writer_pretty(
        File::create("data/dataset.json").expect("Unable to create file"),
        &dataset,
    )
    .unwrap();

    Ok(())
}
