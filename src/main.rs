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

    let query_db: disc_info_db::CdDatabaseQuerier = || disc_info_db::query_db();
    let rip_cd: ripper::Ripper = || ripper::rip_cd();

    // create the GUI:
    user_interface::create_ui(query_db, rip_cd).show_all();
}
