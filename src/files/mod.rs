use std::path::Path;

pub use download::prep_download;
pub use download::receive_file;
pub use upload::upload;

mod download;
mod upload;

pub fn string_to_path(path: &str) -> &Path {
    Path::new(path.trim_matches(char::from(0)).trim())
}
