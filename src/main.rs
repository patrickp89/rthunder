//! # rthunder
//! A GTK+ audio ripper frontend.

use crate::cd_helper::CdPointer;
use crate::user_interface::AlbumGuiWidgets;

use crate::disc_info_db::Disc;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

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

    // a mutable, global hashmap to store disc-related state:
    let discs: Rc<RefCell<HashMap<u32, Disc>>> = Rc::new(RefCell::new(HashMap::new()));
    // TODO: this feels like a ****** anti-pattern :|

    // create the GUI:
    let (album_grid, album_gui_widgets) = user_interface::create_album_entries();
    let tracklist_scrollwindow = user_interface::create_tracklist_entries(track_count);
    let (toolbar, cddb_lookup_button, preferences_button, about_button) =
        user_interface::create_toolbar();
    let window = user_interface::create_main_window();

    user_interface::glue_widgets_together(
        disc_pointer,
        discs,
        toolbar,
        album_grid,
        cddb_lookup_button,
        preferences_button,
        tracklist_scrollwindow,
        window,
        album_gui_widgets,
    );
}
