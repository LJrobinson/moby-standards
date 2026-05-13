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

#[test]
fn normalizes_infused_joint_to_infused_pre_roll() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "product-type", "infused joint"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\"kind\": \"product-type\"")
                .and(predicate::str::contains(
                    "\"canonical\": \"infused_pre_roll\"",
                ))
                .and(predicate::str::contains("\"matched\": true")),
        );
}

#[test]
fn normalizes_disposable_vape_to_vape_disposable() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "product-type", "disposable vape"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"canonical\": \"vape_disposable\"",
        ));
}

#[test]
fn normalizes_live_rosin_to_live_rosin() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "product-type", "live rosin"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"canonical\": \"live_rosin\""));
}

#[test]
fn normalizes_popcorn_to_popcorn_flower() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "product-type", "popcorn"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"canonical\": \"popcorn_flower\"",
        ));
}

#[test]
fn unknown_product_type_returns_unmatched_result() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "product-type", "mystery goblin product"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"matched\": false"));
}
