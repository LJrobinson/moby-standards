# MOBY Standards

Canonical cannabis data standards for MOBY.

`moby-standards` v0.1.0 provides a Rust CLI and YAML-backed registries for canonical cannabis weights, categories, units, starter product types, and aliases.

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

The loaded YAML registries currently include canonical weights, categories, units, starter product types, weight aliases, and category aliases.

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

Normalize a weight:

```bash
cargo run -- normalize weight eighth
```

Normalize a category:

```bash
cargo run -- normalize category cart
```

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
