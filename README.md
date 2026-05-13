# MOBY Standards

Canonical cannabis data standards for MOBY.

`moby-standards` defines normalized cannabis product language for names, weights, sizes, categories, product types, units, and aliases.

It is designed to help cannabis data tools convert messy POS, compliance, menu, COA, and state-specific source data into consistent MOBY-compatible output.

## Purpose

Cannabis data is messy.

Different systems may describe the same thing in different ways:

- `eighth`
- `1/8 oz`
- `3.5 grams`
- `3.5g`

MOBY Standards maps those inputs to one canonical value:

```json
{
  "canonical": "3.5g"
}
```

## Current Scope

v0.1.0 includes:

- canonical weights
- weight aliases
- canonical product categories
- category aliases
- canonical units
- starter product types
- CLI listing
- CLI normalization
- validation of reference data

Commands

List canonical weights:

```bash
cargo run -- list weights
```

List canonical categories:

```bash
cargo run -- list categories
```

Normalize a weight:

```bash
cargo run -- normalize weight eighth
```

Normalize a category:

```bash
cargo run -- normalize category cart
```

Validate standards data:

```bash
cargo run -- validate
```

Export all loaded standards as JSON:

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
