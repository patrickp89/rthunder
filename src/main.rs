//! # rthunder
//! A GTK+ audio ripper frontend.

extern crate libcdio_sys;

pub mod disc_info_db;
pub mod libcddb_wrapper;
pub mod libcdio_wrapper;
pub mod ripper;
pub mod user_interface;

/// Main function. Will launch the GTK+ user interface with
/// all its functionality.
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize! Do you have GTK+ installed?");
        return;
    }

    // create the GUI:
    let ui = user_interface::create_ui();
    ui.show_all();

    disc_info_db::query_db();
    let r = ripper::rip_cd();
    match r {
        Ok(_v) => println!("all fine!"),
        Err(e) => println!("An error occurred: {:?}", e),
    }
}
