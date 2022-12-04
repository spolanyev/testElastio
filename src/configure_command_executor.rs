//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use crate::interfaces::settings_interface::SettingsInterface;

pub struct ConfigureCommandExecutor<'a> {
    pub next: Option<&'a dyn ExecutorChainInterface>,
}

impl<'a> ExecutorChainInterface for ConfigureCommandExecutor<'a> {
    fn execute(
        &self,
        request: &dyn CommandInterface,
        settings: &dyn SettingsInterface,
    ) -> ExecutionResult {
        if "configure" == request.get_command() {
            let provider = request.get_parameter();
            return settings.set_provider(provider);
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
