use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn validates_standards_data() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "OK: MOBY standards data is valid.",
        ));
}

#[test]
fn normalizes_eighth_to_three_point_five_grams() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "weight", "eighth"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"canonical\": \"3.5g\""));
}

#[test]
fn normalizes_cart_to_vape() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "category", "cart"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"canonical\": \"vape\""));
}

#[test]
fn unknown_weight_returns_unmatched_result() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "weight", "goblin sack"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"matched\": false"));
}
