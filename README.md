# About

Task description is [here](/TASK.md)

Auto-generated documentation is [here](https://spolanyev.github.io/testElastio/test_elastio/all.html)

Crates used: [assert_cmd](https://crates.io/crates/assert_cmd), [chrono](https://crates.io/crates/chrono), [directories](https://crates.io/crates/directories), [dotenvs](https://crates.io/crates/dotenvs), [log](https://crates.io/crates/log), [logtest](https://crates.io/crates/logtest), [openweathermap](https://crates.io/crates/openweathermap), [serial_test](https://crates.io/crates/serial_test), [weer_api](https://crates.io/crates/weer_api).

Patterns used:

* Creational
  - [Simple Factory](src/interfaces/weather_provider_factory_interface.rs)


* Behavioral
  - [Chain of Responsibility](src/interfaces/executor_chain_interface.rs)
  - [Strategy](src/interfaces/weather_provider_strategy_interface.rs)

The project is done with TDD approach.<br>
Tests run upon each commit (with GitHub Actions tests run on Windows, Mac and Linux).<br>
Documentation is generated and deployed on each push.

# Usage

An API key is required to access a weather service. The program can retrieve data from [Weather API](https://www.weatherapi.com/) and [OpenWeatherMap](https://openweathermap.org/) services. One can get an API key after registering there.

Keys should be placed in `.env` file near the `weather` executable:
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
weather configure openweathermap
```
or
```shell
weather configure weatherapi
```

![Usage example](https://github.com/spolanyev/testElastio/blob/main/usage-example.png?raw=true)

# Contacts

If you are hiring, contact me at [spolanyev@gmail.com](mailto:spolanyev@gmail.com?subject=Rust%3A%20vacancy)
