#!/bin/bash
cargo build --release
./target/release/sonic_sounds | fluidsynth -a alsa -g 0.5 -l /usr/share/sounds/sf2/FluidR3_GM.sf2
