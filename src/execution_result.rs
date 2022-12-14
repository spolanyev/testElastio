//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::process::{ExitCode, Termination};

#[derive(PartialEq, Eq, Debug)]
pub enum ExecutionResult {
    Ok = 0,
    WrongParams = 2,
    WrongCommand = 4,
    WrongConfigureCommandParams = 8,
    WrongGetCommandParams = 16,
    WeatherProviderError = 32,
    CannotSavePreferences = 64,
    CannotDefinePreferenceDir = 128,
    Err = 256,
    NoApiKeys = 512,
}

impl Termination for ExecutionResult {
    fn report(self) -> ExitCode {
        match self {
            ExecutionResult::Ok => (),
            ExecutionResult::WrongParams => {
                println!(
                    "Expected format is `weather configure weatherapi` or `weather get London`"
                );
            }

            ExecutionResult::WrongCommand => {
                println!("Allowed commands are `configure` and `get`");
            }

            ExecutionResult::WrongConfigureCommandParams => {
                println!("Allowed providers are `weatherapi` and `openweathermap`");
            }

            ExecutionResult::WrongGetCommandParams => {
                println!("Expected date format is `2023-01-31`");
            }

            ExecutionResult::WeatherProviderError => {
                println!("Cannot get data from the provider");
            }

            ExecutionResult::CannotSavePreferences => {
                println!("Cannot save preferences");
            }

            ExecutionResult::CannotDefinePreferenceDir => {
                println!("Cannot define preferences directory");
            }

            ExecutionResult::Err => {
                println!("Internal error");
            }

            ExecutionResult::NoApiKeys => {
                println!("API keys not found");
            }
        }
        ExitCode::from(0)
    }
}
