use crate::models::{AliasEntry, NormalizeResult};
use crate::registry::Registry;

pub fn normalize_weight(registry: &Registry, input: &str) -> NormalizeResult {
    normalize_alias("weight", &registry.weight_aliases.aliases, input)
}

pub fn normalize_category(registry: &Registry, input: &str) -> NormalizeResult {
    normalize_alias("category", &registry.category_aliases.aliases, input)
}

pub fn normalize_product_type(registry: &Registry, input: &str) -> NormalizeResult {
    normalize_alias(
        "product-type",
        &registry.product_type_aliases.aliases,
        input,
    )
}

fn normalize_alias(kind: &str, aliases: &[AliasEntry], input: &str) -> NormalizeResult {
    let normalized_input = normalize_text(input);

    let matched_alias = aliases
        .iter()
        .find(|alias| normalize_text(&alias.input) == normalized_input);

    match matched_alias {
        Some(alias) => NormalizeResult {
            input: input.to_string(),
            kind: kind.to_string(),
            canonical: Some(alias.canonical.clone()),
            confidence: Some(alias.confidence.clone()),
            matched: true,
        },
        None => NormalizeResult {
            input: input.to_string(),
            kind: kind.to_string(),
            canonical: None,
            confidence: None,
            matched: false,
        },
    }
}

fn normalize_text(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace('-', " ")
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
