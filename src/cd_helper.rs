//! # CD helper module
//! A helper module that deals with opening/closing
//! disc devices and so forth.

use libcdio_sys::{
    cdio_destroy, cdio_get_devices, cdio_get_num_tracks, cdio_open, driver_id_t_DRIVER_UNKNOWN,
    track_t, CdIo_t,
};
use std::ptr;

pub struct RawDiscInfo {
    pub disc_length: u32,
    pub disc_toc: Vec<i32>,
}

pub type CdPointer = *mut CdIo_t;
pub type CdPointerWithTrackCount = (CdPointer, track_t);

pub type CdOpener = fn() -> Result<CdPointerWithTrackCount, &'static str>;
pub type CdCloser = fn(p_cdio: CdPointer) -> Result<bool, &'static str>;

/// Accesses the CD device and returns: (the device pointer, the track count)
/// or an error message.
pub fn open_disc() -> Result<CdPointerWithTrackCount, &'static str> {
    println!("[ query_db()           ]  get the default default driver device...");
    // TODO: use libcdio_sys::cdio_get_default_device() instead?
    let p_cdio: CdPointer = unsafe { cdio_open(ptr::null(), driver_id_t_DRIVER_UNKNOWN) };
    return if p_cdio.is_null() {
        Err("Could not open device!")
    } else {
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

        Ok((p_cdio, track_count))
    };
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

/// Looks up all known devices.
pub fn get_all_devices() -> Result<Vec<&'static str>, &'static str> {
    let devices = unsafe { cdio_get_devices(driver_id_t_DRIVER_UNKNOWN) };
    return if devices.is_null() {
        Err("Could not find any devices!")
    } else {
        // TODO: println!("[ get_all_devices() ]  device with capabilities is: {:?}", s);
        unimplemented!("not yet implemented!");
    };
}

pub fn read_disc_toc(p_cdio: CdPointer) -> Result<RawDiscInfo, &'static str> {
    println!("Reading disc TOC...");
    // TODO: implement this function!
    let disc_info = RawDiscInfo {
        disc_length: 3822,
        disc_toc: vec![
            150, 28690, 51102, 75910, 102682, 121522, 149040, 175772, 204387, 231145, 268065,
        ],
    };
    return Ok(disc_info);
}
