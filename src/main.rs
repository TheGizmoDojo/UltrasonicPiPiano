extern crate octasonic;
use octasonic::Octasonic;

mod synth;
use synth::*;

/// State associated with each key
struct Key {
  /// The MIDI note number for the currently playing note, or 0 for no note
  note: u8,
  /// Counter for how many cycles the note has been playing
  counter: u8
}

fn main() {

  let synth = Fluidsynth {};

  // Configure the octasonic breakout board
  let octasonic = Octasonic::new(8).unwrap();
  octasonic.set_max_distance(100);
  octasonic.set_interval(0);

  // Scale to play for each octave
  // The numbers are zero-based indexes into a 12-note octave
  // C scale : 0, 2, 4, 5, 7, 9, 11 (C, D, E, F, G, A, B)
  let scale : Vec<u8> = vec![0, 2, 4, 5, 7, 9, 11 ];

  // init key state
  let mut key : Vec<Key> = vec![];
  for _ in 0 .. 8 {
    key.push(Key { note: 0, counter: 0 });
  }

  let cm_per_note = 2;
  let max_distance : u8 = 7 * cm_per_note;

  let velocity = 127;

  let start_note = 12; // C0

  let instruments : Vec<u8> = vec![ 1, 10, 18, 25, 41, 53, 65, 119 ];

  for i in 0 .. 8 {  
    synth.set_instrument(i as u8 + 1, instruments[i]);
  }

  loop {
    for i in 0 .. 8 {

      let channel = i as u8 + 1;

      // get sensor reading
      let distance = octasonic.get_sensor_reading(i as u8);

      if distance < max_distance {
        let scale_start = start_note + 12 * i as u8;
        let new_note = scale_start + scale[(distance%7) as usize];
        if new_note != key[i].note {

          // stop the previous note on this key (if any) from playing
          if key[i].note > 0 {
            synth.note_off(channel, key[i].note);
          }

          // play the new note
          key[i].note = new_note;
          synth.note_on(channel, key[i].note, velocity);
        }
      } else if key[i].note > 0 {
        key[i].counter = key[i].counter + 1;
        if key[i].counter == 100 {
          synth.note_off(channel, key[i].note);
          key[i].counter = 0;
          key[i].note = 0;
        }
      }
    } 
  }
}
