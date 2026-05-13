# MOBY Standards Progress Log

## v0.1.0 - Framework

Initialized MOBY Standards as a Rust CLI and YAML-backed canonical cannabis reference project.

Implemented:

- canonical weights
- canonical categories
- canonical units
- starter product types
- weight aliases
- category aliases
- list commands
- normalize commands
- validation command
- JSON export command
- CLI tests

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
