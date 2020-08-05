//! # Disc info DB module
//! A module that encapsulates all functions related to
//! querying a disc info database.

use crate::cd_helper::RawDiscInfo;
use libcddb_sys::*;
use std::ffi::{CStr, CString};

#[derive(Debug, Clone)]
pub struct Disc {
    pub disc_id: u32,
    pub artist: String,
    pub title: String,
    pub genre: String,
    pub year: u32,
    pub track_count: i32,
}

type CdDbConnection = *mut cddb_conn_t;
type CdDbDisc = *mut cddb_disc_t;

/// Queries a CD database for disc/track information. Returns all possible
/// matching discs or an error message.
pub fn query_db(disc_info: RawDiscInfo) -> Result<Vec<Disc>, &'static str> {
    // TODO: server URL and port should be configurable via the GUI!
    let server_name: &str = "gnudb.gnudb.org";
    let server_port: u16 = 8880 as u16;

    let conn: CdDbConnection = create_cddb_connection(server_name, server_port)?;
    let raw_disc = create_cddb_disc(disc_info.disc_length)?;
    add_offsets_to_cddb_disc(raw_disc, disc_info.disc_toc)?;

    println!("Looking up discs on CDDB...");
    let matching_disc_count: i32 = unsafe { cddb_query(conn, raw_disc) };
    if matching_disc_count == -1 {
        // TODO: get the error details from cddb_error_print(cddb_errno(conn)) !
        return Err("Could not look up discs on CDDB!");
    }

    println!("There are {} matching discs", matching_disc_count);
    let mut discs: Vec<Disc> = vec![];
    for _m in 0..matching_disc_count {
        let disc = Disc::from(raw_disc);
        println!("Possibly matching {:?} ", disc);
        discs.push(disc);
        if unsafe { cddb_query_next(conn, raw_disc) } == 1 {} // TODO: how to handle the error case?
    }

    unsafe { cddb_disc_destroy(raw_disc) };

    // remove duplicates, by disc ID:
    discs.sort_by(|a, b| a.disc_id.cmp(&b.disc_id));
    discs.dedup_by(|a, b| a.disc_id == b.disc_id);

    return Ok(discs);
}

/// Creates a libcddb network connection structure.
fn create_cddb_connection(
    server_name: &str,
    server_port: u16,
) -> Result<CdDbConnection, &'static str> {
    let conn = unsafe { cddb_new() };
    return if conn.is_null() {
        Err("Could not look up track on CDDB!")
    } else {
        let c_server_name = CString::new(server_name).expect("Could not create CString!");
        unsafe { cddb_set_server_name(conn, c_server_name.as_ptr()) };
        unsafe { cddb_set_server_port(conn, server_port as i32) };
        Ok(conn)
    };
}

/// Creates a libcddb disc structure.
fn create_cddb_disc(disc_length: u32) -> Result<CdDbDisc, &'static str> {
    let disc = unsafe { cddb_disc_new() };
    if disc.is_null() {
        return Err("Could not create disc struct!");
    } else {
        unsafe { cddb_disc_set_length(disc, disc_length) };
        Ok(disc)
    }
}

/// Creates libcddb track structures for all disc offsets.
fn add_offsets_to_cddb_disc(raw_disc: CdDbDisc, disc_toc: Vec<i32>) -> Result<(), &'static str> {
    for offset in disc_toc.iter() {
        let track = unsafe { cddb_track_new() };
        if track.is_null() {
            return Err("Could not create track struct!");
        }
        unsafe { cddb_track_set_frame_offset(track, *offset) };
        unsafe { cddb_disc_add_track(raw_disc, track) };
    }
    Ok(())
}

impl Disc {
    /// Creates a Disc from a raw libcddb disc structure.
    fn from(raw_disc: CdDbDisc) -> Disc {
        let raw_disc_title = unsafe { CStr::from_ptr(cddb_disc_get_title(raw_disc)) };
        let disc_title = raw_disc_title.to_str().expect("to_str() failed!");

        let raw_artist = unsafe { CStr::from_ptr(cddb_disc_get_artist(raw_disc)) };
        let disc_artist = raw_artist.to_str().expect("to_str() failed!");

        let raw_genre = unsafe { CStr::from_ptr(cddb_disc_get_genre(raw_disc)) };
        let disc_genre = raw_genre.to_str().expect("to_str() failed!");

        let year: u32 = unsafe { cddb_disc_get_year(raw_disc) };
        let track_count: i32 = unsafe { cddb_disc_get_track_count(raw_disc) };
        let disc_id: u32 = unsafe { cddb_disc_get_discid(raw_disc) };

        Disc {
            disc_id,
            title: String::from(disc_title),
            artist: String::from(disc_artist),
            genre: String::from(disc_genre),
            year,
            track_count,
        }
    }

    /// Returns a pretty-printed string with the most important disc info.
    pub fn to_pretty_string(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.artist, self.title, self.year, self.genre
        )
    }
}
