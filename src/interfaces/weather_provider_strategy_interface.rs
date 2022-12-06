//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::error::Error;

pub trait WeatherProviderStrategyInterface {
    fn get_forecast(&self, address: &str, date: Option<&String>) -> Result<String, Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy() {
        //Context
        struct WeatherProvider {
            strategy: Box<dyn WeatherProviderStrategyInterface>,
        }

        impl WeatherProvider {
            fn get(&self, address: &str, date: Option<&String>) -> Result<String, Box<dyn Error>> {
                self.strategy.get_forecast(address, date)
            }
        }

        //Concrete strategy - Gismeteo
        struct GismeteoWeatherProvider {}

        impl WeatherProviderStrategyInterface for GismeteoWeatherProvider {
            fn get_forecast(
                &self,
                address: &str,
                _date: Option<&String>,
            ) -> Result<String, Box<dyn Error>> {
                Ok(format!("{}: 8 C, 64 %, 4.8 m/s", address))
            }
        }

        //Concrete strategy - Alvares
        struct AlvaresWeatherProvider {}

        impl WeatherProviderStrategyInterface for AlvaresWeatherProvider {
            fn get_forecast(
                &self,
                address: &str,
                _date: Option<&String>,
            ) -> Result<String, Box<dyn Error>> {
                Ok(format!("{}: 16 C, 32 %, 2.4 m/s", address))
            }
        }

        let strategy = Box::new(GismeteoWeatherProvider {});
        let weather_provider = WeatherProvider { strategy };
        assert_eq!(
            "London: 8 C, 64 %, 4.8 m/s".to_string(),
            weather_provider.get("London", None).unwrap()
        );
    }
}
