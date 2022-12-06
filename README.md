# Draft

Task description is [here](/TASK.md)

Auto-generated documentation is [here](https://spolanyev.github.io/testElastio/test_elastio/all.html)

Crates used: [assert_cmd](https://crates.io/crates/assert_cmd), [chrono](https://crates.io/crates/chrono), [directories](https://crates.io/crates/directories), [dotenvs](https://crates.io/crates/dotenvs), [log](https://crates.io/crates/log), [logtest](https://crates.io/crates/logtest), [openweathermap](https://crates.io/crates/openweathermap), [serial_test](https://crates.io/crates/serial_test), [weer_api](https://crates.io/crates/weer_api).

Patterns used:

* Creational
  - [Simple Factory](src/interfaces/weather_provider_factory_interface.rs)


* Behavioral
  - [Chain of Responsibility](src/interfaces/executor_chain_interface.rs)
  - [Strategy](src/interfaces/weather_provider_strategy_interface.rs)
