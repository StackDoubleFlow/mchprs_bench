use super::load_world;
use mchprs_blocks::BlockPos;
use mchprs_core::plot::PlotWorld;
use mchprs_core::redpiler::{Compiler, CompilerOptions};
use std::time::Instant;

const START_BUTTON: BlockPos = BlockPos::new(187, 99, 115);

fn init_compiler(world: &mut PlotWorld) -> Compiler {
    let mut compiler: Compiler = Default::default();

    let options = CompilerOptions::parse("-O");

    let compile_start = Instant::now();
    compiler.compile(world, options, Vec::new());
    println!("Compile completed in {:?}", compile_start.elapsed());

    compiler.on_use_block(world, START_BUTTON);
    compiler
}

pub fn run() {
    let mut world = load_world("./plots/chungus_mandelbrot_plot");
    let mut compiler = init_compiler(&mut world);
    let start = Instant::now();
    for _ in 0..12411975 {
        compiler.tick(&mut world);
    }
    println!("Mandelbrot benchmark completed in {:?}", start.elapsed());
}
