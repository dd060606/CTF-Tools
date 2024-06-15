use crate::payloads::{Payload, Platform};

pub struct PayloadWeb;

impl Payload for PayloadWeb {
    fn name(&self) -> String {
        String::from("web")
    }

    fn description(&self) -> String {
        String::from("CTF-Tools is automatically downloaded from GitHub using a command.")
    }

    fn generate(&self, ip: String, port: String) -> Result<String, String> {
        todo!()
    }

    fn platform(&self) -> Platform {
        Platform::All
    }
}
