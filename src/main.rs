extern crate octasonic;
use octasonic::Octasonic;

use std::thread::sleep;
use std::time::Duration;

struct Key {
  playing: bool,
  counter: u8
}


fn main() {

  let octasonic = Octasonic::new(8).unwrap();

  octasonic.set_max_distance(100);
  octasonic.set_interval(0);

  let scale : Vec<u8> = vec![0, 2, 4, 5, 7, 9, 11 ];

  // init key state
  let mut key : Vec<Key> = vec![];
  for _ in 0 .. 8 {
    key.push(Key { playing: false, counter: 0 });
  }

  let cm_per_note = 2;
  let max_distance : u8 = 7 * cm_per_note;

  let mut note : Vec<u8> = vec![ 0_u8; 8 ];

  let channel = 1; // single channel for now

  let start_note = 24; // C1

  loop {
    for i in 0 .. 8 {

      // get sensor reading
      let distance = octasonic.get_sensor_reading(i as u8);

      if distance < max_distance {
        let scale_start = start_note + 12 * i as u8;
        let new_note = scale_start + scale[(distance%7) as usize];
        //let new_note = scale_start + scale[(distance/cm_per_note) as usize];
        if new_note != note[i] {
          let velocity = 127;
          note[i] = new_note;
          println!("noteon {} {} {}", channel, note[i], velocity);
        }
      }

    } 
//    sleep(Duration::from_millis(50));
  }
}
