// Chappie CLI: applies rendered agent markdown to tool-specific locations.

mod apply;
mod config;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use config::Config;

/// Apply rendered agent markdown files to tool-specific locations.
#[derive(Parser)]
#[command(name = "chappie", version)]
struct Cli {
    /// Environment to apply to (e.g., "local").
    environment: String,

    /// Specific targets to apply. If omitted, applies all targets in the environment.
    #[arg(short, long)]
    target: Vec<String>,

    /// Path to the config file.
    #[arg(short, long, default_value = "chappie.toml")]
    config: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = cli.config.canonicalize().with_context(|| {
        format!(
            "config file not found: {}",
            cli.config.display()
        )
    })?;
    let config_dir = config_path
        .parent()
        .expect("config file must have a parent directory");

    let config = Config::load(&config_path)?;

    println!("Applying environment '{}':", cli.environment);
    apply::apply(&config, config_dir, &cli.environment, &cli.target)?;
    println!("Done.");

    Ok(())
}
