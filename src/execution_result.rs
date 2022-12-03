//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::process::{ExitCode, Termination};

pub enum ExecutionResult {
    Ok = 0,
    WrongParams = 2,
    WrongCommand = 4,
    WrongConfigureCommandParams = 8,
    WrongGetCommandParams = 16,
    Err = 256,
}

impl Termination for ExecutionResult {
    fn report(self) -> ExitCode {
        match self {
            ExecutionResult::WrongParams => {
                println!("Expected format is `weather configure Gismeteo` or `weather get London`");
            }

            ExecutionResult::WrongCommand => {
                println!("Allowed commands are `configure` and `get`");
            }

            ExecutionResult::WrongConfigureCommandParams => {
                println!("Unknown provider");
            }

            ExecutionResult::WrongGetCommandParams => {
                println!("Expected date format is `2023-01-31`");
            }

            ExecutionResult::Err => {
                println!("Internal error");
            }
            ExecutionResult::Ok => (),
        }
        ExitCode::from(0)
    }
}
