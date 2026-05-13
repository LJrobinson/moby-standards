use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeightRegistry {
    pub weights: Vec<StandardWeight>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StandardWeight {
    pub label: String,
    pub grams: f64,
    #[serde(default)]
    pub category_hint: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryRegistry {
    pub categories: Vec<StandardCategory>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StandardCategory {
    pub key: String,
    pub label: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnitRegistry {
    pub units: Vec<StandardUnit>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StandardUnit {
    pub key: String,
    pub label: String,
    pub dimension: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductTypeRegistry {
    pub product_types: Vec<StandardProductType>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PackageSizeRegistry {
    pub package_sizes: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PotencyFieldRegistry {
    pub potency_fields: Vec<StandardPotencyField>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StandardPotencyField {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PotencyUnitRegistry {
    pub potency_units: Vec<StandardPotencyUnit>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StandardPotencyUnit {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatePackageSizeOverride {
    pub state: String,
    pub categories: BTreeMap<String, StatePackageSizeCategory>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatePackageSizeCategory {
    pub package_context: String,
    pub recognized_weights: Vec<String>,
    pub source_confidence: String,
    pub source_note: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StandardProductType {
    pub key: String,
    pub category: String,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AliasRegistry {
    pub aliases: Vec<AliasEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AliasEntry {
    pub input: String,
    pub canonical: String,
    pub confidence: String,
}

#[derive(Debug, Serialize)]
pub struct NormalizeResult {
    pub input: String,
    pub kind: String,
    pub canonical: Option<String>,
    pub confidence: Option<String>,
    pub matched: bool,
}
