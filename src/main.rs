//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::env;
use test_elastio::app_exit_code::AppExitCode;
use test_elastio::command::configure_command_executor::ConfigureCommandExecutor;
use test_elastio::command::get_command_executor::GetCommandExecutor;
use test_elastio::command::view_command_executor::ViewCommandExecutor;
use test_elastio::interfaces::executor_chain_interface::ExecutorChainInterface;
use test_elastio::request::Request;
use test_elastio::weather_provider::provider_settings::ProviderSettings;
use test_elastio::weather_provider::weather_provider_factory::WeatherProviderFactory;

fn main() -> AppExitCode {
    if dotenv::var("openweathermap").is_err()
        || dotenv::var("openweathermap")
            .expect("We checked it before")
            .is_empty()
        || dotenv::var("weatherapi").is_err()
        || dotenv::var("weatherapi")
            .expect("We checked it before")
            .is_empty()
    {
        return AppExitCode::NoApiKeys;
    }

    let arguments: Box<dyn Iterator<Item = String>> = Box::new(env::args());
    let Ok(request) = Request::try_from(arguments) else {
        return AppExitCode::WrongParams;
    };

    let view_chain = ViewCommandExecutor::default();

    let configure_chain = ConfigureCommandExecutor {
        next: Some(&view_chain),
    };

    let get_chain = GetCommandExecutor {
        next: Some(&configure_chain),
        factory: Box::new(WeatherProviderFactory {}),
    };

    let settings = ProviderSettings {
        available_providers: ["Gismeteo", "Alvares", "Openweathermap", "Weatherapi"],
    };

    get_chain.execute(&request, &settings)
}
