extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType,
            Toolbar, ToolButton,
            Box, Orientation,
            Image, Button,
            Grid, Entry, Label,
            CheckButton, PositionType,
            ScrolledWindow, TreeView};


const APPLICATION_NAME: &str = "rthunder";
const MAIN_WINDOW_DEFAULT_WIDTH: i32 = 800;
const MAIN_WINDOW_DEFAULT_HEIGHT: i32 = 600;


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize! Do you have GTK+ installed?");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title(APPLICATION_NAME);
    window.set_default_size(MAIN_WINDOW_DEFAULT_WIDTH, MAIN_WINDOW_DEFAULT_HEIGHT);

    let toolbar = create_toolbar();
    let album_grid = create_album_entries_and_labels();
    let tracklist_scrollwindow = create_track_entries_and_labels();
    let rip_button = Button::new_with_label("Rip");

    let vbox1 = Box::new(Orientation::Vertical, 0);
    vbox1.pack_start(&toolbar, true, true, 0);
    vbox1.pack_start(&album_grid, true, true, 0);
    vbox1.pack_start(&tracklist_scrollwindow, true, true, 0);
    vbox1.pack_start(&rip_button, false, false, 0); // TODO: should NOT expand!

    window.add(&vbox1);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}


fn create_toolbar() -> gtk::Toolbar {
    let toolbar = Toolbar::new();

    let cddb_lookup_icon = Image::new_from_icon_name("view-refresh", 22); // TODO: size???
    let cddb_lookup_button = ToolButton::new(&cddb_lookup_icon, "CDDB Lookup");
    toolbar.add(&cddb_lookup_button);

    let preferences_image = Image::new_from_icon_name("preferences-system", 22); // TODO: size???
    let preferences_button = ToolButton::new(&preferences_image, "Preferences");
    toolbar.add(&preferences_button);

    let about_image = Image::new_from_icon_name("help-about", 22); // TODO: size???
    let about_button = ToolButton::new(&about_image, "Info");
    toolbar.add(&about_button);

    return toolbar;
}


fn create_album_entries_and_labels() -> gtk::Grid {
    let grid = Grid::new();
    let default_grid_child_height = 1;
    let default_grid_label_width = 1;
    let default_grid_entry_width = 1;

    let album_artist_label = Label::new("Album Artist:");
    let album_artist_entry = Entry::new(); // TODO: create with a buffer assigned!
    grid.attach(&album_artist_label, 0, 0, default_grid_label_width, default_grid_child_height);
    grid.attach_next_to(&album_artist_entry, &album_artist_label, PositionType::Right,
        default_grid_entry_width, default_grid_child_height);

    let single_artist_checkbutton = CheckButton::new_with_label("Single Artist");
    grid.attach_next_to(&single_artist_checkbutton, &album_artist_entry, PositionType::Right,
        default_grid_label_width, default_grid_child_height);

    let album_artist_label = Label::new("Album Title:");
    let album_artist_entry = Entry::new(); // TODO: create with a buffer assigned!
    grid.attach(&album_artist_label, 0, 1, default_grid_label_width, default_grid_child_height);
    grid.attach_next_to(&album_artist_entry, &album_artist_label, PositionType::Right,
        default_grid_entry_width, default_grid_child_height);

    let album_artist_label = Label::new("Genre / Year:");
    let album_artist_entry = Entry::new(); // TODO: create with a buffer assigned!
    grid.attach(&album_artist_label, 0, 2, default_grid_label_width, default_grid_child_height);
    grid.attach_next_to(&album_artist_entry, &album_artist_label, PositionType::Right,
        default_grid_entry_width, default_grid_child_height);

    // TODO: expand all entry widgets!

    return grid;
}


fn create_track_entries_and_labels() -> gtk::ScrolledWindow {
    let tracklist_scrollwindow = ScrolledWindow::new(None, None);
    let tree_view = TreeView::new();

    // TODO: ...

    tracklist_scrollwindow.add(&tree_view);
    return tracklist_scrollwindow;
}
