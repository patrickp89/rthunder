mod user_interface;
mod disc_info_db;
mod ripper;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize! Do you have GTK+ installed?");
        return;
    }

    let ui = user_interface::create_ui();
    ui.show_all();
    disc_info_db::query_db();
    ripper::rip_cd();
}
