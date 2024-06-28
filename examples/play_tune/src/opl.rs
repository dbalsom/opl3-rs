
#[repr(u8)]
pub enum OplNotes {
    NoteC = 0,
    NoteCSharp = 1,
    NoteD = 2,
    NoteDSharp = 3,
    NoteE = 4,
    NoteF = 5,
    NoteFSharp = 6,
    NoteG = 7,
    NoteGSharp = 8,
    NoteA = 9,
    NoteASharp = 10,
    NoteB = 11,
}

pub const OPL_INSTRUMENT_PIANO: [u8; 12] =  [ 0x00, 0x33, 0x5A, 0xB2, 0x50, 0x01, 0x00, 0x31, 0x00, 0xB1, 0xF5, 0x01 ];