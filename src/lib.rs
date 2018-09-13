#![feature(extern_prelude)]  // has to go in crate root. this is lib.rs for libraries
extern crate ramp;           //     and main.rs for programs.

pub mod math;
pub mod crypt;

