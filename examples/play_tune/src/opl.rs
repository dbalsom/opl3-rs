//! OPL helper functions and definitions. These functions comprise tasks such as setting notes,
//! instruments, and frequency blocks.
//!
//! Original code (C) Maarten Janssen (maarten@cheerful.nl) 2016-04-13
//! https://github.com/DhrBaksteen/ArduinoOPL2
//! Hacked for a OPL2LPT test program Peter De Wachter (pdewacht@gmail.com).
//! https://github.com/pdewacht/adlipt/issues
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

#![allow(dead_code)]

use std::cmp;

use opl3_rs::Opl3Device;

pub const NOTE_C: u8 = 0;
pub const NOTE_CS: u8 = 1;
pub const NOTE_D: u8 = 2;
pub const NOTE_DS: u8 = 3;
pub const NOTE_E: u8 = 4;
pub const NOTE_F: u8 = 5;
pub const NOTE_FS: u8 = 6;
pub const NOTE_G: u8 = 7;
pub const NOTE_GS: u8 = 8;
pub const NOTE_A: u8 = 9;
pub const NOTE_AS: u8 = 10;
pub const NOTE_B: u8 = 11;

pub const OPL_INSTRUMENT_BASE_REGS: [u16; 6] = [0x20, 0x40, 0x60, 0x80, 0xE0, 0xC0];
pub const OPL_OFFSET: [[u16; 9]; 2] = [
    [0x00, 0x01, 0x02, 0x08, 0x09, 0x0A, 0x10, 0x11, 0x12], /*  initializers for operator 0 */
    [0x03, 0x04, 0x05, 0x0B, 0x0C, 0x0D, 0x13, 0x14, 0x15], /*  initializers for operator 1 */
];
pub const OPL_NOTE_FREQUENCIES: [u32; 12] = [
    26163, 27718, // C, C#
    29366, 31113, // D, D#
    32963, // E
    34923, 36999, // F, F#
    39200, 41530, // G, G#
    44000, 46616, // A, A#
    49388, // B
];
pub const OPL_F_INTERVALS: [u32; 8] = [48, 95, 190, 379, 759, 1517, 3034, 6069];

#[inline]
pub fn get_ch(channel: usize) -> usize {
    cmp::max(0x00, cmp::min(channel, 0x08))
}

pub fn get_register_offset(channel: usize, operator: usize) -> u16 {
    let opr = operator & 1;
    let ch = get_ch(channel);
    OPL_OFFSET[opr][ch]
}

pub fn set_instrument(opl3: &mut Opl3Device, channel: usize, instrument: &[u8; 12]) {
    set_waveform_select(opl3, true);
    for (i, byte) in instrument.iter().skip(1).enumerate() {
        opl3.write_register(
            OPL_INSTRUMENT_BASE_REGS[i % 6] + get_register_offset(channel, (i > 5) as usize),
            *byte,
            true,
        );
    }
}

pub fn set_waveform_select(opl3: &mut Opl3Device, enable: bool) {
    if enable {
        opl3.write_register(0x01, opl3.read_register(0x01) | 0x20, true);
    } else {
        opl3.write_register(0x01, opl3.read_register(0x01) & 0xDF, true);
    }
}

/// Return the frequency block of the given channel.
pub fn get_block(opl3: &mut Opl3Device, channel: u8) -> u8 {
    let offset = cmp::max(0x00, cmp::min(channel as u16, 0x08));
    return (opl3.read_register(0xB0 + offset) & 0x1C) >> 2;
}

/// Set frequency block for the given channel.
/// 0x00 is lowest, 0x07 is highest. This determines the frequency interval between notes.
/// 0 - 0.048 Hz, Range: 0.047 Hz ->   48.503 Hz
/// 1 - 0.095 Hz, Range: 0.094 Hz ->   97.006 Hz
/// 2 - 0.190 Hz, Range: 0.189 Hz ->  194.013 Hz
/// 3 - 0.379 Hz, Range: 0.379 Hz ->  388.026 Hz
/// 4 - 0.759 Hz, Range: 0.758 Hz ->  776.053 Hz
/// 5 - 1.517 Hz, Range: 1.517 Hz -> 1552.107 Hz
/// 6 - 3.034 Hz, Range: 3.034 Hz -> 3104.215 Hz
/// 7 - 6.069 Hz, Range: 6.068 Hz -> 6208.431 Hz
pub fn set_block(opl3: &mut Opl3Device, channel: u8, octave: u8) {
    let reg: u16 = 0xB0 + cmp::max(0x00, cmp::min(channel as u16, 0x08));
    opl3.write_register(
        reg,
        (opl3.read_register(reg) & 0xE3) | ((octave & 0x07) << 2),
        true,
    );
}

/// Return whether the voice for the given channel is currently on.
pub fn get_key_on(opl3: &mut Opl3Device, channel: u8) -> bool {
    let offset: u16 = cmp::max(0x00, cmp::min(channel as u16, 0x08));
    return (opl3.read_register(0xB0 + offset) & 0x20) != 0;
}

/// Enable the voice for the given channel.
pub fn set_key_on(opl3: &mut Opl3Device, channel: u8, key_on: bool) {
    let reg: u16 = 0xB0 + cmp::max(0x00, cmp::min(channel as u16, 0x08));
    let old_reg = opl3.read_register(reg);
    if key_on {
        opl3.write_register(reg, old_reg | 0x20, true);
    } else {
        opl3.write_register(reg, old_reg & 0xDF, true);
    }
}

pub fn get_note_frequency(opl3: &mut Opl3Device, channel: u8, octave: u8, note: u8) -> u16 {
    let octave = cmp::max(0, cmp::min(octave + (note / 12), 7));
    let interval = OPL_F_INTERVALS[get_block(opl3, channel) as usize];
    let mut freq = OPL_NOTE_FREQUENCIES[(note % 12) as usize] * 10;

    if octave < 4 {
        for _i in 0..(4 - octave) {
            freq /= 2;
        }
    } else if octave > 4 {
        for _i in 0..(octave - 4) {
            freq *= 2;
        }
    }

    let result = cmp::max(0, cmp::min(freq / interval, 1023));
    return result as u16;
}

/// Returns the F-number of the given channel.
pub fn get_frequency(opl3: &mut Opl3Device, channel: u8) -> u16 {
    let offset: u16 = cmp::max(0x00, cmp::min(channel as u16, 0x08));
    return (((opl3.read_register(0xB0 + offset) & 0x03) as u16) << 8)
        | opl3.read_register(0xA0 + offset) as u16;
}

/// Set the F-number of the given channel.
/// Returns the register number that was written to.
pub fn set_frequency(opl3: &mut Opl3Device, channel: u8, frequency: u16) -> u16 {
    let reg = 0xA0 + cmp::max(0x00, cmp::min(channel as u16, 0x08));
    opl3.write_register(reg, (frequency & 0xFF) as u8, true);
    opl3.write_register(
        reg + 0x10,
        (opl3.read_register(reg + 0x10) & 0xFC) | ((frequency & 0x0300) >> 8) as u8,
        true,
    );
    return reg;
}
