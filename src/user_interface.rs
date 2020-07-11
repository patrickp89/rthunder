extern crate gtk;

use gtk::prelude::*;
use gtk::IconSize;
use gtk::{
    Box, Button, ButtonExt, CheckButton, Dialog, Entry, Grid, Image, Label, Orientation,
    PositionType, ScrolledWindow, ToolButton, Toolbar, TreeView, Window, WindowType,
};
use std::fs::File;

use crate::cd_helper::{CdCloser, CdOpener, CdPointer, CdPointerWithTrackCount};
use crate::disc_info_db::CdDatabaseQuerier;
use crate::ripper::Ripper;

const APPLICATION_NAME: &str = "rthunder";
const MAIN_WINDOW_DEFAULT_WIDTH: i32 = 800;
const MAIN_WINDOW_DEFAULT_HEIGHT: i32 = 600;

pub struct RthunderUi {
    window: gtk::Window,
    toolbar: gtk::Toolbar,
    album_grid: gtk::Grid,
    tracklist_scrollwindow: gtk::ScrolledWindow,
    rip_button: gtk::Button,
    options_dialog: gtk::Dialog,
}

impl RthunderUi {
    pub fn show_all(&self) {
        let vbox1 = Box::new(Orientation::Vertical, 0);
        vbox1.pack_start(&self.toolbar, true, true, 0);
        vbox1.pack_start(&self.album_grid, true, true, 0);
        vbox1.pack_start(&self.tracklist_scrollwindow, true, true, 0);
        vbox1.pack_start(&self.rip_button, false, false, 0); // TODO: should NOT expand!

        self.window.add(&vbox1);
        self.window.show_all();
        gtk::main();
    }
}

pub fn create_ui(
    disc_pointer: Option<CdPointer>,
    track_count: Option<u8>,
    open_disc: CdOpener,
    query_db: CdDatabaseQuerier,
    rip_cd: Ripper,
    close_disc: CdCloser,
) -> RthunderUi {
    // TODO: let tracklist_scrollwindow = create_track_entries_and_labels(track_list);

    // TODO: disc_pointer must be stateful and _overridden_,
    // TODO: a) because the initial disc-opening didn't work, or
    // TODO: b) because the user rips more than a single CD!

    return RthunderUi {
        window: create_main_window(close_disc, disc_pointer),
        toolbar: create_toolbar(disc_pointer, open_disc, query_db),
        album_grid: create_album_entries_and_labels(),
        tracklist_scrollwindow: create_track_entries_and_labels(track_count),
        rip_button: create_rip_button(rip_cd),
        options_dialog: create_options_dialog(),
    };
}

fn create_main_window(close_disc: CdCloser, disc_pointer: Option<CdPointer>) -> gtk::Window {
    let window = Window::new(WindowType::Toplevel);

    window.set_title(APPLICATION_NAME);
    window.set_default_size(MAIN_WINDOW_DEFAULT_WIDTH, MAIN_WINDOW_DEFAULT_HEIGHT);

    window.connect_delete_event(move |_, _| {
        match disc_pointer {
            Some(p) => {
                match close_disc(p) {
                    // TODO: nested pattern matching is ugly -> use Optio.err()!
                    Ok(_) => (),
                    Err(e) => println!("An error occurred: {:?}", e),
                }
            }
            None => println!("No valid disc pointer, I didn't call cdio_destroy()!"),
        }
        gtk::main_quit();
        Inhibit(false)
    });

    return window;
}

fn create_toolbar(
    disc_pointer: Option<CdPointer>,
    open_disc: CdOpener,
    query_db: CdDatabaseQuerier,
) -> gtk::Toolbar {
    let toolbar = Toolbar::new();

    let cddb_lookup_icon = Image::from_icon_name(Some("view-refresh"), IconSize::SmallToolbar);
    let cddb_lookup_button = ToolButton::new(Some(&cddb_lookup_icon), Some("CDDB Lookup"));
    cddb_lookup_button.connect_clicked(move |_| {
        match disc_pointer {
            Some(p) => {
                println!("Looking up disc on CDDB...");
                match query_db(p) {
                    Ok(track_list) => println!("all fine! the track list is: ..."), // TODO: then update track list view!
                    Err(e) => println!("An error occurred: {:?}", e),
                }
            }
            None => {
                println!("No opened disc device!");
                // TODO: try to open the default one!
            }
        };
    });
    toolbar.add(&cddb_lookup_button);

    let preferences_image =
        Image::from_icon_name(Some("preferences-system"), IconSize::SmallToolbar);
    let preferences_button = ToolButton::new(Some(&preferences_image), Some("Preferences"));
    preferences_button.connect_clicked(|_| {
        // TODO: show options_dialog!
    });
    toolbar.add(&preferences_button);

    let about_image = Image::from_icon_name(Some("help-about"), IconSize::SmallToolbar);
    let about_button = ToolButton::new(Some(&about_image), Some("Info"));
    toolbar.add(&about_button);

    return toolbar;
}

fn create_album_entries_and_labels() -> gtk::Grid {
    let grid = Grid::new();
    let default_grid_child_height = 1;
    let default_grid_label_width = 1;
    let default_grid_entry_width = 1;

    let album_artist_label = Label::new(Some("Album Artist:"));
    let album_artist_entry = Entry::new(); // TODO: create with a buffer assigned!
    grid.attach(
        &album_artist_label,
        0,
        0,
        default_grid_label_width,
        default_grid_child_height,
    );
    grid.attach_next_to(
        &album_artist_entry,
        Some(&album_artist_label),
        PositionType::Right,
        default_grid_entry_width,
        default_grid_child_height,
    );

    let single_artist_checkbutton = CheckButton::with_label("Single Artist");
    grid.attach_next_to(
        &single_artist_checkbutton,
        Some(&album_artist_entry),
        PositionType::Right,
        default_grid_label_width,
        default_grid_child_height,
    );

    let album_artist_label = Label::new(Some("Album Title:"));
    let album_artist_entry = Entry::new(); // TODO: create with a buffer assigned!
    grid.attach(
        &album_artist_label,
        0,
        1,
        default_grid_label_width,
        default_grid_child_height,
    );
    grid.attach_next_to(
        &album_artist_entry,
        Some(&album_artist_label),
        PositionType::Right,
        default_grid_entry_width,
        default_grid_child_height,
    );

    let album_artist_label = Label::new(Some("Genre / Year:"));
    let album_artist_entry = Entry::new(); // TODO: create with a buffer assigned!
    grid.attach(
        &album_artist_label,
        0,
        2,
        default_grid_label_width,
        default_grid_child_height,
    );
    grid.attach_next_to(
        &album_artist_entry,
        Some(&album_artist_label),
        PositionType::Right,
        default_grid_entry_width,
        default_grid_child_height,
    );

    // TODO: expand all entry widgets!

    return grid;
}

fn create_track_entries_and_labels(track_count: Option<u8>) -> gtk::ScrolledWindow {
    let tracklist_scrollwindow = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let tree_view = TreeView::new();

    // TODO: ...

    tracklist_scrollwindow.add(&tree_view);
    return tracklist_scrollwindow;
}

fn create_rip_button(rip_cd: Ripper) -> gtk::Button {
    let rip_button = Button::with_label("Rip");
    rip_button.connect_clicked(move |_| {
        println!("Let's rip! :)");
        match rip_cd() {
            Ok(ripped_files) => println!("all fine! the files are: ..."),
            Err(e) => println!("An error occurred: {:?}", e),
        }
    });

    return rip_button;
}

fn create_options_dialog() -> Dialog {
    let options_dialog = Dialog::new();
    // TODO: use with_buttons(...) instead!
    // TODO: display a dropdown box with all available disc devices
    // TODO: display other stuff, e.g. available mp3 encodings etc.
    return options_dialog;
}
