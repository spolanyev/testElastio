//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::env;
use test_elastio::configure_command_executor::ConfigureCommandExecutor;
use test_elastio::execution_result::ExecutionResult;
use test_elastio::get_command_executor::GetCommandExecutor;
use test_elastio::interfaces::executor_chain_interface::ExecutorChainInterface;
use test_elastio::request::Request;
use test_elastio::view_command_executor::ViewCommandExecutor;

fn main() -> ExecutionResult {
    let arguments: Box<dyn Iterator<Item = String>> = Box::new(env::args());
    let Ok(request) = Request::try_from(arguments) else {
        return ExecutionResult::WrongParams;
    };

    let view_command_executor = ViewCommandExecutor::default();
    let configure_command_executor = ConfigureCommandExecutor {
        next: Some(&view_command_executor),
    };
    let get_command_executor = GetCommandExecutor {
        next: Some(&configure_command_executor),
    };

    get_command_executor.execute(&request)
}
