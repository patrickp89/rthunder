extern crate gtk;

use gtk::prelude::*;
use gtk::IconSize;
use gtk::{
    Box, Button, CheckButton, Entry, Grid, Image, Label, Orientation, PositionType, ScrolledWindow,
    ToolButton, Toolbar, TreeView, Window, WindowType,
};

const APPLICATION_NAME: &str = "rthunder";
const MAIN_WINDOW_DEFAULT_WIDTH: i32 = 800;
const MAIN_WINDOW_DEFAULT_HEIGHT: i32 = 600;

pub struct RthunderUi {
    window: gtk::Window,
    toolbar: gtk::Toolbar,
    album_grid: gtk::Grid,
    tracklist_scrollwindow: gtk::ScrolledWindow,
    rip_button: gtk::Button,
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

pub fn create_ui() -> RthunderUi {
    let window = create_main_window();
    let toolbar = create_toolbar();
    let album_grid = create_album_entries_and_labels();
    let tracklist_scrollwindow = create_track_entries_and_labels();
    let rip_button = create_rip_button();

    return RthunderUi {
        window: window,
        toolbar: toolbar,
        album_grid: album_grid,
        tracklist_scrollwindow: tracklist_scrollwindow,
        rip_button: rip_button,
    };
}

fn create_main_window() -> gtk::Window {
    let window = Window::new(WindowType::Toplevel);

    window.set_title(APPLICATION_NAME);
    window.set_default_size(MAIN_WINDOW_DEFAULT_WIDTH, MAIN_WINDOW_DEFAULT_HEIGHT);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    return window;
}

fn create_toolbar() -> gtk::Toolbar {
    let toolbar = Toolbar::new();

    let cddb_lookup_icon = Image::new_from_icon_name(Some("view-refresh"), IconSize::SmallToolbar);
    let cddb_lookup_button = ToolButton::new(Some(&cddb_lookup_icon), Some("CDDB Lookup"));
    toolbar.add(&cddb_lookup_button);

    let preferences_image =
        Image::new_from_icon_name(Some("preferences-system"), IconSize::SmallToolbar);
    let preferences_button = ToolButton::new(Some(&preferences_image), Some("Preferences"));
    toolbar.add(&preferences_button);

    let about_image = Image::new_from_icon_name(Some("help-about"), IconSize::SmallToolbar);
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

    let single_artist_checkbutton = CheckButton::new_with_label("Single Artist");
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

fn create_track_entries_and_labels() -> gtk::ScrolledWindow {
    let tracklist_scrollwindow = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let tree_view = TreeView::new();

    // TODO: ...

    tracklist_scrollwindow.add(&tree_view);
    return tracklist_scrollwindow;
}

fn create_rip_button() -> gtk::Button {
    return Button::new_with_label("Rip");
}
