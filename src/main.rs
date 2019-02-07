extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Toolbar, ToolButton, Box, Orientation, Image};

const APPLICATION_NAME: &str = "rthunder";
const MAIN_WINDOW_DEFAULT_WIDTH: i32 = 800;
const MAIN_WINDOW_DEFAULT_HEIGHT: i32 = 600;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize! Do you have GTK+ installed?");
        return;
    }
}
