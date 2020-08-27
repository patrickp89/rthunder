extern crate gtk;

#[macro_use]
extern crate gtk_test;

extern crate rthunder;
use rthunder::cd_helper::CdPointer;
use rthunder::disc_info_db::Disc;
use rthunder::user_interface::{AlbumGuiWidgets, RthunderUi};
use rthunder::*;

use gtk::{
    Button, ButtonExt, ContainerExt, EntryBuffer, GtkWindowExt, Inhibit, Label, LabelExt,
    Orientation, WidgetExt, Window, WindowType,
};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    gtk::init().unwrap();

    let ui = create_ui(None, None);
    let album_gui_widgets = ui.album_gui_widgets.borrow();

    let album_artist_entrybuffer = &album_gui_widgets.album_artist_entrybuffer;
    assert_text!(album_artist_entrybuffer, "");

    let album_year_entrybuffer = &album_gui_widgets.album_year_entrybuffer;
    assert_text!(album_year_entrybuffer, "");

    let album_year_entrybuffer = &album_gui_widgets.album_genre_entrybuffer;
    assert_text!(album_year_entrybuffer, "");

    let album_year_entrybuffer = &album_gui_widgets.album_title_entrybuffer;
    assert_text!(album_year_entrybuffer, "");
}
