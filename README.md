# SonicSounds - Making music on the Raspberry Pi with ultrasonic sensors for input

This code is written for Raspberry Pi with an attached <a href="https://www.tindie.com/products/andygrove73/octasonic-8-x-hc-sr04-ultrasonic-breakout-board/">Octasonic 8 x HC-SR04 breakout board</a>.

This code has been tested on a Raspberry Pi 3 Model B running Raspian Jessie and Rust stable 1.16.0

You must enable SPI on the Raspberry Pi for this software to work! Use the Raspberry Pi Configuration utility to do this.

## Piano

The piano example generates MIDI instructions based on sensor readings and writes them to stdout.

This output can be piped into fluidsynth to generate music.

Install fluidsynth:

```
sudo apt-get install fluidsynth
```

Install Rust

```
curl https://sh.rustup.rs -sSf | sh
```

Compile this code.

```
cargo build --release 
```

Run the code.

```
./target/release/sonic_sounds | fluidsynth -a alsa -g 0.5 -l /usr/share/sounds/sf2/FluidR3_GM.sf2
```
