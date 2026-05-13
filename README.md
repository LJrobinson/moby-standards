# MOBY Standards

Canonical cannabis data standards for MOBY.

`moby-standards` v0.5.0 provides a Rust CLI and YAML-backed registries for canonical cannabis weights, package sizes, categories, units, product types, potency fields, potency units, aliases, and state-aware standards extensions.

It is designed to help cannabis data tools convert messy POS, compliance, menu, COA, and state-specific source data into consistent MOBY-compatible output.

## Purpose

Cannabis data is messy.

Different systems may describe the same thing in different ways:

- `eighth`
- `1/8`
- `1/8 oz`
- `eighth ounce`

MOBY Standards maps known aliases to one canonical value:

```json
{
  "canonical": "3.5g"
}
```

## Current Scope

v0.1.0 Framework is complete and includes:

- Rust CLI initialized
- YAML-backed standards
- Weight normalization
- Category normalization
- Validation command
- JSON export command

v0.2.0 Product Type Aliases is complete and adds:

- product-type aliases
- product-type normalization
- validation that product-type aliases point to canonical product types

v0.3.0 Package Size Standards is complete and adds:

- category-aware package-size standards
- package-size aliases
- package-size listing by category
- package-size normalization by category
- validation that package-size categories and gram-based package sizes are canonical

v0.4.0 Potency Units is complete and adds:

- canonical THC/CBD-related potency fields
- canonical potency units
- potency-field aliases
- potency-field normalization
- potency field/unit listing
- validation that potency-field aliases point to canonical potency fields

v0.5.0 State Overrides is complete and adds:

- state-specific standards folder support
- starter NV package-size override example
- state-aware package-size listing
- validation for state override categories, recognized weights, state codes, and source confidence

Canonical weights are global. Package sizes are category-aware.

For example, `3.5g` is a canonical weight and a recognized flower package size. `0.5g` is a recognized vape, concentrate, and pre-roll package size. `100mg_package` is an edible package-size token, not a mass weight.

Potency fields are canonical field names for cannabinoid and terpene values. For example, `Total Potential THC` normalizes to `total_thc`, while `Delta-9 THC` normalizes to `thc`.

State overrides are state-specific MOBY Standards extensions that can reference or align with MOBY Atlas. MOBY Atlas remains the state-by-state source truth; MOBY Standards does not become a legal database. The included NV package-size override is a starter framework example and should be reconciled with MOBY Atlas source-cited state data before being treated as official.

The loaded YAML registries currently include canonical weights, package sizes, categories, units, product types, potency fields, potency units, state package-size overrides, weight aliases, category aliases, product-type aliases, package-size aliases, and potency-field aliases.

## Commands

List canonical weights:

```bash
cargo run -- list weights
```

List canonical categories:

```bash
cargo run -- list categories
```

List canonical units:

```bash
cargo run -- list units
```

List starter product types:

```bash
cargo run -- list product-types
```

List package sizes for a category:

```bash
cargo run -- list package-sizes flower
```

List potency fields:

```bash
cargo run -- list potency-fields
```

List potency units:

```bash
cargo run -- list potency-units
```

Show a state package-size extension:

```bash
cargo run -- state NV package-sizes flower
```

Normalize a weight:

```bash
cargo run -- normalize weight eighth
```

Normalize a category:

```bash
cargo run -- normalize category cart
```

Normalize a product type:

```bash
cargo run -- normalize product-type "infused joint"
```

Normalize a package size within a category:

```bash
cargo run -- normalize package-size flower eighth
```

```bash
cargo run -- normalize package-size edible "100mg package"
```

Normalize a potency field:

```bash
cargo run -- normalize potency-field "Total Potential THC"
```

The normalize kind determines the registry used. For example, `normalize category cart` returns `vape`, while `normalize product-type cart` returns `vape_cartridge`.

Validate standards and aliases:

```bash
cargo run -- validate
```

Export all loaded standards and aliases as JSON:

```bash
cargo run -- export-json
```

Example Output

```bash
{
  "input": "eighth",
  "kind": "weight",
  "canonical": "3.5g",
  "confidence": "high",
  "matched": true
}
```

Product Type Example Output

```bash
{
  "input": "infused joint",
  "kind": "product-type",
  "canonical": "infused_pre_roll",
  "confidence": "high",
  "matched": true
}
```

Package Size Example Output

```bash
{
  "input": "eighth",
  "kind": "package-size",
  "canonical": "3.5g",
  "confidence": "high",
  "matched": true
}
```

Potency Field Example Output

```bash
{
  "input": "Total Potential THC",
  "kind": "potency-field",
  "canonical": "total_thc",
  "confidence": "high",
  "matched": true
}
```

## Relationship to MOBY Atlas

MOBY Atlas is the state-by-state cannabis reference layer.

MOBY Standards is the canonical vocabulary layer.

Together:

MOBY Atlas
  State-specific truth

MOBY Standards
  Canonical cannabis vocabulary

MOBY Normalizers
  Translate messy source data into clean MOBY-compatible data

---

Example

Nevada might describe flower as:

usable cannabis
plant material
bulk flower
flower cured

MOBY Standards normalizes those to:

```bash
{
  "category": "flower"
}
```
