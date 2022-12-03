//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use chrono::NaiveDate;

pub struct GetExecutor<'a> {
    pub next: Option<&'a dyn ExecutorChainInterface>,
}

impl<'a> ExecutorChainInterface for GetExecutor<'a> {
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
