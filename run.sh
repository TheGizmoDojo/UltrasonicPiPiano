#!/bin/bash
cargo build --release
./target/release/ultrasonic_piano --gesture_change_instrument=129 --gesture_shutdown=24 --mode=linear --cm-per-note=2 1 10 18 25 41 89 49 14 | fluidsynth -a alsa -g 1.0 -l /usr/share/sounds/sf2/FluidR3_GM.sf2
