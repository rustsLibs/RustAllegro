// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_id="allegro_font#5.0.10.1"]

#![comment = "Allegro 5 font addon Rust bindings"]
#![license = "zlib"]
#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(thread_local)]

extern crate allegro = "allegro5#5.0.10.1";
extern crate libc;
extern crate sync;

pub use font::*;
pub use addon::*;

#[cfg(use_link_name)]
mod link_name
{
	#[link(name = "allegro_font")]
	extern "C" {}
}

pub mod ffi;
pub mod addon;
pub mod font;

#[macro_escape]
#[path = "../macros.rs"]
pub mod macros;