# Draft

Task description is [here](/TASK.md)

Auto-generated docs are [here](https://spolanyev.github.io/testElastio/)

As was recommended, I used several crates: [assert_cmd](https://crates.io/crates/assert_cmd), [chrono](https://crates.io/crates/chrono), [log](https://crates.io/crates/log), [logtest](https://crates.io/crates/logtest).

Patterns used:

* Behavioral
    - [Chain of Responsibility](src/interfaces/executor_chain_interface.rs)
