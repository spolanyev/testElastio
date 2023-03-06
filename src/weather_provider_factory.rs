//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::interfaces::weather_provider_factory_interface::WeatherProviderFactoryInterface;
use crate::interfaces::weather_provider_strategy_interface::WeatherProviderStrategyInterface;
use crate::weather_provider::alvares_weather_provider::AlvaresWeatherProvider;
use crate::weather_provider::gismeteo_weather_provider::GismeteoWeatherProvider;
use crate::weather_provider::openweathermap_weather_provider::OpenweathermapWeatherProvider;
use crate::weather_provider::weatherapi_weather_provider::WeatherapiWeatherProvider;

pub struct WeatherProviderFactory {}

impl WeatherProviderFactoryInterface for WeatherProviderFactory {
    fn get_provider(&self, provider: String) -> Box<dyn WeatherProviderStrategyInterface> {
        match provider.to_lowercase().as_str() {
            "openweathermap" => Box::new(OpenweathermapWeatherProvider {}),
            "gismeteo" => Box::new(GismeteoWeatherProvider {}),
            "alvares" => Box::new(AlvaresWeatherProvider {}),
            _ => Box::new(WeatherapiWeatherProvider {}),
        }
    }
}
