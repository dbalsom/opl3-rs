//! Music player interface for play_tune example.
//!
//! Original code (C) Maarten Janssen (maarten@cheerful.nl) 2016-04-13
//! https://github.com/DhrBaksteen/ArduinoOPL2
//! Hacked for a OPL2LPT test program Peter De Wachter (pdewacht@gmail.com).
//! https://github.com/pdewacht/adlipt
//! Rewritten in Rust by Daniel Balsom for opl3-rs
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of this software
//! and associated documentation files (the “Software”), to deal in the Software without
//! restriction, including without limitation the rights to use, copy, modify, merge, publish,
//! distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
//! Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE

use crossbeam_channel::Sender;

use opl3_rs::Opl3Device;

use crate::opl::*;
use crate::opl_instruments::{OPL_INSTRUMENT_PIANO1, OplInstrument};

const TIMER_FREQ: i64 = 100; // We will set a timer callback at 100Hz
const SONG_LENGTH: u32 = 30; // Length of the song in seconds
const SONG_TIMER_LENGTH: u32 = SONG_LENGTH * TIMER_FREQ as u32; // Length of the song in timer ticks

const NOTE_DEFS: [u8; 21] = [
    NOTE_A,
    NOTE_A - 1,
    NOTE_A + 1,
    NOTE_B,
    NOTE_B - 1,
    NOTE_B + 1,
    NOTE_C,
    0,
    NOTE_C + 1,
    NOTE_D,
    NOTE_D - 1,
    NOTE_D + 1,
    NOTE_E,
    NOTE_E - 1,
    NOTE_E + 1,
    NOTE_F,
    NOTE_F - 1,
    NOTE_F + 1,
    NOTE_G,
    NOTE_G - 1,
    NOTE_G + 1,
];

const TUNE_DATA: [&'static str; 3] = [
    "t150m200o5l8egredgrdcerc<b>er<ba>a<a>agdefefedr4.regredgrdcerc<b>er<ba>a<a>agdedcr4.c<g>cea>cr<ag>cr<gfarfearedgrdcfrc<bagab>cdfegredgrdcerc<b>er<ba>a<a>agdedcr4.cro3c2",
    "m85o3l8crer<br>dr<ar>cr<grbrfr>cr<grbr>crer<gb>dgcrer<br>dr<ar>cr<grbrfr>cr<grbr>ceger4.rfrafergedrfdcrec<br>d<bar>c<agrgd<gr4.o4crer<br>dr<ar>cr<grbrfr>cr<grbr>cege",
    "m85o3l8r4gr4.gr4.er4.err4fr4.gr4.gr4.grr4gr4.er4.er4.frr4gr4>ccr4ccr4<aarraar4ggr4ffr4.ro4gab>dr4.r<gr4.gr4.err4er4.fr4.g"
];

pub enum CallbackMessage {
    Error,
    EndPlayback,
    HaveSamples(Vec<i16>),
}

#[derive(Default)]
pub struct TuneData {
    data: Vec<u8>,
    cursor: usize,
}

impl TuneData {
    fn new(data: &str) -> TuneData {
        TuneData {
            data: data.as_bytes().to_vec(),
            cursor: 0,
        }
    }

    fn index(&self) -> usize {
        self.cursor
    }

    fn get(&mut self) -> u8 {
        if self.cursor >= self.data.len() {
            return 0;
        }
        let result = self.data[self.cursor];
        self.cursor += 1;
        return result;
    }

    fn peek(&self) -> u8 {
        if self.cursor >= self.data.len() {
            return 0;
        }
        return self.data[self.cursor];
    }

    fn peek_next(&self) -> u8 {
        if (self.cursor + 1) >= self.data.len() {
            return 0;
        }
        return self.data[self.cursor + 1];
    }

    fn next(&mut self) {
        self.cursor += 1;
    }

    fn prev(&mut self) {
        self.cursor -= 1;
    }

    fn reset(&mut self) {
        self.cursor = 0;
    }
}

struct Tune {
    data: TuneData,
    channel: u8,
    octave: i32,
    note_duration: i32,
    note_length: u32,
    next_note_time: u32,
    release_time: u32,
}

impl Default for Tune {
    fn default() -> Tune {
        Tune {
            data: TuneData::default(),
            channel: 4,
            octave: 4,
            note_duration: 4,
            note_length: 85,
            next_note_time: 0,
            release_time: 0,
        }
    }
}

impl Tune {
    fn new(index: usize) -> Tune {
        Tune {
            channel: index as u8,
            data: TuneData::new(TUNE_DATA[index]),
            ..Tune::default()
        }
    }
}

pub struct MusicPlayer {
    tunes: Vec<Tune>,
    tempo: u32,
    timer: u32,
    sample_buf: Vec<i16>,
    opl3: Opl3Device,
    sender: Sender<CallbackMessage>,
    playing: bool,
    debug: bool,
}

impl MusicPlayer {
    pub fn new(sample_rate: u32, sender: Sender<CallbackMessage>, debug: bool) -> MusicPlayer {
        let samples_per_interval = (sample_rate / TIMER_FREQ as u32) as usize;
        println!("Samples per interval: {}", samples_per_interval);
        MusicPlayer {
            tunes: vec![Tune::new(0), Tune::new(1), Tune::new(2)],
            tempo: 120,
            timer: 0,
            sample_buf: vec![0; samples_per_interval * 2],
            opl3: Opl3Device::new(sample_rate),
            sender,
            playing: false,
            debug,
        }
    }

    pub fn setup(&mut self) {
        self.tempo = 120;
        self.opl3.reset(None);

        self.opl3.write_register(0x01, 0x20, true); // Set WSE=1

        set_instrument(&mut self.opl3, 0, &OPL_INSTRUMENT_PIANO1);
        set_block(&mut self.opl3, 0, 5);
        set_instrument(&mut self.opl3, 1, &OPL_INSTRUMENT_PIANO1);
        set_block(&mut self.opl3, 1, 4);
        set_instrument(&mut self.opl3, 2, &OPL_INSTRUMENT_PIANO1);
        set_block(&mut self.opl3, 2, 4);

        self.playing = true;
    }

    pub fn play_test_note(&mut self, instrument: &OplInstrument, note: u8, octave: u8) {
        set_instrument(&mut self.opl3, 0, instrument);
        set_block(&mut self.opl3, 0, octave);
        set_key_on(&mut self.opl3, 0, false);
        let f = get_note_frequency(&mut self.opl3, 0, 4, NOTE_DEFS[note as usize]);
        set_frequency(&mut self.opl3, 0, f);
        set_key_on(&mut self.opl3, 0, true);
    }

    pub fn generate_direct(&mut self, samples: &mut [i16]) {
        self.opl3.generate_samples(samples);
    }

    pub fn timer_callback(&mut self) {
        //println!("Hello from timer callback! Playing is {:?}", self.playing);
        if self.playing {
            self.timer += 1;

            if self.timer >= SONG_TIMER_LENGTH {
                self.playing = false;
                self.sender.send(CallbackMessage::EndPlayback).unwrap();
                return;
            }
            self.main_loop();
            self.opl3.generate_samples(&mut self.sample_buf);

            self.sender
                .send(CallbackMessage::HaveSamples(self.sample_buf.clone()))
                .unwrap();
        }
    }

    pub fn get_timer(&self) -> u32 {
        self.timer
    }

    pub fn main_loop(&mut self) {
        //println!("Hello from main_loop()!");
        let mut busy = false;
        for i in 0..3 {
            if self.get_timer() >= self.tunes[i].release_time
                && get_key_on(&mut self.opl3, self.tunes[i].channel)
            {
                //println!("Releasing note.");
                set_key_on(&mut self.opl3, self.tunes[i].channel, false);
            }
            if self.get_timer() >= self.tunes[i].next_note_time && self.tunes[i].data.peek() != 0 {
                self.parse_tune(i);
            }
            if self.tunes[i].data.peek() != 0 || self.get_timer() < self.tunes[i].next_note_time {
                busy = true;
            }
        }

        if !busy {
            // If all tunes are done, reset them.
            println!("Resetting music...");
            for i in 0..3 {
                self.tunes[i].data.reset();
                self.tunes[i].next_note_time = self.get_timer() + 500;
            }
        }
    }

    fn parse_tune(&mut self, t: usize) {
        // Read and process tune data until we find a note or pause command.
        while self.tunes[t].data.peek() != 0 {
            if self.tunes[t].data.peek() == b'<' && self.tunes[t].octave > 1 {
                // '<': Decrease octave if greater than 1.
                self.tunes[t].octave -= 1;
            } else if self.tunes[t].data.peek() == b'>' && self.tunes[t].octave < 7 {
                // '>': Increase octave if less than 7.
                self.tunes[t].octave += 1;
            } else if self.tunes[t].data.peek() == b'o'
                && self.tunes[t].data.peek_next() >= b'1'
                && self.tunes[t].data.peek_next() <= b'7'
            {
                // 'o': Set octave.
                self.tunes[t].octave = (self.tunes[t].data.peek_next() - b'0') as i32;
                self.tunes[t].data.next();
            } else if self.tunes[t].data.peek() == b'l' {
                // 'l': Set default note duration.
                self.tunes[t].data.next();
                let duration = self.parse_number(t);
                if duration != 0 {
                    self.tunes[t].note_duration = duration as i32;
                }
            } else if self.tunes[t].data.peek() == b'm' {
                // 'm': Set note length in percent.
                self.tunes[t].data.next();
                self.tunes[t].note_length = self.parse_number(t) as u32;
            } else if self.tunes[t].data.peek() == b't' {
                // 't': Set song tempo.
                self.tunes[t].data.next();
                self.tempo = self.parse_number(t) as u32;
                if self.tempo == 0 {
                    // Tempo cannot be 0
                    self.tempo = 1;
                }
            } else if self.tunes[t].data.peek() == b'p' || self.tunes[t].data.peek() == b'r' {
                // 'p' or 'r': Pause.
                self.tunes[t].data.next();
                self.tunes[t].next_note_time = self.get_timer() + self.parse_duration(t);
                break;
            } else if self.tunes[t].data.peek() >= b'a' && self.tunes[t].data.peek() <= b'g' {
                // 'a'-'g': Play note.
                self.parse_note(t);
                break;
            }

            self.tunes[t].data.next();
        }
    }

    fn parse_note(&mut self, t: usize) {
        // Get index of note in base frequency table.
        let note_char = self.tunes[t].data.peek() as char;
        let note_idx = self.tunes[t].data.index();

        // Get relative note index, and adjust times 3 for sharp/flat notes.
        let mut note: u8 = (self.tunes[t].data.get() - b'a') * 3;

        if self.tunes[t].data.peek() == b'-' {
            self.tunes[t].data.next();
            // Flat note.
            note += 1;
        } else if self.tunes[t].data.peek() == b'+' {
            self.tunes[t].data.next();
            // Sharp note.
            note += 2;
        }

        // Get duration, set delay and play note.
        let duration = self.parse_duration(t);
        self.tunes[t].next_note_time = self.get_timer() + duration;
        self.tunes[t].release_time =
            self.get_timer() + (duration * self.tunes[t].note_length / 100);

        set_key_on(&mut self.opl3, self.tunes[t].channel, false);
        let f = get_note_frequency(
            &mut self.opl3,
            self.tunes[t].channel,
            self.tunes[t].octave as u8,
            NOTE_DEFS[note as usize],
        );
        set_frequency(&mut self.opl3, self.tunes[t].channel, f);

        if self.debug {
            println!(
                "Index: {:05} Next: {:05} Char: {} Note: {:02}, channel: {} octave: {} freq: {:04} timer: {:05} duration: {} length: {} release_time: {}",
                note_idx,
                self.tunes[t].data.index(),
                note_char,
                note,
                self.tunes[t].channel,
                self.tunes[t].octave,
                f,
                self.get_timer(),
                duration,
                self.tunes[t].note_length,
                self.tunes[t].release_time
            );
        }

        set_key_on(&mut self.opl3, self.tunes[t].channel, true);
    }

    fn parse_duration(&mut self, t: usize) -> u32 {
        let mut duration: u32 = self.parse_number(t) as u32;
        if duration == 0 {
            duration = self.tunes[t].note_duration as u32;
        }

        // See whether we need to double the duration
        let base;
        if self.tunes[t].data.peek_next() == b'.' {
            self.tunes[t].data.next();
            base = 6;
        } else {
            base = 4;
        }

        // Calculate note duration in timer ticks (0.01s)
        let ticks = 6000u32 * base / duration / self.tempo;
        return ticks;
    }

    fn parse_number(&mut self, t: usize) -> u8 {
        let mut number = 0;
        if self.tunes[t].data.peek() != 0
            && self.tunes[t].data.peek() >= b'0'
            && self.tunes[t].data.peek() <= b'9'
        {
            // Data is number. Parse it...
            while self.tunes[t].data.peek() != 0
                && self.tunes[t].data.peek() >= b'0'
                && self.tunes[t].data.peek() <= b'9'
            {
                // Keep multiplying by 10 as long as we have additional digits.
                number = number * 10 + (self.tunes[t].data.get() - b'0');
            }
            // Last character wasn't a digit, so go back one step.
            self.tunes[t].data.prev();
        }
        return number;
    }
}
