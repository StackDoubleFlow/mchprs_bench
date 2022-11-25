#!/bin/sh

cargo build --release
perf record --call-graph=dwarf ./target/release/mchprs_test
