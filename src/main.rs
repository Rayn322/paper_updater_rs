mod paper_api;
use rfd::FileDialog;
use std::path::PathBuf;

fn main() {
    let files = FileDialog::new()
        .add_filter("Paper Jar (*.jar)", &["jar"])
        .pick_file();

    match files {
        Some(file) => {
            update_server(file);
        }
        None => println!("No file selected!"),
    }
}

fn update_server(file: PathBuf) {
    match paper_api::fetch_latest() {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Error!"),
    }
}
