//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::env;
use test_elastio::configure_command_executor::ConfigureCommandExecutor;
use test_elastio::execution_result::ExecutionResult;
use test_elastio::get_command_executor::GetCommandExecutor;
use test_elastio::interfaces::executor_chain_interface::ExecutorChainInterface;
use test_elastio::request::Request;
use test_elastio::settings::Settings;
use test_elastio::view_command_executor::ViewCommandExecutor;
use test_elastio::weather_provider_factory::WeatherProviderFactory;

fn main() -> ExecutionResult {
    if dotenv::var("openweathermap").is_err()
        || dotenv::var("openweathermap")
            .expect("We checked it before")
            .is_empty()
        || dotenv::var("weatherapi").is_err()
        || dotenv::var("weatherapi")
            .expect("We checked it before")
            .is_empty()
    {
        return ExecutionResult::NoApiKeys;
    }

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
        factory: Box::new(WeatherProviderFactory {}),
    };

    let settings = Settings {
        available_providers: ["Gismeteo", "Alvares", "Openweathermap", "Weatherapi"],
    };

    get_command_executor.execute(&request, &settings)
}
