use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct VersionList {
    versions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct BuildList {
    builds: Vec<i16>,
}

pub fn fetch_latest_download() -> Result<String, ()> {
    let version_list = match get_version_list() {
        Ok(version_list) => version_list,
        Err(_) => return Err(()),
    };

    let latest_version = match version_list.versions.get(version_list.versions.len() - 1) {
        Some(version) => version,
        None => return Err(()),
    };

    let build_list = match get_build_list(latest_version) {
        Ok(build_list) => build_list,
        Err(_) => return Err(()),
    };

    let latest_build = match build_list.builds.get(build_list.builds.len() - 1) {
        Some(build) => build,
        None => return Err(()),
    };

    println!("version: {0}, build: {1}", latest_version, latest_build);

    let download_url = format!(
        "https://papermc.io/api/v2/projects/paper/versions/{0}/builds/{1}/downloads/paper-{0}-{1}.jar",
        latest_version, latest_build
    );

    Ok(download_url)
}

fn get_version_list() -> reqwest::Result<VersionList> {
    let version_list: VersionList =
        reqwest::blocking::get("https://papermc.io/api/v2/projects/paper")?.json()?;

    Ok(version_list)
}

fn get_build_list(version: &str) -> reqwest::Result<BuildList> {
    let build_list: BuildList = reqwest::blocking::get(format!(
        "https://papermc.io/api/v2/projects/paper/versions/{}",
        version
    ))?
    .json()?;

    Ok(build_list)
}
