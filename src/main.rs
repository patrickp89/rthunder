//! # rthunder
//! A GTK+ audio ripper frontend.

extern crate libcdio_sys;

use crate::cd_helper::{CdCloser, CdOpener, CdPointer, CdPointerWithTrackCount};
use crate::disc_info_db::CdDatabaseQuerier;
use crate::ripper::Ripper;

pub mod cd_helper;
pub mod disc_info_db;
pub mod libcddb_wrapper;
pub mod libcdio_wrapper;
pub mod ripper;
pub mod user_interface;

/// Main function. Will launch the user interface with
/// all its magic.
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize! Do you have GTK+ installed?");
        return;
    }

    let open_disc: CdOpener = || cd_helper::open_disc();
    let query_db: CdDatabaseQuerier = |p| disc_info_db::query_db(p);
    let rip_cd: Ripper = || ripper::rip_cd();
    let close_disc: CdCloser = |p| cd_helper::destroy_disc_pointer(p);

    // query_db() is run when the user clicks the "refresh" button,
    // but we should try to get the track list (and the corresponding
    // tracks from the CDDB) initially, too:
    let (disc_pointer, track_count): (Option<CdPointer>, Option<u8>) = match cd_helper::open_disc()
    {
        Ok(p) => {
            let (a, b) = p;
            (Some(a), Some(b))
        }
        Err(e) => {
            println!("An error occurred: {:?}", e);
            (None, None)
        }
    };

    // create the GUI:
    user_interface::create_ui(
        disc_pointer,
        track_count,
        open_disc,
        query_db,
        rip_cd,
        close_disc,
    )
    .show_all();
}
