use crate::payloads::{PayloadDefault, PayloadWeb};

pub enum Platform {
    Unix,
    Windows,
    All,
}
pub trait Payload {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn generate(&self, ip: String, port: String) -> Result<String, String>;

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
        payloads.push(Box::new(PayloadWeb {}));

        Payloads { payloads }
    }
}

//Convert a Platform to String
pub fn platform_to_string(platform: Platform) -> String {
    match platform {
        Platform::All => String::from("Windows & Unix"),
        Platform::Unix => String::from("Unix"),
        Platform::Windows => String::from("Windows"),
    }
}
