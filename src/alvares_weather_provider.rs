//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::interfaces::weather_provider_strategy_interface::WeatherProviderStrategyInterface;

pub struct AlvaresWeatherProvider {}

impl WeatherProviderStrategyInterface for AlvaresWeatherProvider {
    fn get_forecast(&self, address: &str, _date: Option<&String>) -> Result<String, String> {
        Ok(format!("{}: 16 C, 64 %, 4.8 m/s", address))
    }
}
