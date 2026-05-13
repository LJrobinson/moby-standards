mod models;
mod normalize;
mod registry;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use normalize::{
    normalize_category, normalize_package_size, normalize_product_type, normalize_weight,
};
use registry::Registry;

#[derive(Parser)]
#[command(name = "moby-standards")]
#[command(about = "Canonical YAML-backed cannabis standards for MOBY")]
#[command(
    long_about = "Canonical YAML-backed cannabis standards for MOBY.\n\nSupports listing registries, normalizing weights, categories, product types, and category-aware package sizes, validating YAML data, and exporting loaded standards as JSON."
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List canonical standards from the loaded YAML registries.
    List {
        /// Registry to list.
        #[arg(value_enum)]
        kind: ListKind,

        /// Category key for category-aware registries.
        category: Option<String>,
    },

    /// Normalize a value into MOBY standards.
    Normalize {
        /// Value kind to normalize.
        #[arg(value_enum)]
        kind: NormalizeKind,

        /// Raw input, or category key when normalizing package sizes.
        #[arg(value_name = "CATEGORY_OR_INPUT")]
        category_or_input: String,

        /// Raw package size input when kind is package-size.
        #[arg(value_name = "INPUT")]
        input: Option<String>,
    },

    /// Validate YAML standards and aliases for internal consistency.
    Validate,

    /// Export all loaded standards and aliases as JSON.
    ExportJson,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ListKind {
    Weights,
    Categories,
    Units,
    ProductTypes,
    PackageSizes,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum NormalizeKind {
    Weight,
    Category,
    ProductType,
    PackageSize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let registry = Registry::load()?;

    match cli.command {
        Commands::List { kind, category } => match kind {
            ListKind::Weights => {
                reject_category_argument(category)?;
                for weight in registry.weights.weights {
                    println!("{} = {}g", weight.label, weight.grams);
                }
            }
            ListKind::Categories => {
                reject_category_argument(category)?;
                for category in registry.categories.categories {
                    println!("{} - {}", category.key, category.label);
                }
            }
            ListKind::Units => {
                reject_category_argument(category)?;
                for unit in registry.units.units {
                    println!("{} - {} ({})", unit.key, unit.label, unit.dimension);
                }
            }
            ListKind::ProductTypes => {
                reject_category_argument(category)?;
                for product_type in registry.product_types.product_types {
                    println!(
                        "{} - {} [{}]",
                        product_type.key, product_type.label, product_type.category
                    );
                }
            }
            ListKind::PackageSizes => {
                let Some(category) = category else {
                    anyhow::bail!("list package-sizes requires a category key");
                };
                let Some(sizes) = registry.package_sizes.package_sizes.get(&category) else {
                    anyhow::bail!("Unknown package size category '{}'", category);
                };
                for size in sizes {
                    println!("{}", size);
                }
            }
        },

        Commands::Normalize {
            kind,
            category_or_input,
            input,
        } => {
            let result = match kind {
                NormalizeKind::Weight => {
                    reject_package_size_input(input)?;
                    normalize_weight(&registry, &category_or_input)
                }
                NormalizeKind::Category => {
                    reject_package_size_input(input)?;
                    normalize_category(&registry, &category_or_input)
                }
                NormalizeKind::ProductType => {
                    reject_package_size_input(input)?;
                    normalize_product_type(&registry, &category_or_input)
                }
                NormalizeKind::PackageSize => {
                    let Some(input) = input else {
                        anyhow::bail!("normalize package-size requires <CATEGORY> <INPUT>");
                    };
                    normalize_package_size(&registry, &category_or_input, &input)
                }
            };

            println!("{}", serde_json::to_string_pretty(&result)?);
        }

        Commands::Validate => {
            registry.validate()?;
            println!("OK: MOBY standards data is valid.");
        }

        Commands::ExportJson => {
            let export = serde_json::json!({
                "weights": registry.weights.weights,
                "categories": registry.categories.categories,
                "units": registry.units.units,
                "product_types": registry.product_types.product_types,
                "package_sizes": registry.package_sizes.package_sizes,
                "weight_aliases": registry.weight_aliases.aliases,
                "category_aliases": registry.category_aliases.aliases,
                "product_type_aliases": registry.product_type_aliases.aliases,
                "package_size_aliases": registry.package_size_aliases.aliases,
            });

            println!("{}", serde_json::to_string_pretty(&export)?);
        }
    }

    Ok(())
}

fn reject_category_argument(category: Option<String>) -> Result<()> {
    if category.is_some() {
        anyhow::bail!("Only list package-sizes accepts a category argument");
    }

    Ok(())
}

fn reject_package_size_input(input: Option<String>) -> Result<()> {
    if input.is_some() {
        anyhow::bail!("Only normalize package-size accepts a category argument");
    }

    Ok(())
}
