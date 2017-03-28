extern crate octasonic;
use octasonic::Octasonic;

struct Key {
  note: u8,
  counter: u8
}

trait Synth {
  fn note_on(&self, channel: u8, note: u8, velocity: u8);
  fn note_off(&self, channel: u8, note: u8);
}

struct Fluidsynth;

impl Synth for Fluidsynth {

  fn note_on(&self, channel: u8, note: u8, velocity: u8) {
    println!("noteon {} {} {}", channel, note, velocity);
  }
  
  fn note_off(&self, channel: u8, note: u8) {
    println!("noteoff {} {}", channel, note);
  }

}


fn main() {

  let synth = Fluidsynth;

  let octasonic = Octasonic::new(8).unwrap();

  octasonic.set_max_distance(100);
  octasonic.set_interval(0);

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

  let channel = 1; // single channel for now
  println!("prog 1 18"); // organ

  loop {
    for i in 0 .. 8 {

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
