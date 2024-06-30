# opl3-rs

`opl3-rs` is a small library to provide bindings for the [Nuked-OPL3 library](https://github.com/nukeykt/Nuked-OPL3).

# Usage

Nuked-OPL3 is not a turn-key implementation of the OPL3 chip - functions such as the status register, timers and
interrupts are left as implementation details. Eventually this library will aim to provide a wrapper type that will
provide example implementations for easier integrations with emulators.

For now, you can access the Nuked-OPL3 API via the `Opl3Chip` struct.

# Examples

An example of music playback is provided in the play_tune directory under /examples.
This example uses the rodio library for audio playback and crossbeam channels for inter-thread communication.

# Credits

[Nuked-OPL3](https://github.com/nukeykt/Nuked-OPL3) is (C) 2013-2020 Nuke.YKT and licensed under LGPL 2.1

play_tune example based off code by Maarten Janssen and Peter De Wachter.