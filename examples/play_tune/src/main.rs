//! This is a example program that plays a simple, 3-channel tune via opl3-rs and the rodio audio
//! library.
//!
//! Original code by Maarten Janssen (maarten@cheerful.nl) 2016-04-13
//! Most recent version of the library can be found at my GitHub: https://github.com/DhrBaksteen/ArduinoOPL2
//! Hacked for a OPL2LPT test program by pdewacht@gmail.com.

mod opl;

fn main() {
    println!("Hello, world!");
}

struct Tune {
    data: Vec<u8>,
    channel: i32,
    octave: i32,
    note_duration: i32,
    note_length: i32,
    unsigned long nextNoteTime;
    unsigned long releaseTime;
    int index;
};
