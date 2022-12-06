//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::interfaces::weather_provider_strategy_interface::WeatherProviderStrategyInterface;
use chrono::{Datelike, NaiveDate, Timelike, Utc};
use openweathermap::blocking::weather;

pub struct OpenweathermapWeatherProvider {}

impl WeatherProviderStrategyInterface for OpenweathermapWeatherProvider {
    fn get_forecast(&self, address: &str, date: Option<&String>) -> Result<String, String> {
        let (is_future, _year, _month, _day, _hour, _minute, _second) = {
            let now = Utc::now();
            let current_date = now.date_naive();
            let current_time = now.time();

            if date.is_none() {
                (
                    false,
                    current_date.year(),
                    current_date.month(),
                    current_date.day(),
                    current_time.hour(),
                    current_time.minute(),
                    current_time.second(),
                )
            } else {
                //2022-12-06
                let parts: Vec<&str> = date
                    .as_ref()
                    .expect("We checked it before")
                    .split('-')
                    .collect();

                let year = parts[0]
                    .parse::<i32>()
                    .expect("We checked the format previously");
                let month = parts[1]
                    .parse::<u32>()
                    .expect("We checked the format previously");
                let day = parts[2]
                    .parse::<u32>()
                    .expect("We checked the format previously");

                let is_future = {
                    let time = NaiveDate::from_ymd_opt(year, month, day)
                        .expect("We checked date before")
                        .and_hms_opt(0, 0, 0)
                        .expect("Nothing breaking here");
                    (time - now.naive_utc()).num_days() > 0
                };

                (
                    is_future, year, month, day,
                    13u32, //13 hours, consider it the best middle time for forecast ;)
                    0u32, 0u32,
                )
            }
        };

        if is_future {
            Ok("This API doesn't support future request".to_owned())
        } else {
            match &weather(
                address,
                "metric",
                "en",
                dotenv::var("openweathermap")
                    .expect("We checked key existence before")
                    .as_str(),
            ) {
                Ok(current) => Ok(format!(
                    "{}: {:.0} C, {:.0} %, {:.1} m/s",
                    current.name.as_str(),
                    current.main.temp,
                    current.main.humidity,
                    current.wind.speed
                )),
                Err(_) => Err("".to_owned()),
            }
        }
    }
}
