//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use crate::settings::Settings;

#[derive(Default)]
pub struct ViewCommandExecutor {}

impl ExecutorChainInterface for ViewCommandExecutor {
    fn execute(&self, request: &dyn CommandInterface, settings: Settings) -> ExecutionResult {
        if "view" == request.get_command() {
            let entity = request.get_parameter();
            if "settings" == entity {
                match settings.get_provider() {
                    Ok(provider) => println!("provider: {}", provider),
                    Err(error) => return error,
                };
            }
            return ExecutionResult::Ok;
        }
        ExecutionResult::WrongCommand
    }

    fn next(&self) -> Option<&dyn ExecutorChainInterface> {
        None
    }
}
