//! # Audio ripper module
//! A module that encapsulates all functions realated to
//! ripping an audio disc.

use std::fs::File;

pub type Ripper = fn() -> Result<Vec<File>, &'static str>;

/// Rips all tracks on a given CD device.
pub fn rip_cd() -> Result<Vec<File>, &'static str> {
    return Err("TODO: not yet implemented!");
}
