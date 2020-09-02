//! ## lib.rs
//! Encapsulates the logic to initialize the application.
//! Note: this was done to ensure integration tests actually
//! [do work](https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects).

use cd_helper::CdPointer;
use disc_info_db::Disc;
use user_interface::{AlbumGuiWidgets, RthunderUi};

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

pub mod cd_helper;
pub mod disc_info_db;
pub mod ripper;
pub mod user_interface;

pub fn run() {
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

    create_ui(None, disc_pointer).show_all();
}

pub fn create_ui(track_count: Option<u8>, disc_pointer: Option<CdPointer>) -> RthunderUi {
    // a mutable, global hashmap to store disc-related state:
    let rc_discs: Rc<RefCell<HashMap<u32, Disc>>> = Rc::new(RefCell::new(HashMap::new()));
    let rc_currently_chosen_disc: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(None));
    // TODO: this feels like a ****** anti-pattern :|

    // create the GUI:
    let (album_grid, album_gui_widgets) = user_interface::create_album_entries();

    let (tracklist_scrollwindow, tracklist_tree_view) = user_interface::create_tracklist_entries(
        rc_discs.clone(),
        rc_currently_chosen_disc.clone(),
        track_count,
    );

    let (toolbar, cddb_lookup_button, preferences_button, about_button) =
        user_interface::create_toolbar();

    let window = user_interface::create_main_window();

    user_interface::glue_widgets_together(
        disc_pointer,
        rc_discs,
        rc_currently_chosen_disc,
        toolbar,
        album_grid,
        cddb_lookup_button,
        preferences_button,
        tracklist_scrollwindow,
        window,
        album_gui_widgets,
        Rc::new(tracklist_tree_view),
    )
}
