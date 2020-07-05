//! # Disc info DB module
//! A module that encapsulates all functions related to
//! querying a disc info database.

use crate::libcddb_wrapper;
extern crate libcdio_sys;

use libcdio_sys::{cdio_get_num_tracks, cdio_open, driver_id_t_DRIVER_UNKNOWN, track_t, CdIo_t};
use std::ptr;

pub type CdDatabaseQuerier = fn() -> Result<Vec<&'static str>, &'static str>;

/// Accesses the CD device and queries a CD database for
/// track information.
pub fn query_db() -> Result<Vec<&'static str>, &'static str> {
    println!("[ query_db()           ]  get the default default driver device...");
    let p_cdio: *mut CdIo_t = unsafe { cdio_open(ptr::null(), driver_id_t_DRIVER_UNKNOWN) };
    if p_cdio.is_null() {
        return Err("Could not open device!");
    }
    println!(
        "[ query_db()           ]  default driver device is: {:?}",
        p_cdio
    );

    println!("[ query_db()           ]  get the number of tracks...");
    let track_count: track_t = unsafe { cdio_get_num_tracks(p_cdio) };
    println!(
        "[ query_db()           ]  there are {:?} tracks!",
        track_count
    );

    return Ok(Vec::new());
}
