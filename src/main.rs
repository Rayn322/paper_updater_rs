mod paper_api;
use rfd::FileDialog;
use std::path::PathBuf;

fn main() {
    let file = FileDialog::new()
        .add_filter("Paper Jar (*.jar)", &["jar"])
        .pick_file();

    match file {
        Some(file) => {
            update_server(file);
        }
        None => println!("No file selected!"),
    }
}

fn update_server(file: PathBuf) -> Result<(), ()> {
    let url = match paper_api::fetch_latest_download() {
        Ok(url) => url,
        Err(_) => return Err(()),
    };

    Ok(())
}
