//! # rthunder
//! A GTK+ audio ripper frontend.

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use rthunder::*;
// pub mod lib;

/// Main function. Will launch the user interface with
/// all its magic.
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize! Do you have GTK+ installed?");
        return;
    }
    run();
}
