//! # Disc info DB module
//! A module that encapsulates all functions realated to
//! querying a disc info database.

use std::ptr::*;
use std::ffi::{CString, c_void};
use std::os::raw::c_char;
use crate::libcdio_wrapper;
use crate::libcddb_wrapper;

/// Accesses the CD device and queries a CD database for
/// track information.
pub fn query_db() -> bool {
    println!("[ query_db()           ]  get the default CD device...");
    let default_device_result = libcdio_wrapper::get_default_device();

    // TODO: chain this Result and the following ones via map() or and_then() together!
    if default_device_result.is_err() {
        let e = default_device_result.unwrap();
        println!("An error occurred: {:?}", e);
        return false;
    }
    let default_device = default_device_result.unwrap();
    println!("[ query_db()           ]  default device is: {:?}", default_device);

    println!("[ query_db()           ]  get the track count...");
    //track_count: track_t = cdio_get_num_tracks(p_cdio);

    //println!("initialize disc struct...");
    //let disc = cddb_disc_new();

    println!("get the first track...");
    //let d: *mut cddb_disc_t = null_mut();
    //let ft = cddb_disc_get_track_first(di);

    // TODO: cddb_disc_get_track_first(disc)
    // TODO: foreach(cddb_disc_get_track_next(disc)) {
    //    cddb_track_get_length(...)
    // }

    return true;
}
