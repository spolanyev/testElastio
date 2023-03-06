//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::app_exit_code::AppExitCode;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::executor_chain_interface::ExecutorChainInterface;
use crate::interfaces::provider_settings_interface::ProviderSettingsInterface;
use crate::interfaces::weather_provider_factory_interface::WeatherProviderFactoryInterface;
use chrono::NaiveDate;

pub struct GetCommandExecutor<'a> {
    pub next: Option<&'a dyn ExecutorChainInterface>,
    pub factory: Box<dyn WeatherProviderFactoryInterface>,
}

impl<'a> ExecutorChainInterface for GetCommandExecutor<'a> {
    fn execute(
        &self,
        request: &dyn CommandInterface,
        settings: &dyn ProviderSettingsInterface,
    ) -> AppExitCode {
        if "get" == request.get_command() {
            let address = request.get_parameter();
            let date = request.get_date();

            if date.is_some()
                && NaiveDate::parse_from_str(date.expect("We checked it before"), "%Y-%m-%d")
                    .is_err()
            {
                return AppExitCode::WrongGetCommandParams;
            }

            let provider = match settings.get_provider() {
                Ok(provider) => provider,
                Err(error) => {
                    return error;
                }
            };

            let weather_provider = self.factory.get_provider(provider);

            let Ok(forecast) = weather_provider.get_forecast(address, date) else {
                return AppExitCode::WeatherProviderError;
            };

            println!("{}", forecast);

            return AppExitCode::Ok;
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
