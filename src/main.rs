use std::process::Command;
//use std::time::Duration;
//use std::thread::sleep;

extern crate octasonic;
use octasonic::Octasonic;

extern crate sysfs_gpio;
use sysfs_gpio::{Direction, Pin};

extern crate argparse;
use argparse::{ArgumentParser, Store, List};

mod synth;
use synth::*;

#[derive(Debug)]
enum Mode {
  Modulus,
  Linear
}

#[derive(Debug)]
enum InstrMode {
  /// all sensors play a single instrument, which can be cycled with a gesture
  Single,
  /// each sensor plays a different instrument
  Orchestra
}

/// State associated with each key
struct Key {
  /// The MIDI note number for the currently playing note, or 0 for no note
  note: u8,
  /// Counter for how many cycles the note has been playing
  counter: u8
}

impl Key {

  fn new() -> Self {
    Key { note: 0, counter: 0 }
  }

  fn set_note(&mut self, n: u8) {
    self.note = n;
    self.counter = 0;
  }

}

fn main() {

  //speak(format!("Raspberry Pi Piano Starting Up"));

  // Scale to play for each octave
  // The numbers are zero-based indexes into a 12-note octave
  // C scale : 0, 2, 4, 5, 7, 9, 11 (C, D, E, F, G, A, B)
  let scale : Vec<u8> = vec![0, 2, 4, 5, 7, 9, 11 ];

  // set GPIO21 as input for shutting down the pi
  // connect to 3.3V to shutdown
  let pin = Pin::new(21);
  match pin.export() {
    Ok(_) => println!("# pin exported OK"),
    Err(_) => println!("# Failed to export pin. Did you enable GPIO?")
  }
  match pin.set_direction(Direction::In) {
    Ok(_) => println!("# set pin direction to IN"),
    Err(_) => println!("# Failed to set pin direction to IN")
  }

  // Set the lowest note on the keyboard
  // C0 = 12, C1 = 24, C2 = 36, ...
  let mut start_note = 12;
  let mut octave_offset = 12;

  // choose MIDI instrument to associate with each key
  // see https://en.wikipedia.org/wiki/General_MIDI
  // 1 = Piano, 14 = Xylophone, 18 = Percussive Organ, 41 = Violin
  let mut instruments : Vec<u8> = vec![ 1, 10, 18, 25, 41, 89, 49, 14 ];
  let mut instrument_mode_str = "single".to_string();

  // we use a fixed velocity of 127 (the max value)
  let velocity = 127;

  // determine the max distance to measure
  let mut cm_per_note = 5;
  let mut mode_string = "linear".to_string();

  let mut gesture_change_instrument = 129_u8; // two outermost sensors
  let mut gesture_shutdown = 24_u8; // middle two sensors

  {
    let mut ap = ArgumentParser::new();
    ap.refer(&mut start_note)
      .add_option(&["-a", "--start-note"], Store, "Start note");
    ap.refer(&mut octave_offset)
      .add_option(&["-o", "--octave-offset"], Store, "Octave offset per sensor");
    ap.refer(&mut cm_per_note)
      .add_option(&["-n", "--cm-per-note"], Store, "Distance allocated to each note");
    ap.refer(&mut mode_string)
      .add_option(&["-m", "--mode"], Store, "Mode (linear or modulus)");
    ap.refer(&mut gesture_change_instrument)
      .add_option(&["-c", "--gesture_change_instrument"], Store, "Gesture for changing instrument");
    ap.refer(&mut gesture_shutdown)
      .add_option(&["-s", "--gesture_shutdown"], Store, "Gesture for shutting down");
    ap.refer(&mut instrument_mode_str)
      .add_option(&["-i", "--instrument-mode"], Store, "Instrument mode (single or orchestra)");
    ap.refer(&mut instruments)
        .add_argument("instruments", List, "MIDI instrument numbers");
    ap.parse_args_or_exit();
  }

  let mode = match mode_string.as_ref() {
    "linear" => Mode::Linear,
    _ => Mode::Modulus
  };

  let instrument_mode = match instrument_mode_str.as_ref() {
    "single" => InstrMode::Single,
    _ => InstrMode::Orchestra
  };

  println!("# cm_per_note = {}", cm_per_note);
  println!("# mode = {:?}", mode);
  println!("# instruments: {:?}", instruments);


  let max_distance : u8 = scale.len() as u8 * cm_per_note;

  // Configure the octasonic breakout board
  let octasonic = match Octasonic::new(8) {
    Ok(o) => o,
    Err(_) => panic!("Failed to initialize SPI - have you enabled SPI in the Raspberry Pi Configuration Utility?")
  };	
  octasonic.set_max_distance(2); // 2= 48 cm
  octasonic.set_interval(2); // no pause between taking sensor readings
  let mut distance = vec![0_u8; 8];

  // init key state
  let mut key : Vec<Key> = vec![];
  for _ in 0 .. 8 {
    key.push(Key::new());
  }

  let mut instrument_index = 0_usize;

  // create the synth and set instruments per channel
  let synth = Fluidsynth {};
  for i in 0 .. 8 {
    let instrument_code = match instrument_mode {
      InstrMode::Single => instruments[instrument_index],
      InstrMode::Orchestra => instruments[i]
    };
    synth.set_instrument(i as u8 + 1, instrument_code);
  }

  let mut gesture : u8 = 0;
  let mut gesture_counter : u32 = 0;

  // play scale to indicate that the instrument is ready
  synth.play_scale(1, 48, 12);

  let mut counter = 0_u32;

  loop {

    // check shutdown switch but not every time around the loop
    counter = counter + 1;
    if counter == 100 {
      counter = 0;

      match pin.get_value() {
        Ok(n) if n == 1 => shutdown(&synth, &key),
        _ => {}
      }
    }

    for i in 0 .. 8 {

      let channel = i as u8 + 1;

      // get sensor reading
      distance[i] = octasonic.get_sensor_reading(i as u8);

      // is the key covered?
      if distance[i] < max_distance {

        // the key is covered, so figure out which note to play
        let scale_start = start_note + octave_offset * i as u8;

        // this is a bit funky ... we use modulus to pick the note within the scale ... it
        // seemed to sound better than trying to divide the distance by the number of notes
        let new_note = match mode {
          Mode::Modulus => scale_start + scale[(distance[i]%7) as usize],
          Mode::Linear => scale_start + scale[(distance[i]/cm_per_note) as usize]
        };

        // is this a different note to the one already playing?
        if new_note != key[i].note {

          // stop the previous note on this key (if any) from playing
          if key[i].note > 0 {
            synth.note_off(channel, key[i].note);
          }

          // play the new note
          key[i].set_note(new_note);
          synth.note_on(channel, key[i].note, velocity);
        }

      } else if key[i].note > 0 {
        // a note was playing but the key is not currently covered
        key[i].counter = key[i].counter + 1;
        if key[i].counter == 100 {
          // its time to stop playing this note
          synth.note_off(channel, key[i].note);
          key[i].set_note(0);
        }
      }
    } 

    // convert key distances to single binary number
    let new_gesture :u8 = distance.iter()
              .enumerate()
              .map(|(i,val)| if *val < 15_u8 { 1_u8 << i } else { 0_u8 })
              .sum();

    if gesture == new_gesture {
      gesture_counter += 1;
      if gesture_counter == 150 {

        if gesture == gesture_change_instrument {

            match instrument_mode {
              InstrMode::Orchestra => {},
              InstrMode::Single => {

                // stop existing notes
                for i in 0 .. 8 { synth.note_off(i+1, key[i as usize].note) }

                // choose the next instrument
                instrument_index += 1;
                if instrument_index == instruments.len() { instrument_index = 0; }
                for i in 0 .. 8 { 
                  synth.set_instrument(i as u8 + 1, instruments[instrument_index]); 
                }

                // play a quick scale to indicate that the instrument changed
                synth.play_scale(1, 48, 12);
              }
            }
        } else if gesture == gesture_shutdown {
            shutdown(&synth, &key);
        }

        gesture_counter = 0;
      }
    } else { 
      //println!("gesture: {}", new_gesture);
      // reset counter
      gesture = new_gesture;
      gesture_counter = 0; 
    }
  }
}

fn shutdown(synth: &Synth, key: &Vec<Key>) {
  println!("# shutting down");
            
  // stop existing notes
  for i in 0 .. 8 { synth.note_off(i+1, key[i as usize].note) }
 
  // play scale (hi to lo)
  synth.play_scale(1, 48, 12);

  // issue shutdown command
  Command::new("sh")
    .arg("-c")
    .arg("shutdown now")
    .output()
    .expect("failed to execute shutdown command");

}


