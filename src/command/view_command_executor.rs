//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::app_exit_code::AppExitCode;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use crate::interfaces::provider_settings_interface::ProviderSettingsInterface;

#[derive(Default)]
pub struct ViewCommandExecutor {}

impl ExecutorChainInterface for ViewCommandExecutor {
    fn execute(
        &self,
        request: &dyn CommandInterface,
        settings: &dyn ProviderSettingsInterface,
    ) -> AppExitCode {
        if "view" == request.get_command() {
            let entity = request.get_parameter();
            if "settings" == entity {
                match settings.get_provider() {
                    Ok(provider) => println!("Current service is {}", provider),
                    Err(error) => return error,
                };
            }
            return AppExitCode::Ok;
        }
        AppExitCode::WrongCommand
    }

    fn next(&self) -> Option<&dyn ExecutorChainInterface> {
        None
    }
}
