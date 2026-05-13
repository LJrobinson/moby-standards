use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::fs;

use crate::models::{
    AliasRegistry, CategoryRegistry, ProductTypeRegistry, UnitRegistry, WeightRegistry,
};

pub struct Registry {
    pub weights: WeightRegistry,
    pub categories: CategoryRegistry,
    pub units: UnitRegistry,
    pub product_types: ProductTypeRegistry,
    pub weight_aliases: AliasRegistry,
    pub category_aliases: AliasRegistry,
    pub product_type_aliases: AliasRegistry,
}

impl Registry {
    pub fn load() -> Result<Self> {
        Ok(Self {
            weights: load_yaml("data/standards/weights.yaml")?,
            categories: load_yaml("data/standards/categories.yaml")?,
            units: load_yaml("data/standards/units.yaml")?,
            product_types: load_yaml("data/standards/product-types.yaml")?,
            weight_aliases: load_yaml("data/aliases/weights.yaml")?,
            category_aliases: load_yaml("data/aliases/categories.yaml")?,
            product_type_aliases: load_yaml("data/aliases/product-types.yaml")?,
        })
    }

    pub fn validate(&self) -> Result<()> {
        let mut category_keys = HashSet::new();
        for category in &self.categories.categories {
            if category.key.trim().is_empty() {
                anyhow::bail!("Category has empty key");
            }
            if category.label.trim().is_empty() {
                anyhow::bail!("Category '{}' has empty label", category.key);
            }
            if category.description.trim().is_empty() {
                anyhow::bail!("Category '{}' has empty description", category.key);
            }
            if !category_keys.insert(category.key.as_str()) {
                anyhow::bail!("Duplicate category key '{}'", category.key);
            }
        }

        let mut weight_labels = HashSet::new();
        for weight in &self.weights.weights {
            if weight.label.trim().is_empty() {
                anyhow::bail!("Weight has empty label");
            }
            if !weight.grams.is_finite() || weight.grams <= 0.0 {
                anyhow::bail!(
                    "Weight '{}' has invalid gram value '{}'",
                    weight.label,
                    weight.grams
                );
            }
            if !weight_labels.insert(weight.label.as_str()) {
                anyhow::bail!("Duplicate weight label '{}'", weight.label);
            }
            for category_hint in &weight.category_hint {
                if !category_keys.contains(category_hint.as_str()) {
                    anyhow::bail!(
                        "Weight '{}' has unknown category hint '{}'",
                        weight.label,
                        category_hint
                    );
                }
            }
        }

        let mut unit_keys = HashSet::new();
        for unit in &self.units.units {
            if unit.key.trim().is_empty() {
                anyhow::bail!("Unit has empty key");
            }
            if unit.label.trim().is_empty() {
                anyhow::bail!("Unit '{}' has empty label", unit.key);
            }
            if unit.dimension.trim().is_empty() {
                anyhow::bail!("Unit '{}' has empty dimension", unit.key);
            }
            if !unit_keys.insert(unit.key.as_str()) {
                anyhow::bail!("Duplicate unit key '{}'", unit.key);
            }
        }

        let mut product_type_keys = HashSet::new();
        for product_type in &self.product_types.product_types {
            if product_type.key.trim().is_empty() {
                anyhow::bail!("Product type has empty key");
            }
            if product_type.category.trim().is_empty() {
                anyhow::bail!("Product type '{}' has empty category", product_type.key);
            }
            if product_type.label.trim().is_empty() {
                anyhow::bail!("Product type '{}' has empty label", product_type.key);
            }
            if !product_type_keys.insert(product_type.key.as_str()) {
                anyhow::bail!("Duplicate product type key '{}'", product_type.key);
            }
            if !category_keys.contains(product_type.category.as_str()) {
                anyhow::bail!(
                    "Product type '{}' points to missing category '{}'",
                    product_type.key,
                    product_type.category
                );
            }
        }

        let mut weight_alias_inputs = HashSet::new();
        for alias in &self.weight_aliases.aliases {
            if alias.input.trim().is_empty() {
                anyhow::bail!("Weight alias has empty input");
            }
            if alias.canonical.trim().is_empty() {
                anyhow::bail!("Weight alias '{}' has empty canonical value", alias.input);
            }
            if alias.confidence.trim().is_empty() {
                anyhow::bail!("Weight alias '{}' has empty confidence", alias.input);
            }
            if !weight_alias_inputs.insert(alias.input.as_str()) {
                anyhow::bail!("Duplicate weight alias input '{}'", alias.input);
            }
            if !weight_labels.contains(alias.canonical.as_str()) {
                anyhow::bail!(
                    "Weight alias '{}' points to missing canonical weight '{}'",
                    alias.input,
                    alias.canonical
                );
            }
        }

        let mut category_alias_inputs = HashSet::new();
        for alias in &self.category_aliases.aliases {
            if alias.input.trim().is_empty() {
                anyhow::bail!("Category alias has empty input");
            }
            if alias.canonical.trim().is_empty() {
                anyhow::bail!("Category alias '{}' has empty canonical value", alias.input);
            }
            if alias.confidence.trim().is_empty() {
                anyhow::bail!("Category alias '{}' has empty confidence", alias.input);
            }
            if !category_alias_inputs.insert(alias.input.as_str()) {
                anyhow::bail!("Duplicate category alias input '{}'", alias.input);
            }
            if !category_keys.contains(alias.canonical.as_str()) {
                anyhow::bail!(
                    "Category alias '{}' points to missing canonical category '{}'",
                    alias.input,
                    alias.canonical
                );
            }
        }

        let mut product_type_alias_inputs = HashSet::new();
        for alias in &self.product_type_aliases.aliases {
            if alias.input.trim().is_empty() {
                anyhow::bail!("Product type alias has empty input");
            }
            if alias.canonical.trim().is_empty() {
                anyhow::bail!(
                    "Product type alias '{}' has empty canonical value",
                    alias.input
                );
            }
            if alias.confidence.trim().is_empty() {
                anyhow::bail!("Product type alias '{}' has empty confidence", alias.input);
            }
            if !product_type_alias_inputs.insert(alias.input.as_str()) {
                anyhow::bail!("Duplicate product type alias input '{}'", alias.input);
            }
            if !product_type_keys.contains(alias.canonical.as_str()) {
                anyhow::bail!(
                    "Product type alias '{}' points to missing canonical product type '{}'",
                    alias.input,
                    alias.canonical
                );
            }
        }

        Ok(())
    }
}

fn load_yaml<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let raw = fs::read_to_string(path).with_context(|| format!("Failed to read {}", path))?;
    serde_yaml::from_str(&raw).with_context(|| format!("Failed to parse {}", path))
}
