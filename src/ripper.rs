//! # Audio ripper module
//! A module that encapsulates all functions realated to
//! ripping an audio disc.

use std::process::*;
use crate::libcdio_wrapper;
use crate::libcddb_wrapper;

pub fn rip_cd() -> Result<Command, &'static str> {
    // get_devices_with_cap();
    return Err("TODO: not yet implemented!");
}
