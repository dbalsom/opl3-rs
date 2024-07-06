// #opl3-rs
// A simple wrapper around the OPL3 chip library.
// Bindings generated by Daniel Balsom.
//
// Nuked OPL3 Copyright (C) 2013-2020 Nuke.YKT
#![warn(missing_docs)]
#![doc = include_str!("./docs.md")]

/*
* Nuked OPL3 is free software: you can redistribute it and/or modify
* it under the terms of the GNU Lesser General Public License as
* published by the Free Software Foundation, either version 2.1
* of the License, or (at your option) any later version.
*
* Nuked OPL3 is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Lesser General Public License for more details.
*
* You should have received a copy of the GNU Lesser General Public License
* along with Nuked OPL3. If not, see <https://www.gnu.org/licenses/>.

*  Nuked OPL3 emulator.
*  Thanks:
*      MAME Development Team(Jarek Burczynski, Tatsuyuki Satoh):
*          Feedback and Rhythm part calculation information.
*      forums.submarine.org.uk(carbon14, opl3):
*          Tremolo and phase generator calculation information.
*      OPLx decapsulated(Matthew Gambrell, Olli Niemitalo):
*          OPL2 ROMs.
*      siliconpr0n.org(John McMaster, digshadow):
*          YMF262 and VRC VII decaps and die shots.
*/

use thiserror::Error;

mod bindings;

unsafe impl Send for Opl3Chip {}

// OPL3 register addresses for registers not handled by Nuked-OPL3 directly.
const OPL_TIMER_1_REGISTER: u8 = 0x02;
const OPL_TIMER_2_REGISTER: u8 = 0x03;
const OPL_TIMER_CONTROL_REGISTER: u8 = 0x04;

const OPL_IRQ_FLAG: u8 = 0b1000_0000;
const OPL_TIMER_1_MASK: u8 = 0b0100_0000;
const OPL_TIMER_2_MASK: u8 = 0b0010_0000;
const OPL_TIMER_1_START: u8 = 0b0000_0001;
const OPL_TIMER_2_START: u8 = 0b0000_0010;

const OPL_TICK_RATE: f64 = 80.0; // Perform a timer tick every 80us.
const OPL_TIMER_1_RATE: u32 = 80; // Timer 1 tick rate is every 80us.
const OPL_TIMER_2_RATE: u32 = 320; // Timer 2 tick rate is every 320us.

#[derive(Error, Debug)]
/// The `OplError` enum represents errors that can occur when using the `opl3-rs` library.
pub enum OplError {
    #[error("Buffer slice provided was too small")]
    /// The buffer slice provided was too small to contain the generated samples.
    BufferUndersized,
    #[error("Buffer slices must be equal in length")]
    /// The buffer slices provided to generate_4ch_stream were not equal in length.
    BufferMismatch,
    #[error("Register number out of range")]
    /// The specified register number is out of range.
    RegisterOutOfRange,
    #[error("Failed to lock mutex")]
    /// Failed to lock the mutex for the OPL3 device.
    MutexLockFailed,
}

#[derive(Debug)]
/// The `Opl3RegisterFile` enum represents the two register files available on the OPL3 chip.
/// If in OPL2 mode, only the primary register file is available.
pub enum OplRegisterFile {
    /// Select the OPL2 register file.
    Primary,
    /// Select the extended OPL3 register file.
    Secondary,
}

/// The `Opl3DeviceStats` struct contains statistics about the OPL3 device.
/// It can be retrieved via the `get_stats` function on `Opl3Device`.
#[derive(Copy, Clone, Default)]
pub struct Opl3DeviceStats {
    /// The number of writes to the OPL3 data register since reset.
    pub data_writes: usize,
    /// The number of writes to the OPL3 address register since reset.
    pub addr_writes: usize,
    /// The number of reads from the OPL3 status register since reset.
    pub status_reads: usize,
    /// The number of samples generated since reset. A stereo pair (left and right) is considered
    /// one sample.
    pub samples_generated: usize,
}

/// The `Opl3Device` maintains two internal timers.
#[derive(Default, Debug)]
struct OplTimer {
    enabled: bool,
    masked: bool,
    rate: u32,
    preset: u8,
    counter: u8,
    usec_accumulator: f64,
    elapsed: bool,
}

impl OplTimer {
    fn new(rate: u32) -> Self {
        OplTimer {
            enabled: false,
            masked: false,
            rate,
            preset: 0,
            counter: 0,
            usec_accumulator: 0.0,
            elapsed: false,
        }
    }

    fn mask(&mut self, masked: bool) {
        self.masked = masked;
    }

    fn is_elapsed(&self) -> bool {
        if self.masked {
            false
        } else {
            self.elapsed
        }
    }

    fn enable(&mut self, state: bool) {
        self.enabled = state;
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.counter = self.preset;
        self.elapsed = false;
    }

    fn reset_elapsed(&mut self) {
        self.elapsed = false;
    }

    fn tick(&mut self, usec: f64) {
        self.usec_accumulator += usec;
        while self.usec_accumulator >= self.rate as f64 {
            self.usec_accumulator -= self.rate as f64;
            self.count();
        }
    }

    #[inline]
    fn count(&mut self) {
        if self.enabled {
            if self.counter == 255 {
                self.elapsed = true;
                self.counter = self.preset;
            } else {
                self.counter += 1;
            }
        }
    }
}

/// The `Opl3Device` struct provides convenience functions for fully implementing an OPL3 device on
/// top of Nuked-OPL3.
/// By keeping a copy of all registers written, we can implement a read_register function.
pub struct Opl3Device {
    addr_reg: [u8; 2],
    sample_rate: u32,
    registers: [[u8; 256]; 2],
    timers: [OplTimer; 2],
    stats: Opl3DeviceStats,
    inner_chip: Opl3Chip,
    samples_fpart: f64,
    usec_accumulator: f64,
}

impl Opl3Device {
    /// Create a new OPL3 device instance.
    /// `Opl3Device` is a convenience wrapper around the Nuked-OPL3's direct wrapper, `Opl3Chip`.
    /// It provides the rest of an OPL3 implementation on top of the chip, including register
    /// tracking and a read_register function.
    pub fn new(sample_rate: u32) -> Self {
        Opl3Device {
            addr_reg: [0, 0],
            sample_rate,
            registers: [[0; 256], [0; 256]],
            timers: [
                OplTimer::new(OPL_TIMER_1_RATE),
                OplTimer::new(OPL_TIMER_2_RATE),
            ],
            stats: Opl3DeviceStats::default(),
            inner_chip: Opl3Chip::new(sample_rate),
            samples_fpart: 0.0,
            usec_accumulator: 0.0,
        }
    }

    /// Retrieve the statistics for the OPL3 device in the form of an `Opl3DeviceStats` struct.
    ///
    /// # Returns
    /// An `Opl3DeviceStats` struct containing the statistics for the OPL3 device.
    pub fn stats(&self) -> Opl3DeviceStats {
        self.stats
    }

    /// Update the `Opl3Device` instance. This function should be called periodically to update the
    /// state of the OPL3 timers.
    /// # Arguments
    ///
    /// * `usec` - The number of microseconds that have passed since the last call to `run`.
    ///
    /// # Returns
    /// The number of samples that correspond to the specified microseconds that elapsed.
    /// The Opl3Device maintains a fractional accumulator, so you can use this returned value to
    /// determine how many samples to generate.
    pub fn run(&mut self, usec: f64) -> usize {
        self.usec_accumulator += usec;
        while self.usec_accumulator >= OPL_TICK_RATE {
            self.usec_accumulator -= OPL_TICK_RATE;
            self.timers[0].tick(OPL_TICK_RATE);
            self.timers[1].tick(OPL_TICK_RATE);
        }

        let samples_f = (usec / 1_000_000.0 * self.sample_rate as f64) + self.samples_fpart;

        let samples = samples_f as usize;
        self.samples_fpart = samples_f - samples_f.floor();

        samples
    }

    /// Read a byte from the OPL3 device's Status register.
    /// The Nuked-OPL3 library does not natively provide emulation of the OPL3 status register.
    /// The status register contains bits that indicate the status of the OPL3's timers. To properly
    /// emulate this timer state, it is necessary to call run() on the OPL3 device periodically.
    pub fn read_status(&mut self) -> u8 {
        self.stats.status_reads = self.stats.status_reads.saturating_add(1);

        let mut status_reg = 0;

        status_reg |= if self.timers[0].is_elapsed() {
            OPL_TIMER_1_MASK
        } else {
            0
        };

        status_reg |= if self.timers[1].is_elapsed() {
            OPL_TIMER_2_MASK
        } else {
            0
        };

        status_reg |= if self.timers[0].is_elapsed() || self.timers[1].is_elapsed() {
            OPL_IRQ_FLAG
        } else {
            0
        };

        status_reg
    }

    /// Write a byte to the OPL3 device's Address register.
    /// This function, along with write_data, is likely the primary interface for an emulator
    /// implementing an OPL device.
    ///
    /// # Arguments
    ///
    /// * `addr` - The register address to write to the OPL3 device, in the range 0..=255.
    /// * `file` - The register file to write to. OPL3 devices have two register files, the Primary
    ///            and Secondary files. OPL2 devices only have the Primary register file.
    pub fn write_address(&mut self, addr: u8, file: OplRegisterFile) -> Result<(), OplError> {
        match file {
            OplRegisterFile::Primary => self.addr_reg[0] = addr,
            OplRegisterFile::Secondary => self.addr_reg[1] = addr,
        }
        Ok(())
    }

    /// Write a byte to the OPL3 device's Data register.
    /// This function, along with write_address, is likely the primary interface function for an
    /// emulator implementing an OPL device.
    ///
    /// The actual internal register to be written should be set by writing to the OPL3 address
    /// register via `write_address` before calling `write_data`.
    ///
    /// # Arguments
    ///
    /// * `data`     - The byte of data to write to the OPL3 device.
    /// * `buffered` - Whether to write the data in buffered mode. In buffered mode, Nuked-OPL3
    ///                will store the write in a buffer and execute it after any necessary delay.
    ///                This is useful for controlling the library manually, but if you are
    ///                implementing an emulator the software controlling the OPL3 module will
    ///                likely write registers with appropriate timings.
    /// * `file` - The register file to write to. OPL3 devices have two register files, the Primary
    ///            and Secondary files. OPL2 devices only have the Primary register file.
    pub fn write_data(
        &mut self,
        data: u8,
        file: OplRegisterFile,
        buffered: bool,
    ) -> Result<(), OplError> {
        let addr = match file {
            OplRegisterFile::Primary => self.addr_reg[0],
            OplRegisterFile::Secondary => self.addr_reg[1],
        };
        self.write_register(addr, data, file, buffered);
        Ok(())
    }

    /// Return the value of the given chip register from internal state.
    /// The OPL3 registers are not natively readable. `Opl3Device` keeps a copy of all registers
    /// written so that they can be queried. This internal state will become desynchronized if
    /// registers are written directly to the OPL3 chip.
    ///
    /// # Arguments
    ///
    /// * `reg`  - The internal register index to read.
    /// * `file` - The register file to write to. OPL3 devices have two register files, the Primary
    ///            and Secondary files. OPL2 devices only have the Primary register file
    ///
    /// # Returns
    ///
    /// The u8 value of the requested register.
    pub fn read_register(&self, reg: u8, file: OplRegisterFile) -> u8 {
        match file {
            OplRegisterFile::Primary => self.registers[0][reg as usize],
            OplRegisterFile::Secondary => self.registers[1][reg as usize],
        }
    }

    /// Write to the specified register directly. This will update the internal state of the
    /// Opl3Device so that the register value can later be read.
    ///
    /// # Arguments
    ///
    /// * `reg` - The internal register index to write.
    /// * `value` - The value to write to the register.
    /// * `buffered` - Whether to write the data in buffered mode. In buffered mode, Nuked-OPL3
    ///                will store the write in a buffer and execute it after any necessary delay.
    ///                This is useful for controlling the library manually, but if you are
    ///                implementing an emulator the software controlling the OPL3 module will
    ///                likely write registers with appropriate timings.
    /// * `file` - The register file to write to. OPL3 devices have two register files, the Primary
    ///            and Secondary files. OPL2 devices only have the Primary register file
    pub fn write_register(&mut self, reg: u8, value: u8, file: OplRegisterFile, buffered: bool) {
        let reg16 = match file {
            OplRegisterFile::Primary => {
                self.registers[0][reg as usize] = value;
                reg as u16
            }
            OplRegisterFile::Secondary => {
                self.registers[1][reg as usize] = value;
                reg as u16 | 0x100
            }
        };

        // We need to intercept certain register addresses that Nuked-OPL3 doesn't emulate, namely
        // the timer registers.
        if let OplRegisterFile::Primary = file {
            match reg {
                OPL_TIMER_1_REGISTER => {
                    self.timers[0].counter = value;
                }
                OPL_TIMER_2_REGISTER => {
                    self.timers[1].counter = value;
                }
                OPL_TIMER_CONTROL_REGISTER => {
                    if (value & OPL_IRQ_FLAG) != 0 {
                        // Reset the timer and IRQ flags in the status register.
                        // All other bits are ignored when this bit is set.
                        self.timers[0].reset_elapsed();
                        self.timers[1].reset_elapsed();
                    } else {
                        // Mask & enable the timers based on the timer start bits.
                        self.timers[0].mask((value & OPL_TIMER_1_MASK) != 0);
                        self.timers[1].mask((value & OPL_TIMER_2_MASK) != 0);
                        self.timers[0].enable((value & OPL_TIMER_1_START) != 0);
                        self.timers[1].enable((value & OPL_TIMER_2_START) != 0);
                    }
                }
                _ => {}
            }
        }

        self.stats.data_writes = self.stats.data_writes.saturating_add(1);
        if buffered {
            self.inner_chip.write_register_buffered(reg16, value);
        } else {
            self.inner_chip.write_register(reg16, value);
        }
    }

    /// Reset the Opl3Device.
    /// Reset the state of the OPL3 device, including the internal registers and the internal
    /// Nuked-OPL3 instance.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - An option that either contains the new sample rate to reinitialize with
    ///                   or None to keep the current sample rate.
    ///
    /// # Returns
    ///
    /// A Result containing either `()` on success or an `OplError` on failure.
    pub fn reset(&mut self, sample_rate: Option<u32>) -> Result<(), OplError> {
        let new_sample_rate = sample_rate.unwrap_or(self.sample_rate);
        self.inner_chip.reset(new_sample_rate);
        for file in 0..2 {
            for reg in 0..256 {
                self.registers[file][reg] = 0;
            }
        }
        self.stats = Opl3DeviceStats::default();
        Ok(())
    }

    /// Generate a 2 channel audio sample in interleaved i16 format.
    ///
    /// # Arguments
    ///
    /// * `sample` - A mutable reference to a two-element slice that will receive the audio sample.
    ///              The first element will contain the left channel sample, and the second element
    ///              will contain the right channel sample.
    ///
    /// # Returns
    ///
    /// A Result containing either `()` on success or an `OplError` on failure.
    pub fn generate(&mut self, sample: &mut [i16]) -> Result<(), OplError> {
        self.inner_chip.generate(sample)
    }

    /// Generate a stream of 2 channel, interleaved audio samples in i16 format.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a buffer slice that will be filled with stereo, i
    ///              interleaved audio samples.
    ///
    /// # Returns
    ///
    /// A Result containing either `()` on success or an `OplError` on failure.
    pub fn generate_samples(&mut self, buffer: &mut [i16]) -> Result<(), OplError> {
        self.inner_chip.generate_stream(buffer)
    }
}

/// The `Opl3Chip` struct provides a safe interface for interacting with the Nuked-OPL3 library.
pub struct Opl3Chip {
    chip: *mut bindings::Opl3Chip,
}

impl Opl3Chip {
    /// Creates a new OPL3 chip instance. The chip is initialized with the given sample rate.
    /// The internal chip device is Pinned to ensure that it is not moved in memory. The Nuked-OPL3
    /// instance contains many self-referencing pointers, which would be invalidated if moved.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - The sample rate to initialize the OPL3 chip with.
    ///
    /// # Returns
    ///
    /// The new Opl3Chip instance.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// ```
    pub fn new(sample_rate: u32) -> Self {
        unsafe {
            let layout = std::alloc::Layout::new::<bindings::Opl3Chip>();
            let chip = std::alloc::alloc(layout) as *mut bindings::Opl3Chip;
            bindings::Opl3Reset(chip, sample_rate);
            Opl3Chip { chip }
        }
    }

    /// Reinitialize the OPL3 chip instance.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - The sample rate to initialize the OPL3 chip with.
    ///                   I have not tested the effects of reinitializing the chip with a different
    ///                   sample rate than the one initially used.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// chip.reset(44100);
    /// ```
    pub fn reset(&mut self, sample_rate: u32) {
        unsafe {
            bindings::Opl3Reset(&mut *self.chip, sample_rate);
        }
    }

    /// Generate an audio sample.
    ///
    /// Internally, this calls Opl3Generate4Ch and returns samples for the first 2 channels.
    ///
    /// # Arguments
    ///
    /// * `sample` - A mutable slice of 2 elements that will receive the sample.
    ///
    /// # Returns
    ///
    /// A Result containing either `()` on success or an `OplError` on failure.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 2];
    /// _ = chip.generate(&mut buffer);
    /// ```
    pub fn generate(&mut self, sample: &mut [i16]) -> Result<(), OplError> {
        if sample.len() < 2 {
            return Err(OplError::BufferUndersized);
        }
        unsafe {
            bindings::Opl3Generate(&mut *self.chip, sample.as_mut_ptr());
        }
        Ok(())
    }

    /// Generate a resampled audio sample.
    ///
    /// # Arguments
    ///
    /// * `sample` - A mutable slice of 2 elements that will receive the sample.
    ///
    /// # Returns
    ///
    /// A Result containing either `()` on success or an `OplError` on failure.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 2];
    /// _ = chip.generate_resampled(&mut buffer);
    /// ```
    pub fn generate_resampled(&mut self, sample: &mut [i16]) -> Result<(), OplError> {
        if sample.len() < 2 {
            return Err(OplError::BufferUndersized);
        }
        unsafe {
            bindings::Opl3GenerateResampled(&mut *self.chip, sample.as_mut_ptr());
        }
        Ok(())
    }

    /// Writes a value to an OPL register.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to write to.
    /// * `value` - The value to write to the register.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// chip.write_register(0x20, 0x01);
    /// ```
    pub fn write_register(&mut self, reg: u16, value: u8) {
        unsafe {
            bindings::Opl3WriteReg(&mut *self.chip, reg, value);
        }
    }

    /// Write a value to an OPL register, in buffered mode.
    ///
    /// The OPL3 normally requires a delay between register writes. This function
    /// will queue the write operation and execute it after any necessary delay.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to write to.
    /// * `value` - The value to write to the register.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// chip.write_register_buffered(0x20, 0x01);
    /// ```
    pub fn write_register_buffered(&mut self, reg: u16, value: u8) {
        unsafe {
            bindings::Opl3WriteRegBuffered(&mut *self.chip, reg, value);
        }
    }

    /// Generates a stream of resampled audio samples.
    ///
    /// The number of samples generated is determined by the size of the buffer provided.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a slice of i16 that will be filled with resampled audio
    ///              samples.
    ///
    /// # Returns
    ///
    /// A Result containing either `()` on success or an `OplError` on failure.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 1024 * 2];
    /// _ = chip.generate_stream(&mut buffer);
    /// ```
    pub fn generate_stream(&mut self, buffer: &mut [i16]) -> Result<(), OplError> {
        if buffer.len() < 2 {
            return Err(OplError::BufferUndersized);
        }
        unsafe {
            bindings::Opl3GenerateStream(
                &mut *self.chip,
                buffer.as_mut_ptr(),
                buffer.len() as u32 / 2,
            );
        }
        Ok(())
    }

    /// Generate a 4 channel audio sample.
    ///
    /// # Arguments
    ///
    /// * `sample` - A mutable, 4-element slice of i16 that will receive the sample.
    ///
    /// # Returns
    ///
    /// A Result containing either `()` on success or an `OplError` on failure.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 4];
    /// _ = chip.generate_4ch(&mut buffer);
    /// ```
    pub fn generate_4ch(&mut self, sample: &mut [i16]) -> Result<(), OplError> {
        if sample.len() < 4 {
            return Err(OplError::BufferUndersized);
        }
        unsafe {
            bindings::Opl3Generate4Ch(&mut *self.chip, sample.as_mut_ptr());
        }
        Ok(())
    }

    /// Generate a 4-channel resampled audio sample.
    ///
    /// # Arguments
    ///
    /// * `sample` - A mutable, 4-element slice of i16 that will receive the sample.
    ///
    /// # Returns
    ///
    /// A Result containing either `()` on success or an `OplError` on failure.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 4];
    /// _ = chip.generate_4ch_resampled(&mut buffer);
    /// ```
    pub fn generate_4ch_resampled(&mut self, sample: &mut [i16]) -> Result<(), OplError> {
        if sample.len() < 4 {
            return Err(OplError::BufferUndersized);
        }
        unsafe {
            bindings::Opl3Generate4ChResampled(&mut *self.chip, sample.as_mut_ptr());
        }
        Ok(())
    }

    /// Generates a stream of 4-channel audio samples, resampled to the configured sample rate.
    /// The OPL3 was capable of 4-channel output, although this feature was not widely used. Most
    /// cards simply didn't provide 4-channel outputs, although there now exist modern reproduction
    /// cards that do.
    ///
    /// The number of samples is determined by the size of the input buffers.
    ///
    /// # Arguments
    ///
    /// * `buffer1` - A mutable reference to a slice that will be filled with the first stereo
    ///               audio samples, interleaved between left and right channels.
    /// * `buffer2` - A mutable reference to a slice that will be filled with audio samples for the
    ///               channels 2 and 3.
    ///               The length of buffer1 should equal the length of buffer2.
    ///
    /// # Returns
    ///
    /// A Result containing either `()` on success or an `OplError` on failure.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer1 = [0i16; 1024 * 2];
    /// let mut buffer2 = [0i16; 1024 * 2];
    /// _ = chip.generate_4ch_stream(&mut buffer1, &mut buffer2);
    /// ```
    pub fn generate_4ch_stream(
        &mut self,
        buffer1: &mut [i16],
        buffer2: &mut [i16],
    ) -> Result<(), OplError> {
        if buffer1.len() != buffer2.len() {
            return Err(OplError::BufferMismatch);
        }
        if buffer1.len() < 4 || buffer2.len() < 4 {
            return Err(OplError::BufferUndersized);
        }
        unsafe {
            bindings::Opl3Generate4ChStream(
                &mut *self.chip,
                buffer1.as_mut_ptr(),
                buffer2.as_mut_ptr(),
                buffer1.len() as u32 / 2,
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
