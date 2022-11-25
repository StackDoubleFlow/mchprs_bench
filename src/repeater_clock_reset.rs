use mchprs_blocks::BlockPos;
use mchprs_core::redpiler::Compiler;
use mchprs_world::{TickEntry, TickPriority};

use super::load_world;

pub fn run() {
    let mut world = load_world("./plots/repeater_clock");
    let mut compiler: Compiler = Default::default();

    let ticks = world.to_be_ticked.drain(..).collect();
    dbg!(&ticks);
    compiler.compile(&mut world, Default::default(), ticks);

    compiler.tick(&mut world);
    compiler.flush(&mut world);
    compiler.reset(&mut world);

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
