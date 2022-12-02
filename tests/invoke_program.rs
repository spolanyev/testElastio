use assert_cmd::Command;

#[test]
fn invoke_without_params() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.assert();
    assert.stdout("Expected format is `weather configure Gismeteo` or `weather get London`\n");
}

#[test]
fn invoke_with_params() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("SomeWeatherProvider").assert();
    assert.success().code(0);
}

#[test]
fn invoke_with_unknown_command() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("unknown").arg("andSomething").assert();
    assert.stdout("Allowed commands are `configure` and `get`\n");
}

#[test]
fn invoke_with_known_provider() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Gismeteo").assert();
    assert.success().code(0);
}

#[test]
fn invoke_with_unknown_provider() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("configure").arg("Amazon").assert();
    assert.stdout("Unknown provider\n");
}

#[test]
fn invoke_with_without_date() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").assert();
    assert.success().code(0);
}

#[test]
fn invoke_with_valid_date() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").arg("2022-12-02").assert();
    assert.success().code(0);
}

#[test]
fn invoke_with_invalid_date() {
    let mut cmd = Command::cargo_bin("weather").expect("We have this binary");

    let assert = cmd.arg("get").arg("London").arg("2022-12").assert();
    assert.stdout("Expected date format is `2023-01-31`\n");
}
