use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
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
        })
    }

    pub fn validate(&self) -> Result<()> {
        for alias in &self.weight_aliases.aliases {
            let exists = self
                .weights
                .weights
                .iter()
                .any(|weight| weight.label == alias.canonical);

            if !exists {
                anyhow::bail!(
                    "Weight alias '{}' points to missing canonical weight '{}'",
                    alias.input,
                    alias.canonical
                );
            }
        }

        for alias in &self.category_aliases.aliases {
            let exists = self
                .categories
                .categories
                .iter()
                .any(|category| category.key == alias.canonical);

            if !exists {
                anyhow::bail!(
                    "Category alias '{}' points to missing canonical category '{}'",
                    alias.input,
                    alias.canonical
                );
            }
        }

        for product_type in &self.product_types.product_types {
            let exists = self
                .categories
                .categories
                .iter()
                .any(|category| category.key == product_type.category);

            if !exists {
                anyhow::bail!(
                    "Product type '{}' points to missing category '{}'",
                    product_type.key,
                    product_type.category
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
