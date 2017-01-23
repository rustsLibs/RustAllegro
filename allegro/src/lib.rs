// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro"]
#![crate_type = "lib"]

extern crate libc;
extern crate allegro_sys as ffi;
#[macro_use]
extern crate allegro_util;

pub use bitmap::external::*;
pub use bitmap_like::*;
pub use color::*;
pub use config::external::*;
pub use core::external::*;
pub use display::external::*;
pub use events::external::*;
pub use joystick::*;
pub use keycodes::*;
pub use run::*;
#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
pub use shader::*;
pub use timer::*;
pub use transformations::external::*;
pub use allegro_util::*;

pub mod bitmap;
pub mod bitmap_like;
pub mod color;
pub mod config;
pub mod core;
pub mod display;
pub mod events;
pub mod joystick;
pub mod keycodes;
pub mod run;
#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
pub mod shader;
pub mod timer;
pub mod transformations;
pub mod allegro_main;
