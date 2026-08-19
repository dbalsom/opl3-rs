v0.2.4 - 2026/08/19
----------

Deprecated crate in favor of [nuked-opl3](https://crates.io/crates/nuked-opl3)

v0.2.3
------
Released 2026/04/28

* Fixed Opl3Device timer register writes to update timer reload presets. (Thanks TheAllTinker)

v0.2.2
------
Released 2025/03/01

* Fixed symbol names when building on macos (Thanks einstein95)

v0.2.1
------

* Yanked due to build issues

v0.2.0
------
Released 2024/07/06

* Added stats() function to Opl3Device for retrieving statistics.
* Added an option to the example `play_tune` to save the output as a wav via `hound`.
* Added a `buffered` parameter to `write_register`. This is technically a breaking change,
  but nobody's using this yet, so...

v0.1.2
------
Released 2024/06/30

* Fixed build and provided a working example of music playback.

v0.1.0, v0.1.1
--------------
Released 2024/06/29, Yanked 2024/06/30

* Broken. Less said the better. Was still figuring out how to publish an FFI crate.
