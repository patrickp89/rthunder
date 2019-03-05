//! # libcddb Wrapper
//! A slim wrapper around (some) libcddb functions.

use std::ptr::*;
use std::ffi::{CString, c_void};
use std::os::raw::c_char;

#[allow(non_camel_case_types)]
type track_t = u8;

#[repr(C)]
struct cddb_track_t {
    bla: i32    // TODO!
}

#[repr(C)]
struct cddb_disc_t {
    revision: u32,
    discid: u32,
    category: cddb_cat_t,
    genre: *const c_char,
    title: *const c_char,
    artist: *const c_char,
    length: u32,
    year: u32,
    ext_data: *const c_char,
    track_cnt: i32,
    tracks: cddb_track_t,
    iterator: cddb_track_t
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum cddb_cat_t {
    CDDB_CAT_DATA,
    CDDB_CAT_FOLK,
    CDDB_CAT_JAZZ
    //...
}

extern "C" {
    // wraps: `cddb_disc_t *cddb_disc_new(void);`
    fn cddb_disc_new() -> cddb_disc_t;

    // wraps: `cddb_track_t *cddb_disc_get_track_first(cddb_disc_t *disc);`
    fn cddb_disc_get_track_first(disc: *mut cddb_disc_t) -> cddb_track_t;
}
