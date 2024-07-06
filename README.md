opl3-rs
=======

A small library to provide bindings for the [Nuked-OPL3 library](https://github.com/nukeykt/Nuked-OPL3).

[![Crates.io version][crate-img]][crate]
[![Changelog][changelog-img]](CHANGELOG.md)
[![Documentation][docs-img]][docs]

# Usage

Nuked-OPL3 is not a turn-key implementation of the OPL3 chip - functions such as the status register, timers and
interrupts are left as implementation details.

You can access the Nuked-OPL3 API via the `Opl3Chip` struct, if needed, but with the caveat that directly writing
registers to Nuked-OPL3 will prevent you from reading the OPL registers correctly.

If you intend to utilize `opl3-rs` in an emulator, you will probably want to use the `Opl3Device` wrapper which provides
a full, device-oriented OPL3 implementation including the status, address and data registers, plus the OPL3 timers.

# Docs

Documentation can be found on [docs.rs](https://docs.rs/opl3-rs/latest/opl3_rs/)

# Examples

An example of music playback is provided in the play_tune directory under /examples.
This example uses the rodio library for audio playback and crossbeam channels for inter-thread communication.

opl3-rs was primarily built for use with the [MartyPC PC emulator](https://github.com/dbalsom/martypc). It is used to
implement an AdLib Music Card device.

# Credits

[Nuked-OPL3](https://github.com/nukeykt/Nuked-OPL3) is (C) 2013-2020 Nuke.YKT and licensed under LGPL 2.1

play_tune example based off code by Maarten Janssen and Peter De Wachter.

[crate]:         https://crates.io/crates/opl3-rs

[crate-img]:     https://img.shields.io/crates/v/opl3-rs.svg

[changelog-img]: https://img.shields.io/badge/changelog-online-blue.svg

[docs]:          https://docs.rs/opl3-rs

[docs-img]:      https://img.shields.io/badge/docs-online-blue.svg