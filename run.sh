#!/bin/bash

# kill any pianos that are already running (perhaps from bootup)
echo "Killing any already running pianos ..."
sudo killall -9 ultrasonic_piano 2>/dev/null

# make sure the latest code is compiled
echo "Compiling code ..."
cargo build --release

# now run the piano, piping the output into fluidsynth
echo "Launching piano ..."
./target/release/ultrasonic_piano --gesture_change_instrument=129 --gesture_shutdown=24 --mode=linear --cm-per-note=2 1 10 18 25 41 89 49 14 | fluidsynth -a alsa -g 1.0 -l /usr/share/sounds/sf2/FluidR3_GM.sf2
