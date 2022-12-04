//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use crate::settings::Settings;
use chrono::NaiveDate;

pub struct GetCommandExecutor<'a> {
    pub next: Option<&'a dyn ExecutorChainInterface>,
}

impl<'a> ExecutorChainInterface for GetCommandExecutor<'a> {
    fn execute(&self, request: &dyn CommandInterface, settings: Settings) -> ExecutionResult {
        if "get" == request.get_command() {
            let _address = request.get_parameter();
            let date = request.get_date();

            if date.is_some()
                && NaiveDate::parse_from_str(date.expect("We checked it before"), "%Y-%m-%d")
                    .is_err()
            {
                return ExecutionResult::WrongGetCommandParams;
            }

            let _provider = settings.get_provider();

            return ExecutionResult::Ok;
        }

        if let Some(next_executor) = self.next {
            return next_executor.execute(request, settings);
        }
        ExecutionResult::Err
    }

    fn next(&self) -> Option<&'a dyn ExecutorChainInterface> {
        self.next
    }
}
