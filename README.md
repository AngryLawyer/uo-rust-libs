UO Rust Libs [![Crates.io](https://img.shields.io/crates/v/uo-rust-libs.svg)](https://crates.io/crates/uo-rust-libs) ![License](https://img.shields.io/crates/l/uo-rust-libs.svg) [![Documentation](https://docs.rs/uo-rust-libs/badge.svg)](https://docs.rs/uo-rust-libs/)
============

This library contains various modules for reading and writing pre-UOP Ultima Online data files, written in Rust.

This library is still pre-1.0 (although is stabilising), so the API may shift underfoot while you're working with it.

By default, the library contains helper methods for converting to [Image](https://crates.io/crates/image) types. Import it with `default-features = false` if you don't need this for your use-case.

This has been tested on a fresh install of Ultima Online: Age of Shadows, but should support clients up to Mondain's Legacy.

Supported files
---------------

There are currently readers for the following filetypes:

* anim.mul/anim.idx (and successive files) - Animated characters
* art.mul/art.idx - Tiles and static art
* fonts.mul - Fonts
* gumpart.mul/gumpidx.mul - GUI elements
* hues.mul - Palette swap colours
* map[n].mul - World maps
* mapdif[n].mul/mapdifl[n].mul - Patches for world maps
* radarcol.mul - Color lookup table for map and static tiles
* skills.mul/skills.idx - Skill names
* stadif[n].mul/stadifl[n].mul/stadifi[n].mul - Patches for static locations
* statics[n].mul - Static locations
* texmaps.mul/texidx.mul - 3D texture maps
* tiledata.mul - Information about tiles and statics

Features yet to be added
------------------------

* sound.mul/soundidx.mul support
* animdata.mul/animinfo.mul
* palette.mul
* skillgrp.mul
* unifont.mul
* speech.mul
* multi.mul/multi.idx
* Converting from images to UO assets/Writing muls back to files
* A built-in viewer application

References
----------

https://uo.stratics.com/heptazane/fileformats.shtml
