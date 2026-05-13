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

pub fn normalize_package_size(registry: &Registry, category: &str, input: &str) -> NormalizeResult {
    let Some(category_sizes) = registry.package_sizes.package_sizes.get(category) else {
        return unmatched("package-size", input);
    };

    let normalized_input = normalize_text(input);
    let matched_alias = registry
        .package_size_aliases
        .aliases
        .iter()
        .find(|alias| normalize_text(&alias.input) == normalized_input);

    let matched = match matched_alias {
        Some(alias) => Some((
            alias.canonical.clone(),
            alias.confidence.clone(),
            alias.source.clone(),
        )),
        None => category_sizes
            .iter()
            .find(|size| normalize_text(size) == normalized_input)
            .map(|size| (size.clone(), "high".to_string(), None)),
    };

    let Some((canonical, confidence, source)) = matched else {
        return unmatched("package-size", input);
    };

    if category_sizes.iter().any(|size| size == &canonical) {
        NormalizeResult {
            input: input.to_string(),
            kind: "package-size".to_string(),
            canonical: Some(canonical),
            confidence: Some(confidence),
            source,
            matched: true,
        }
    } else {
        unmatched("package-size", input)
    }
}

pub fn normalize_potency_field(registry: &Registry, input: &str) -> NormalizeResult {
    normalize_alias(
        "potency-field",
        &registry.potency_field_aliases.aliases,
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
            source: alias.source.clone(),
            matched: true,
        },
        None => unmatched(kind, input),
    }
}

fn unmatched(kind: &str, input: &str) -> NormalizeResult {
    NormalizeResult {
        input: input.to_string(),
        kind: kind.to_string(),
        canonical: None,
        confidence: None,
        source: None,
        matched: false,
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
