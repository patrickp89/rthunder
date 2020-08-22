use gtk::prelude::*;
use gtk::{
    Box, Button, ButtonExt, CheckButton, ComboBoxText, Dialog, Entry, EntryBuffer, Grid, IconSize,
    Image, Label, Orientation, PositionType, ScrolledWindow, ToolButton, Toolbar, TreeView, Window,
    WindowType,
};

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fs::File;
use std::ptr; // TODO: remove!
use std::rc::Rc;

use crate::cd_helper::{destroy_disc_pointer, read_disc_toc, CdPointer};
use crate::disc_info_db::{query_db, Disc};
use crate::ripper::rip_cd;

const APPLICATION_NAME: &str = "rthunder";
const MAIN_WINDOW_DEFAULT_WIDTH: i32 = 800;
const MAIN_WINDOW_DEFAULT_HEIGHT: i32 = 600;

pub struct RthunderUi {
    window: Window,
    toolbar: Toolbar,
    album_grid: Grid,
    tracklist_scrollwindow: ScrolledWindow,
    rip_button: Button,
    options_dialog: Dialog,
    pub album_gui_widgets: Rc<RefCell<AlbumGuiWidgets>>,
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

pub fn glue_widgets_together(
    disc_pointer: Option<CdPointer>,
    discs: Rc<RefCell<HashMap<u32, Disc>>>,
    toolbar: Toolbar,
    album_grid: Grid,
    cddb_lookup_button: ToolButton,
    preferences_button: ToolButton,
    tracklist_scrollwindow: ScrolledWindow,
    window: Window,
    album_gui_widgets: AlbumGuiWidgets,
) -> RthunderUi {
    // TODO: disc_pointer must be stateful and _overridden_,
    // TODO: a) because the initial disc-opening didn't work, or
    // TODO: b) because the user rips more than a single CD!

    let album_gui_widgets = Rc::new(RefCell::new(album_gui_widgets));

    let album_gui_widgets_clone = album_gui_widgets.clone();
    let discs_clone = discs.clone(); // TODO: redundant?!
    cddb_lookup_button.connect_clicked(move |_| {
        query_matching_discs(album_gui_widgets_clone.clone(), discs_clone.clone());
    });

    preferences_button.connect_clicked(|_| {
        // TODO: show the options_dialog!
    });

    let album_gui_widgets_clone2 = album_gui_widgets.clone();
    let discs_clone2 = discs.clone(); // TODO: redundant?!
    album_gui_widgets
        .borrow()
        .disc_choice_combobox
        .connect_changed(move |_| {
            choose_disc(album_gui_widgets_clone2.clone(), discs_clone2.clone())
        });

    window.connect_delete_event(move |_, _| {
        match disc_pointer {
            Some(p) => {
                match destroy_disc_pointer(p) {
                    // TODO: nested pattern matching is ugly -> use Option.err()!
                    Ok(_) => (),
                    Err(e) => eprintln!("An error occurred: {:?}", e),
                }
            }
            None => println!("No valid disc pointer, I didn't call cdio_destroy()!"),
        }
        gtk::main_quit();
        Inhibit(false)
    });

    return RthunderUi {
        window,
        toolbar,
        album_grid,
        tracklist_scrollwindow,
        rip_button: create_rip_button(),
        options_dialog: create_options_dialog(),
        album_gui_widgets: album_gui_widgets.clone()
    };
}

pub fn create_main_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title(APPLICATION_NAME);
    window.set_default_size(MAIN_WINDOW_DEFAULT_WIDTH, MAIN_WINDOW_DEFAULT_HEIGHT);
    return window;
}

pub fn create_toolbar() -> (Toolbar, ToolButton, ToolButton, ToolButton) {
    let toolbar = Toolbar::new();

    let cddb_lookup_icon = Image::from_icon_name(Some("view-refresh"), IconSize::SmallToolbar);
    let cddb_lookup_button = ToolButton::new(Some(&cddb_lookup_icon), Some("CDDB Lookup"));
    toolbar.add(&cddb_lookup_button);

    let preferences_image =
        Image::from_icon_name(Some("preferences-system"), IconSize::SmallToolbar);
    let preferences_button = ToolButton::new(Some(&preferences_image), Some("Preferences"));
    toolbar.add(&preferences_button);

    let about_image = Image::from_icon_name(Some("help-about"), IconSize::SmallToolbar);
    let about_button = ToolButton::new(Some(&about_image), Some("Info"));
    toolbar.add(&about_button);

    return (
        toolbar,
        cddb_lookup_button,
        preferences_button,
        about_button,
    );
}

fn query_matching_discs(
    // disc_pointer: Option<CdPointer>
    rc_album_gui_widgets: Rc<RefCell<AlbumGuiWidgets>>,
    // mut discs: RefMut<HashMap<u32, Disc>>,
    rc_discs: Rc<RefCell<HashMap<u32, Disc>>>,
) {
    let disc_pointer = Some(ptr::null_mut()); // TODO: remove!
    let album_gui_widgets: Ref<AlbumGuiWidgets> = rc_album_gui_widgets.borrow();
    let disc_choice_combobox = &album_gui_widgets.disc_choice_combobox;
    let mut discs: RefMut<HashMap<u32, Disc>> = rc_discs.borrow_mut();
    match disc_pointer {
        Some(p) => {
            let discs_result = read_disc_toc(p).and_then(|disc_info| query_db(disc_info));
            match discs_result {
                Ok(new_discs) => {
                    discs.clear();
                    disc_choice_combobox.remove_all();
                    for disc in new_discs {
                        // add the (disc_id, disc string) tuple to the choice combobox:
                        let disc_id_str: &str = &format!("{}", disc.disc_id);
                        let id = Some(disc_id_str);
                        let text = &disc.to_pretty_string();
                        disc_choice_combobox.insert(-1, id, text);

                        // add the disc to our global hashmap:
                        discs.insert(disc.disc_id, disc);
                    }
                }
                Err(e) => eprintln!("An error occurred: {:?}", e),
            }
        }
        None => {
            println!("No opened disc device!");
            // TODO: try to open the default one!
        }
    };
}

pub struct AlbumGuiWidgets {
    pub disc_choice_combobox: ComboBoxText,
    pub album_artist_entrybuffer: EntryBuffer,
    pub album_title_entrybuffer: EntryBuffer,
    pub album_genre_entrybuffer: EntryBuffer,
    pub album_year_entrybuffer: EntryBuffer,
}

pub fn create_album_entries() -> (Grid, AlbumGuiWidgets) {
    let grid = Grid::new();
    let default_grid_child_height = 1;
    let default_grid_label_width = 1;
    let default_grid_entry_width = 1;

    let disc_choice_combobox = ComboBoxText::new();
    let disc_choice_label = Label::new(Some("Album:"));
    grid.attach(
        &disc_choice_label,
        0,
        0,
        default_grid_label_width,
        default_grid_child_height,
    );
    grid.attach_next_to(
        &disc_choice_combobox,
        Some(&disc_choice_label),
        PositionType::Right,
        default_grid_entry_width,
        default_grid_child_height,
    );

    let album_artist_label = Label::new(Some("Artist:"));
    let album_artist_entrybuffer = EntryBuffer::new(Some(""));
    let album_artist_entry = Entry::with_buffer(&album_artist_entrybuffer);
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

    let single_artist_checkbutton = CheckButton::with_label("Single Artist");
    grid.attach_next_to(
        &single_artist_checkbutton,
        Some(&album_artist_entry),
        PositionType::Right,
        default_grid_label_width,
        default_grid_child_height,
    );

    let album_title_label = Label::new(Some("Title:"));
    let album_title_entrybuffer = EntryBuffer::new(Some(""));
    let album_title_entry = Entry::with_buffer(&album_title_entrybuffer);
    grid.attach(
        &album_title_label,
        0,
        2,
        default_grid_label_width,
        default_grid_child_height,
    );
    grid.attach_next_to(
        &album_title_entry,
        Some(&album_title_label),
        PositionType::Right,
        default_grid_entry_width,
        default_grid_child_height,
    );

    let album_genre_label = Label::new(Some("Genre:"));
    let album_genre_entrybuffer = EntryBuffer::new(Some(""));
    let album_genre_entry = Entry::with_buffer(&album_genre_entrybuffer);
    grid.attach(
        &album_genre_label,
        0,
        3,
        default_grid_label_width,
        default_grid_child_height,
    );
    grid.attach_next_to(
        &album_genre_entry,
        Some(&album_genre_label),
        PositionType::Right,
        default_grid_entry_width,
        default_grid_child_height,
    );

    let album_year_label = Label::new(Some("Year:"));
    let album_year_entrybuffer = EntryBuffer::new(Some(""));
    let album_year_entry = Entry::with_buffer(&album_year_entrybuffer);
    grid.attach(
        &album_year_label,
        0,
        4,
        default_grid_label_width,
        default_grid_child_height,
    );
    grid.attach_next_to(
        &album_year_entry,
        Some(&album_year_label),
        PositionType::Right,
        default_grid_entry_width,
        default_grid_child_height,
    );

    let album_gui_widgets = AlbumGuiWidgets {
        disc_choice_combobox,
        album_artist_entrybuffer,
        album_title_entrybuffer,
        album_genre_entrybuffer,
        album_year_entrybuffer,
    };

    return (grid, album_gui_widgets);
}

fn choose_disc(
    rc_album_gui_widgets: Rc<RefCell<AlbumGuiWidgets>>,
    rc_discs: Rc<RefCell<HashMap<u32, Disc>>>,
) {
    let discs: Ref<HashMap<u32, Disc>> = rc_discs.borrow();
    // TODO: this borrow ^^^ fails with a panic when __querying the disc__ a second time!
    // TODO: why? this borrow should not be a mutable one?!

    let album_gui_widgets: Ref<AlbumGuiWidgets> = rc_album_gui_widgets.borrow();
    let disc_choice_combobox = &album_gui_widgets.disc_choice_combobox;

    // TODO: the None match occurs not only on error but when querying
    // TODO: the CDDB more then once! -> remove it (i.e. the None match)!
    match disc_choice_combobox.get_active_text() {
        Some(active_text) => println!(" the selected disc is: {}", active_text),
        None => eprintln!("Couldn't unwrap active_text!"),
    }

    // TODO: the None match occurs not only on error but when querying
    // TODO: the CDDB more then once! -> remove it (i.e. the None match)!
    match disc_choice_combobox.get_active_id() {
        None => eprintln!("Couldn't unwrap active_id!"),
        Some(active_id) => {
            let id = active_id
                .as_str()
                .parse::<u32>()
                .expect("Could not parse disc ID!");
            println!(" the selected disc ID is: {}", id);
            let disc_option = discs.get(&id);
            match disc_option {
                Some(disc) => {
                    album_gui_widgets
                        .album_artist_entrybuffer
                        .set_text(disc.artist.as_str());
                    album_gui_widgets
                        .album_title_entrybuffer
                        .set_text(disc.title.as_str());
                    album_gui_widgets
                        .album_genre_entrybuffer
                        .set_text(disc.genre.as_str());
                    let year = format!("{}", disc.year);
                    album_gui_widgets
                        .album_year_entrybuffer
                        .set_text(year.as_str());
                    // TODO: change track items...
                }
                None => eprintln!("Couldn't find disc with ID {} in hash map!", id),
            }
        }
    }
}

pub fn create_tracklist_entries(track_count: Option<u8>) -> ScrolledWindow {
    let tracklist_scrollwindow = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let tree_view = TreeView::new();

    // TODO: ...

    tracklist_scrollwindow.add(&tree_view);
    return tracklist_scrollwindow;
}

fn create_rip_button() -> gtk::Button {
    let rip_button = Button::with_label("Rip");
    rip_button.connect_clicked(move |_| {
        println!("Let's rip! :)");
        match rip_cd() {
            Ok(ripped_files) => println!("all fine! the files are: ..."),
            Err(e) => eprintln!("An error occurred: {:?}", e),
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
