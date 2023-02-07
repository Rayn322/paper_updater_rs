use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct VersionList {
    versions: Vec<String>,
}

pub fn fetch_latest() -> serde_json::Result<()> {
    let version_list = get_version_list();
    println!(
        "{}",
        version_list
            .versions
            .get(version_list.versions.len() - 1)
            .unwrap()
    );

    Ok(())
}

fn get_version_list() -> VersionList {
    let response = match fetch_version_list() {
        Ok(response) => response,
        Err(_) => panic!("Error!"),
    };

    let version_list: VersionList = match serde_json::from_str(&response) {
        Ok(version_list) => version_list,
        Err(_) => panic!("Error!"),
    };

    version_list
}

fn fetch_version_list() -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get("https://papermc.io/api/v2/projects/paper")?.text()?;
    Ok(response)
}
