//! # Disc info DB module
//! A module that encapsulates all functions related to
//! querying a disc info database.

use crate::libcddb_wrapper;
extern crate libcdio_sys;

use libcdio_sys::{
    cdio_destroy, cdio_get_num_tracks, cdio_open, driver_id_t_DRIVER_UNKNOWN, track_t, CdIo_t,
};
use std::ptr;

use crate::cd_helper::{CdOpener, CdPointer, CdPointerWithTrackCount};

pub type CdDatabaseQuerier = fn(CdPointer) -> Result<Vec<&'static str>, &'static str>;

/// Queries a CD database for track information. Returns the track names
/// or an error message.
pub fn query_db(disc_pointer: CdPointer) -> Result<Vec<&'static str>, &'static str> {
    return Err("TODO: not yet implemented!");
}
