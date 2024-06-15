use crate::payloads::{Payload, Platform};

pub struct PayloadDefault;

impl Payload for PayloadDefault {
    fn name(&self) -> String {
        String::from("default")
    }

    fn description(&self) -> String {
        String::from("CTF-Tools is bundled in a command, you have to copy/paste this command on the target machine.")
    }

    fn generate(&self, ip: String, port: String) -> Result<String, String> {
        todo!()
    }

    fn platform(&self) -> Platform {
        Platform::All
    }
}
