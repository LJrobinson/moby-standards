pub fn json_schema() -> &'static str {
    r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://moby.dev/standards/moby-standards.schema.json",
  "title": "MOBY Standards Bundle",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "weights",
    "categories",
    "units",
    "product_types",
    "package_sizes",
    "potency_fields",
    "potency_units",
    "state_package_sizes",
    "weight_aliases",
    "category_aliases",
    "product_type_aliases",
    "package_size_aliases",
    "potency_field_aliases"
  ],
  "properties": {
    "weights": {
      "type": "array",
      "items": { "$ref": "#/$defs/StandardWeight" }
    },
    "categories": {
      "type": "array",
      "items": { "$ref": "#/$defs/StandardCategory" }
    },
    "units": {
      "type": "array",
      "items": { "$ref": "#/$defs/StandardUnit" }
    },
    "product_types": {
      "type": "array",
      "items": { "$ref": "#/$defs/StandardProductType" }
    },
    "package_sizes": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": { "type": "string" }
      }
    },
    "potency_fields": {
      "type": "array",
      "items": { "$ref": "#/$defs/StandardPotencyField" }
    },
    "potency_units": {
      "type": "array",
      "items": { "$ref": "#/$defs/StandardPotencyUnit" }
    },
    "state_package_sizes": {
      "type": "object",
      "additionalProperties": { "$ref": "#/$defs/StatePackageSizeOverride" }
    },
    "weight_aliases": {
      "type": "array",
      "items": { "$ref": "#/$defs/AliasEntry" }
    },
    "category_aliases": {
      "type": "array",
      "items": { "$ref": "#/$defs/AliasEntry" }
    },
    "product_type_aliases": {
      "type": "array",
      "items": { "$ref": "#/$defs/AliasEntry" }
    },
    "package_size_aliases": {
      "type": "array",
      "items": { "$ref": "#/$defs/AliasEntry" }
    },
    "potency_field_aliases": {
      "type": "array",
      "items": { "$ref": "#/$defs/AliasEntry" }
    }
  },
  "$defs": {
    "StandardWeight": {
      "type": "object",
      "additionalProperties": false,
      "required": ["label", "grams", "category_hint"],
      "properties": {
        "label": { "type": "string" },
        "grams": { "type": "number" },
        "category_hint": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "StandardCategory": {
      "type": "object",
      "additionalProperties": false,
      "required": ["key", "label", "description"],
      "properties": {
        "key": { "type": "string" },
        "label": { "type": "string" },
        "description": { "type": "string" }
      }
    },
    "StandardUnit": {
      "type": "object",
      "additionalProperties": false,
      "required": ["key", "label", "dimension"],
      "properties": {
        "key": { "type": "string" },
        "label": { "type": "string" },
        "dimension": { "type": "string" }
      }
    },
    "StandardProductType": {
      "type": "object",
      "additionalProperties": false,
      "required": ["key", "category", "label"],
      "properties": {
        "key": { "type": "string" },
        "category": { "type": "string" },
        "label": { "type": "string" }
      }
    },
    "StandardPotencyField": {
      "type": "object",
      "additionalProperties": false,
      "required": ["key", "label"],
      "properties": {
        "key": { "type": "string" },
        "label": { "type": "string" }
      }
    },
    "StandardPotencyUnit": {
      "type": "object",
      "additionalProperties": false,
      "required": ["key", "label"],
      "properties": {
        "key": { "type": "string" },
        "label": { "type": "string" }
      }
    },
    "AliasEntry": {
      "type": "object",
      "additionalProperties": false,
      "required": ["input", "canonical", "confidence"],
      "properties": {
        "input": { "type": "string" },
        "canonical": { "type": "string" },
        "confidence": { "type": "string" },
        "source": { "$ref": "#/$defs/AliasSource" }
      }
    },
    "AliasSource": {
      "type": "object",
      "additionalProperties": false,
      "required": ["type"],
      "properties": {
        "type": { "type": "string" },
        "state": { "type": "string" },
        "authority": { "type": "string" },
        "note": { "type": "string" }
      }
    },
    "StatePackageSizeOverride": {
      "type": "object",
      "additionalProperties": false,
      "required": ["state", "categories"],
      "properties": {
        "state": { "type": "string" },
        "categories": {
          "type": "object",
          "additionalProperties": { "$ref": "#/$defs/StatePackageSizeCategory" }
        }
      }
    },
    "StatePackageSizeCategory": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "package_context",
        "recognized_weights",
        "source_confidence",
        "source_note"
      ],
      "properties": {
        "package_context": { "type": "string" },
        "recognized_weights": {
          "type": "array",
          "items": { "type": "string" }
        },
        "source_confidence": { "type": "string" },
        "source_note": { "type": "string" }
      }
    },
    "NormalizeResult": {
      "type": "object",
      "additionalProperties": false,
      "required": ["input", "kind", "canonical", "confidence", "matched"],
      "properties": {
        "input": { "type": "string" },
        "kind": { "type": "string" },
        "canonical": {
          "type": ["string", "null"]
        },
        "confidence": {
          "type": ["string", "null"]
        },
        "source": { "$ref": "#/$defs/AliasSource" },
        "matched": { "type": "boolean" }
      }
    }
  }
}"##
}

pub fn typescript_definitions() -> &'static str {
    r#"export type Confidence = string;

export interface StandardWeight {
  label: string;
  grams: number;
  category_hint: string[];
}

export interface StandardCategory {
  key: string;
  label: string;
  description: string;
}

export interface StandardUnit {
  key: string;
  label: string;
  dimension: string;
}

export interface StandardProductType {
  key: string;
  category: string;
  label: string;
}

export interface StandardPotencyField {
  key: string;
  label: string;
}

export interface StandardPotencyUnit {
  key: string;
  label: string;
}

export interface AliasSource {
  type: string;
  state?: string;
  authority?: string;
  note?: string;
}

export interface AliasEntry {
  input: string;
  canonical: string;
  confidence: Confidence;
  source?: AliasSource;
}

export interface NormalizeResult {
  input: string;
  kind: string;
  canonical: string | null;
  confidence: Confidence | null;
  source?: AliasSource;
  matched: boolean;
}

export interface StatePackageSizeCategory {
  package_context: string;
  recognized_weights: string[];
  source_confidence: string;
  source_note: string;
}

export interface StatePackageSizeOverride {
  state: string;
  categories: Record<string, StatePackageSizeCategory>;
}

export interface StandardsBundle {
  weights: StandardWeight[];
  categories: StandardCategory[];
  units: StandardUnit[];
  product_types: StandardProductType[];
  package_sizes: Record<string, string[]>;
  potency_fields: StandardPotencyField[];
  potency_units: StandardPotencyUnit[];
  state_package_sizes: Record<string, StatePackageSizeOverride>;
  weight_aliases: AliasEntry[];
  category_aliases: AliasEntry[];
  product_type_aliases: AliasEntry[];
  package_size_aliases: AliasEntry[];
  potency_field_aliases: AliasEntry[];
}
"#
}
