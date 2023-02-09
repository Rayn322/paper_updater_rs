mod paper_api;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use rfd::FileDialog;
use std::{cmp::min, error::Error, fs::File, io::Write, path::PathBuf};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let file = FileDialog::new()
        .add_filter("Paper Jar (*.jar)", &["jar"])
        .pick_file();

    match file {
        Some(file) => {
            update_server(file).await.expect("Failed to update server!");
        }
        None => println!("No file selected!"),
    }

    Ok(())
}

async fn update_server(mut file: PathBuf) -> Result<(), ()> {
    let download = match paper_api::fetch_latest_download().await {
        Ok(download) => download,
        Err(_) => return Err(()),
    };

    match std::fs::remove_file(&file) {
        Ok(_) => {
            file.pop();
            file.push(format!("paper-{}-{}.jar", download.version, download.build));

            match file.to_str() {
                Some(file) => match download_file(&download.url, file).await {
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

    Ok(())
}

// adapted from https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
async fn download_file(url: &str, path: &str) -> Result<(), Box<dyn Error>> {
    println!("Downloading {} to {}", url, path);

    let res = reqwest::get(url).await?;
    let total_size = res.content_length().ok_or("Couldn't get content length")?;

    let bar = ProgressBar::new(total_size);
    let mut style = match ProgressStyle::default_bar()
    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})") {
        Ok(style) => style,
        Err(_) => return Err("Error while creating progress bar".into()),
    };

    style = style.progress_chars("#>-");
    bar.set_style(style);
    bar.set_message(format!("Downloading {}", url));

    let mut file = File::create(path)?;
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
