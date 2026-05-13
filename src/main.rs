mod models;
mod normalize;
mod registry;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use normalize::{normalize_category, normalize_weight};
use registry::Registry;

#[derive(Parser)]
#[command(name = "moby-standards")]
#[command(about = "Canonical cannabis data standards for MOBY")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List canonical standards.
    List {
        #[arg(value_enum)]
        kind: ListKind,
    },

    /// Normalize messy cannabis input into MOBY standards.
    Normalize {
        #[arg(value_enum)]
        kind: NormalizeKind,

        /// Raw value to normalize.
        input: String,
    },

    /// Validate standards and aliases.
    Validate,

    /// Export all loaded registries as JSON.
    ExportJson,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ListKind {
    Weights,
    Categories,
    Units,
    ProductTypes,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum NormalizeKind {
    Weight,
    Category,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let registry = Registry::load()?;

    match cli.command {
        Commands::List { kind } => match kind {
            ListKind::Weights => {
                for weight in registry.weights.weights {
                    println!("{} = {}g", weight.label, weight.grams);
                }
            }
            ListKind::Categories => {
                for category in registry.categories.categories {
                    println!("{} - {}", category.key, category.label);
                }
            }
            ListKind::Units => {
                for unit in registry.units.units {
                    println!("{} - {} ({})", unit.key, unit.label, unit.dimension);
                }
            }
            ListKind::ProductTypes => {
                for product_type in registry.product_types.product_types {
                    println!(
                        "{} - {} [{}]",
                        product_type.key, product_type.label, product_type.category
                    );
                }
            }
        },

        Commands::Normalize { kind, input } => {
            let result = match kind {
                NormalizeKind::Weight => normalize_weight(&registry, &input),
                NormalizeKind::Category => normalize_category(&registry, &input),
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
                "weight_aliases": registry.weight_aliases.aliases,
                "category_aliases": registry.category_aliases.aliases,
            });

            println!("{}", serde_json::to_string_pretty(&export)?);
        }
    }

    Ok(())
}
