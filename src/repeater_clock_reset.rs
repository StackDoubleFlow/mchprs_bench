use mchprs_blocks::BlockPos;
use mchprs_core::redpiler::Compiler;
use mchprs_world::{TickEntry, TickPriority};

use super::load_world;

pub fn run() {
    let mut world = load_world("./plots/repeater_clock");
    let mut compiler: Compiler = Default::default();

    let ticks = world.to_be_ticked.drain(..).collect();
    let bounds = world.get_corners();
    compiler.compile(
        &mut world,
        bounds,
        Default::default(),
        ticks,
        Default::default(),
    );

    compiler.tick();
    compiler.flush(&mut world);
    compiler.reset(&mut world, bounds);

    let expected = vec![
        TickEntry {
            ticks_left: 1,
            tick_priority: TickPriority::Higher,
            pos: BlockPos {
                x: 128,
                y: 8,
                z: 133,
            },
        },
        TickEntry {
            ticks_left: 1,
            tick_priority: TickPriority::High,
            pos: BlockPos {
                x: 129,
                y: 8,
                z: 133,
            },
        },
    ];
    assert_eq!(world.to_be_ticked, expected);
}
