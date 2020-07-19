//! # rthunder
//! A GTK+ audio ripper frontend.

use crate::cd_helper::CdPointer;

pub mod cd_helper;
pub mod disc_info_db;
pub mod ripper;
pub mod user_interface;

/// Main function. Will launch the user interface with
/// all its magic.
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize! Do you have GTK+ installed?");
        return;
    }

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
    user_interface::create_ui(disc_pointer, track_count).show_all();
}
