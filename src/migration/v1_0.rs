//! Migrating to `termimage` v1.0
//!
//! In `termimage` v0.5.1 and earlier, the only ANSI colour table available was based on a black-on-white terminal screenshot.
//!
//! This has proved to be a problem for standard white-on-black terminals, and so another one was added in v1.0.0;
//! however, the single-table approach was pervasive and required breakages:
//! 1. `termimage::util::ANSI_COLOURS` was renamed to [`termimage::util::ANSI_COLOURS_WHITE_BG`](../../util/static.ANSI_COLOURS_WHITE_BG.html), and
//!    [`termimage::util::ANSI_COLOURS_BLACK_BG`](../../util/static.ANSI_COLOURS_BLACK_BG.html) was added
//! 2. `termimage::util::ANSI_BG_COLOURS` was removed, and replaced with [`termimage::util::bg_colours_for()`](../../util/fn.bg_colours_for.html)
//! 3. [`termimage::ops::write_ansi()`](../../ops/fn.write_ansi.html) now takes a `foreground_colours` argument and derives the background therefrom;
//!    supply [`termimage::util::ANSI_COLOURS_WHITE_BG`](../../util/static.ANSI_COLOURS_WHITE_BG.html) for the old behaviour
//! 4. the encapsulated value in [`termimage::Options::ansi_out`](../../struct.Options.html#structfield.ansi_out) is now an enum,
//!    explicitly delineating which of truecolor/simple-on-black/simple-on-white to print, with the values updated to [variant][-background]
