//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::interfaces::weather_provider_strategy_interface::WeatherProviderStrategyInterface;
use chrono::Datelike;
use weer_api::chrono::{NaiveDate, TimeZone, Timelike};
use weer_api::{chrono::Utc, *};

pub struct WeatherapiWeatherProvider {}

impl WeatherProviderStrategyInterface for WeatherapiWeatherProvider {
    fn get_forecast(&self, address: &str, date: Option<&String>) -> Result<String, String> {
        let (is_future, year, month, day, hour, minute, second) = {
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

        let client = Client::new(
            dotenv::var("weatherapi")
                .expect("We checked key existence before")
                .as_str(),
            true,
        );
        if is_future {
            let result = client
                .future()
                .query(Query::City(address.to_string()))
                .dt(Utc
                    .with_ymd_and_hms(year, month, day, hour, minute, second)
                    .unwrap())
                .call()
                .expect("future failed");

            //println!("{:#?}", result.forecast.forecast_day);

            Ok(format!(
                "{}: {:.0} C, {:.0} %, {:.1} m/s",
                result.location.name,
                result.forecast.forecast_day[0].day.avgtemp_c,
                result.forecast.forecast_day[0].day.avghumidity,
                result.forecast.forecast_day[0].day.avgvis_km / 2.237
            ))
        } else {
            let result = client
                .forecast()
                .query(Query::City(address.to_string()))
                .dt(Utc
                    .with_ymd_and_hms(year, month, day, hour, minute, second)
                    .unwrap())
                .call()
                .expect("forecast failed");

            Ok(format!(
                "{}: {:.0} C, {:.0} %, {:.1} m/s",
                result.location.name,
                result.current.temp_c,
                result.current.humidity,
                result.current.wind_kph / 2.237
            ))
        }
    }
}
