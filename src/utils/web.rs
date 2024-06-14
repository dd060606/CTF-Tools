use std::fs::File;
use std::io::Write;
use std::path::Path;

use curl::easy::Easy;

//Download a file from a web URL
pub fn web_download(url: &str, output_file: &Path) -> Result<(), String> {
    if output_file.is_file() {
        match File::create(output_file) {
            Ok(mut file) => {
                let mut easy = Easy::new();
                if easy.url(url).is_err() {
                    return Err("Error initializing curl".to_string());
                }
                // Set up a write function to write the response body to the file
                match easy.write_function(move |data| {
                    file.write_all(data).unwrap();
                    Ok(data.len())
                }) {
                    Ok(_) => {
                        // Perform the file download
                        match easy.perform() {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e.to_string()),
                        }
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => return Err(e.to_string()),
        }
    } else {
        Err("The output path is not a file!".to_string())
    }
}
