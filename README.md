# SonicSounds - Making music on the Raspberry Pi with ultrasonic sensors for input

This code is written for Raspberry Pi with an attached <a href="https://www.tindie.com/products/andygrove73/octasonic-8-x-hc-sr04-ultrasonic-breakout-board/">Octasonic 8 x HC-SR04 breakout board</a>. 

This code has been tested on a Raspberry Pi 3 Model B running Raspian Jessie and Rust stable 1.16.0

You must enable SPI on the Raspberry Pi for this software to work! Use the Raspberry Pi Configuration utility to do this.

The piano example generates MIDI instructions based on sensor readings and writes them to stdout.

This output can be piped into fluidsynth to generate music.

# Video

[![Raspberry Pi Octasonic Piano](https://img.youtube.com/vi/3iLIQvG_j-8/0.jpg)](https://www.youtube.com/watch?v=3iLIQvG_j-8)

# Wiring

TBD

# Software

## Operating System

Start with a clean install of Raspbian Jessie, then update it to the latest version:

```
sudo apt-get update
sudo apt-get upgrade
```

## Install fluidsynth:

```
sudo apt-get install fluidsynth
```

## Install the Rust Programming Language

```
curl https://sh.rustup.rs -sSf | sh
```

## Compile this code.

```
git clone git@github.com:TheGizmoDojo/SonicSounds.git
cd SonicSounds
cargo build --release 
```

## Run the code

This runs the sonic_sounds application and pipes the stdout output into the stdin input of fluidsynth. The `-g` specifies the "gain" or volume.
```
./target/release/sonic_sounds | fluidsynth -a alsa -g 0.5 -l /usr/share/sounds/sf2/FluidR3_GM.sf2
```

# Run on startup

Add this to `/etc/rc.local` and be sure to use the correct paths for where you installed this software.

```
. /home/pi/.cargo/env
cd /home/pi/projects/SonicSounds
./run.sh
```

# Troubleshooting

If there is no audio from the audio jack, run this command to force audio to the headphone jack rather than HDMI:

```
sudo amixer cset numid=3 1
```
