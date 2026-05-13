# Standard Keys

MOBY Standards v1.0.0 treats canonical identifiers as stable public contracts.
These keys are the values other MOBY projects should store, compare, and exchange.

## Stable Key Policy

- Stable keys should not be renamed or reused for a different meaning.
- Additive entries are preferred over breaking changes.
- If a future change must replace a key, document the replacement and keep alias or migration behavior where practical.
- `docs/roadmap.md` remains the roadmap source of truth. This document defines v1 key contracts, not a second roadmap.

## Weights

Weight `label` is the stable identifier for v1.0.0.

Examples:

- `0.5g`
- `1g`
- `3.5g`
- `7g`
- `14g`
- `28g`
- `56g`
- `70g`

The `grams` value is the numeric mass represented by the label. `category_hint` is advisory metadata. v1.0.0 does not introduce a separate weight key field.

## Categories

Categories use `key` as the stable identifier.

Examples include `flower`, `pre_roll`, `vape`, `concentrate`, and `edible`.

## Product Types

Product types use `key` as the stable identifier.

Examples include `infused_pre_roll`, `vape_cartridge`, `vape_disposable`, `live_rosin`, and `gummy`.

Each product type references a canonical category key through `category`.

## Units

General units use `key` as the stable identifier.

The canonical percent unit key is `percent`. The symbol `%` is a label or display concern, not the v1.0.0 canonical unit key.

Examples include `g`, `mg`, `oz`, `percent`, `mg_per_g`, `mg_per_serving`, `mg_per_package`, `each`, and `ml`.

## Potency Fields

Potency fields use `key` as the stable identifier.

Examples include `thc`, `thca`, `total_thc`, `cbd`, `cbda`, `total_cbd`, `total_cannabinoids`, and `total_terpenes`.

## Potency Units

Potency units use `key` as the stable identifier.

Examples include `percent`, `mg_per_g`, `mg_per_package`, `mg_per_serving`, `mg`, and `mg_per_ml`.

## Package-Size Tokens

Package sizes use string tokens scoped by category.

Examples:

- flower package size: `3.5g`
- vape package size: `0.5g`
- edible package size: `100mg_package`
- edible serving size: `10mg_serving`

Package-size tokens may represent mass, serving potency, or package potency depending on category context. Gram-based package-size tokens must also exist as canonical weight labels. Non-gram tokens are valid package-size tokens when the category context supports them.

## CLI Names And Export Keys

CLI command names are user-facing and hyphenated:

- `product-type`
- `package-size`
- `potency-field`
- `package-sizes`
- `potency-fields`
- `potency-units`

Export bundle keys are machine-facing and snake_case:

- `product_types`
- `package_sizes`
- `potency_fields`
- `potency_units`
- `weight_aliases`
- `category_aliases`
- `product_type_aliases`
- `package_size_aliases`
- `potency_field_aliases`

Package-size commands keep their category argument because package sizes are category-aware:

```bash
cargo run -- list package-sizes flower
cargo run -- normalize package-size flower eighth
```

State package-size commands keep both state and category because state extensions are state-aware and category-aware:

```bash
cargo run -- state NV package-sizes flower
```

## Deprecation Policy

Future stable-key changes should be explicit and conservative:

- Do not silently change the meaning of an existing key.
- Prefer adding aliases when raw input changes but canonical meaning remains the same.
- Prefer additive registry entries when a new canonical concept is needed.
- If a canonical key must be replaced, document the old key, new key, reason, and migration guidance in the relevant docs and release notes.
