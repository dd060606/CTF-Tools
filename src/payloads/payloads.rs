use crate::payloads::PayloadDefault;

pub enum Platform {
    Unix,
    Windows,
    All,
}
pub trait Payload {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn generate(&self, ip: &str, port: &str, platform: Platform) -> Result<(), String>;

    fn platform(&self) -> Platform;
}

pub struct Payloads {
    pub payloads: Vec<Box<dyn Payload>>,
}

impl Payloads {
    pub fn new() -> Payloads {
        let mut payloads: Vec<Box<dyn Payload>> = Vec::new();
        //Register payloads
        payloads.push(Box::new(PayloadDefault {}));

        Payloads { payloads }
    }
}

//Convert a Platform to a String
pub fn platform_to_string(platform: Platform) -> String {
    match platform {
        Platform::All => String::from("Windows & Unix"),
        Platform::Unix => String::from("Unix"),
        Platform::Windows => String::from("Windows"),
    }
}

pub fn string_to_platform(platform: &str) -> Option<Platform> {
    match platform.to_ascii_lowercase().as_str() {
        "unix" | "linux" => Some(Platform::Unix),
        "win" | "windows" => Some(Platform::Windows),
        _ => None,
    }
}
