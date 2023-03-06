//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::app_exit_code::AppExitCode;
use crate::interfaces::provider_settings_interface::ProviderSettingsInterface;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::ops::Add;

pub struct ProviderSettings<'a> {
    pub available_providers: [&'a str; 4],
}

impl<'a> ProviderSettingsInterface for ProviderSettings<'a> {
    fn set_provider(&self, provider: &str) -> AppExitCode {
        if !self.available_providers.contains(&provider) {
            return AppExitCode::WrongConfigureCommandParams;
        }

        let Ok(env_file) = File::open(".env") else {
            return AppExitCode::EnvFileNotFound;
        };

        let mut lines: Vec<String> = BufReader::new(env_file)
            .lines()
            .map(|line| line.expect("Failed to read line"))
            .collect();
        let line_index = lines.iter().position(|line| line.starts_with("service="));

        if let Some(index) = line_index {
            lines[index] = String::from("service=").add(provider);
        } else {
            lines.push(String::from("service=").add(provider));
        }

        let Ok(mut env_file) = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(".env") else {
            return AppExitCode::CannotSavePreferences;
        };

        let content = lines.join("\n");

        let Ok(_) = env_file.write(content.as_bytes()) else {
            return AppExitCode::CannotSavePreferences;
        };

        println!("{} set", provider);

        AppExitCode::Ok
    }

    fn get_provider(&self) -> Result<String, AppExitCode> {
        let Ok(instance) = dotenv::from_path(".env") else {
            return Err(AppExitCode::EnvFileNotFound);
        };

        instance.load_override();

        if dotenv::var("service").is_err()
            || dotenv::var("service").expect("We have the key").is_empty()
        {
            return Ok("Gismeteo".to_owned());
        }

        Ok(dotenv::var("service").expect("We have the key"))
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn set_and_get() {
        let settings = ProviderSettings {
            available_providers: ["Mercury", "Venus", "Gismeteo", "Alvares"],
        };

        settings.set_provider("Mercury");
        assert_eq!(Ok("Mercury".to_string()), settings.get_provider());

        settings.set_provider("Venus");
        assert_eq!(Ok("Venus".to_string()), settings.get_provider());
    }
}
