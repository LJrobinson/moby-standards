use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

use crate::models::{
    AliasRegistry, CategoryRegistry, PackageSizeRegistry, PotencyFieldRegistry,
    PotencyUnitRegistry, ProductTypeRegistry, StatePackageSizeOverride, UnitRegistry,
    WeightRegistry,
};

pub struct Registry {
    pub weights: WeightRegistry,
    pub categories: CategoryRegistry,
    pub units: UnitRegistry,
    pub product_types: ProductTypeRegistry,
    pub package_sizes: PackageSizeRegistry,
    pub potency_fields: PotencyFieldRegistry,
    pub potency_units: PotencyUnitRegistry,
    pub state_package_sizes: BTreeMap<String, StatePackageSizeOverride>,
    pub weight_aliases: AliasRegistry,
    pub category_aliases: AliasRegistry,
    pub product_type_aliases: AliasRegistry,
    pub package_size_aliases: AliasRegistry,
    pub potency_field_aliases: AliasRegistry,
}

impl Registry {
    pub fn load() -> Result<Self> {
        Ok(Self {
            weights: load_yaml("data/standards/weights.yaml")?,
            categories: load_yaml("data/standards/categories.yaml")?,
            units: load_yaml("data/standards/units.yaml")?,
            product_types: load_yaml("data/standards/product-types.yaml")?,
            package_sizes: load_yaml("data/standards/package-sizes.yaml")?,
            potency_fields: load_yaml("data/standards/potency-fields.yaml")?,
            potency_units: load_yaml("data/standards/potency-units.yaml")?,
            state_package_sizes: load_state_package_size_overrides("data/states")?,
            weight_aliases: load_yaml("data/aliases/weights.yaml")?,
            category_aliases: load_yaml("data/aliases/categories.yaml")?,
            product_type_aliases: load_yaml("data/aliases/product-types.yaml")?,
            package_size_aliases: load_yaml("data/aliases/package-sizes.yaml")?,
            potency_field_aliases: load_yaml("data/aliases/potency-fields.yaml")?,
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

        let mut package_size_tokens = HashSet::new();
        for (category, sizes) in &self.package_sizes.package_sizes {
            if category.trim().is_empty() {
                anyhow::bail!("Package size category has empty key");
            }
            if !category_keys.contains(category.as_str()) {
                anyhow::bail!(
                    "Package size category '{}' is not a canonical category",
                    category
                );
            }

            let mut category_sizes = HashSet::new();
            for size in sizes {
                if size.trim().is_empty() {
                    anyhow::bail!("Package size category '{}' has empty size", category);
                }
                if !category_sizes.insert(size.as_str()) {
                    anyhow::bail!(
                        "Duplicate package size '{}' in category '{}'",
                        size,
                        category
                    );
                }
                if is_gram_size_token(size) && !weight_labels.contains(size.as_str()) {
                    anyhow::bail!(
                        "Package size '{}' in category '{}' is not a canonical weight",
                        size,
                        category
                    );
                }
                if !is_gram_size_token(size) && category != "edible" {
                    anyhow::bail!(
                        "Non-gram package size '{}' is only supported for edible packages",
                        size
                    );
                }
                package_size_tokens.insert(size.as_str());
            }
        }

        let mut package_size_alias_inputs = HashSet::new();
        for alias in &self.package_size_aliases.aliases {
            if alias.input.trim().is_empty() {
                anyhow::bail!("Package size alias has empty input");
            }
            if alias.canonical.trim().is_empty() {
                anyhow::bail!(
                    "Package size alias '{}' has empty canonical value",
                    alias.input
                );
            }
            if alias.confidence.trim().is_empty() {
                anyhow::bail!("Package size alias '{}' has empty confidence", alias.input);
            }
            if !package_size_alias_inputs.insert(alias.input.as_str()) {
                anyhow::bail!("Duplicate package size alias input '{}'", alias.input);
            }
            if !package_size_tokens.contains(alias.canonical.as_str()) {
                anyhow::bail!(
                    "Package size alias '{}' points to missing package size '{}'",
                    alias.input,
                    alias.canonical
                );
            }
        }

        let mut potency_field_keys = HashSet::new();
        for potency_field in &self.potency_fields.potency_fields {
            if potency_field.key.trim().is_empty() {
                anyhow::bail!("Potency field has empty key");
            }
            if potency_field.label.trim().is_empty() {
                anyhow::bail!("Potency field '{}' has empty label", potency_field.key);
            }
            if !potency_field_keys.insert(potency_field.key.as_str()) {
                anyhow::bail!("Duplicate potency field key '{}'", potency_field.key);
            }
        }

        let mut potency_unit_keys = HashSet::new();
        for potency_unit in &self.potency_units.potency_units {
            if potency_unit.key.trim().is_empty() {
                anyhow::bail!("Potency unit has empty key");
            }
            if potency_unit.label.trim().is_empty() {
                anyhow::bail!("Potency unit '{}' has empty label", potency_unit.key);
            }
            if !potency_unit_keys.insert(potency_unit.key.as_str()) {
                anyhow::bail!("Duplicate potency unit key '{}'", potency_unit.key);
            }
        }

        let mut potency_field_alias_inputs = HashSet::new();
        for alias in &self.potency_field_aliases.aliases {
            if alias.input.trim().is_empty() {
                anyhow::bail!("Potency field alias has empty input");
            }
            if alias.canonical.trim().is_empty() {
                anyhow::bail!(
                    "Potency field alias '{}' has empty canonical value",
                    alias.input
                );
            }
            if alias.confidence.trim().is_empty() {
                anyhow::bail!("Potency field alias '{}' has empty confidence", alias.input);
            }
            if !potency_field_alias_inputs.insert(alias.input.as_str()) {
                anyhow::bail!("Duplicate potency field alias input '{}'", alias.input);
            }
            if !potency_field_keys.contains(alias.canonical.as_str()) {
                anyhow::bail!(
                    "Potency field alias '{}' points to missing canonical potency field '{}'",
                    alias.input,
                    alias.canonical
                );
            }
        }

        for (state, override_data) in &self.state_package_sizes {
            if state.trim().is_empty() {
                anyhow::bail!("State package-size override has empty state code");
            }
            if override_data.state.trim().is_empty() {
                anyhow::bail!("State package-size override has missing state field");
            }

            for (category, state_category) in &override_data.categories {
                if category.trim().is_empty() {
                    anyhow::bail!("State '{}' has empty package-size category", state);
                }
                if !category_keys.contains(category.as_str()) {
                    anyhow::bail!(
                        "State '{}' package-size category '{}' is not canonical",
                        state,
                        category
                    );
                }
                if state_category.package_context.trim().is_empty() {
                    anyhow::bail!(
                        "State '{}' category '{}' has empty package context",
                        state,
                        category
                    );
                }
                if state_category.source_confidence.trim().is_empty() {
                    anyhow::bail!(
                        "State '{}' category '{}' has empty source confidence",
                        state,
                        category
                    );
                }
                if state_category.source_note.trim().is_empty() {
                    anyhow::bail!(
                        "State '{}' category '{}' has empty source note",
                        state,
                        category
                    );
                }

                let mut recognized_weights = HashSet::new();
                for weight in &state_category.recognized_weights {
                    if weight.trim().is_empty() {
                        anyhow::bail!(
                            "State '{}' category '{}' has empty recognized weight",
                            state,
                            category
                        );
                    }
                    if !recognized_weights.insert(weight.as_str()) {
                        anyhow::bail!(
                            "State '{}' category '{}' has duplicate recognized weight '{}'",
                            state,
                            category,
                            weight
                        );
                    }
                    if !weight_labels.contains(weight.as_str()) {
                        anyhow::bail!(
                            "State '{}' category '{}' recognized weight '{}' is not canonical",
                            state,
                            category,
                            weight
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

fn is_gram_size_token(value: &str) -> bool {
    value
        .strip_suffix('g')
        .and_then(|number| number.parse::<f64>().ok())
        .is_some()
}

fn load_yaml<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    load_yaml_path(Path::new(path))
}

fn load_yaml_path<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    let raw =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    serde_yaml::from_str(&raw).with_context(|| format!("Failed to parse {}", path.display()))
}

fn load_state_package_size_overrides(
    root: &str,
) -> Result<BTreeMap<String, StatePackageSizeOverride>> {
    let mut overrides = BTreeMap::new();
    let root_path = Path::new(root);

    if !root_path.exists() {
        return Ok(overrides);
    }

    for entry in fs::read_dir(root_path).with_context(|| format!("Failed to read {}", root))? {
        let entry = entry.with_context(|| format!("Failed to read entry in {}", root))?;
        if !entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", entry.path().display()))?
            .is_dir()
        {
            continue;
        }

        let path = entry.path().join("package-sizes.yaml");
        if !path.exists() {
            continue;
        }

        let override_data: StatePackageSizeOverride = load_yaml_path(&path)?;
        let state = override_data.state.to_uppercase();

        if overrides.insert(state.clone(), override_data).is_some() {
            anyhow::bail!("Duplicate state package-size override for '{}'", state);
        }
    }

    Ok(overrides)
}
