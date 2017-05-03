#!/bin/bash
cargo build --release
./target/release/ultrasonic_piano --mode=modulus --cm-per-note=2 1 10 18 25 41 89 49 14 | fluidsynth -a alsa -g 0.5 -l /usr/share/sounds/sf2/FluidR3_GM.sf2
