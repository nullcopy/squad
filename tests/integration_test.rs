use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_squad_help() {
    let mut cmd = Command::cargo_bin("squad").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Assemble your squad of AI agents"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("rm"))
        .stdout(predicate::str::contains("ps"));
}

#[test]
fn test_squad_version() {
    let mut cmd = Command::cargo_bin("squad").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("squad"));
}

#[test]
fn test_create_help() {
    let mut cmd = Command::cargo_bin("squad").unwrap();
    cmd.args(["create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Create and start new agent containers",
        ))
        .stdout(predicate::str::contains("Names of the agents to create"));
}

#[test]
fn test_list_help() {
    let mut cmd = Command::cargo_bin("squad").unwrap();
    cmd.args(["list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List all active agents"));
}

#[test]
fn test_rm_help() {
    let mut cmd = Command::cargo_bin("squad").unwrap();
    cmd.args(["rm", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stop and remove agent containers"))
        .stdout(predicate::str::contains("Remove all agents"));
}

#[test]
fn test_ps_help() {
    let mut cmd = Command::cargo_bin("squad").unwrap();
    cmd.args(["ps", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Show detailed status information"));
}

#[test]
fn test_create_without_args() {
    let mut cmd = Command::cargo_bin("squad").unwrap();
    cmd.arg("create")
        .assert()
        .success()
        .stdout(predicate::str::contains("No agent names provided"));
}

#[test]
fn test_rm_without_args() {
    let mut cmd = Command::cargo_bin("squad").unwrap();
    cmd.arg("rm")
        .assert()
        .success()
        .stdout(predicate::str::contains("No agent names provided"));
}

// Note: Full integration tests with actual container runtime
// would require Docker/Podman to be available in test environment
// These tests focus on CLI behavior and argument parsing

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("squad").unwrap();
    cmd.arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}
