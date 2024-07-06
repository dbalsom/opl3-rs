# opl3-rs

`opl3-rs` is a small library to provide bindings for the [Nuked-OPL3 library](https://github.com/nukeykt/Nuked-OPL3).

# Usage

Nuked-OPL3 is not a turn-key implementation of the OPL3 chip - functions such as the status register, timers and
interrupts are left as implementation details.

You can access the Nuked-OPL3 API via the `Opl3Chip` struct, if needed, but with the caveat that directly writing
registers to Nuked-OPL3 will prevent you from reading the OPL registers correctly.

If you intend to utilize `opl3-rs` in an emulator, you will probably want to use the `Opl3Device` wrapper which provides
a full OPL3 implementation including the status registers and timers.

# Credits

[Nuked-OPL3](https://github.com/nukeykt/Nuked-OPL3) is (C) 2013-2020 Nuke.YKT and licensed under LGPL 2.1