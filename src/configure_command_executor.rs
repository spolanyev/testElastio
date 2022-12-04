//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use directories::ProjectDirs;
use std::fs;

pub struct ConfigureCommandExecutor<'a> {
    pub next: Option<&'a dyn ExecutorChainInterface>,
}

impl<'a> ExecutorChainInterface for ConfigureCommandExecutor<'a> {
    fn execute(&self, request: &dyn CommandInterface) -> ExecutionResult {
        if "configure" == request.get_command() {
            let provider = request.get_parameter();
            let known_providers = vec!["Gismeteo", "Alvares"];
            if !known_providers.contains(&provider) {
                return ExecutionResult::WrongConfigureCommandParams;
            }

            let Some(project_dirs) = ProjectDirs::from("com", "spolaniev",  "testElastio")  else {
                return ExecutionResult::CannotDefinePreferenceDir;
            };

            let preferences = project_dirs.config_dir().join("preferences");
            if fs::create_dir_all(project_dirs.config_dir()).is_err()
                || fs::write(preferences, provider).is_err()
            {
                return ExecutionResult::CannotSavePreferences;
            }

            println!("{} set", provider);

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
