use rfd::FileDialog;

fn main() {
    let files = FileDialog::new()
        .add_filter("Jar", &["jar"])
        .pick_file()
        .expect("No file selected!");
    println!("{:?}", files);
}
