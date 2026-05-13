# Output Contracts

MOBY Standards v1.0.0 exposes stable stdout contracts for CLI consumers and stable data shapes for machine-readable exports.

## NormalizeResult

`cargo run -- normalize ...` prints a JSON `NormalizeResult`.

Matched result:

```json
{
  "input": "eighth",
  "kind": "weight",
  "canonical": "3.5g",
  "confidence": "high",
  "matched": true
}
```

Unmatched result:

```json
{
  "input": "unknown input",
  "kind": "weight",
  "canonical": null,
  "confidence": null,
  "matched": false
}
```

Fields:

- `input`: original input string.
- `kind`: hyphenated CLI kind, such as `weight`, `category`, `product-type`, `package-size`, or `potency-field`.
- `canonical`: canonical key, weight label, or package-size token when matched; otherwise `null`.
- `confidence`: alias confidence when matched; otherwise `null`.
- `source`: optional alias source metadata, emitted only when the matched alias provides it.
- `matched`: `true` when normalization found a valid canonical value.

## AliasEntry

Alias files use this shape:

```yaml
input: "eighth"
canonical: "3.5g"
confidence: "high"
source:
  type: "retail_common_term"
  note: "Common cannabis retail shorthand."
```

Required fields:

- `input`
- `canonical`
- `confidence`

Optional field:

- `source`

Existing aliases without `source` remain valid.

## Source Metadata

Alias source metadata uses this shape:

```yaml
source:
  type: "state_term"
  state: "NV"
  authority: "Nevada CCB"
  note: "Use only when reconciled with MOBY Atlas."
```

Rules:

- `type` is required when `source` is present and must not be empty.
- `state` is optional.
- `authority` is optional.
- `note` is optional, but must not be empty if provided.

## StandardsBundle

`cargo run -- export-json` prints the loaded standards bundle as JSON.

Top-level keys:

```json
{
  "weights": [],
  "categories": [],
  "units": [],
  "product_types": [],
  "package_sizes": {},
  "potency_fields": [],
  "potency_units": [],
  "state_package_sizes": {},
  "weight_aliases": [],
  "category_aliases": [],
  "product_type_aliases": [],
  "package_size_aliases": [],
  "potency_field_aliases": []
}
```

Export bundle keys are snake_case. CLI command names remain hyphenated where they are user-facing.

## Package-Size Registry

Package sizes are category-aware:

```yaml
package_sizes:
  flower:
    - 1g
    - 3.5g
  edible:
    - 10mg_serving
    - 100mg_package
```

Package-size values are string tokens. A token may represent a mass package, serving potency, or package potency depending on category context.

## State Package-Size Extension

State package-size extensions live under `data/states/<STATE>/package-sizes.yaml`.

```yaml
state: NV
categories:
  flower:
    package_context: prepacked
    recognized_weights:
      - 1g
      - 3.5g
    source_confidence: medium
    source_note: "Starter state-aware example. Should be reconciled with MOBY Atlas source-cited state data before being treated as official."
```

State overrides are extension examples until reconciled with MOBY Atlas source-cited state truth. They are not legal claims.

## Export Schema

`cargo run -- export-schema` prints a static JSON Schema for the standards bundle to stdout.

The schema describes:

- registry entry shapes
- alias and source metadata shapes
- state package-size extension shape
- the top-level `StandardsBundle` keys

Consumers can redirect stdout when they want a file:

```bash
cargo run -- export-schema > moby-standards.schema.json
```

## Export TypeScript

`cargo run -- export-typescript` prints TypeScript definitions to stdout.

The export includes interfaces and types for:

- `StandardWeight`
- `StandardCategory`
- `StandardUnit`
- `StandardProductType`
- `AliasEntry`
- `NormalizeResult`
- `StandardsBundle`

Consumers can redirect stdout when they want a file:

```bash
cargo run -- export-typescript > moby-standards.types.ts
```
