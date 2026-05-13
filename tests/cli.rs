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
fn lists_weights() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["list", "weights"])
        .assert()
        .success()
        .stdout(predicate::str::contains("3.5g"));
}

#[test]
fn lists_categories() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["list", "categories"])
        .assert()
        .success()
        .stdout(predicate::str::contains("flower - Flower"));
}

#[test]
fn lists_units() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["list", "units"])
        .assert()
        .success()
        .stdout(predicate::str::contains("percent - percent (potency)"));
}

#[test]
fn lists_product_types() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["list", "product-types"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "infused_pre_roll - Infused Pre-Roll [pre_roll]",
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
fn normalizes_weight_eighth_with_source_metadata() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "weight", "eighth"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\"source\"")
                .and(predicate::str::contains("\"type\": \"retail_common_term\""))
                .and(predicate::str::contains(
                    "\"note\": \"Common cannabis retail shorthand.\"",
                )),
        );
}

#[test]
fn aliases_without_source_metadata_still_normalize() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "weight", "half gram"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\"canonical\": \"0.5g\"")
                .and(predicate::str::contains("\"source\"").not()),
        );
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
fn lists_flower_package_sizes() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["list", "package-sizes", "flower"])
        .assert()
        .success()
        .stdout(predicate::str::contains("3.5g"));
}

#[test]
fn normalizes_flower_eighth_package_size() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "package-size", "flower", "eighth"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"canonical\": \"3.5g\""));
}

#[test]
fn normalizes_vape_half_gram_package_size() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "package-size", "vape", "half gram"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"canonical\": \"0.5g\""));
}

#[test]
fn normalizes_edible_package_size() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "package-size", "edible", "100mg package"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"canonical\": \"100mg_package\""));
}

#[test]
fn package_size_must_be_valid_for_category() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "package-size", "vape", "eighth"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"matched\": false"));
}

#[test]
fn lists_potency_units() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["list", "potency-units"])
        .assert()
        .success()
        .stdout(predicate::str::contains("percent"));
}

#[test]
fn lists_potency_fields() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["list", "potency-fields"])
        .assert()
        .success()
        .stdout(predicate::str::contains("total_thc - Total THC"));
}

#[test]
fn normalizes_total_potential_thc_to_total_thc() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "potency-field", "Total Potential THC"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\"kind\": \"potency-field\"")
                .and(predicate::str::contains("\"canonical\": \"total_thc\""))
                .and(predicate::str::contains("\"matched\": true")),
        );
}

#[test]
fn normalizes_delta_nine_thc_to_thc() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "potency-field", "Delta-9 THC"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"canonical\": \"thc\""));
}

#[test]
fn normalizes_total_terpenes_to_total_terpenes() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["normalize", "potency-field", "Total Terpenes"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"canonical\": \"total_terpenes\"",
        ));
}

#[test]
fn lists_nv_flower_package_size_override() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["state", "NV", "package-sizes", "flower"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("State: NV")
                .and(predicate::str::contains("Category: flower"))
                .and(predicate::str::contains("Package context: prepacked"))
                .and(predicate::str::contains("3.5g")),
        );
}

#[test]
fn unknown_state_override_request_fails_with_useful_error() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.args(["state", "NV", "package-sizes", "edible"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "No package-size override for state 'NV' category 'edible'",
        ));
}

#[test]
fn export_schema_outputs_json_schema() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    let output = cmd.arg("export-schema").output().unwrap();

    assert!(output.status.success());
    let schema: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(schema.get("$schema").is_some());
}

#[test]
fn export_schema_includes_core_bundle_properties() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.arg("export-schema").assert().success().stdout(
        predicate::str::contains("\"weights\"")
            .and(predicate::str::contains("\"categories\""))
            .and(predicate::str::contains("\"product_types\"")),
    );
}

#[test]
fn export_typescript_outputs_type_definitions() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    cmd.arg("export-typescript").assert().success().stdout(
        predicate::str::contains("export type")
            .and(predicate::str::contains("export interface StandardWeight"))
            .and(predicate::str::contains("export interface AliasEntry"))
            .and(predicate::str::contains("export interface NormalizeResult"))
            .and(predicate::str::contains("export interface StandardsBundle")),
    );
}

#[test]
fn export_json_outputs_parseable_standards_bundle() {
    let mut cmd = Command::cargo_bin("moby-standards").unwrap();

    let output = cmd.arg("export-json").output().unwrap();

    assert!(output.status.success());
    let bundle: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(bundle.get("weights").is_some());
    assert!(bundle.get("categories").is_some());
    assert!(bundle.get("units").is_some());
    assert!(bundle.get("product_types").is_some());
    assert!(bundle.get("package_sizes").is_some());
    assert!(bundle.get("potency_fields").is_some());
    assert!(bundle.get("potency_units").is_some());
    assert!(bundle.get("weight_aliases").is_some());
    assert!(bundle.get("category_aliases").is_some());
    assert!(bundle.get("product_type_aliases").is_some());
    assert!(bundle.get("package_size_aliases").is_some());
    assert!(bundle.get("potency_field_aliases").is_some());
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
