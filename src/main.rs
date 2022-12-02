//@author Stanislav Polaniev <spolanyev@gmail.com>

use chrono::NaiveDate;
use std::env;
use test_elastio::execution_result::ExecutionResult;

fn main() -> ExecutionResult {
    let mut arguments = env::args();
    arguments.next(); //skip file path
    let (Some(command), Some(parameter)) = (arguments.next(), arguments.next()) else {
        return ExecutionResult::WrongParams;
    };

    match command.as_str() {
        "configure" => {
            let provider = parameter;
            let known_providers = vec!["Gismeteo"];
            if !known_providers.contains(&&*provider) {
                return ExecutionResult::WrongConfigureCommandParams;
            }
        }
        "get" => {
            let _address = parameter;
            let date = {
                let mut result = String::new();
                if let Some(date) = arguments.next() {
                    result = date;
                }
                result
            };

            if !date.is_empty() && NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d").is_err() {
                return ExecutionResult::WrongGetCommandParams;
            }
        }
        _ => return ExecutionResult::WrongCommand,
    }

    ExecutionResult::Ok
}
