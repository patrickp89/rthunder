//! # Audio ripper module
//! A module that encapsulates all functions realated to
//! ripping an audio disc.

use crate::libcddb_wrapper;
use crate::libcdio_wrapper;
use std::fs::File;

pub type Ripper = fn() -> Result<Vec<File>, &'static str>;

/// Rips all tracks on a given CD device.
pub fn rip_cd() -> Result<Vec<File>, &'static str> {
    // get_devices_with_cap();
    return Err("TODO: not yet implemented!");
}
