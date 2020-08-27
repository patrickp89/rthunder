//! # rthunder
//! A GTK+ audio ripper frontend.

use rthunder::*;

/// Main function. Will launch the user interface with
/// all its magic.
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize! Do you have GTK+ installed?");
        return;
    }
    run();
}
