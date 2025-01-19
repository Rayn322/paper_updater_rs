mod paper_api;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use paper_api::Download;
use rfd::FileDialog;
use std::{
    cmp::min,
    fs::{self},
    io::{self, Write},
    path::PathBuf,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = FileDialog::new()
        .add_filter("Paper Jar (*.jar)", &["jar"])
        .pick_file();

    match file {
        Some(file) => {
            update_server(file).await.expect("Failed to update server!");
        }
        None => println!("No file selected!"),
    }

    println!("Press enter to exit...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(())
}

async fn update_server(mut file: PathBuf) -> Result<(), ()> {
    let download = match paper_api::fetch_latest_download().await {
        Ok(download) => download,
        Err(_) => return Err(()),
    };

    let old_name = match file.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => name.to_owned(),
            None => return Err(()),
        },
        None => return Err(()),
    };
    let mut folder: PathBuf = file.clone().into();
    folder.pop();

    match fs::remove_file(&file) {
        Ok(_) => {
            file.pop();
            file.push(format!("paper-{}-{}.jar", download.version, download.build));

            match file.to_str() {
                Some(file) => match download_file(&download, file).await {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(());
                    }
                },
                None => {
                    return Err(());
                }
            }
        }
        Err(_) => {
            return Err(());
        }
    }

    match update_start_script(folder, download, old_name) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to update start script!");
        }
    }

    Ok(())
}

// adapted from https://gist.github.com/giuliano-macedo/4d11d6b3bb003dba3a1b53f43d81b30d
async fn download_file(download: &Download, path: &str) -> Result<(), String> {
    println!(
        "Downloading Paper version {}, build {} to {}",
        download.version, download.build, path
    );

    let res = reqwest::get(&download.url)
        .await
        .or(Err("Failed to download file"))?;
    let total_size = res.content_length().ok_or("Couldn't get content length")?;

    let bar = ProgressBar::new(total_size);
    let mut style = match ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})") {
        Ok(style) => style,
        Err(_) => return Err("Error while creating progress bar".into()),
    };

    style = style.progress_chars("#>-");
    bar.set_style(style);
    bar.set_message("Downloading file...");

    let mut file = fs::File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file"))?;
        file.write_all(&chunk)
            .or(Err("Error while writing to file"))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        bar.set_position(new);
    }

    bar.finish_with_message("Downloaded file successfully!");

    Ok(())
}

fn update_start_script(folder: PathBuf, download: Download, old_name: String) -> Result<(), ()> {
    let script_path: PathBuf = match get_script_path(folder.clone()) {
        Some(path) => path,
        None => {
            println!("No start script found!");
            return Err(());
        }
    };

    let content = match fs::read_to_string(&script_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Failed to read start script!");
            return Err(());
        }
    };

    let new_content = content.replace(
        old_name.as_str(),
        format!("paper-{}-{}.jar", download.version, download.build).as_str(),
    );

    match fs::write(script_path, new_content) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to write to start script!");
            return Err(());
        }
    }

    Ok(())
}

fn get_script_path(path: PathBuf) -> Option<PathBuf> {
    let names = vec!["start.bat", "start.sh", "start", "run.bat", "run.sh", "run"];

    for name in names {
        let mut path: PathBuf = path.clone();
        path.push(name);

        if path.exists() {
            return Some(path);
        }
    }

    None
}
