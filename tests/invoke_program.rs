//@author Stanislav Polaniev <spolanyev@gmail.com>

use assert_cmd::Command;
use serial_test::serial;

#[test]
#[serial]
fn invoke_without_params() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.assert();
    assert.stdout("Expected format is `weather configure Weatherapi` or `weather get London`\n");
}

#[test]
#[serial]
fn invoke_with_params() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Gismeteo").assert();
    assert.stdout("Gismeteo set\n");
}

#[test]
#[serial]
fn invoke_with_unknown_command() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("unknown").arg("andSomething").assert();
    assert.stdout("Allowed commands are `configure` and `get`\n");
}

#[test]
#[serial]
fn invoke_with_known_provider() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Gismeteo").assert();
    assert.stdout("Gismeteo set\n");
}

#[test]
#[serial]
fn invoke_with_unknown_provider() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Amazon").assert();
    assert.stdout("Allowed services are `Weatherapi` and `Openweathermap`\n");
}

#[test]
#[serial]
fn invoke_with_without_date() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Gismeteo").assert();
    assert.stdout("Gismeteo set\n");

    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").assert();
    assert.stdout("London: 8 C, 32 %, 2.4 m/s\n");
}

#[test]
#[serial]
fn invoke_with_valid_date() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Gismeteo").assert();
    assert.stdout("Gismeteo set\n");

    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").arg("2022-12-02").assert();
    assert.stdout("London: 4 C, 16 %, 1.2 m/s\n");
}

#[test]
#[serial]
fn invoke_with_invalid_date() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").arg("2022-12").assert();
    assert.stdout("Expected date format is `2023-01-31`\n");
}

#[test]
#[serial]
fn set_provider_and_check_it() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Gismeteo").assert();
    assert.stdout("Gismeteo set\n");

    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Alvares").assert();
    assert.stdout("Alvares set\n");

    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("view").arg("settings").assert();
    assert.stdout("Current service is Alvares\n");

    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").assert();
    assert.stdout("London: 16 C, 64 %, 4.8 m/s\n");
}

#[test]
#[ignore]
fn invoke_without_api_keys() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").arg("2022-12").assert();
    assert.stdout("API keys not found\n");
}

#[test]
#[ignore]
#[serial]
fn production_set_openweathermap_and_check() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Openweathermap").assert();
    assert.stdout("Openweathermap set\n");

    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").arg("2023-01-01").assert();
    assert.stdout("This API does not support future requests\n");

    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").assert();
    assert.stdout("London: 16 C, 64 %, 4.8 m/s\n");
}

#[test]
#[ignore]
#[serial]
fn production_set_weatherapi_and_check() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Weatherapi").assert();
    assert.stdout("Weatherapi set\n");

    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").arg("2023-01-01").assert();
    assert.stdout("London: 16 C, 64 %, 4.8 m/s\n");

    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").assert();
    assert.stdout("London: 16 C, 64 %, 4.8 m/s\n");
}
