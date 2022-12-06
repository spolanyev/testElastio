//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::alvares_weather_provider::AlvaresWeatherProvider;
use crate::gismeteo_weather_provider::GismeteoWeatherProvider;
use crate::interfaces::weather_provider_factory_interface::WeatherProviderFactoryInterface;
use crate::interfaces::weather_provider_strategy_interface::WeatherProviderStrategyInterface;
use crate::openweathermap_weather_provider::OpenweathermapWeatherProvider;
use crate::weatherapi_weather_provider::WeatherapiWeatherProvider;

pub struct WeatherProviderFactory {}

impl WeatherProviderFactoryInterface for WeatherProviderFactory {
    fn get_provider(&self, provider: String) -> Box<dyn WeatherProviderStrategyInterface> {
        match provider.to_lowercase().as_str() {
            "openweathermap" => Box::new(OpenweathermapWeatherProvider {}),
            "weatherapi" => Box::new(WeatherapiWeatherProvider {}),
            "alvares" => Box::new(AlvaresWeatherProvider {}),
            _ => Box::new(GismeteoWeatherProvider {}),
        }
    }
}
