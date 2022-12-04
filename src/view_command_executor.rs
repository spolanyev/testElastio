//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use directories::ProjectDirs;
use std::fs;

#[derive(Default)]
pub struct ViewCommandExecutor {}

impl ExecutorChainInterface for ViewCommandExecutor {
    fn execute(&self, request: &dyn CommandInterface) -> ExecutionResult {
        if "view" == request.get_command() {
            let entity = request.get_parameter();
            if "settings" == entity {
                let provider = {
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

                println!("provider: {}", provider);
            }
            return ExecutionResult::Ok;
        }
        ExecutionResult::WrongCommand
    }

    fn next(&self) -> Option<&dyn ExecutorChainInterface> {
        None
    }
}
