//! # Audio ripper module
//! A module that encapsulates all functions realated to
//! ripping an audio disc.

use crate::libcddb_wrapper;
use crate::libcdio_wrapper;
use std::process::*;

pub fn rip_cd() -> Result<Command, &'static str> {
    // get_devices_with_cap();
    return Err("TODO: not yet implemented!");
}
