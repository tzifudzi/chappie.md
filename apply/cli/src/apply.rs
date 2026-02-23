// Apply logic: copies rendered markdown files to target-specific destinations.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::config::{Config, Mapping};

/// Expand a leading `~` in a path to the user's home directory.
///
/// Uses `USERPROFILE` (Windows) or `HOME` (Unix). Returns the path unchanged
/// if it doesn't start with `~` or no home directory is found.
fn expand_tilde(path: &Path) -> PathBuf {
    if !path.starts_with("~") {
        return path.to_path_buf();
    }
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from);
    match home {
        Some(home) => home.join(path.strip_prefix("~").unwrap()),
        None => path.to_path_buf(),
    }
}

/// Apply all mappings for the given environment and targets.
///
/// If `targets` is empty, applies all targets in the environment.
pub fn apply(config: &Config, config_dir: &Path, environment: &str, targets: &[String]) -> Result<()> {
    let env = config
        .environments
        .get(environment)
        .with_context(|| format!("unknown environment: {environment}"))?;

    let output_dir = config_dir.join(&config.output_dir);
    // Expand ~ to home directory, then resolve relative to config_dir.
    let expanded_root = expand_tilde(&env.root);
    let env_root = if expanded_root.is_absolute() {
        expanded_root
    } else {
        config_dir.join(&expanded_root)
    };

    let target_names: Vec<&String> = if targets.is_empty() {
        let mut keys: Vec<_> = env.targets.keys().collect();
        keys.sort();
        keys
    } else {
        // Validate all target names exist before applying any.
        for name in targets {
            if !env.targets.contains_key(name) {
                let known: Vec<_> = env.targets.keys().map(String::as_str).collect();
                bail!("unknown target '{name}' in environment '{environment}'. known: {known:?}");
            }
        }
        targets.iter().collect()
    };

    for name in &target_names {
        let target = &env.targets[name.as_str()];
        println!("  {name}");
        for mapping in &target.mappings {
            apply_mapping(&output_dir, &env_root, mapping)
                .with_context(|| format!("target '{name}': {} → {}", mapping.source.display(), mapping.dest.display()))?;
        }
    }

    Ok(())
}

fn apply_mapping(output_dir: &Path, env_root: &Path, mapping: &Mapping) -> Result<()> {
    let src = output_dir.join(&mapping.source);
    let dest = env_root.join(&mapping.dest);

    if !src.exists() {
        bail!(
            "source file not found: {}. Did you run 'make render' first?",
            src.display()
        );
    }

    // Ensure parent directories exist.
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating directory {}", parent.display()))?;
    }

    std::fs::copy(&src, &dest)
        .with_context(|| format!("copying {} → {}", src.display(), dest.display()))?;

    println!("    {} → {}", mapping.source.display(), mapping.dest.display());
    Ok(())
}

#[cfg(test)]
#[path = "apply_test.rs"]
mod tests;
