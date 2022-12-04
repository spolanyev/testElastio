//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use chrono::NaiveDate;
use directories::ProjectDirs;
use std::fs;

pub struct GetCommandExecutor<'a> {
    pub next: Option<&'a dyn ExecutorChainInterface>,
}

impl<'a> ExecutorChainInterface for GetCommandExecutor<'a> {
    fn execute(&self, request: &dyn CommandInterface) -> ExecutionResult {
        if "get" == request.get_command() {
            let _address = request.get_parameter();
            let date = request.get_date();

            if date.is_some()
                && NaiveDate::parse_from_str(date.expect("We checked it before"), "%Y-%m-%d")
                    .is_err()
            {
                return ExecutionResult::WrongGetCommandParams;
            }

            let _provider = {
                let Some(project_dirs) = ProjectDirs::from("com", "spolaniev",  "testElastio")  else {
                    return ExecutionResult::CannotDefinePreferenceDir;
                };

                let preferences = project_dirs.config_dir().join("preferences");

                match fs::read_to_string(preferences) {
                    Ok(provider) => {
                        let mut result = "Gismeteo".to_owned();
                        let known_providers = vec!["Gismeteo", "Alvares"];
                        if known_providers.contains(&provider.as_str()) {
                            result = provider;
                        }
                        result
                    }
                    Err(_) => "Gismeteo".to_owned(),
                }
            };

            return ExecutionResult::Ok;
        }

        if let Some(next_executor) = self.next {
            return next_executor.execute(request);
        }
        ExecutionResult::Err
    }

    fn next(&self) -> Option<&'a dyn ExecutorChainInterface> {
        self.next
    }
}
