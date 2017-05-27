use std::{thread, time};

/// use a trait to define the synth to make it easier to run against other synths in the future
pub trait Synth {
    fn note_on(&self, channel: u8, note: u8, velocity: u8);
    fn note_off(&self, channel: u8, note: u8);
    fn set_instrument(&self, channel: u8, instrument: u8);
    fn play_scale(&self, channel: u8, start_note: u8, count: i8);
}

/// struct to represent a fluidsynth process
pub struct Fluidsynth {
    //NOTE: there is no state associated with the synth
}

impl Synth for Fluidsynth {

    fn note_on(&self, channel: u8, note: u8, velocity: u8) {
        println!("noteon {} {} {}", channel, note, velocity);
    }

    fn note_off(&self, channel: u8, note: u8) {
        println!("noteoff {} {}", channel, note);
    }

    fn set_instrument(&self, channel: u8, instrument: u8) {
        println!("prog {} {}", channel, instrument);
    }


    fn play_scale(&self, channel: u8, start_note: u8, count: i8) {
      for i in 0 .. count {
        let note = (start_note as i8 + i) as u8;
        self.note_on(channel, note, 127);
        thread::sleep(time::Duration::from_millis(250));
        self.note_off(channel, note);
      }
    }
}
