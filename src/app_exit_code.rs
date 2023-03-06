//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::process::{ExitCode, Termination};

#[derive(PartialEq, Eq, Debug)]
pub enum AppExitCode {
    Ok = 0,
    WrongParams = 2,
    WrongCommand = 4,
    WrongConfigureCommandParams = 8,
    WrongGetCommandParams = 16,
    WeatherProviderError = 32,
    CannotSavePreferences = 64,
    EnvFileNotFound = 128,
    Err = 256,
    NoApiKeys = 512,
}

impl Termination for AppExitCode {
    fn report(self) -> ExitCode {
        match self {
            AppExitCode::Ok => (),
            AppExitCode::WrongParams => {
                println!(
                    "Expected format is `weather configure Weatherapi` or `weather get London`"
                );
            }

            AppExitCode::WrongCommand => {
                println!("Allowed commands are `configure` and `get`");
            }

            AppExitCode::WrongConfigureCommandParams => {
                println!("Allowed services are `Weatherapi` and `Openweathermap`");
            }

            AppExitCode::WrongGetCommandParams => {
                println!("Expected date format is `2023-01-31`");
            }

            AppExitCode::WeatherProviderError => {
                println!("Cannot get data from the service");
            }

            AppExitCode::CannotSavePreferences => {
                println!("Cannot save preferences");
            }

            AppExitCode::EnvFileNotFound => {
                println!("Cannot find .env file");
            }

            AppExitCode::Err => {
                println!("Internal error");
            }

            AppExitCode::NoApiKeys => {
                println!("API keys not found");
            }
        }
        ExitCode::from(0)
    }
}
