//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::interfaces::weather_provider_strategy_interface::WeatherProviderStrategyInterface;
use std::error::Error;

pub struct GismeteoWeatherProvider {}

impl WeatherProviderStrategyInterface for GismeteoWeatherProvider {
    fn get_forecast(&self, address: &str, date: Option<&String>) -> Result<String, Box<dyn Error>> {
        if date.is_none() {
            return Ok(format!("{}: 8 C, 32 %, 2.4 m/s", address));
        }

        Ok(format!("{}: 4 C, 16 %, 1.2 m/s", address))
    }
}
