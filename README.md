# About

Task description is [here](/TASK.md)

Auto-generated documentation is [here](https://spolanyev.github.io/testElastio/test_elastio/all.html)

The task required me to use appropriate crates, so I opted for: [assert_cmd](https://crates.io/crates/assert_cmd), [chrono](https://crates.io/crates/chrono), [directories](https://crates.io/crates/directories), [dotenvs](https://crates.io/crates/dotenvs), [log](https://crates.io/crates/log), [logtest](https://crates.io/crates/logtest), [openweathermap](https://crates.io/crates/openweathermap), [serial_test](https://crates.io/crates/serial_test), [weer_api](https://crates.io/crates/weer_api).

Patterns used:

* Creational
  - [Simple Factory](src/interfaces/weather_provider_factory_interface.rs)


* Behavioral
  - [Chain of Responsibility](src/interfaces/executor_chain_interface.rs)
  - [Strategy](src/interfaces/weather_provider_strategy_interface.rs)

The project is done with TDD approach.<br>
Tests are run on each commit, and GitHub Actions runs tests on Windows, Mac, and Linux.<br>
Documentation is generated and deployed on each push.

# Usage

An API key is required to access a weather service. The program can fetch data from the [Weather API](https://www.weatherapi.com/) and [OpenWeatherMap](https://openweathermap.org/). To obtain an API key, you can register on those websites.
The keys should be placed in the `.env` file located in the current working directory:
```dotenv
# .env file
weatherapi=b2172e34d3bc4943992130745111111
openweathermap=a83031352279cdd3e5890ac225111111
```

## Terminal commands

### Get a weather
```shell
weather get London
```
or
```shell
weather get London 2023-01-31
```

### See the current service
```shell
weather view settings
```

### Set a service
```shell
weather configure Openweathermap
```
or
```shell
weather configure Weatherapi
```

![Usage example](https://github.com/spolanyev/testElastio/blob/main/usage-example.png?raw=true)

# Contacts

If you are hiring, contact me at [spolanyev@gmail.com](mailto:spolanyev@gmail.com?subject=Rust%3A%20vacancy)
