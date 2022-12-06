//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::interfaces::weather_provider_strategy_interface::WeatherProviderStrategyInterface;

pub trait WeatherProviderFactoryInterface {
    fn get_provider(&self, provider: String) -> Box<dyn WeatherProviderStrategyInterface>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        //weather providers
        struct GismeteoWeatherProvider {}

        impl WeatherProviderStrategyInterface for GismeteoWeatherProvider {
            fn get_forecast(
                &self,
                _address: &str,
                _date: Option<&String>,
            ) -> Result<String, String> {
                Ok("Gismeteo: 8 C, 64 %, 4.8 m/s".to_owned())
            }
        }

        struct AlvaresWeatherProvider {}

        impl WeatherProviderStrategyInterface for AlvaresWeatherProvider {
            fn get_forecast(
                &self,
                _address: &str,
                _date: Option<&String>,
            ) -> Result<String, String> {
                Ok("Alvares: 16 C, 32 %, 2.4 m/s".to_owned())
            }
        }

        //factory
        struct WeatherProviderFactory {}

        impl WeatherProviderFactoryInterface for WeatherProviderFactory {
            fn get_provider(&self, provider: String) -> Box<dyn WeatherProviderStrategyInterface> {
                match provider.as_str() {
                    "Alvares" => Box::new(AlvaresWeatherProvider {}),
                    _ => Box::new(GismeteoWeatherProvider {}),
                }
            }
        }

        let factory = WeatherProviderFactory {};

        let actual = factory
            .get_provider("Gismeteo".to_string())
            .get_forecast("London", Some(&"2022-12-06".to_owned()));
        let expected = "Gismeteo: 8 C, 64 %, 4.8 m/s".to_owned();
        assert_eq!(expected, actual.unwrap());

        let actual = factory
            .get_provider("Alvares".to_string())
            .get_forecast("London", Some(&"2022-12-06".to_owned()));
        let expected = "Alvares: 16 C, 32 %, 2.4 m/s".to_owned();
        assert_eq!(expected, actual.unwrap());
    }
}
