#![allow(bad_style)]
extern crate libc;

pub mod netmap;
pub mod netmap_user;
mod netmap_util;

#[cfg(feature = "netmap_with_libs")]
mod netmap_with_libs;

#[cfg(feature = "libnetmap")]
mod libnetmap;