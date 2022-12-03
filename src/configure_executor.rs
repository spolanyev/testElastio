//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;

#[derive(Default)]
pub struct ConfigureExecutor {}

impl ExecutorChainInterface for ConfigureExecutor {
    fn execute(&self, request: &dyn CommandInterface) -> ExecutionResult {
        if "configure" == request.get_command() {
            let provider = request.get_parameter();
            let known_providers = vec!["Gismeteo"];
            if !known_providers.contains(&provider) {
                return ExecutionResult::WrongConfigureCommandParams;
            }
            return ExecutionResult::Ok;
        }
        ExecutionResult::WrongCommand
    }

    fn next(&self) -> Option<&dyn ExecutorChainInterface> {
        None
    }
}
