mod chungus_bench;
mod repeater_clock_reset;

use mchprs_core::plot::{PlotWorld, PLOT_WIDTH};
use mchprs_core::world::storage::Chunk;
use mchprs_save_data::plot_data::PlotData;
use std::env;
use std::path::Path;

fn load_world(path: impl AsRef<Path>) -> PlotWorld {
    let data = PlotData::load_from_file(path).unwrap();

    let chunks: Vec<Chunk> = data
        .chunk_data
        .into_iter()
        .enumerate()
        .map(|(i, c)| Chunk::load(i as i32 / PLOT_WIDTH, i as i32 % PLOT_WIDTH, c))
        .collect();
    PlotWorld {
        x: 0,
        z: 0,
        chunks,
        to_be_ticked: data.pending_ticks,
        packet_senders: Vec::new(),
    }
}

fn main() {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a test to run");
    }

    let name = args[1].as_str();
    match name {
        "chungus_bench" => chungus_bench::run(),
        "repeater_clock_reset" => repeater_clock_reset::run(),
        _ => panic!("Could not find specified test"),
    }
}
