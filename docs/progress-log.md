# MOBY Standards Progress Log

## v0.3.0 - Package Size Standards

Status: complete.

Implemented:

- category-aware package-size registry
- package-size aliases
- package-size listing by category
- package-size normalization by category
- validation for package-size categories and gram-based package sizes
- CLI tests for package-size listing, normalization, and category-aware matching

## v0.2.0 - Product Type Aliases

Status: complete.

Implemented:

- product-type aliases
- product-type normalization
- validation that product-type aliases point to canonical product types
- CLI tests for product-type normalization and unmatched product-type input

## v0.1.0 - Framework

Status: complete.

Implemented:

- Rust CLI initialized
- YAML-backed standards
- Weight normalization
- Category normalization
- Validation command
- JSON export command

Supporting framework pieces:

- canonical weights, categories, units, and starter product types
- weight and category aliases
- list commands for loaded registries
- CLI tests for validation and normalization behavior

## Design Notes

MOBY Standards is not intended to be a POS, compliance engine, or state law database.

It is the canonical vocabulary layer for cannabis data.

Examples:

- `eighth` -> `3.5g`
- `cart` -> `vape`
- `usable cannabis` -> `flower`
- `joint` -> `pre_roll`

## Relationship to MOBY Atlas

MOBY Atlas tracks state-specific reference data.

MOBY Standards defines normalized cannabis language.

Future state-aware standards may allow queries like:

- Which flower package weights are recognized in Nevada?
- Which category names does Oregon use?
- How does Massachusetts define product limits?
