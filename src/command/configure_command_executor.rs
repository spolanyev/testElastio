//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::app_exit_code::AppExitCode;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use crate::interfaces::provider_settings_interface::ProviderSettingsInterface;

pub struct ConfigureCommandExecutor<'a> {
    pub next: Option<&'a dyn ExecutorChainInterface>,
}

impl<'a> ExecutorChainInterface for ConfigureCommandExecutor<'a> {
    fn execute(
        &self,
        request: &dyn CommandInterface,
        settings: &dyn ProviderSettingsInterface,
    ) -> AppExitCode {
        if "configure" == request.get_command() {
            let provider = request.get_parameter();
            return settings.set_provider(provider);
        }

        if let Some(next_executor) = self.next {
            return next_executor.execute(request, settings);
        }
        AppExitCode::Err
    }

    fn next(&self) -> Option<&'a dyn ExecutorChainInterface> {
        self.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::view_command_executor::ViewCommandExecutor;
    use crate::request::Request;
    use crate::weather_provider::provider_settings::ProviderSettings;
    use serial_test::serial;

    #[test]
    #[serial]
    fn set_provider_and_check_next() {
        let arguments: Box<dyn Iterator<Item = String>> = Box::new(
            vec![
                "path_will_be_skipped".to_owned(),
                "configure".to_owned(),
                "Venus".to_owned(),
            ]
            .into_iter(),
        );
        let request = Request::try_from(arguments).expect("We provided arguments");

        let view_command_executor = ViewCommandExecutor::default();
        let configure_command_executor = ConfigureCommandExecutor {
            next: Some(&view_command_executor),
        };

        let settings = ProviderSettings {
            available_providers: ["Mercury", "Venus", "Gismeteo", "Alvares"],
        };

        assert_eq!(
            AppExitCode::Ok,
            configure_command_executor.execute(&request, &settings)
        );

        assert!(configure_command_executor.next().is_some());
    }
}
