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

  // Scale to play for each octave
  // The numbers are zero-based indexes into a 12-note octave
  // C scale : 0, 2, 4, 5, 7, 9, 11 (C, D, E, F, G, A, B)
  let scale : Vec<u8> = vec![0, 2, 4, 5, 7, 9, 11 ];

  // Set the lowest note on the keyboard
  // C0 = 12, C1 = 24, C2 = 36, ...
  let start_note = 12;

  // choose MIDI instrument to associate with each key
  // see https://en.wikipedia.org/wiki/General_MIDI
  // 1 = Piano, 14 = Xylophone, 18 = Percussive Organ, 41 = Violin
  let instruments : Vec<u8> = vec![ 1, 10, 18, 25, 41, 53, 65, 119 ];

  // we use a fixed velocity of 127 (the max value)
  let velocity = 127;

  // determine the max distance to measure
  let cm_per_note = 2;
  let max_distance : u8 = scale.len() as u8 * cm_per_note;

  // Configure the octasonic breakout board
  let octasonic = Octasonic::new(8).unwrap();
  octasonic.set_max_distance(max_distance);
  octasonic.set_interval(0); // no pause between taking sensor readings

  // init key state
  let mut key : Vec<Key> = vec![];
  for _ in 0 .. 8 {
    key.push(Key::new());
  }

  // create the synth and set instruments per channel
  let synth = Fluidsynth {};
  for i in 0 .. 8 {
    synth.set_instrument(i as u8 + 1, instruments[i]);
  }

  loop {
    for i in 0 .. 8 {

      let channel = i as u8 + 1;

      // get sensor reading
      let distance = octasonic.get_sensor_reading(i as u8);

      // is the key covered?
      if distance < max_distance {

        // the key is covered, so figure out which note to play
        let scale_start = start_note + 12 * i as u8;

        // this is a bit funky ... we use modulus to pick the note within the scale ... it
        // seemed to sound better than trying to divide the distance by the number of notes
        let new_note = scale_start + scale[(distance%7) as usize];

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
          key[i].set_note(0);
          synth.note_off(channel, key[i].note);
        }
      }
    } 
  }
}
