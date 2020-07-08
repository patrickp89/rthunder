//! # CD helper module
//! A helper module that deals with opening/closing
//! disc devices and so forth.

extern crate libcdio_sys;

use libcdio_sys::{
    cdio_destroy, cdio_get_num_tracks, cdio_open, driver_id_t_DRIVER_UNKNOWN, track_t, CdIo_t,
};
use std::ptr;

pub type CdPointer = *mut CdIo_t;
pub type CdPointerWithTrackCount = (CdPointer, track_t);

pub type CdOpener = fn() -> Result<CdPointerWithTrackCount, &'static str>;
pub type CdCloser = fn(p_cdio: CdPointer) -> Result<bool, &'static str>;

/// Accesses the CD device and returns: (the device pointer, the track count)
/// or an error message.
pub fn open_disc() -> Result<CdPointerWithTrackCount, &'static str> {
    println!("[ query_db()           ]  get the default default driver device...");
    let p_cdio: CdPointer = unsafe { cdio_open(ptr::null(), driver_id_t_DRIVER_UNKNOWN) };
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

    return Ok((p_cdio, track_count));
}

/// Attempts to gracefully destroy the disc pointer.
pub fn destroy_disc_pointer(p_cdio: CdPointer) -> Result<bool, &'static str> {
    return if p_cdio.is_null() {
        Err("Could not open device!")
    } else {
        unsafe { cdio_destroy(p_cdio) };
        Ok(true)
    };
}
