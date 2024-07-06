//! Example OPL instrument patches.
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
#![cfg_attr(rustfmt, rustfmt_skip)]
pub type OplInstrument = [u8; 12];
pub const OPL_INSTRUMENT_PIANO1: OplInstrument   = [ 0x00, 0x33, 0x5A, 0xB2, 0x50, 0x01, 0x00, 0x31, 0x00, 0xB1, 0xF5, 0x01 ];
pub const OPL_INSTRUMENT_PIANO2: OplInstrument   = [ 0x00, 0x31, 0x49, 0xF2, 0x53, 0x07, 0x01, 0x11, 0x03, 0xF1, 0xF5, 0x00 ];
pub const OPL_INSTRUMENT_PIANO3: OplInstrument   = [ 0x00, 0x31, 0x95, 0xD1, 0x83, 0x0D, 0x01, 0x32, 0x03, 0xC1, 0xF5, 0x00 ];
pub const OPL_INSTRUMENT_HONKTONK: OplInstrument = [ 0x00, 0x34, 0x9B, 0xF3, 0x63, 0x01, 0x01, 0x11, 0x00, 0x92, 0xF5, 0x01 ];
pub const OPL_INSTRUMENT_EP1: OplInstrument      = [ 0x00, 0x27, 0x28, 0xF8, 0xB7, 0x01, 0x02, 0x91, 0x00, 0xF1, 0xF9, 0x00 ];
pub const OPL_INSTRUMENT_EP2: OplInstrument      = [ 0x00, 0x1A, 0x2D, 0xF3, 0xEE, 0x01, 0x01, 0x11, 0x00, 0xF1, 0xF5, 0x00 ];
pub const OPL_INSTRUMENT_HARPSIC: OplInstrument  = [ 0x00, 0x35, 0x95, 0xF2, 0x58, 0x0F, 0x01, 0x32, 0x02, 0x81, 0xF6, 0x01 ];
pub const OPL_INSTRUMENT_CLAVIC: OplInstrument   = [ 0x00, 0x31, 0x85, 0xC9, 0x40, 0x01, 0x00, 0x35, 0x00, 0xC2, 0xB9, 0x01 ];
pub const OPL_INSTRUMENT_CELESTA: OplInstrument  = [ 0x00, 0x09, 0x15, 0xC7, 0x64, 0x08, 0x00, 0x01, 0x05, 0xB2, 0x35, 0x00 ];
pub const OPL_INSTRUMENT_GLOCK: OplInstrument    = [ 0x00, 0x06, 0x03, 0xF4, 0x44, 0x00, 0x01, 0x01, 0x1B, 0xF2, 0x34, 0x00 ];
pub const OPL_INSTRUMENT_MUSICBOX: OplInstrument = [ 0x00, 0x04, 0x06, 0xA9, 0x24, 0x0A, 0x01, 0x01, 0x01, 0xF5, 0x74, 0x00 ];
pub const OPL_INSTRUMENT_VIBES: OplInstrument    = [ 0x00, 0xD4, 0x00, 0xF6, 0x33, 0x00, 0x00, 0xF1, 0x00, 0x61, 0xE3, 0x00 ];
pub const OPL_INSTRUMENT_MARIMBA: OplInstrument  = [ 0x00, 0xD4, 0x00, 0xF7, 0xE8, 0x04, 0x00, 0xD1, 0x00, 0xA4, 0x64, 0x00 ];
pub const OPL_INSTRUMENT_XYLO: OplInstrument     = [ 0x00, 0x36, 0x16, 0xF7, 0xF7, 0x01, 0x00, 0x31, 0x07, 0xB5, 0xF5, 0x00 ];
pub const OPL_INSTRUMENT_TUBEBELL: OplInstrument = [ 0x00, 0x03, 0x1B, 0xA2, 0x43, 0x0B, 0x00, 0x00, 0x00, 0xF3, 0x74, 0x00 ];
pub const OPL_INSTRUMENT_SANTUR: OplInstrument   = [ 0x00, 0xC3, 0x8E, 0xF8, 0x35, 0x01, 0x01, 0x11, 0x00, 0xC3, 0x94, 0x01 ];
pub const OPL_INSTRUMENT_ORGAN1: OplInstrument   = [ 0x00, 0xE2, 0x07, 0xF4, 0x1B, 0x06, 0x01, 0xE0, 0x00, 0xF4, 0x0D, 0x01 ];
pub const OPL_INSTRUMENT_ORGAN2: OplInstrument   = [ 0x00, 0xF2, 0x00, 0xF6, 0x2C, 0x04, 0x00, 0xF0, 0x00, 0xF5, 0x0B, 0x01 ];
pub const OPL_INSTRUMENT_ORGAN3: OplInstrument   = [ 0x00, 0xF1, 0x06, 0xB6, 0x15, 0x0A, 0x00, 0xF0, 0x00, 0xBF, 0x07, 0x00 ];
pub const OPL_INSTRUMENT_PIPEORG: OplInstrument  = [ 0x00, 0x22, 0x03, 0x79, 0x16, 0x08, 0x01, 0xE0, 0x00, 0x6D, 0x08, 0x01 ];
pub const OPL_INSTRUMENT_REEDORG: OplInstrument  = [ 0x00, 0x31, 0x27, 0x63, 0x06, 0x01, 0x00, 0x72, 0x00, 0x51, 0x17, 0x01 ];
pub const OPL_INSTRUMENT_ACORDIAN: OplInstrument = [ 0x00, 0xB4, 0x1D, 0x53, 0x16, 0x0F, 0x01, 0x71, 0x00, 0x51, 0x17, 0x01 ];
pub const OPL_INSTRUMENT_HARMONIC: OplInstrument = [ 0x00, 0x25, 0x29, 0x97, 0x15, 0x01, 0x00, 0x32, 0x00, 0x53, 0x08, 0x01 ];
pub const OPL_INSTRUMENT_BANDNEON: OplInstrument = [ 0x00, 0x24, 0x9E, 0x67, 0x15, 0x0F, 0x00, 0x31, 0x00, 0x53, 0x06, 0x01 ];
pub const OPL_INSTRUMENT_NYLONGT: OplInstrument  = [ 0x00, 0x13, 0x27, 0xA3, 0xB4, 0x05, 0x01, 0x31, 0x00, 0xD2, 0xF8, 0x00 ];
pub const OPL_INSTRUMENT_STEELGT: OplInstrument  = [ 0x00, 0x17, 0xA3, 0xF3, 0x32, 0x01, 0x00, 0x11, 0x00, 0xE2, 0xC7, 0x01 ];
pub const OPL_INSTRUMENT_JAZZGT: OplInstrument   = [ 0x00, 0x33, 0x24, 0xD2, 0xC1, 0x0F, 0x01, 0x31, 0x00, 0xF1, 0x9C, 0x00 ];
pub const OPL_INSTRUMENT_CLEANGT: OplInstrument  = [ 0x00, 0x31, 0x05, 0xF8, 0x44, 0x01, 0x00, 0x32, 0x02, 0xF2, 0xC9, 0x01 ];
pub const OPL_INSTRUMENT_MUTEGT: OplInstrument   = [ 0x00, 0x21, 0x09, 0x9C, 0x7B, 0x07, 0x00, 0x02, 0x03, 0x95, 0xFB, 0x00 ];
pub const OPL_INSTRUMENT_OVERDGT: OplInstrument  = [ 0x00, 0x21, 0x84, 0x81, 0x98, 0x07, 0x01, 0x21, 0x04, 0xA1, 0x59, 0x00 ];
pub const OPL_INSTRUMENT_DISTGT: OplInstrument   = [ 0x00, 0xB1, 0x0C, 0x78, 0x43, 0x01, 0x00, 0x22, 0x03, 0x91, 0xFC, 0x03 ];
pub const OPL_INSTRUMENT_GTHARMS: OplInstrument  = [ 0x00, 0x00, 0x0A, 0x82, 0x8C, 0x09, 0x00, 0x08, 0x02, 0xB4, 0xEC, 0x00 ];
pub const OPL_INSTRUMENT_ACOUBASS: OplInstrument = [ 0x00, 0x21, 0x13, 0xAB, 0x46, 0x01, 0x00, 0x21, 0x00, 0x93, 0xF7, 0x00 ];
pub const OPL_INSTRUMENT_FINGBASS: OplInstrument = [ 0x00, 0x01, 0x0A, 0xF9, 0x32, 0x01, 0x00, 0x22, 0x04, 0xC1, 0x58, 0x00 ];
pub const OPL_INSTRUMENT_PICKBASS: OplInstrument = [ 0x00, 0x21, 0x07, 0xFA, 0x77, 0x0B, 0x00, 0x22, 0x02, 0xC3, 0x6A, 0x00 ];
pub const OPL_INSTRUMENT_FRETLESS: OplInstrument = [ 0x00, 0x21, 0x17, 0x71, 0x57, 0x0B, 0x00, 0x21, 0x00, 0x62, 0x87, 0x00 ];
pub const OPL_INSTRUMENT_SLAPBAS1: OplInstrument = [ 0x00, 0x25, 0x01, 0xFA, 0x78, 0x07, 0x01, 0x12, 0x00, 0xF3, 0x97, 0x00 ];
pub const OPL_INSTRUMENT_SLAPBAS2: OplInstrument = [ 0x00, 0x21, 0x03, 0xFA, 0x88, 0x0D, 0x00, 0x13, 0x00, 0xB3, 0x97, 0x00 ];
pub const OPL_INSTRUMENT_SYNBASS1: OplInstrument = [ 0x00, 0x21, 0x09, 0xF5, 0x7F, 0x09, 0x01, 0x23, 0x04, 0xF3, 0xCC, 0x00 ];
pub const OPL_INSTRUMENT_SYNBASS2: OplInstrument = [ 0x00, 0x01, 0x10, 0xA3, 0x9B, 0x09, 0x00, 0x01, 0x00, 0x93, 0xAA, 0x00 ];
pub const OPL_INSTRUMENT_VIOLIN: OplInstrument   = [ 0x00, 0xE2, 0x19, 0xF6, 0x29, 0x0D, 0x01, 0xE1, 0x00, 0x78, 0x08, 0x01 ];
pub const OPL_INSTRUMENT_VIOLA: OplInstrument    = [ 0x00, 0xE2, 0x1C, 0xF6, 0x29, 0x0D, 0x01, 0xE1, 0x00, 0x78, 0x08, 0x01 ];
pub const OPL_INSTRUMENT_CELLO: OplInstrument    = [ 0x00, 0x61, 0x19, 0x69, 0x16, 0x0B, 0x01, 0x61, 0x00, 0x54, 0x27, 0x01 ];
pub const OPL_INSTRUMENT_CONTRAB: OplInstrument  = [ 0x00, 0x71, 0x18, 0x82, 0x31, 0x0D, 0x01, 0x32, 0x00, 0x61, 0x56, 0x00 ];
pub const OPL_INSTRUMENT_TREMSTR: OplInstrument  = [ 0x00, 0xE2, 0x23, 0x70, 0x06, 0x0D, 0x01, 0xE1, 0x00, 0x75, 0x16, 0x01 ];
pub const OPL_INSTRUMENT_PIZZ: OplInstrument     = [ 0x00, 0x02, 0x00, 0x88, 0xE6, 0x08, 0x00, 0x61, 0x00, 0xF5, 0xF6, 0x01 ];
pub const OPL_INSTRUMENT_HARP: OplInstrument     = [ 0x00, 0x12, 0x20, 0xF6, 0xD5, 0x0F, 0x01, 0x11, 0x80, 0xF3, 0xE3, 0x00 ];
pub const OPL_INSTRUMENT_TIMPANI: OplInstrument  = [ 0x00, 0x61, 0x0E, 0xF4, 0xF4, 0x01, 0x01, 0x00, 0x00, 0xB5, 0xF5, 0x00 ];
pub const OPL_INSTRUMENT_STRINGS: OplInstrument  = [ 0x00, 0x61, 0x1E, 0x9C, 0x04, 0x0F, 0x01, 0x21, 0x80, 0x71, 0x16, 0x00 ];
pub const OPL_INSTRUMENT_SLOWSTR: OplInstrument  = [ 0x00, 0xA2, 0x2A, 0xC0, 0xD6, 0x0F, 0x02, 0x21, 0x00, 0x30, 0x55, 0x01 ];
pub const OPL_INSTRUMENT_SYNSTR1: OplInstrument  = [ 0x00, 0x61, 0x21, 0x72, 0x35, 0x0F, 0x01, 0x61, 0x00, 0x62, 0x36, 0x01 ];
pub const OPL_INSTRUMENT_SYNSTR2: OplInstrument  = [ 0x00, 0x21, 0x1A, 0x72, 0x23, 0x0F, 0x01, 0x21, 0x02, 0x51, 0x07, 0x00 ];
pub const OPL_INSTRUMENT_CHOIR: OplInstrument    = [ 0x00, 0xE1, 0x16, 0x97, 0x31, 0x09, 0x00, 0x61, 0x00, 0x62, 0x39, 0x00 ];
pub const OPL_INSTRUMENT_OOHS: OplInstrument     = [ 0x00, 0x22, 0xC3, 0x79, 0x45, 0x01, 0x00, 0x21, 0x00, 0x66, 0x27, 0x00 ];
pub const OPL_INSTRUMENT_SYNVOX: OplInstrument   = [ 0x00, 0x21, 0xDE, 0x63, 0x55, 0x01, 0x01, 0x21, 0x00, 0x73, 0x46, 0x00 ];
pub const OPL_INSTRUMENT_ORCHIT: OplInstrument   = [ 0x00, 0x42, 0x05, 0x86, 0xF7, 0x0A, 0x00, 0x50, 0x00, 0x74, 0x76, 0x01 ];
pub const OPL_INSTRUMENT_TRUMPET: OplInstrument  = [ 0x00, 0x31, 0x1C, 0x61, 0x02, 0x0F, 0x00, 0x61, 0x81, 0x92, 0x38, 0x00 ];
pub const OPL_INSTRUMENT_TROMBONE: OplInstrument = [ 0x00, 0x71, 0x1E, 0x52, 0x23, 0x0F, 0x00, 0x61, 0x02, 0x71, 0x19, 0x00 ];
pub const OPL_INSTRUMENT_TUBA: OplInstrument     = [ 0x00, 0x21, 0x1A, 0x76, 0x16, 0x0F, 0x00, 0x21, 0x01, 0x81, 0x09, 0x00 ];
pub const OPL_INSTRUMENT_MUTETRP: OplInstrument  = [ 0x00, 0x25, 0x28, 0x89, 0x2C, 0x07, 0x02, 0x20, 0x00, 0x83, 0x4B, 0x02 ];
pub const OPL_INSTRUMENT_FRHORN: OplInstrument   = [ 0x00, 0x21, 0x1F, 0x79, 0x16, 0x09, 0x00, 0xA2, 0x05, 0x71, 0x59, 0x00 ];
pub const OPL_INSTRUMENT_BRASS1: OplInstrument   = [ 0x00, 0x21, 0x19, 0x87, 0x16, 0x0F, 0x00, 0x21, 0x03, 0x82, 0x39, 0x00 ];
pub const OPL_INSTRUMENT_SYNBRAS1: OplInstrument = [ 0x00, 0x21, 0x17, 0x75, 0x35, 0x0F, 0x00, 0x22, 0x82, 0x84, 0x17, 0x00 ];
pub const OPL_INSTRUMENT_SYNBRAS2: OplInstrument = [ 0x00, 0x21, 0x22, 0x62, 0x58, 0x0F, 0x00, 0x21, 0x02, 0x72, 0x16, 0x00 ];
pub const OPL_INSTRUMENT_SOPSAX: OplInstrument   = [ 0x00, 0xB1, 0x1B, 0x59, 0x07, 0x01, 0x01, 0xA1, 0x00, 0x7B, 0x0A, 0x00 ];
pub const OPL_INSTRUMENT_ALTOSAX: OplInstrument  = [ 0x00, 0x21, 0x16, 0x9F, 0x04, 0x0B, 0x00, 0x21, 0x00, 0x85, 0x0C, 0x01 ];
pub const OPL_INSTRUMENT_TENSAX: OplInstrument   = [ 0x00, 0x21, 0x0F, 0xA8, 0x20, 0x0D, 0x00, 0x23, 0x00, 0x7B, 0x0A, 0x01 ];
pub const OPL_INSTRUMENT_BARISAX: OplInstrument  = [ 0x00, 0x21, 0x0F, 0x88, 0x04, 0x09, 0x00, 0x26, 0x00, 0x79, 0x18, 0x01 ];
pub const OPL_INSTRUMENT_OBOE: OplInstrument     = [ 0x00, 0x31, 0x18, 0x8F, 0x05, 0x01, 0x00, 0x32, 0x01, 0x73, 0x08, 0x00 ];
pub const OPL_INSTRUMENT_ENGLHORN: OplInstrument = [ 0x00, 0xA1, 0x0A, 0x8C, 0x37, 0x01, 0x01, 0x24, 0x04, 0x77, 0x0A, 0x00 ];
pub const OPL_INSTRUMENT_BASSOON: OplInstrument  = [ 0x00, 0x31, 0x04, 0xA8, 0x67, 0x0B, 0x00, 0x75, 0x00, 0x51, 0x19, 0x00 ];
pub const OPL_INSTRUMENT_CLARINET: OplInstrument = [ 0x00, 0xA2, 0x1F, 0x77, 0x26, 0x01, 0x01, 0x21, 0x01, 0x74, 0x09, 0x00 ];
pub const OPL_INSTRUMENT_PICCOLO: OplInstrument  = [ 0x00, 0xE1, 0x07, 0xB8, 0x94, 0x01, 0x01, 0x21, 0x01, 0x63, 0x28, 0x00 ];
pub const OPL_INSTRUMENT_FLUTE1: OplInstrument   = [ 0x00, 0xA1, 0x93, 0x87, 0x59, 0x01, 0x00, 0xE1, 0x00, 0x65, 0x0A, 0x00 ];
pub const OPL_INSTRUMENT_RECORDER: OplInstrument = [ 0x00, 0x22, 0x10, 0x9F, 0x38, 0x01, 0x00, 0x61, 0x00, 0x67, 0x29, 0x00 ];
pub const OPL_INSTRUMENT_PANFLUTE: OplInstrument = [ 0x00, 0xE2, 0x0D, 0x88, 0x9A, 0x01, 0x01, 0x21, 0x00, 0x67, 0x09, 0x00 ];
pub const OPL_INSTRUMENT_BOTTLEB: OplInstrument  = [ 0x00, 0xA2, 0x10, 0x98, 0x94, 0x0F, 0x00, 0x21, 0x01, 0x6A, 0x28, 0x00 ];
pub const OPL_INSTRUMENT_SHAKU: OplInstrument    = [ 0x00, 0xF1, 0x1C, 0x86, 0x26, 0x0F, 0x00, 0xF1, 0x00, 0x55, 0x27, 0x00 ];
pub const OPL_INSTRUMENT_WHISTLE: OplInstrument  = [ 0x00, 0xE1, 0x3F, 0x9F, 0x09, 0x00, 0x00, 0xE1, 0x00, 0x6F, 0x08, 0x00 ];
pub const OPL_INSTRUMENT_OCARINA: OplInstrument  = [ 0x00, 0xE2, 0x3B, 0xF7, 0x19, 0x01, 0x00, 0x21, 0x00, 0x7A, 0x07, 0x00 ];
pub const OPL_INSTRUMENT_SQUARWAV: OplInstrument = [ 0x00, 0x22, 0x1E, 0x92, 0x0C, 0x0F, 0x00, 0x61, 0x06, 0xA2, 0x0D, 0x00 ];
pub const OPL_INSTRUMENT_SAWWAV: OplInstrument   = [ 0x00, 0x21, 0x15, 0xF4, 0x22, 0x0F, 0x01, 0x21, 0x00, 0xA3, 0x5F, 0x00 ];
pub const OPL_INSTRUMENT_SYNCALLI: OplInstrument = [ 0x00, 0xF2, 0x20, 0x47, 0x66, 0x03, 0x01, 0xF1, 0x00, 0x42, 0x27, 0x00 ];
pub const OPL_INSTRUMENT_CHIFLEAD: OplInstrument = [ 0x00, 0x61, 0x19, 0x88, 0x28, 0x0F, 0x00, 0x61, 0x05, 0xB2, 0x49, 0x00 ];
pub const OPL_INSTRUMENT_CHARANG: OplInstrument  = [ 0x00, 0x21, 0x16, 0x82, 0x1B, 0x01, 0x00, 0x23, 0x00, 0xB2, 0x79, 0x01 ];
pub const OPL_INSTRUMENT_SOLOVOX: OplInstrument  = [ 0x00, 0x21, 0x00, 0xCA, 0x93, 0x01, 0x00, 0x22, 0x00, 0x7A, 0x1A, 0x00 ];
pub const OPL_INSTRUMENT_FIFTHSAW: OplInstrument = [ 0x00, 0x23, 0x00, 0x92, 0xC9, 0x08, 0x01, 0x22, 0x00, 0x82, 0x28, 0x01 ];
pub const OPL_INSTRUMENT_BASSLEAD: OplInstrument = [ 0x00, 0x21, 0x1D, 0xF3, 0x7B, 0x0F, 0x00, 0x22, 0x02, 0xC3, 0x5F, 0x00 ];
pub const OPL_INSTRUMENT_FANTASIA: OplInstrument = [ 0x00, 0xE1, 0x00, 0x81, 0x25, 0x00, 0x01, 0xA6, 0x86, 0xC4, 0x95, 0x01 ];
pub const OPL_INSTRUMENT_WARMPAD: OplInstrument  = [ 0x00, 0x21, 0x27, 0x31, 0x01, 0x0F, 0x00, 0x21, 0x00, 0x44, 0x15, 0x00 ];
pub const OPL_INSTRUMENT_POLYSYN: OplInstrument  = [ 0x00, 0x60, 0x14, 0x83, 0x35, 0x0D, 0x02, 0x61, 0x00, 0xD1, 0x06, 0x00 ];
pub const OPL_INSTRUMENT_SPACEVOX: OplInstrument = [ 0x00, 0xE1, 0x5C, 0xD3, 0x01, 0x01, 0x01, 0x62, 0x00, 0x82, 0x37, 0x00 ];
pub const OPL_INSTRUMENT_BOWEDGLS: OplInstrument = [ 0x00, 0x28, 0x38, 0x34, 0x86, 0x01, 0x02, 0x21, 0x00, 0x41, 0x35, 0x00 ];
pub const OPL_INSTRUMENT_METALPAD: OplInstrument = [ 0x00, 0x24, 0x12, 0x52, 0xF3, 0x05, 0x01, 0x23, 0x02, 0x32, 0xF5, 0x01 ];
pub const OPL_INSTRUMENT_HALOPAD: OplInstrument  = [ 0x00, 0x61, 0x1D, 0x62, 0xA6, 0x0B, 0x00, 0xA1, 0x00, 0x61, 0x26, 0x00 ];
pub const OPL_INSTRUMENT_SWEEPPAD: OplInstrument = [ 0x00, 0x22, 0x0F, 0x22, 0xD5, 0x0B, 0x01, 0x21, 0x84, 0x3F, 0x05, 0x01 ];
pub const OPL_INSTRUMENT_ICERAIN: OplInstrument  = [ 0x00, 0xE3, 0x1F, 0xF9, 0x24, 0x01, 0x00, 0x31, 0x01, 0xD1, 0xF6, 0x00 ];
pub const OPL_INSTRUMENT_SOUNDTRK: OplInstrument = [ 0x00, 0x63, 0x00, 0x41, 0x55, 0x06, 0x01, 0xA2, 0x00, 0x41, 0x05, 0x01 ];
pub const OPL_INSTRUMENT_CRYSTAL: OplInstrument  = [ 0x00, 0xC7, 0x25, 0xA7, 0x65, 0x01, 0x01, 0xC1, 0x05, 0xF3, 0xE4, 0x00 ];
pub const OPL_INSTRUMENT_ATMOSPH: OplInstrument  = [ 0x00, 0xE3, 0x19, 0xF7, 0xB7, 0x01, 0x01, 0x61, 0x00, 0x92, 0xF5, 0x01 ];
pub const OPL_INSTRUMENT_BRIGHT: OplInstrument   = [ 0x00, 0x66, 0x9B, 0xA8, 0x44, 0x0F, 0x00, 0x41, 0x04, 0xF2, 0xE4, 0x01 ];
pub const OPL_INSTRUMENT_GOBLIN: OplInstrument   = [ 0x00, 0x61, 0x20, 0x22, 0x75, 0x0D, 0x00, 0x61, 0x00, 0x45, 0x25, 0x00 ];
pub const OPL_INSTRUMENT_ECHODROP: OplInstrument = [ 0x00, 0xE1, 0x21, 0xF6, 0x84, 0x0F, 0x00, 0xE1, 0x01, 0xA3, 0x36, 0x00 ];
pub const OPL_INSTRUMENT_STARTHEM: OplInstrument = [ 0x00, 0xE2, 0x14, 0x73, 0x64, 0x0B, 0x01, 0xE1, 0x01, 0x98, 0x05, 0x01 ];
pub const OPL_INSTRUMENT_SITAR: OplInstrument    = [ 0x00, 0x21, 0x0B, 0x72, 0x34, 0x09, 0x00, 0x24, 0x02, 0xA3, 0xF6, 0x01 ];
pub const OPL_INSTRUMENT_BANJO: OplInstrument    = [ 0x00, 0x21, 0x16, 0xF4, 0x53, 0x0D, 0x00, 0x04, 0x00, 0xF6, 0xF8, 0x00 ];
pub const OPL_INSTRUMENT_SHAMISEN: OplInstrument = [ 0x00, 0x21, 0x18, 0xDA, 0x02, 0x0D, 0x00, 0x35, 0x00, 0xF3, 0xF5, 0x00 ];
pub const OPL_INSTRUMENT_KOTO: OplInstrument     = [ 0x00, 0x25, 0x0F, 0xFA, 0x63, 0x09, 0x00, 0x02, 0x00, 0x94, 0xE5, 0x01 ];
pub const OPL_INSTRUMENT_KALIMBA: OplInstrument  = [ 0x00, 0x32, 0x07, 0xF9, 0x96, 0x01, 0x00, 0x11, 0x00, 0x84, 0x44, 0x00 ];
pub const OPL_INSTRUMENT_BAGPIPE: OplInstrument  = [ 0x00, 0x20, 0x0E, 0x97, 0x18, 0x09, 0x02, 0x25, 0x03, 0x83, 0x18, 0x01 ];
pub const OPL_INSTRUMENT_FIDDLE: OplInstrument   = [ 0x00, 0x61, 0x18, 0xF6, 0x29, 0x01, 0x00, 0x62, 0x01, 0x78, 0x08, 0x01 ];
pub const OPL_INSTRUMENT_SHANNAI: OplInstrument  = [ 0x00, 0xE6, 0x21, 0x76, 0x19, 0x0B, 0x00, 0x61, 0x03, 0x8E, 0x08, 0x01 ];
pub const OPL_INSTRUMENT_TINKLBEL: OplInstrument = [ 0x00, 0x27, 0x23, 0xF0, 0xD4, 0x01, 0x00, 0x05, 0x09, 0xF2, 0x46, 0x00 ];
pub const OPL_INSTRUMENT_AGOGO: OplInstrument    = [ 0x00, 0x1C, 0x0C, 0xF9, 0x31, 0x0F, 0x01, 0x15, 0x00, 0x96, 0xE8, 0x01 ];
pub const OPL_INSTRUMENT_STEELDRM: OplInstrument = [ 0x00, 0x02, 0x00, 0x75, 0x16, 0x06, 0x02, 0x01, 0x00, 0xF6, 0xF6, 0x01 ];
pub const OPL_INSTRUMENT_WOODBLOK: OplInstrument = [ 0x00, 0x25, 0x1B, 0xFA, 0xF2, 0x01, 0x00, 0x12, 0x00, 0xF6, 0x9A, 0x00 ];
pub const OPL_INSTRUMENT_TAIKO: OplInstrument    = [ 0x00, 0x02, 0x1D, 0xF5, 0x93, 0x01, 0x00, 0x00, 0x00, 0xC6, 0x45, 0x00 ];
pub const OPL_INSTRUMENT_MELOTOM: OplInstrument  = [ 0x00, 0x11, 0x15, 0xF5, 0x32, 0x05, 0x00, 0x10, 0x00, 0xF4, 0xB4, 0x00 ];
pub const OPL_INSTRUMENT_SYNDRUM: OplInstrument  = [ 0x00, 0x22, 0x06, 0xFA, 0x99, 0x09, 0x00, 0x01, 0x00, 0xD5, 0x25, 0x00 ];
pub const OPL_INSTRUMENT_REVRSCYM: OplInstrument = [ 0x00, 0x2E, 0x00, 0xFF, 0x00, 0x0F, 0x02, 0x0E, 0x0E, 0x21, 0x2D, 0x00 ];
pub const OPL_INSTRUMENT_FRETNOIS: OplInstrument = [ 0x00, 0x30, 0x0B, 0x56, 0xE4, 0x01, 0x01, 0x17, 0x00, 0x55, 0x87, 0x02 ];
pub const OPL_INSTRUMENT_BRTHNOIS: OplInstrument = [ 0x00, 0x24, 0x00, 0xFF, 0x03, 0x0D, 0x00, 0x05, 0x08, 0x98, 0x87, 0x01 ];
pub const OPL_INSTRUMENT_SEASHORE: OplInstrument = [ 0x00, 0x0E, 0x00, 0xF0, 0x00, 0x0F, 0x02, 0x0A, 0x04, 0x17, 0x04, 0x03 ];
pub const OPL_INSTRUMENT_BIRDS: OplInstrument    = [ 0x00, 0x20, 0x08, 0xF6, 0xF7, 0x01, 0x00, 0x0E, 0x05, 0x77, 0xF9, 0x02 ];
pub const OPL_INSTRUMENT_TELEPHON: OplInstrument = [ 0x00, 0x20, 0x14, 0xF1, 0x08, 0x01, 0x00, 0x2E, 0x02, 0xF4, 0x08, 0x00 ];
pub const OPL_INSTRUMENT_HELICOPT: OplInstrument = [ 0x00, 0x20, 0x04, 0xF2, 0x00, 0x03, 0x01, 0x23, 0x00, 0x36, 0x05, 0x01 ];
pub const OPL_INSTRUMENT_APPLAUSE: OplInstrument = [ 0x00, 0x2E, 0x00, 0xFF, 0x02, 0x0F, 0x00, 0x2A, 0x05, 0x32, 0x55, 0x03 ];