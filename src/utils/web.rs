use std::env;
use std::fs::File;
use std::io::Write;

use curl::easy::Easy;
use serde::Deserialize;

// Structure to represent the URL of assets in a GitHub release
#[derive(Deserialize)]
struct Asset {
    browser_download_url: String,
    name: String,
}

// Structure to represent a GitHub release
#[derive(Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

// Function to download the latest release for Linux from a GitHub repository
pub fn download_latest_release(
    owner: &str,
    repo: &str,
    r_pattern: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Construct the API URL to get the latest release
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );

    // Initialize the HTTP request
    let mut easy = Easy::new();
    easy.url(&url)?;
    easy.useragent("curl/7.47.0")?;

    // Buffer to store the JSON response
    let mut response_data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            response_data.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    // Parse the JSON response to get the release information
    let release: Release = serde_json::from_slice(&response_data)?;

    // Find the asset for Linux
    let asset = release
        .assets
        .into_iter()
        .find(|asset| asset.name.contains(r_pattern))
        .ok_or("Asset not found")?;

    // Download the asset
    let mut easy = Easy::new();
    easy.url(&asset.browser_download_url)?;
    easy.follow_location(true)?;
    let path = env::temp_dir().join(asset.name);
    let mut file = File::create(&path)?;

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            match file.write_all(data) {
                Ok(_) => {}
                Err(e) => println!("{}", e.to_string()),
            };
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    Ok(path.to_string_lossy().to_string())
}
