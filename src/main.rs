//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::env;
use test_elastio::configure_executor::ConfigureExecutor;
use test_elastio::execution_result::ExecutionResult;
use test_elastio::get_executor::GetExecutor;
use test_elastio::interfaces::executor_chain_interface::ExecutorChainInterface;
use test_elastio::request::Request;
use test_elastio::view_command_executor::ViewCommandExecutor;

fn main() -> ExecutionResult {
    let mut arguments = env::args();
    arguments.next(); //skip file path
    let (Some(command), Some(parameter)) = (arguments.next(), arguments.next()) else {
        return ExecutionResult::WrongParams;
    };

    let date = {
        let mut result = None;
        if let Some(date) = arguments.next() {
            result = Some(date);
        }
        result
    };

    let request = Request {
        command,
        parameter,
        date,
    };

    let view_command_executor = ViewCommandExecutor::default();
    let configure_executor = ConfigureExecutor {
        next: Some(&view_command_executor),
    };
    let get_executor = GetExecutor {
        next: Some(&configure_executor),
    };

    get_executor.execute(&request)
}
